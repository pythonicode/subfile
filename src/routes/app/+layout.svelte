<script>
    import { page } from "$app/stores";
    import Folder from "$lib/icons/Folder.svelte";
    import Settings from "$lib/icons/Settings.svelte";
    import Upload from "$lib/icons/Upload.svelte";

    let selected = "/files";
    page.subscribe(p => {
        selected = p.url.pathname.slice(p.url.pathname.lastIndexOf("/"));
    })
</script>

<header class="grid grid-cols-3 h-10">
    <a href="/app/files" class="cursor-pointer flex text-center items-center justify-center grow h-full transition-all border-b {selected == "/files" ? "bg-dark border-b-light" : "hover:bg-dark border-b-transparent"}">
        <Folder />
    </a>
    <a href="/app/upload" class="cursor-pointer flex text-center items-center justify-center grow h-full transition-all border-b {selected == "/upload" ? "bg-dark border-b-light" : "hover:bg-dark border-b-transparent"}">
        <Upload />
    </a>
    <a href="/app/settings" class="cursor-pointer flex text-center items-center justify-center grow h-full transition-all border-b {selected == "/settings" ? "bg-dark border-b-light" : "hover:bg-dark border-b-transparent"}">
        <Settings/>
    </a>
</header>

<slot />