<!-- src/lib/components/LanguageButton.svelte -->
<script lang="ts">
    import { Wrench } from "@lucide/svelte";
    import { page } from "$app/state";
    import { getLocale, locales } from "$lib/paraglide/runtime";
    import { getLocaleName, redirectLocale } from "$lib/utils";

    // Svelte 5 rune
    let locale = $state(getLocale());
</script>

<button
    tabindex="0"
    type="button"
    class="no-drag bg-base-200 hover:bg-base-100 fw-header-fs relative hidden rounded-full px-3 py-3 md:block"
>
    <Wrench class="h-5" strokeWidth="1" />
</button>
