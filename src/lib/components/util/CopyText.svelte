<script>
    import { writeText } from '@tauri-apps/api/clipboard'
    import Copy from '$lib/icons/Copy.svelte';
    import Check from '$lib/icons/Check.svelte';
    import { show_notification } from '$lib/stores/notification';
  
    /**
     * @type {string}
     */
    export let content;

    export let size = 18;
  
    let copied = false
  
    function copy() {
      if (copied) return
      show_notification("Wallet ID was copied to clipboard.", "success", 3000)
      writeText(content)
      copied = true
      setTimeout(() => {
        copied = false
      }, 3000)
    }
  </script>
  
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="cursor-pointer" on:click={copy}>
    {#if copied}
        <Check size={size} />
    {:else}
        <Copy size={size}/>
    {/if}
  </div>
  