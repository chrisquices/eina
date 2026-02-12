<script lang="ts">
    import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';
    import {Button} from "$lib/components/ui/button";
    import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "$lib/components/ui/table";
    import {Badge} from "$lib/components/ui/badge";
    import * as Tooltip from "$lib/components/ui/tooltip";
    import Convert from 'ansi-to-html';

    import PlayIcon from "@lucide/svelte/icons/play";
    import Trash2Icon from "@lucide/svelte/icons/trash-2";
    import PauseIcon from "@lucide/svelte/icons/pause";
    import RotateCcwIcon from "@lucide/svelte/icons/rotate-ccw";
    import LoaderCircleIcon from "@lucide/svelte/icons/loader-circle";
    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";

    interface PM2Process {
        pm_id: number;
        name: string;
        status: string;
        cpu: string;
        memory: string;
        uptime: string;
        restarts: number;
    }

    let pm2Installed = $state(false);
    let processes = $state<PM2Process[]>([]);
    let selectedProcess = $state<number | null>(null);
    let logs = $state<string>("");
    let loading = $state(false);
    let error = $state<string | null>(null);
    let actionLoading = $state<string | null>(null); // 'start', 'stop', 'restart', 'delete'

    async function checkPM2() {
        try {
            pm2Installed = await invoke<boolean>("check_pm2_installed");
        } catch (e) {
            console.error("Failed to check PM2:", e);
        }
    }

    async function loadProcesses(isRefresh = false) {
        try {
            if (!isRefresh) {
                loading = true;
            }
            error = null;
            const newProcesses = await invoke<PM2Process[]>("get_pm2_list");

            // Check for status changes
            if (isRefresh && previousProcessStates.size > 0) {
                for (const process of newProcesses) {
                    const oldStatus = previousProcessStates.get(process.name);
                    if (oldStatus && oldStatus !== process.status) {
                        await notifyProcessChange(process.name, oldStatus, process.status);
                    }
                }
            }

            // Update states
            previousProcessStates = new Map(newProcesses.map(p => [p.name, p.status]));
            processes = newProcesses;
        } catch (e) {
            error = e as string;
            console.error("Failed to load processes:", e);
        } finally {
            if (!isRefresh) {
                loading = false;
            }
        }
    }

    const convert = new Convert();

    async function loadLogs(name: string) {
        try {
            const rawLogs = await invoke<string>("pm2_logs", {name, lines: 100});
            logs = convert.toHtml(rawLogs);
        } catch (e) {
            console.error("Failed to load logs:", e);
        }
    }

    async function startProcess(name: string) {
        try {
            actionLoading = 'start';
            await new Promise(resolve => setTimeout(resolve, 1000));
            await invoke("pm2_start", {name});
            await loadProcesses();
            if (selectedProcess !== null) {
                await loadLogs(processes[selectedProcess].name);
            }
        } catch (e) {
            console.error("Failed to start process:", e);
        } finally {
            actionLoading = null;
        }
    }

    async function stopProcess(name: string) {
        try {
            actionLoading = 'stop';
            await new Promise(resolve => setTimeout(resolve, 1000));
            await invoke("pm2_stop", {name});
            await loadProcesses();
            if (selectedProcess !== null) {
                await loadLogs(processes[selectedProcess].name);
            }
        } catch (e) {
            console.error("Failed to stop process:", e);
        } finally {
            actionLoading = null;
        }
    }

    async function restartProcess(name: string) {
        try {
            actionLoading = 'restart';
            await new Promise(resolve => setTimeout(resolve, 1000));
            await invoke("pm2_restart", {name});
            await loadProcesses();
            if (selectedProcess !== null) {
                await loadLogs(processes[selectedProcess].name);
            }
        } catch (e) {
            console.error("Failed to restart process:", e);
        } finally {
            actionLoading = null;
        }
    }

    async function deleteProcess(name: string) {
        try {
            actionLoading = 'delete';
            await new Promise(resolve => setTimeout(resolve, 1000));
            await invoke("pm2_delete", {name});
            selectedProcess = null;
            logs = "";
            await loadProcesses();
        } catch (e) {
            console.error("Failed to delete process:", e);
        } finally {
            actionLoading = null;
        }
    }

    function selectProcess(index: number) {
        selectedProcess = index;
        loadLogs(processes[index].name);
    }

    onMount(async () => {
        await checkPM2();
        if (pm2Installed) {
            await loadProcesses();
            const interval = setInterval(() => loadProcesses(true), 2000);
            return () => clearInterval(interval);
        }
    });

    let previousProcessStates = $state<Map<string, string>>(new Map());

    async function checkNotificationPermission() {
        let permissionGranted = await isPermissionGranted();
        if (!permissionGranted) {
            const permission = await requestPermission();
            permissionGranted = permission === 'granted';
        }
        return permissionGranted;
    }

    async function notifyProcessChange(name: string, oldStatus: string, newStatus: string) {
        const hasPermission = await checkNotificationPermission();
        if (!hasPermission) return;

        if (newStatus === 'stopped' || newStatus === 'errored') {
            await sendNotification({
                title: `PM2: ${name}`,
                body: `Process ${oldStatus} → ${newStatus}`,
            });
        }
    }

