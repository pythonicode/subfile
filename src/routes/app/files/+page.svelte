<script lang="ts">
    import Directory from "$lib/components/ui/files/Directory.svelte";
    import { client, create_url } from "$lib/helpers/api";
    import Collection from "$lib/icons/Collection.svelte";
    import List from "$lib/icons/List.svelte";
    import SadFace from "$lib/icons/SadFace.svelte";
    import { display, settings, type Settings } from "$lib/stores/settings";
    import { invoke } from "@tauri-apps/api";
    import { onMount } from "svelte";

    let contents: any[] | null = null;

    let setting: Settings | null = null;
    settings.subscribe(result => {
        setting = result;
    });

    let path = "/";

    onMount(() => {
        if(!setting || !setting.wallet || !setting.api) return;
        client(create_url(`/content?coluuid=${setting.wallet.id}&dir=${path}`), {
            method: "get",
            headers: {
                "Authorization": `Bearer ${setting.api.key}`,
                "Key": setting.wallet.key
            },
            timeout: 20000
        }).then(result => {
            contents = result.data;
        }).catch(error => {
            console.error(error);
        });
    });

    let mode: "list" | "category" | null = null;
    display.subscribe(result => {
        mode = result;
    })

    const switch_mode = () => {
        if(mode) invoke('update_mode', {mode : mode == "list" ? "category" : "list" });
    }

    const navigate_to = (dir: string) => {
        path = path + dir
        contents = null;
        if(!setting || !setting.wallet || !setting.api) return;
        client(create_url(`/content?coluuid=${setting.wallet.id}&dir=${path}`), {
            method: "get",
            headers: {
                "Authorization": `Bearer ${setting.api.key}`,
                "Key": setting.wallet.key
            },
            timeout: 20000
        }).then(result => {
            contents = result.data;
        }).catch(error => {
            console.error(error);
        });
    }
</script>


<main class="flex flex-col grow">
    <div class="flex flex-row gap-2 items-center justify-center w-full p-4 text-sm">
        <input type="search" placeholder="Search" class="w-full border rounded" />
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div class="p-2 hover:bg-dark transition-colors rounded cursor-pointer" on:click={switch_mode}>
            {#if mode == "category"}
                <Collection size={24}/>
            {:else}
                <List size={24}/>
            {/if}
        </div>
    </div>
    {#if contents == null || $display == null}
        <div class="grid {mode == "category" ? "grid-cols-3" : "grid-cols-1"} gap-2 px-2">
            <div class="{mode == "category" ? "h-20" : "h-10"} bg-dark animate-pulse rounded"/>
            <div class="{mode == "category" ? "h-20" : "h-10"} bg-dark animate-pulse rounded"/>
            <div class="{mode == "category" ? "h-20" : "h-10"} bg-dark animate-pulse rounded"/>  
        </div>
    {:else if contents.length == 0}
        <div class="w-full text-neutral-500 grow flex flex-col items-center justify-center text-center mb-10">
            <SadFace size={50} color={"rgb(115, 115, 115)"} class="mb-4"/>
            <div>Nothing Here Yet!</div>
            <div>Try <a href="/app/upload" class="underline hover:text-light transition-colors">uploading</a> a file.</div>
        </div>
    {:else}
        <div class="grid {mode == "category" ? "grid-cols-3" : "grid-cols-1"} gap-2 px-2">
            {#each contents as content (content.name) }
                {#if content.type == "directory"}
                    <Directory name={content.name} mode={$display} onclick={() => { navigate_to(content.name) }} />
                {/if}
            {/each}
        </div>
    {/if}
</main>