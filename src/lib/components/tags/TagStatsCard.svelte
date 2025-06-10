<script lang="ts">
  import BaseCard from '$lib/components/common/BaseCard.svelte';
  import type { TagDocument } from '$lib/types';
  import { Users, ShieldCheck, Send } from 'lucide-svelte';
  import { queryDocsByKey } from '$lib/docs-crud/query_by_key';
  import { dummyProfileData } from '$lib/data/dummyProfileData';

  const { 
    tag, 
    loading = false
  }: { 
    tag: TagDocument | null; 
    loading?: boolean;
  } = $props();

  // Preview data constants
  const PREVIEW_TAG_KEY = '___PREVIEW_DATA___';

  let stats = $state<{
    totalUsers: number | null;
    trustedUsers: number | null; 
    totalVotes: number | null;
    loading: boolean;
  }>({
    totalUsers: null,
    trustedUsers: null,
    totalVotes: null,
    loading: false  // Start as false, will be set to true when we have a tag to fetch
  });

  async function fetchTagStats() {
    if (!tag?.data?.tag_ulid) return;
    
    const tagUlid = tag.data.tag_ulid;
    
    try {
      // Query reputation documents for this tag using the tag's ulid
      const reputationResults = await queryDocsByKey('reputations', `tag_${tagUlid}_`);
      const totalUsers = reputationResults.items.length;
      
      // Count trusted users (those with voting power)
      const trustedUsers = reputationResults.items.filter(
        (rep: any) => rep.data?.has_voting_power === true
      ).length;

      // Query votes for this tag using the tag's ulid
      const voteResults = await queryDocsByKey('votes', `tag_${tagUlid}_`);
      const totalVotes = voteResults.items.length;

      stats = {
        totalUsers,
        trustedUsers,
        totalVotes,
        loading: false
      };
    } catch (e) {
      console.error(`[TagStatsCard] Failed to fetch stats for tag ${tag.data.tag_handle}:`, e);
      stats = {
        totalUsers: 0,
        trustedUsers: 0,
        totalVotes: 0,
        loading: false
      };
    }
  }

  // Track the current tag ULID to prevent unnecessary re-fetches
  let currentTagUlid = $state<string | null>(null);

  // Fetch stats when tag changes
  $effect(() => {
    // Handle preview mode
    if (tag?.key === PREVIEW_TAG_KEY) {
      currentTagUlid = PREVIEW_TAG_KEY;
      stats = {
        totalUsers: dummyProfileData.dummyUsers.length,
        trustedUsers: Math.floor(dummyProfileData.dummyUsers.length / 3),
        totalVotes: dummyProfileData.recentVotes.length,
        loading: false
      };
      return;
    }
    
    const tagUlid = tag?.data?.tag_ulid;
    
    if (tagUlid && tagUlid !== currentTagUlid) {
      currentTagUlid = tagUlid;
      stats = {
        totalUsers: null,
        trustedUsers: null,
        totalVotes: null,
        loading: true
      };
      fetchTagStats();
    } else if (!tagUlid && currentTagUlid) {
      currentTagUlid = null;
      stats = {
        totalUsers: null,
        trustedUsers: null,
        totalVotes: null,
        loading: false
      };
    }
  });

  const statItems = $derived([
    { 
      name: 'Total Users', 
      value: stats.totalUsers, 
      color: 'bg-primary-500',
      icon: Users
    },
    { 
      name: 'Trusted Users', 
      value: stats.trustedUsers, 
      color: 'bg-success-500',
              icon: ShieldCheck
    },
    { 
      name: 'Total Votes', 
      value: stats.totalVotes, 
      color: 'bg-warning-500',
      icon: Send
    }
  ]);
</script>

{#each statItems as statItem, i}
  <BaseCard classes={loading || !tag ? 'opacity-50' : ''}>
    {#snippet header()}
      <h3 class="text-sm opacity-70">{statItem.name}</h3>
    {/snippet}
    
    {#snippet children()}
      <div class="flex items-center gap-3 mb-3">
        <svelte:component this={statItem.icon} size={24} class="text-surface-500" />
        <div class="flex-1">
          {#if stats.loading}
            <div class="placeholder animate-pulse h-6 w-16 rounded"></div>
          {:else}
            <p class="text-2xl font-bold">{statItem.value ?? 0}</p>
          {/if}
        </div>
      </div>
      <div class="h-1 w-full bg-surface-200-800 rounded-full overflow-hidden">
        <div 
          class="h-full {statItem.color}" 
          style="width: {statItem.name === 'Total Users' ? 100 : ((statItem.value ?? 0) / (stats.totalUsers ?? 1) * 100)}%"
        ></div>
      </div>
    {/snippet}
  </BaseCard>
{/each} 