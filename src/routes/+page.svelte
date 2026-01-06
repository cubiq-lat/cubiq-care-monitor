<script lang="ts">
    import { TriangleAlert } from "@lucide/svelte";
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
        { date: "15/12/2025 10:20", completed: true, type: "Backup de Dados", status: "SUCESSO" },
        { date: "14/12/2025 16:30", completed: false, type: "Diagnóstico de Hardware", status: "PENDENTE" },
        { date: "13/12/2025 09:15", completed: true, type: "Limpeza Física", status: "SUCESSO" },
        { date: "12/12/2025 14:00", completed: true, type: "Atualização de Drivers", status: "SUCESSO" },
        { date: "11/12/2025 11:45", completed: true, type: "Verificação de Segurança", status: "SUCESSO" },
    ]);

    async function greet(event: Event) {
        event.preventDefault();
        // greetMsg = await invoke("greet", { name });
    }

    async function fetchReport() {
        try {
            // Simulando dados para o exemplo
            report = {
                os_name: "Ubuntu",
                os_version: "22.04 LTS",
                kernel_version: "5.15.0-91-generic",
                host_name: "cubiq-workstation-01",
                cpu_arch: "x86_64",
                hardware: {
                    product_name: "ThinkPad X1 Carbon Gen 9",
                    manufacturer: "Lenovo",
                    serial_number: "PF3K8X2Y",
                    uuid: "4c4c4544-0050-4610-8033-b8c04f4b3832",
                },
            };
        } catch (err) {
            console.error("Error:", err);
        }
    }

    onMount(() => {
        fetchReport();
    });
</script>

<div class="min-h-screen h-screen bg-black text-white flex flex-col overflow-hidden">
    <!-- Header -->
    <div class="flex items-center justify-between px-6 py-4 border-b border-zinc-800 flex-shrink-0">
        <!-- Right -->
        <div class="flex items-center justify-center gap-4">
            <img src="/images/logos/cubiq-logo-white.svg" alt="cubiq-logo" class="h-8 relative -top-[1px]" />
            <span class="text-zinc-500">></span>
            <span class="text-zinc-400 text-lg">Dashboard</span>
        </div>

        <!-- Left -->
        <div class="flex items-center gap-3">
            <span class="text-sm text-zinc-500">STATUS</span>
            <button
                class="px-6 py-1.5 bg-green-600 text-white rounded-full text-sm font-semibold hover:bg-green-700 transition"
            >
                BOM
            </button>
            <button class="p-1">
                <div class="w-8 h-8 rounded-full bg-green-600 flex items-center justify-center text-white font-bold">
                    S
                </div>
            </button>
        </div>
    </div>

    <div class="grid grid-cols-2 flex-1 overflow-hidden">
        <!-- Left Panel -->
        <div class="border-r border-zinc-800 flex flex-col overflow-hidden">
            <!-- Basic Data Section - Fixed -->
            <div class="p-8 border-b border-zinc-800 flex-shrink-0">
                <div class="flex items-center justify-between mb-8">
                    <h2 class="text-lg font-bold text-red-600">DADOS BÁSICOS</h2>
                    <button
                        class="flex items-center gap-2 px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition text-sm font-semibold"
                    >
                        <TriangleAlert class="h-4" />
                        REPORTAR PROBLEMA
                    </button>
                </div>

                {#if report}
                    <div class="flex gap-10 justify-start">
                        <div>
                            <div class="text-sm text-zinc-500 mb-1">PRÓXIMA PREVENTIVA</div>
                            <div class="text-xl font-bold mb-1">Em 5 días</div>
                            <button class="text-red-600 hover:text-red-500 text-sm font-medium">Re-Agendar</button>
                        </div>

                        <div>
                            <div class="text-sm text-zinc-500 mb-1">ORGANIZAÇÃO</div>
                            <div class="text-lg">Cubiq</div>
                        </div>

                        <div>
                            <div class="text-sm text-zinc-500 mb-1">COLABORADOR</div>
                            <div class="text-lg">Pepe Gonzáles</div>
                        </div>
                    </div>
                {:else}
                    <div class="animate-pulse space-y-4">
                        <div class="h-8 bg-zinc-800 rounded w-1/2"></div>
                        <div class="h-4 bg-zinc-800 rounded w-3/4"></div>
                    </div>
                {/if}
            </div>

            <!-- Equipment Data Section - Scrollable -->
            <div
                class="flex-1 overflow-y-scroll p-8 [&::-webkit-scrollbar]:w-2 [&::-webkit-scrollbar-track]:bg-zinc-900 [&::-webkit-scrollbar-thumb]:bg-zinc-700 [&::-webkit-scrollbar-thumb]:rounded [&::-webkit-scrollbar-thumb:hover]:bg-zinc-600"
            >
                <h2 class="text-lg font-bold text-red-600 mb-8">DADOS DO EQUIPAMENTO</h2>

                {#if report}
                    <div class="space-y-6">
                        <div>
                            <div class="text-sm text-zinc-500 mb-1">NÚMERO DE SERIE</div>
                            <div class="text-lg font-mono">{report.hardware.serial_number}</div>
                        </div>

                        <div>
                            <div class="text-sm text-zinc-500 mb-1">MODELO</div>
                            <div class="text-lg">{report.hardware.product_name}</div>
                        </div>

                        <div>
                            <div class="text-sm text-zinc-500 mb-1">FABRICANTE</div>
                            <div class="text-lg">{report.hardware.manufacturer}</div>
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
        <div class="flex flex-col overflow-hidden">
            <!-- Header - Fixed -->
            <div class="p-8 pb-4 flex-shrink-0">
                <div class="flex items-center justify-between mb-6">
                    <h2 class="text-lg font-bold text-red-600">HISTÓRICO DE SERVIÇOS</h2>
                    <div class="flex items-center gap-2">
                        <div class="px-3 py-1 border border-zinc-700 rounded text-sm">6 aparelhos</div>
                        <button class="px-4 py-2 hover:bg-zinc-800 rounded transition text-sm">VER TODOS</button>
                    </div>
                </div>
            </div>

            <!-- Table - Scrollable -->
            <div
                class="flex-1 overflow-y-scroll px-8 pb-8 [&::-webkit-scrollbar]:w-2 [&::-webkit-scrollbar-track]:bg-zinc-900 [&::-webkit-scrollbar-thumb]:bg-zinc-700 [&::-webkit-scrollbar-thumb]:rounded [&::-webkit-scrollbar-thumb:hover]:bg-zinc-600"
            >
                <table class="w-full">
                    <thead class="sticky top-0 bg-black">
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
