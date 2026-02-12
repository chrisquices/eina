use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct PM2Process {
    pm_id: u32,
    name: String,
    status: String,
    cpu: String,
    memory: String,
    uptime: String,
    restarts: u32,
}

fn find_pm2() -> Result<(String, String), String> {
    let home = std::env::var("HOME").unwrap_or_default();
    let pm2_paths = vec![
        ("pm2".to_string(), "/usr/local/bin:/usr/bin:/bin".to_string()),
        ("/usr/local/bin/pm2".to_string(), "/usr/local/bin:/usr/bin:/bin".to_string()),
        ("/opt/homebrew/bin/pm2".to_string(), "/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin".to_string()),
        (
            format!("{}/Library/Application Support/Herd/config/nvm/versions/node/v22.21.1/bin/pm2", home),
            format!("{}/Library/Application Support/Herd/config/nvm/versions/node/v22.21.1/bin:/usr/local/bin:/usr/bin:/bin", home)
        ),
        (format!("{}/.npm-global/bin/pm2", home), format!("{}/.npm-global/bin:/usr/local/bin:/usr/bin:/bin", home)),
    ];

    for (pm2_path, path_env) in &pm2_paths {
        let mut cmd = Command::new(pm2_path);
        cmd.env("PATH", path_env);
        if cmd.arg("--version").output().is_ok() {
            return Ok((pm2_path.clone(), path_env.clone()));
        }
    }

    Err("PM2 not found".to_string())
}

#[tauri::command]
fn check_pm2_installed() -> Result<bool, String> {
    Ok(find_pm2().is_ok())
}

#[tauri::command]
fn get_pm2_list() -> Result<Vec<PM2Process>, String> {
    let (pm2_path, path_env) = find_pm2()?;

    let output = Command::new(&pm2_path)
        .env("PATH", &path_env)
        .args(["list"])
        .output()
        .map_err(|e| format!("Failed to execute pm2: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let jlist_output = Command::new(&pm2_path)
        .env("PATH", &path_env)
        .args(["jlist"])
        .output()
        .map_err(|e| format!("Failed to execute pm2 jlist: {}", e))?;

    let output_str = String::from_utf8_lossy(&jlist_output.stdout);

    // Find JSON array in output (skip any header text)
    let json_start = output_str.find('[').ok_or("No JSON array found in pm2 output")?;
    let json_str = &output_str[json_start..];

    let pm2_data: Vec<serde_json::Value> = serde_json::from_str(json_str)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let processes = pm2_data.iter().map(|p| {
        PM2Process {
            pm_id: p["pm_id"].as_u64().unwrap_or(0) as u32,
            name: p["name"].as_str().unwrap_or("").to_string(),
            status: p["pm2_env"]["status"].as_str().unwrap_or("unknown").to_string(),
            cpu: format!("{}%", p["monit"]["cpu"].as_f64().unwrap_or(0.0)),
            memory: format!("{} MB", p["monit"]["memory"].as_u64().unwrap_or(0) / 1024 / 1024),
            uptime: calculate_uptime(p["pm2_env"]["pm_uptime"].as_u64().unwrap_or(0)),
            restarts: p["pm2_env"]["restart_time"].as_u64().unwrap_or(0) as u32,
        }
    }).collect();

    Ok(processes)
}

#[tauri::command]
fn pm2_start(name: String) -> Result<String, String> {
    execute_pm2_command(vec!["start", &name])
}

#[tauri::command]
fn pm2_stop(name: String) -> Result<String, String> {
    execute_pm2_command(vec!["stop", &name])
}

#[tauri::command]
fn pm2_restart(name: String) -> Result<String, String> {
    execute_pm2_command(vec!["restart", &name])
}

#[tauri::command]
fn pm2_delete(name: String) -> Result<String, String> {
    execute_pm2_command(vec!["delete", &name])
}

#[tauri::command]
fn pm2_logs(name: String, _lines: Option<u32>) -> Result<String, String> {
    let home = std::env::var("HOME").unwrap_or_default();
    let out_log = format!("{}/.pm2/logs/{}-out.log", home, name);
    let err_log = format!("{}/.pm2/logs/{}-error.log", home, name);

    let out_content = std::fs::read_to_string(&out_log).unwrap_or_default();
    let err_content = std::fs::read_to_string(&err_log).unwrap_or_default();

    Ok(format!("{}\n{}", err_content, out_content))
}

fn execute_pm2_command(args: Vec<&str>) -> Result<String, String> {
    let (pm2_path, path_env) = find_pm2()?;

    let output = Command::new(&pm2_path)
        .env("PATH", path_env)
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to execute pm2: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn calculate_uptime(timestamp: u64) -> String {
    if timestamp == 0 {
        return "0s".to_string();
    }

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let uptime_secs = now - (timestamp / 1000);
    let days = uptime_secs / 86400;
    let hours = (uptime_secs % 86400) / 3600;

    if days > 0 {
        format!("{}d {}h", days, hours)
    } else if hours > 0 {
        format!("{}h", hours)
    } else {
        format!("{}s", uptime_secs)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            check_pm2_installed,
            get_pm2_list,
            pm2_start,
            pm2_stop,
            pm2_restart,
            pm2_delete,
            pm2_logs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}