<script lang="ts">
  import { Expand, X } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import BaseCard from '$lib/components/common/BaseCard.svelte';
  import SigmaGraph from '$lib/components/graph/SigmaGraph.svelte';
  import type { TagDocument } from '$lib/types';
  import { Popover } from '@skeletonlabs/skeleton-svelte';
  import { fetchTagGraphData, type GraphData } from '$lib/utils/graphApi';

  const { 
    tag, 
    loading = false,
    isPreview = false
  }: { 
    tag: TagDocument | null; 
    loading?: boolean;
    isPreview?: boolean;
  } = $props();

  // Component state
  let expandPopoverOpen = $state(false);
  let graphData: GraphData | null = $state(null);
  let graphLoading = $state(false);
  let graphError = $state<string | null>(null);

  // Debug logging to ensure component renders
  console.log('[TagGraphCard] Component rendered/updated with:', {
    tag: tag ? { key: tag.key, data: tag.data } : null,
    loading,
    isPreview
  });

  // HOW: Load graph data when component mounts
  onMount(async () => {
    console.log('[TagGraphCard] Component mounted with props:', {
      tag: tag,
      loading: loading,
      isPreview: isPreview,
      shouldLoadData: tag && !isPreview
    });
    
    if (tag && !isPreview) {
      console.log('[TagGraphCard] Conditions met, loading graph data...');
      await loadGraphData();
    } else {
      console.log('[TagGraphCard] Skipping graph data load:', {
        hasTag: !!tag,
        isPreview: isPreview,
        reason: !tag ? 'no tag' : isPreview ? 'preview mode' : 'unknown'
      });
    }
  });

  // HOW: Effect to load graph data when tag becomes available
  $effect(() => {
    if (tag && !isPreview && !graphData && !graphLoading) {
      console.log('[TagGraphCard] Tag became available, loading graph data reactively...');
      loadGraphData();
    }
  });

  /**
   * HOW: Loads graph data for the current tag
   * - Uses the tag's ULID to fetch vote relationships
   * - Handles loading and error states appropriately
   * - Provides foundation for interactive visualization
   */
  async function loadGraphData() {
    if (!tag) return;
    
    console.log('[TagGraphCard] Loading graph data for tag:', {
      tagKey: tag.key,
      tagData: tag.data,
      tagUlid: tag.data?.tag_ulid
    });
    
    graphLoading = true;
    graphError = null;
    
    try {
      // HOW: Use tag_ulid from tag data, not the full document key
      const tagUlid = tag.data?.tag_ulid;
      if (!tagUlid) {
        throw new Error('Tag ULID not found in tag data');
      }
      
      console.log(`[TagGraphCard] Fetching graph data for tag ULID: ${tagUlid}`);
      graphData = await fetchTagGraphData(tagUlid);
      console.log('[TagGraphCard] Graph data loaded successfully:', {
        nodes: graphData.nodes.length,
        edges: graphData.edges.length,
        data: graphData
      });
    } catch (error) {
      console.error('[TagGraphCard] Failed to load graph data:', error);
      graphError = error instanceof Error ? error.message : 'Failed to load graph data';
    } finally {
      graphLoading = false;
    }
  }

  // Helper function to close expand popover
  function closeExpandPopover() {
    expandPopoverOpen = false;
  }
</script>

<BaseCard classes="2xl:col-span-1 lg:col-span-2">
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
              Interactive network visualization showing vote relationships within this tag. Click nodes to see user details, edges to see vote counts. Curved edges represent negative votes, straight edges represent positive votes.
            </p>
            {#if graphData}
              <div class="text-sm space-y-1 mt-2">
                <p><strong>Nodes:</strong> {graphData.nodes.length} users</p>
                <p><strong>Edges:</strong> {graphData.edges.length} vote relationships</p>
              </div>
            {/if}
          </article>
        {/snippet}
      </Popover>
    {/if}
  {/snippet}
  
  {#snippet children()}
    <div class="w-full h-[454px] bg-surface-200-800 rounded">
      {#if loading || graphLoading}
        <div class="w-full h-full flex items-center justify-center">
          <div class="placeholder animate-pulse w-3/4 h-8 rounded"></div>
        </div>
      {:else if isPreview}
        <div class="w-full h-full flex items-center justify-center">
          <span class="opacity-50">Graph visualization for Preview Mode.</span>
        </div>
      {:else if graphError}
        <div class="w-full h-full flex flex-col items-center justify-center gap-2">
          <span class="text-error-500 font-medium">Failed to load graph</span>
          <span class="text-sm opacity-60">{graphError}</span>
          <button class="btn btn-sm variant-ghost-surface" onclick={loadGraphData}>
            Retry
          </button>
        </div>
      {:else if !tag}
        <div class="w-full h-full flex items-center justify-center">
          <span class="opacity-50">Loading graph...</span>
        </div>
      {:else if graphData && (graphData.nodes.length > 0 || graphData.edges.length > 0)}
        <!-- HOW: Show interactive graph with real data -->
        <SigmaGraph 
          height="100%" 
          width="100%"
          showControls={false}
          {graphData}
          loading={graphLoading}
        />
      {:else}
        <div class="w-full h-full flex items-center justify-center">
          <span class="opacity-50">No voting relationships found in this tag yet.</span>
        </div>
      {/if}
    </div>
  {/snippet}
</BaseCard> 