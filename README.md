<br />
<div align="center">
    <img src="./resources/eina-logo.svg" alt="Eina Logo" height="100">
</div>

<h1 align="center">Eina</h1>

<p align="center">
    Desktop PM2 process manager for macOS. Monitor, control, and debug your Node.js processes with real-time updates and beautiful ANSI color logs.<br />
    <br />
    <br />
    <a href="https://tauri.app">
        <img height="32" src="https://img.shields.io/badge/Tauri-FFC131?style=for-the-badge&logo=tauri&logoColor=white" alt="Tauri Badge" />
    </a>
    <a href="https://www.rust-lang.org/">
        <img height="32" src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust Badge" />
    </a>
    <a href="https://kit.svelte.dev/">
        <img height="32" src="https://img.shields.io/badge/SvelteKit-FF3E00?style=for-the-badge&logo=svelte&logoColor=white" alt="SvelteKit Badge" />
    </a>
    <a href="https://www.typescriptlang.org/">
        <img height="32" src="https://img.shields.io/badge/TypeScript-3178C6?style=for-the-badge&logo=typescript&logoColor=white" alt="TypeScript Badge" />
    </a>
    <a href="https://tailwindcss.com/">
        <img height="32" src="https://img.shields.io/badge/Tailwind_CSS-06B6D4?style=for-the-badge&logo=tailwindcss&logoColor=white" alt="Tailwind CSS Badge" />
    </a>
</p>

---

## ⚙️ Prerequisites

Before installing, make sure you have the following:

- **macOS** (Apple Silicon or Intel)
- **PM2** installed globally: `npm install -g pm2`
- **Node.js** `^18.x` (for building from source)
- **Rust** `^1.70` (for building from source)

---

## 🚀 Installation

### Option 1: Download Binary

Download the latest `.app` or `.dmg` from [Releases](https://github.com/yourusername/eina/releases)

### Option 2: Build from Source

1. **Clone the repository**
```bash
   git clone https://github.com/yourusername/eina.git
   cd eina
```

2. **Install dependencies**
```bash
   npm install
```

3. **Build the application**
```bash
   npm run tauri build
```

4. **Locate the built app**
```
   src-tauri/target/release/bundle/macos/eina.app
```

---

## 💻 Usage

1. **Launch Eina**

2. **View your PM2 processes**
    - All running PM2 processes appear in the table
    - Real-time updates every 2 seconds

3. **Manage processes**
    - Click any process to view details and logs
    - Use control buttons:
        - ▶️ **Start** - Start a stopped process
        - ⏸️ **Stop** - Stop a running process
        - 🔄 **Restart** - Restart a process
        - 🗑️ **Delete** - Remove a process from PM2

4. **View logs**
    - Full ANSI color support
    - Automatically loads last 100 lines
    - Updates when you select a different process

---

## ✨ Features

- **Process Management** - Start, stop, restart, and delete PM2 processes with a single click
- **Real-time Monitoring** - Auto-refreshing process list with CPU, memory, uptime, and restart count
- **Log Viewer** - View process logs with full ANSI color support directly from PM2 log files
- **Desktop Notifications** - Get notified when processes crash or restart unexpectedly
- **Clean UI** - Dark mode interface built with Tailwind CSS and shadcn/ui components
- **Lightweight** - Native desktop app with minimal resource usage

---

## 🎯 Part of DanMachi Infrastructure Suite

- **Eina** - PM2 process manager (this project)
- **Hestia** - All-in-one personal management platform
- **Lili** - Chrome extension companion

---

## 🧑‍💻 Development

To run in development mode:
```bash
npm install
npm run tauri dev
```

To build for production:
```bash
npm run tauri build
```

---

## 🤝 Contributing

Contributions are welcome!  
Please fork the repository and create a pull request, or open an issue for discussion before making changes.

---

## 📄 License

This project is open-sourced under the [MIT License](LICENSE).

---