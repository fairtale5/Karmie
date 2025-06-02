<script lang="ts">
  import { Construction, Info } from 'lucide-svelte';
  export let classes = '';          // extra Tailwind / Skeleton classes
  export let underConstruction = false; // true = under construction variant
  let showPopup = false;
</script>

<div class={`card shadow bg-surface-100-900 border border-surface-200-800 p-4 ${underConstruction ? 'preset-outlined-error-500' : ''} ${classes}`}>
  <div class="flex flex-col mb-4">
    <slot />
    {#if underConstruction}
      <div class="flex items-center gap-2">
        <button
          class="chip-icon preset-tonal-surface"
          title="Under Construction"
          on:click={() => showPopup = true}
        >
          <Construction class="text-error-500" size={16}/>
        </button>
        <button
          class="chip-icon preset-tonal-surface"
          title="More Info"
          on:click={() => showPopup = true}
        >
          <Info class="text-error-500" size={16}/>
        </button>
      </div>
    {/if}
  </div>
  {#if showPopup}
    <div class="popup">
      <p>Under Construction: This feature is coming soon and currently does not use real data.</p>
      <button on:click={() => showPopup = false}>Close</button>
    </div>
  {/if}
</div> 