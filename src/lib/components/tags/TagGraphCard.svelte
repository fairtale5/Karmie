<script lang="ts">
  import { goto } from '$app/navigation';
  import { Expand } from 'lucide-svelte';
  import BaseCard from '$lib/components/common/BaseCard.svelte';
  import type { TagDocument } from '$lib/types';

  const { 
    tag, 
    loading = false,
    isPreview = false
  }: { 
    tag: TagDocument | null; 
    loading?: boolean;
    isPreview?: boolean;
  } = $props();
</script>

<BaseCard underConstruction={true} classes="2xl:col-span-1 lg:col-span-2">
  {#snippet header()}
    <h2 class="text-lg font-bold {(!tag || loading) ? 'opacity-50' : ''}">Graph Overview</h2>
  {/snippet}
  
  {#snippet actions()}
    {#if tag && !isPreview}
      <button 
        type="button" 
        class="chip-icon preset-tonal-surface" 
        onclick={() => goto(`/tag/${tag?.data.tag_handle}/graph`)} 
        disabled={loading} 
        title="View Full Graph"
      >
        <Expand size={16} />
      </button>
    {/if}
  {/snippet}
  
  {#snippet children()}
    <div class="w-full h-64 bg-surface-200-800 rounded flex items-center justify-center">
      {#if loading}
        <div class="placeholder animate-pulse w-3/4 h-8 rounded"></div>
      {:else if isPreview}
        <span class="opacity-50">Graph visualization for Preview Mode.</span>
      {:else if tag}
        <span class="opacity-50">Graph visualization coming soon…</span>
      {:else}
        <span class="opacity-50">Loading graph...</span>
      {/if}
    </div>
  {/snippet}
</BaseCard> 