<script lang="ts">
    import Folder from "$lib/icons/Folder.svelte";

    export let name: string;
    export let mode: string = "category";
    export let onclick: (event: MouseEvent) => void;
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div on:click={onclick} class="flex {mode == "category" ? "flex-col" : "flex-row"} gap-1 items-center justify-center p-2 {mode == "category" ? "h-20" : "h-10"}  
                             text-neutral-500 hover:text-light border border-neutral-500 hover:border-light text-sm transition-colors grow">
    <Folder />
    <div class="whitespace-nowrap overflow-hidden">{name}</div>
</div>