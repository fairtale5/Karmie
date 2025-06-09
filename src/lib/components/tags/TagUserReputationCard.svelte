<script lang="ts">
  import { Expand, X } from 'lucide-svelte';
  import BaseCard from '$lib/components/common/BaseCard.svelte';
  import { authUserDoc } from '$lib/stores/authUserDoc';
  import type { TagDocument } from '$lib/types';
  import { Popover } from '@skeletonlabs/skeleton-svelte';

  const { 
    tag, 
    userReputation, 
    userRecentActivity, 
    loading = false,
    isPreview = false
  }: { 
    tag: TagDocument | null; 
    userReputation: any; 
    userRecentActivity: any[];
    loading?: boolean;
    isPreview?: boolean;
  } = $props();
  
  let userActivityFilter = $state('all');

  // Popover state for expand icon
  let expandPopoverOpen = $state(false);

  // Helper function to close expand popover
  function closeExpandPopover() {
    expandPopoverOpen = false;
  }
</script>

<BaseCard classes="h-[400px] flex flex-col">
  {#snippet header()}
    <h2 class="text-lg font-bold {(!tag || !$authUserDoc) ? 'opacity-50' : ''}">
      Your Reputation in {tag?.data.tag_handle || '...'}
    </h2>
  {/snippet}
  
  {#snippet actions()}
    {#if $authUserDoc && tag && !isPreview}
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
            <p class="font-bold">Reputation Details</p>
            <button class="btn-icon hover:preset-tonal" onclick={closeExpandPopover}><X class="w-4 h-4" /></button>
          </header>
          <article>
            <p class="opacity-60">
              This feature isn't available yet. In the future, you'll be able to view a detailed breakdown of your reputation history, voting patterns, trust metrics, and contribution analytics for this tag.
            </p>
          </article>
        {/snippet}
      </Popover>
    {/if}
  {/snippet}
  
  {#snippet children()}
    {#if !$authUserDoc}
      <p class="text-center opacity-60 py-10">Log in to see your activity and reputation for this tag.</p>
    {:else if loading}
      <div class="placeholder animate-pulse w-full h-40 rounded"></div>
    {:else if tag && userReputation}
      <div class="grid grid-cols-2 gap-4 mb-4">
        <div class="p-3 bg-surface-200-800 rounded">
          <span class="text-sm opacity-70">Your Score</span>
          <p class="text-2xl font-bold">{userReputation.score}</p>
        </div>
        <div class="p-3 bg-surface-200-800 rounded">
          <span class="text-sm opacity-70">Rank</span>
          <p class="text-2xl font-bold">#{userReputation.rank}</p>
        </div>
      </div>
      <div class="flex-1 flex flex-col min-h-0">
        <div class="flex justify-start border-b-[1px] border-surface-200-800 mb-2 gap-2">
          <button type="button" class="chip text-xs {userActivityFilter === 'all' ? 'preset-filled-primary-500' : 'preset-tonal-surface'}" onclick={() => userActivityFilter = 'all'}>All</button>
          <button type="button" class="chip text-xs {userActivityFilter === 'in' ? 'preset-filled-secondary-500' : 'preset-tonal-surface'}" onclick={() => userActivityFilter = 'in'}>In</button>
          <button type="button" class="chip text-xs {userActivityFilter === 'out' ? 'preset-filled-tertiary-500' : 'preset-tonal-surface'}" onclick={() => userActivityFilter = 'out'}>Out</button>
          <button type="button" class="chip text-xs {userActivityFilter === 'positive' ? 'preset-filled-success-500' : 'preset-tonal-surface'}" onclick={() => userActivityFilter = 'positive'}>+</button>
          <button type="button" class="chip text-xs {userActivityFilter === 'negative' ? 'preset-filled-error-500' : 'preset-tonal-surface'}" onclick={() => userActivityFilter = 'negative'}>-</button>
        </div>
        <div class="flex-1 overflow-y-auto bg-surface-200-800 rounded p-2 space-y-1">
          {#if userRecentActivity.length > 0}
            {#each userRecentActivity.filter(activity => { 
              if (userActivityFilter === 'all') return true; 
              if (userActivityFilter === 'in') return activity.type === 'received'; 
              if (userActivityFilter === 'out') return activity.type === 'cast'; 
              if (userActivityFilter === 'positive') return activity.value > 0; 
              if (userActivityFilter === 'negative') return activity.value < 0; 
              return true; 
            }) as activity (activity.id || (activity.date + (activity.target || activity.peerName)))} 
              <div class="text-xs p-1 rounded {activity.value > 0 ? 'bg-success-500/10' : 'bg-error-500/10'}">
                {#if activity.type === 'received'}
                  Received <span class="font-semibold">{activity.value > 0 ? `+${activity.value}` : activity.value}</span> vote from <strong>{activity.peerName}</strong>
                {:else if activity.type === 'cast'}
                  Cast <span class="font-semibold">{activity.value > 0 ? `+${activity.value}` : activity.value}</span> vote to <strong>{activity.peerName}</strong>
                {:else}
                  Vote: <span class="font-semibold">{activity.value > 0 ? `+${activity.value}` : activity.value}</span> regarding <strong>{activity.target || activity.peerName}</strong>
                {/if}
                ({new Date(activity.date).toLocaleDateString()})
              </div>
            {:else}
              <p class="text-center text-xs opacity-50 py-2">No activities match the filter.</p>
            {/each}
          {:else}
            <p class="text-center text-xs opacity-50 py-2">No recent activity for this tag.</p>
          {/if}
        </div>
      </div>
    {:else}
      <p class="text-center opacity-70 py-10">Select a tag to see your activity.</p>
    {/if}
  {/snippet}
</BaseCard> 