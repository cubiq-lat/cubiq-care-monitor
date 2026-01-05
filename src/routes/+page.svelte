<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    interface DmiReport {
        os_name: string;
        os_version: string;
        kernel_version: string;
        host_name: string;
        cpu_arch: string;
        hardware: {
            product_name: string;
            manufacturer: string;
            serial_number: string;
            uuid: string;
        };
    }

    interface ServiceRecord {
        date: string;
        completed: boolean;
        type: string;
        status: "SUCESSO" | "PENDENTE" | "FALHA";
    }

    let name = $state("");
    let greetMsg = $state("");
    let report = $state<DmiReport | null>(null);

    let serviceHistory = $state<ServiceRecord[]>([
        { date: "16/12/2025 14:53", completed: true, type: "Instalação/Atualização de Software", status: "SUCESSO" },
        { date: "16/12/2025 14:51", completed: true, type: "Manutenção Agendada", status: "SUCESSO" },
        { date: "16/12/2025 14:51", completed: true, type: "Manutenção Agendada", status: "SUCESSO" },
    ]);

    async function greet(event: Event) {
        event.preventDefault();
        greetMsg = await invoke("greet", { name });
    }

    async function fetchReport() {
        try {
            report = await invoke("get_dmi_report");
        } catch (err) {
            console.error("Rust Error:", err);
        }
    }

    onMount(() => {
        fetchReport();
    });
</script>

