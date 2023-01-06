<script lang="ts">
    import { goto } from "$app/navigation";
    import CopyText from "$lib/components/util/CopyText.svelte";
    import { show_notification } from "$lib/stores/notification";
    import { wallet, filepath } from "$lib/stores/settings";
    import { invoke } from "@tauri-apps/api";
    import { open } from "@tauri-apps/api/dialog";
    import { tweened } from "svelte/motion";


    const select_dir = async () => {
        const selected = await open({
            directory: true
        });
        invoke('update_path', {path: selected}).then(() => {
            show_notification("Upload directory was successfully changed!", "success")
        })
    }

    const scale = tweened(0, { duration: 3000 });

    let text = "Sign Out";
    let timeout: NodeJS.Timeout;
    const change_wallet = () => {
        text = "Hold to Confirm";
        timeout = setTimeout(() => {
            invoke('delete_wallet').then(() => {
                show_notification("You have been signed out.")
                goto("/");
            })
        }, 3000);
        scale.set(100);
    }
    
    const cancel = () => {
        text = "Sign Out";
        if(timeout) clearTimeout(timeout);
        scale.set(0);
    }

</script>

<main class="flex flex-col gap-2 p-4 h-screen overflow-y-scroll pb-20">
    
    <h3 class="-mb-2">File Path</h3>
    <div class="text-xs text-neutral-500">Automatic syncing for files in this directory.</div>
    <div on:click={select_dir} on:keypress={select_dir} class="text-sm text-center p-2 mb-4 w-full overflow-scroll border hover:bg-dark cursor-pointer transition-colors">{$filepath}</div>
    <h3 class="-mb-2">Wallet ID</h3>    
    <div class="text-xs text-neutral-500">Used to access your files from another device.</div>
    <div class="flex flex-row justify-between text-xs text-center p-2 w-full overflow-x-scroll border cursor-text select-text">
        <div>{$wallet?.id}</div>
        <CopyText content={$wallet?.id || ''}/>
    </div>
    <button on:mousedown={change_wallet} on:mouseup={cancel} on:mouseleave={cancel}>{text}</button>
    <div style="width: {$scale}%;" class="bg-white h-2 -mt-2"/>
</main>