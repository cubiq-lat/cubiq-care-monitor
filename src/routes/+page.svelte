<script lang="ts">
    import ConfigButton from "$lib/components/buttons/ConfigButton.svelte";
    import LanguageButton from "$lib/components/buttons/LanguageButton.svelte";
    import { m } from "$lib/paraglide/messages";
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
        { date: "15/12/2025 10:20", completed: true, type: "Backup de Dados", status: "SUCESSO" },
        { date: "14/12/2025 16:30", completed: false, type: "Diagnóstico de Hardware", status: "PENDENTE" },
        { date: "13/12/2025 09:15", completed: true, type: "Limpeza Física", status: "SUCESSO" },
        { date: "12/12/2025 14:00", completed: true, type: "Atualização de Drivers", status: "SUCESSO" },
        { date: "11/12/2025 11:45", completed: true, type: "Verificação de Segurança", status: "SUCESSO" },
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

<div class="min-h-screen h-screen bg-black text-white flex flex-col overflow-hidden">
    <!-- Header -->
    <div class="flex items-center justify-between px-6 py-4 border-b border-zinc-800 shrink-0">
        <!-- Right -->
        <div class="flex items-center justify-center gap-4">
            <img src="/images/logos/cubiq-logo-white.svg" alt="cubiq-logo" class="h-8 relative -top-px" />
            <span class="text-zinc-500">></span>
            <span class="text-zinc-400 text-lg">{m.dashboard()}</span>
            <!-- <span class="text-zinc-400 text-lg">{m.example_message({ username: "Pepe" })}</span> -->
        </div>

        <!-- Left -->
        <div class="flex items-center gap-3">
            <span class="text-sm text-zinc-500 uppercase">{m.status()}</span>
            <button
                class="px-6 py-3 bg-green-600 text-white rounded-full text-sm font-semibold hover:bg-green-700 transition"
            >
                BOM
            </button>

            <ConfigButton />

            <LanguageButton />
        </div>
    </div>

    <!-- Body -->
    <div class="grid grid-cols-3 flex-1 overflow-hidden">
        <!-- Left Panel -->
        <div class="border-r border-zinc-800 flex flex-col overflow-hidden">
            <!-- Basic Data Section - Fixed -->
            <div class="border-b border-zinc-800 shrink-0">
                <div class="border-b border-zinc-800 px-8 py-5">
                    <div class="flex items-center justify-between">
                        <h2 class="text-lg font-bold text-red-600 uppercase">{m.basic_data()}</h2>
                        <button
                            class="btn btn-sm flex items-center gap-2 px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition text-sm font-semibold"
                        >
                            <!-- <TriangleAlert class="h-4" /> -->
                            {m.report_problem()}
                        </button>
                    </div>
                </div>
                <div class="p-8 pt-5 pb-4">
                    {#if report}
                        <div class="flex gap-10 justify-start">
                            <div>
                                <div class="text-sm text-zinc-500 mb-1 uppercase">{m.next_preventive()}</div>
                                <div class="flex flex-col gap-0 justify-start items-start">
                                    <div class="text-xl font-bold mb-0">Em 5 días</div>
                                    <button class="text-red-600 hover:text-red-500 text-sm font-medium">
                                        {m.reschedule()}
                                    </button>
                                </div>
                            </div>

                            <div>
                                <div class="text-sm text-zinc-500 mb-1 uppercase">{m.organization()}</div>
                                <div class="text-lg">Cubiq</div>
                            </div>
                        </div>
                        <div class="flex gap-10 justify-start">
                            <div class="mt-3">
                                <div class="text-sm text-zinc-500 mb-1 uppercase">{m.contributor()}</div>
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
            </div>

            <!-- Equipment Data Section - Scrollable -->
            <div class="border-b py-4 mb-5 px-8 border-zinc-800">
                <h2 class="text-lg font-bold text-red-600 uppercase">{m.equipment_details()}</h2>
            </div>
            <!-- Actual "table" -->
            <div
                class="flex-1 overflow-y-scroll [&::-webkit-scrollbar]:w-2 [&::-webkit-scrollbar-track]:bg-zinc-900 [&::-webkit-scrollbar-thumb]:bg-zinc-700 [&::-webkit-scrollbar-thumb]:rounded [&::-webkit-scrollbar-thumb:hover]:bg-zinc-600"
            >
                <div class="px-8">
                    {#if report}
                        <div class="space-y-2">
                            <div>
                                <div class="text-sm text-zinc-500 mb-1 uppercase">{m.serial_number()}</div>
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
        </div>

        <!-- Right Panel - Service History -->
        <div class="col-span-2 flex flex-col overflow-hidden">
            <!-- Header - Fixed -->
            <div class="p-8 pt-4 pb-0! shrink-0 border-b border-zinc-800">
                <div class="flex items-center justify-between mb-4">
                    <h2 class="text-lg font-bold text-red-600 uppercase">{m.service_history()}</h2>
                    <div class="flex items-center gap-2">
                        <div class="px-3 py-1 border border-zinc-700 rounded text-sm lowercase">6 {m.services()}</div>
                        <button
                            class="btn btm-sm btn-soft px-4 py-2 hover:bg-zinc-800 rounded transition text-sm uppercase"
                        >
                            {m.see_all()}
                        </button>
                    </div>
                </div>
            </div>

            <!-- Table - Scrollable -->
            <div
                class="flex-1 overflow-y-scroll px-0 pb-8 [&::-webkit-scrollbar]:w-2 [&::-webkit-scrollbar-track]:bg-zinc-900 [&::-webkit-scrollbar-thumb]:bg-zinc-700 [&::-webkit-scrollbar-thumb]:rounded [&::-webkit-scrollbar-thumb:hover]:bg-zinc-600"
            >
                <table class="w-full">
                    <thead class="sticky top-0 bg-black">
                        <tr class="border-b border-zinc-800">
                            <th class="text-left py-3 px-4 text-zinc-400 font-normal text-sm">{m.date()}</th>
                            <th class="text-left py-3 px-4 text-zinc-400 font-normal text-sm">{m.completed()}</th>
                            <th class="text-left py-3 px-4 text-zinc-400 font-normal text-sm">{m.service_type()}</th>
                            <th class="text-left py-3 px-4 text-zinc-400 font-normal text-sm">{m.status()}</th>
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
