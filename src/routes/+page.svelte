<script lang="ts">
    import Logo from "$lib/icons/Logo.svelte";
    import { settings, type Settings } from "$lib/stores/settings";
    import { goto } from "$app/navigation";
    import Loader from "$lib/components/ui/Loader.svelte";
    import { onDestroy, onMount } from "svelte";
    import { invoke } from "@tauri-apps/api";

    let s: Settings;
    const unsub = settings.subscribe(s => {
      if(s && s.wallet && s.api && s.password) goto("/app/files");
    })

    onMount(() => invoke('create_token'));
    onDestroy(() => unsub());
</script>

{#if $settings && (!$settings.api || !$settings.wallet || !$settings.password)}
  <main class="flex flex-col items-center gap-2 p-8">
    <Logo class="my-4"/>
    <a href="/create?step=1" class="w-full">
    <button class="w-full">
      Create New Wallet
    </button>
    </a>
    <div class="flex flex-row flex-nowrap w-full justify-between items-center my-4 gap-8">
      <hr class="w-1/3 border">
      <div>or</div>
      <hr class="w-1/3 border">
    </div>
    <input type="text" placeholder="Wallet ID" class="w-full text-sm"/>
    <input type="password" placeholder="Password" class="w-full text-sm"/>
    <button class="w-full my-2">
      Import Existing Wallet
    </button>
  </main>
{:else}
  <Loader />
{/if}
