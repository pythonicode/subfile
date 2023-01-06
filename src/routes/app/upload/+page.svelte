<script lang="ts">

    import { open } from "@tauri-apps/api/dialog";
    import { onMount, onDestroy } from "svelte";
    import { appWindow } from "@tauri-apps/api/window";
    import type { UnlistenFn } from "@tauri-apps/api/event";
    import { file_name } from "$lib/helpers/files";
    import Folder from "$lib/icons/Folder.svelte";
    import File from "$lib/icons/File.svelte";
    import { invoke } from "@tauri-apps/api";

    let unlisten: UnlistenFn | null = null
    let hovering: string[] | null = null

    onMount(async () => {
        unlisten = await appWindow.onFileDropEvent((event) => {
        if (event.payload.type === 'hover') {
            console.log('User hovering', event.payload.paths)
            hovering = event.payload.paths.length > 0 ? event.payload.paths : null
        } else if (event.payload.type === 'drop') {
            console.log('User dropped', event.payload.paths)
        } else {
            console.log('File drop cancelled')
            hovering = null
        }
        })
    })

    onDestroy(() => {
        if (unlisten) unlisten()
    })

    const chooseFiles = async () => {
        const selected = await open({
            multiple: true,
        })
        if (!selected) return;
        for (const path of selected) {
            invoke('upload', { path: path });
        }
    }

    const chooseDirectory = async () => {
        const selected = await open({
        directory: true,
        })
        if (selected) console.log(selected)
    }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<main class="flex flex-col p-8">
    <div
    on:click={chooseFiles}
    class="flex flex-col items-center justify-center gap-2 w-full grow border-4 border-dashed border-dark my-4 py-10 hover:bg-dark cursor-pointer hover:border-light transition-colors duration-300 {
        hovering ? 'bg-dark border-light' : ''
    }"
    >
        <File size={32} />
        <div class="text-xl font-bold">Upload {hovering ? file_name(hovering[0]) : 'Files'}</div>
        <div class="text-sm text-gray-500">Maximum 10 GB</div>
    </div>

    <div
    on:click={chooseDirectory}
    class="flex flex-col items-center justify-center gap-2 w-full grow border-4 border-dashed border-dark my-4 py-10 hover:bg-dark cursor-pointer hover:border-light transition-colors duration-300 {
        hovering ? 'bg-dark border-light' : ''
    }"
    >
        <Folder size={32}/>
        <div class="text-xl font-bold">Upload {hovering ? file_name(hovering[0]) : 'Folder'}</div>
        <div class="text-sm text-gray-500">Maximum 10 GB</div>
    </div>
</main>