<div class="min-h-screen bg-black text-white">
    <!-- Header -->
    <div class="flex items-center justify-between px-6 py-4 border-b border-zinc-800">
        <div class="flex items-center gap-4">
            <div class="flex items-center gap-2">
                <div class="w-8 h-8 bg-red-600 rounded flex items-center justify-center">
                    <span class="text-white font-bold text-xl">✕</span>
                </div>
                <span class="text-2xl font-bold">Cubiq</span>
            </div>
            <span class="text-zinc-500">></span>
            <span class="text-zinc-400">Care Dashboard</span>
        </div>
        <div class="flex items-center gap-3">
            <span class="text-sm text-zinc-500">STATUS</span>
            <button
                class="px-6 py-1.5 bg-green-600 text-white rounded-full text-sm font-semibold hover:bg-green-700 transition"
            >
                BOM
            </button>
            <button class="p-2 hover:bg-zinc-800 rounded transition">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"
                    />
                </svg>
            </button>
            <button class="p-1">
                <div class="w-8 h-8 rounded-full bg-green-600 flex items-center justify-center text-white font-bold">
                    S
                </div>
            </button>
        </div>
    </div>

    <div class="grid grid-cols-2 min-h-[calc(100vh-64px)]">
        <!-- Left Panel -->
        <div class="border-r border-zinc-800">
            <!-- Basic Data Section -->
            <div class="p-8 border-b border-zinc-800">
                <div class="flex items-center justify-between mb-8">
                    <h2 class="text-xl font-bold text-red-600">DADOS BÁSICOS</h2>
                    <button
                        class="flex items-center gap-2 px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition text-sm font-semibold"
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                            />
                        </svg>
                        REPORTAR PROBLEMA
                    </button>
                </div>

                {#if report}
                    <div class="space-y-6">
                        <div>
                            <div class="text-sm text-zinc-500 mb-1">PRÓXIMA PREVENTIVA</div>
                            <div class="text-2xl font-bold mb-1">Em 5 días</div>
                            <button class="text-red-600 hover:text-red-500 text-sm font-medium">Re-Agendar</button>
                        </div>

                        <div>
                            <div class="text-sm text-zinc-500 mb-1">ORGANIZAÇÃO</div>
                            <div class="text-xl">Cubiq</div>
                        </div>

                        <div>
                            <div class="text-sm text-zinc-500 mb-1">COLABORADOR</div>
                            <div class="text-xl">Pepe Gonzáles</div>
                        </div>
                    </div>
                {:else}
                    <div class="animate-pulse space-y-4">
                        <div class="h-8 bg-zinc-800 rounded w-1/2"></div>
                        <div class="h-4 bg-zinc-800 rounded w-3/4"></div>
                    </div>
                {/if}
            </div>

            <!-- Equipment Data Section -->
            <div class="p-8">
                <h2 class="text-xl font-bold text-red-600 mb-8">DADOS DO EQUIPAMENTO</h2>

                {#if report}
                    <div class="space-y-6">
                        <div>
                            <div class="text-sm text-zinc-500 mb-1">NÚMERO DE SERIE</div>
                            <div class="text-xl font-mono">{report.hardware.serial_number}</div>
                        </div>

                        <div>
                            <div class="text-sm text-zinc-500 mb-1">MODELO</div>
                            <div class="text-xl">{report.hardware.product_name}</div>
                        </div>

                        <div>
                            <div class="text-sm text-zinc-500 mb-1">FABRICANTE</div>
                            <div class="text-xl">{report.hardware.manufacturer}</div>
                        </div>

                        <div>
                            <div class="text-sm text-zinc-500 mb-1">UUID</div>
                            <div class="text-xs font-mono text-zinc-400 break-all uppercase">
                                {report.hardware.uuid}
                            </div>
                        </div>

                        <div class="pt-4">
                            <div class="text-sm text-zinc-500 mb-1">SISTEMA OPERACIONAL</div>
                            <div class="text-lg">{report.os_name} {report.os_version}</div>
                        </div>

                        <div>
                            <div class="text-sm text-zinc-500 mb-1">ARQUITETURA</div>
                            <div class="text-lg">{report.cpu_arch}</div>
                        </div>

                        <div>
                            <div class="text-sm text-zinc-500 mb-1">KERNEL</div>
                            <div class="text-lg font-mono">{report.kernel_version}</div>
                        </div>

                        <div>
                            <div class="text-sm text-zinc-500 mb-1">HOSTNAME</div>
                            <div class="text-lg">{report.host_name}</div>
                        </div>

                        <div>
                            <div class="text-sm text-zinc-500 mb-1">QR CODE</div>
                            <div class="mt-4 w-32 h-32 bg-white rounded p-2">
                                <svg viewBox="0 0 100 100" class="w-full h-full">
                                    <rect width="100" height="100" fill="white" />
                                    <rect x="10" y="10" width="20" height="20" fill="black" />
                                    <rect x="70" y="10" width="20" height="20" fill="black" />
                                    <rect x="10" y="70" width="20" height="20" fill="black" />
                                    <rect x="40" y="40" width="20" height="20" fill="black" />
                                    {#each Array(15) as _, i}
                                        <rect
                                            x={(i * 6 + 5) % 90}
                                            y={(i * 7 + 5) % 90}
                                            width="4"
                                            height="4"
                                            fill="black"
                                        />
                                    {/each}
                                </svg>
                            </div>
                        </div>
                    </div>
                {:else}
                    <div class="animate-pulse space-y-4">
                        <div class="h-4 bg-zinc-800 rounded w-2/3"></div>
                        <div class="h-4 bg-zinc-800 rounded w-1/2"></div>
                    </div>
                {/if}
            </div>
        </div>

        <!-- Right Panel - Service History -->
        <div class="p-8">
            <div class="flex items-center justify-between mb-6">
                <h2 class="text-xl font-bold text-red-600">HISTÓRICO DE SERVIÇOS</h2>
                <div class="flex items-center gap-2">
                    <div class="px-3 py-1 border border-zinc-700 rounded text-sm">6 aparelhos</div>
                    <button class="px-4 py-2 hover:bg-zinc-800 rounded transition text-sm">VER TODOS</button>
                </div>
            </div>

            <div class="overflow-x-auto">
                <table class="w-full">
                    <thead>
                        <tr class="border-b border-zinc-800">
                            <th class="text-left py-3 px-4 text-zinc-400 font-normal text-sm">Data</th>
                            <th class="text-left py-3 px-4 text-zinc-400 font-normal text-sm">Finalizado</th>
                            <th class="text-left py-3 px-4 text-zinc-400 font-normal text-sm">Tipo de serviço</th>
                            <th class="text-left py-3 px-4 text-zinc-400 font-normal text-sm">Status</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each serviceHistory as service}
                            <tr class="border-b border-zinc-800 hover:bg-zinc-900/50 transition">
                                <td class="py-3 px-4 text-sm">{service.date}</td>
                                <td class="py-3 px-4">
                                    {#if service.completed}
                                        <span class="text-green-500">✓ Sim</span>
                                    {:else}
                                        <span class="text-zinc-500">—</span>
                                    {/if}
                                </td>
                                <td class="py-3 px-4 text-sm">{service.type}</td>
                                <td class="py-3 px-4">
                                    <span
                                        class="inline-block px-3 py-1 bg-green-600 text-white rounded text-xs font-semibold"
                                    >
                                        {service.status}
                                    </span>
                                </td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
        </div>
    </div>
</div>
