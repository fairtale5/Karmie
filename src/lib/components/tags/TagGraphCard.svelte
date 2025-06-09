<script lang="ts">
  import { Expand, X } from 'lucide-svelte';
  import BaseCard from '$lib/components/common/BaseCard.svelte';
  import type { TagDocument } from '$lib/types';
  import { Popover } from '@skeletonlabs/skeleton-svelte';

  const { 
    tag, 
    loading = false,
    isPreview = false
  }: { 
    tag: TagDocument | null; 
    loading?: boolean;
    isPreview?: boolean;
  } = $props();

  // Popover state for expand icon
  let expandPopoverOpen = $state(false);

  // Helper function to close expand popover
  function closeExpandPopover() {
    expandPopoverOpen = false;
  }
</script>

<BaseCard underConstruction={true} classes="2xl:col-span-1 lg:col-span-2">
  {#snippet header()}
    <h2 class="text-lg font-bold {(!tag || loading) ? 'opacity-50' : ''}">Graph Overview</h2>
  {/snippet}
  
  {#snippet actions()}
    {#if tag && !isPreview}
      <Popover
        open={expandPopoverOpen}
        onOpenChange={(e) => (expandPopoverOpen = e.open)}
        positioning={{ placement: 'top', flip: true }}
        triggerBase="chip-icon preset-tonal-surface"
        contentBase="card bg-surface-200-800 p-4 space-y-4 max-w-[320px]"
        arrow
        arrowBackground="!bg-surface-200 dark:!bg-surface-800"
      >
        {#snippet trigger()}
          <Expand size={16} />
        {/snippet}
        {#snippet content()}
          <header class="flex justify-between">
            <p class="font-bold">Graph View</p>
            <button class="btn-icon hover:preset-tonal" onclick={closeExpandPopover}><X class="w-4 h-4" /></button>
          </header>
          <article>
            <p class="opacity-60">
              This feature isn't available yet. In the future, you'll be able to view an interactive network visualization showing reputation relationships, vote flows, and community connections within this tag.
            </p>
          </article>
        {/snippet}
      </Popover>
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