</script>

<Tooltip.Provider>
    <div class="h-screen font-mono !text-sm">

        <main class="container mx-auto p-8 flex flex-col gap-6 h-full">

            {#if !pm2Installed}
                <div class="bg-card/60 p-8 rounded-xl flex flex-col items-center justify-center gap-6 h-full">
                    <div class="text-center">
                        <h2 class="text-2xl font-bold mb-2">PM2 Not Installed</h2>
                        <p class="text-muted-foreground mb-4">PM2 is required to manage processes.</p>
                        <p class="text-muted-foreground">Install it by running:</p>
                        <code class="block bg-muted p-4 rounded mt-2 font-mono">npm install -g pm2</code>
                    </div>
                </div>
            {:else}
                <!-- Process Table -->
                <div class="bg-card/60 p-8 rounded-xl overflow-hidden flex-shrink-0">
                    {#if error}
                        <div class="text-destructive mb-4">Error: {error}</div>
                    {/if}

                    {#if loading && processes.length === 0}
                        <div class="text-muted-foreground">Loading processes...</div>
                    {:else if processes.length === 0}
                        <div class="text-muted-foreground text-center py-8">No PM2 processes found. Start a process
                            to see it here.
                        </div>
                    {:else}
                        <Table>
                            <TableHeader>
                                <TableRow class="h-16">
                                    <TableHead>Name</TableHead>
                                    <TableHead>Status</TableHead>
                                    <TableHead>CPU</TableHead>
                                    <TableHead>Memory</TableHead>
                                    <TableHead>Uptime</TableHead>
                                    <TableHead>Restarts</TableHead>
                                </TableRow>
                            </TableHeader>
                            <TableBody>
                                {#each processes as process, index}
                                    <TableRow
                                            class="h-16 even:bg-primary/5 !rounded-xl cursor-pointer {selectedProcess === index ? 'bg-muted' : ''}"
                                            onclick={() => selectProcess(index)}
                                    >
                                        <TableCell class="font-medium">{process.name}</TableCell>
                                        <TableCell>
                                            <Badge variant={process.status === 'online' ? 'success' : 'default'}>
                                                {process.status}
                                            </Badge>
                                        </TableCell>
                                        <TableCell>{process.cpu}</TableCell>
                                        <TableCell>{process.memory}</TableCell>
                                        <TableCell>{process.uptime}</TableCell>
                                        <TableCell>{process.restarts}</TableCell>
                                    </TableRow>
                                {/each}
                            </TableBody>
                        </Table>
                    {/if}
                </div>

                <!-- Selected Process Details -->
                {#if selectedProcess !== null && processes[selectedProcess]}
                    <div class="bg-card/60 flex-1 rounded-xl p-8 flex flex-col gap-8 flex-grow min-h-0">
                        <div class="flex justify-between items-center">
                            <h2 class="text-xl font-semibold">{processes[selectedProcess].name}</h2>
                            <div class="flex gap-4">
                                <Tooltip.Root>
                                    <Tooltip.Trigger>
                                        <Button
                                                variant="secondary"
                                                size="icon-lg"
                                                onclick={() => startProcess(processes[selectedProcess!].name)}
                                                disabled={processes[selectedProcess].status === 'online' || actionLoading !== null}
                                        >
                                            {#if actionLoading === 'start'}
                                                <LoaderCircleIcon class="animate-spin"/>
                                            {:else}
                                                <PlayIcon/>
                                            {/if}
                                        </Button>
                                    </Tooltip.Trigger>
                                    <Tooltip.Content>
                                        <p>Start Process</p>
                                    </Tooltip.Content>
                                </Tooltip.Root>

                                <Tooltip.Root>
                                    <Tooltip.Trigger>
                                        <Button
                                                variant="secondary"
                                                size="icon-lg"
                                                onclick={() => stopProcess(processes[selectedProcess!].name)}
                                                disabled={processes[selectedProcess].status !== 'online' || actionLoading !== null}
                                        >
                                            {#if actionLoading === 'stop'}
                                                <LoaderCircleIcon class="animate-spin"/>
                                            {:else}
                                                <PauseIcon/>
                                            {/if}
                                        </Button>
                                    </Tooltip.Trigger>
                                    <Tooltip.Content>
                                        <p>Stop Process</p>
                                    </Tooltip.Content>
                                </Tooltip.Root>

                                <Tooltip.Root>
                                    <Tooltip.Trigger>
                                        <Button
                                                variant="secondary"
                                                size="icon-lg"
                                                onclick={() => restartProcess(processes[selectedProcess!].name)}
                                                disabled={actionLoading !== null}
                                        >
                                            {#if actionLoading === 'restart'}
                                                <LoaderCircleIcon class="animate-spin"/>
                                            {:else}
                                                <RotateCcwIcon/>
                                            {/if}
                                        </Button>
                                    </Tooltip.Trigger>
                                    <Tooltip.Content>
                                        <p>Restart Process</p>
                                    </Tooltip.Content>
                                </Tooltip.Root>

                                <Tooltip.Root>
                                    <Tooltip.Trigger>
                                        <Button
                                                variant="secondary"
                                                size="icon-lg"
                                                onclick={() => deleteProcess(processes[selectedProcess!].name)}
                                                disabled={actionLoading !== null}
                                        >
                                            {#if actionLoading === 'delete'}
                                                <LoaderCircleIcon class="animate-spin"/>
                                            {:else}
                                                <Trash2Icon/>
                                            {/if}
                                        </Button>
                                    </Tooltip.Trigger>
                                    <Tooltip.Content>
                                        <p>Delete Process</p>
                                    </Tooltip.Content>
                                </Tooltip.Root>
                            </div>
                        </div>

                        <div class="flex flex-wrap gap-8">
                            <div>
                                <span class="text-muted-foreground">ID:</span>
                                <span class=" font-mono">{processes[selectedProcess].pm_id}</span>
                            </div>
                            <div>
                                <span class="text-muted-foreground">Restarts:</span>
                                <span class="">{processes[selectedProcess].restarts}</span>
                            </div>
                            <div>
                                <span class="text-muted-foreground">Uptime:</span>
                                <span class="">{processes[selectedProcess].uptime}</span>
                            </div>
                        </div>

                        <!-- Log Viewer -->
                        <div class="flex-1 rounded overflow-y-auto whitespace-pre-wrap">
                            {@html logs || "No logs available"}
                        </div>
                    </div>
                {:else}
                    <div class="bg-card/60 flex-1 rounded-xl flex items-center justify-center text-muted-foreground">
                        Select a process to view details
                    </div>
                {/if}
            {/if}
        </main>

    </div>
</Tooltip.Provider>

