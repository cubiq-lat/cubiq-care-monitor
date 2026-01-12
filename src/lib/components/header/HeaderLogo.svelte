<!-- src/lib/components/Header/HeaderLogo.svelte -->
<script lang="ts">
    import { localizeHref } from "$lib/paraglide/runtime";
    import CubiqLogoWhite from "../logos/CubiqLogoWhite.svelte";
    interface Props {
        isDashboard?: boolean;
        isCollapsed?: boolean;
    }
    let { isDashboard = false, isCollapsed = false }: Props = $props();
</script>

<div class="flex items-start justify-start">
    <a href={localizeHref("/")}>
        <div class="flex flex-row justify-start">
            <CubiqLogoWhite class="relative -left-6 h-10 self-start md:-left-7 md:h-10.5" />
        </div>
    </a>
</div>
