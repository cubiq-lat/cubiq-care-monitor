<script lang="ts">
    import { Command, Globe, Menu, Search, X } from "@lucide/svelte";
    import { page } from "$app/state";
    import { onMount } from "svelte";

    // import { toggleDrawer, drawerState } from "$stores/DrawerState.state.svelte";
    import { getLocale, locales, localizeHref } from "$lib/paraglide/runtime";
    import { getLocaleName, redirectLocale } from "$lib/utils";

    let isMac = $state<boolean | null>(null);
    let locale = $state(getLocale());

    onMount(() => {
        isMac = /Mac/i.test(navigator.userAgent);
    });
</script>

<div class="flex items-center gap-10 md:gap-16">
    <!-- <HeaderMenu /> -->
    <div class="flex items-center gap-2.5 md:gap-4">
        <!-- Help -->
        <a
            href={localizeHref("/help")}
            class="bg-base-200 hover:bg-base-100 flex items-center rounded-full px-5 py-3 text-[13px] font-light uppercase md:inline-block md:px-6 md:text-[15px]"
        >
            <span class="relative top-px flex items-center">Ajuda</span>
        </a>
    </div>
</div>
