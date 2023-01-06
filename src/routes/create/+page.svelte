<script lang="ts">
    import { onDestroy, onMount } from "svelte";

    import Loader from "$lib/components/ui/Loader.svelte";
    import Back from "$lib/icons/Back.svelte";
    
    import { invoke } from '@tauri-apps/api/tauri';
    import { writeText } from '@tauri-apps/api/clipboard';
    import { wallet, password as pwd } from "$lib/stores/settings";

    import { page } from "$app/stores";

    let step = 1;
    page.subscribe(p => {
        let result = p.url.searchParams.get('step');
        step = result ? parseInt(result) : 1;
    })

    import CopyText from "$lib/components/util/CopyText.svelte";

    let copyText = "Copy";

    const copyToClipboard = (content: string) => {
        writeText(content).then(() => {
            copyText = "Copied";
            setTimeout(() => { 
                copyText = "Copy";
            }, 3000);
        })
    }

    onMount(() => invoke('create_wallet'));

    let password: string | undefined = undefined;
    let confirm: string | undefined = undefined;

    const unsubPwd = pwd.subscribe(value => {
        password = value ? value : undefined;
        confirm = value ? value : undefined;
    })


    function calculatePasswordStrength(password: string) {
        const len = password.length;
        const upper = password.length - password.replace(/[A-Z]/g, '').length;
        const lower = password.length - password.replace(/[a-z]/g, '').length;
        const symbols = password.length - password.replace(/[!@#\$%\^\&*\)\(+=._-]+$/g, '').length;
        return Math.min(100, 5 * len - Math.abs(upper - lower) + 5 * symbols + 10);
    }

    $: strength = password ? calculatePasswordStrength(password) : 10;

    import { goto } from "$app/navigation";

    let error: string | undefined;

    let id: string = '';

    const unsubWallet = wallet.subscribe(wallet => {
        id = wallet?.id || "";
    })

    const next = () => {
        if(id.length == 0) {
            error = "Check you network connection."
            return;
        }
        else if(strength < 50) {
            error = "Password too weak. Try again."
            return;
        }
        else if(password != confirm) {
            error = "Passwords do not match. Try again."
            return;
        }
        goto('/create?step=2');
    }

    import { filepath } from "$lib/stores/settings";
    import { open } from "@tauri-apps/api/dialog";

    const select_dir = async () => {
        const selected = await open({
            directory: true
        });
        invoke('update_path', {path: selected});
    }

    const finish = () => {
        invoke('update_password', {password: password})
        goto('/app/files');
    }

    onDestroy(() => {
        unsubPwd();
        unsubWallet();
    })
</script>

{#if step === 1}
    {#if $wallet}
    <a href="/" class="absolute top-2 left-2 p-2 cursor-pointer hover:bg-dark transition-colors rounded">
        <Back  />
    </a>
    <main class="flex flex-col gap-2 p-4 justify-center grow">
        <h3 class="-mb-2">Wallet ID</h3>    
        <div class="text-xs text-neutral-500">Used to access your files from another device.</div>
        <div class="flex flex-row justify-between text-xs text-center p-2 mb-4 w-full overflow-x-scroll border cursor-text select-text">
            <div>{$wallet?.id}</div>
            <CopyText content={$wallet?.id || ''}/>
        </div>
        <h3 class="-mb-2">Password</h3>   
        <div class="text-xs text-neutral-500">Used to securely encrypt your files when uploaded.</div>
        <input type="password" placeholder="Password" bind:value={password} on:input={() => error = undefined} class="text-sm {error == "Password too weak. Try again." ? "border-red-500" : ""}"/>
        <input type="password" placeholder="Confirm Password" bind:value={confirm} on:input={() => error = undefined} class="text-sm {error == "Passwords do not match. Try again." ? "border-red-500" : ""}" />
        <div class="h-2 bg-light rounded" style="width: {strength}%"/>
        <div class="w-full flex flex-row items-center justify-center text-xs">|</div>
        <div class="flex flex-row justify-between text-xs w-full text-neutral-500">
            <div>Least Secure</div>
            <div>Required</div>
            <div>Most Secure</div>
        </div>
        <button on:click={next} class="bg-light text-black rounded hover:bg-neutral-400 transition-colors my-4">Next</button>
        <div class="text-center text-xs {!error ? "text-transparent" : "text-red-500"}">{error}</div>
    </main>
    {:else} 
    <Loader />
    {/if}
{:else if step == 2}
    <a href="/create?step=1" class="absolute top-2 left-2 p-2 cursor-pointer hover:bg-dark transition-colors rounded">
        <Back  />
    </a>
    <main class="flex flex-col gap-2 p-4 justify-center h-screen">
        <h3 class="-mb-2">File Path</h3>
        <div class="text-xs text-neutral-500">Directory where your files will be uploaded.</div>
        <div on:click={select_dir} on:keypress={select_dir} class="text-sm text-center p-2 w-full overflow-scroll border hover:bg-dark cursor-pointer transition-colors">{$filepath}</div>
        <h3 class="mt-4">Important</h3>
        <div class="text-sm mb-4">Make sure your wallet ID and password are stored securely or you may not be able to access your files.</div>
        <button on:click={finish} class="light">Finish</button>
    </main>
{/if}
