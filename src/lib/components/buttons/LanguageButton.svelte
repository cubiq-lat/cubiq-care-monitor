<!-- src/lib/components/LanguageButton.svelte -->
<script lang="ts">
    import { Globe } from "@lucide/svelte";
    import { page } from "$app/state";
    import { getLocale, locales } from "$lib/paraglide/runtime";
    import { getLocaleName, redirectLocale } from "$lib/utils";

    // Svelte 5 rune
    let locale = $state(getLocale());
</script>

<div class="dropdown dropdown-end">
    <!-- TRIGGER -->
    <button
        tabindex="0"
        type="button"
        class="no-drag bg-base-200 hover:bg-base-100 fw-header-fs relative hidden rounded-full px-3 py-3 md:block"
    >
        <img src="/images/flags/{locale}.png" alt="flag" class="absolute -top-2 -right-1 h-5.5 w-5.5" />
        <Globe class="h-5" strokeWidth="1" />
    </button>

    <!-- DROPDOWN -->
    <ul tabindex="-1" class="menu dropdown-content bg-base-200 rounded-box z-50 mt-4 w-40 p-2 shadow-sm no-drag">
        {#each locales as loc, i (loc)}
            <li>
                <button
                    id="locale-{i}"
                    type="button"
                    class="flex items-center gap-2 capitalize"
                    onclick={() => redirectLocale(loc, page.url.href)}
                >
                    <img src="/images/flags/{loc}.png" alt="flag-{loc}" class="aspect-1 h-4.25" />
                    <span class="self-center font-semibold tracking-wider capitalize">
                        {getLocaleName(loc)}
                    </span>
                </button>
            </li>
        {/each}
    </ul>
</div>
