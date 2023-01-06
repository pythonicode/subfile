<script>
    // @ts-nocheck
    import Check from "$lib/icons/Check.svelte";
    import Error from "$lib/icons/Error.svelte";
    import Warning from "$lib/icons/Warning.svelte";
    import { notifications } from "$lib/stores/notification";
    import { fly } from "svelte/transition";
</script>

<div class="absolute flex flex-col bottom-0 left-0 w-full p-4 gap-2">
    {#each $notifications as notification (notification.id)}
        <div 
            class="relative w-full text-center text-sm bg-dark p-2 rounded border"
            transition:fly|local={{ x: -200 }}
        >
            <div class="flex items-center justify-center absolute -top-2 -left-2">
                {#if notification.type == "success"}
                    <div class="bg-green-600 p-1 rounded">
                        <Check size={15} />
                    </div>
                {:else if notification.type == "warning"}
                    <div class="bg-yellow-600 p-1 rounded">
                        <Warning size={15} />
                    </div>
                {:else if notification.type == "error"}
                    <div class="bg-red-600 p-1 rounded">
                        <Error size={15} />
                    </div>
                {/if}
            </div>
            {notification.message}
        </div>
    {/each}
</div>