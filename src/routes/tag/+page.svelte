<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { listDocs } from '@junobuild/core';
  import { initJuno } from '$lib/juno';
  import { setPageMeta } from '$lib/stores/page';
  import { authUserDoc } from '$lib/stores/authUserDoc';
  import NotLoggedInAlert from '$lib/components/common/NotLoggedInAlert.svelte';
  import BaseCard from '$lib/components/common/BaseCard.svelte';
  import { Plus, Hash, Users, ShieldCheck, Send } from 'lucide-svelte';
  import type { TagDocument } from '$lib/types';
  import { queryDocsByKey } from '$lib/docs-crud/query_by_key';

  let tags = $state<(TagDocument & { 
    stats?: { 
      totalUsers: number | null; 
      trustedUsers: number | null; 
      totalVotes: number | null;
      loading: boolean;
    } 
  })[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);

  async function fetchTagStats(tag: TagDocument) {
    // First, get the tag's ulid for proper querying
    const tagUlid = tag.data.tag_ulid || tag.key;
    
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

      return {
        totalUsers,
        trustedUsers,
        totalVotes,
        loading: false
      };
    } catch (e) {
      console.error(`Failed to fetch stats for tag ${tag.data.tag_handle}:`, e);
      return {
        totalUsers: 0,
        trustedUsers: 0,
        totalVotes: 0,
        loading: false
      };
    }
  }

  onMount(async () => {
    setPageMeta({ title: 'Explore Reputation #tags' });
    
    try {
      await initJuno();
      const result = await listDocs({ collection: 'tags' });
      const fetchedTags = result.items as TagDocument[];
      
      // Initialize tags with loading stats
      tags = fetchedTags.map(tag => ({
        ...tag,
        stats: {
          totalUsers: null,
          trustedUsers: null,
          totalVotes: null,
          loading: true
        }
      }));
      
      loading = false; // Show the tags immediately
      
      // Fetch stats for each tag and update individually
      fetchedTags.forEach(async (tag, index) => {
        try {
          const stats = await fetchTagStats(tag);
          // Update the specific tag's stats
          tags[index] = { ...tags[index], stats };
        } catch (e) {
          console.error(`Failed to fetch stats for tag ${tag.data.tag_handle}:`, e);
          tags[index] = { 
            ...tags[index], 
            stats: {
              totalUsers: 0,
              trustedUsers: 0,
              totalVotes: 0,
              loading: false
            }
          };
        }
      });
      
    } catch (e) {
      console.error('Failed to load tags:', e);
      error = e instanceof Error ? e.message : 'Failed to load tags';
      loading = false;
    }
  });

  function navigateToTag(tagHandle: string | undefined) {
    if (tagHandle) {
      goto(`/tag/${tagHandle}`);
    }
  }
</script>

<NotLoggedInAlert />

<div class="p-4">
  <!-- Header -->
  <div class="flex justify-between items-center mb-6">
    <div>
      <h1 class="text-3xl font-bold mb-2">Explore Reputation #tags</h1>
      <p class="text-surface-500">Each #tag represents a community, app, store, games, or other type of reputation system.</p>
    </div>
    {#if $authUserDoc}
      <button 
        class="btn preset-filled-primary-500"
        onclick={() => goto('/tag/create')}
      >
        <Plus size={20} />
        Create Tag
      </button>
    {/if}
  </div>

  {#if error}
    <div class="alert alert-error mb-6">{error}</div>
  {/if}

  <!-- Preview Mode Card -->
  <BaseCard classes="mb-6 preset-outlined-warning-500">
    {#snippet header()}
      <h2 class="text-lg font-bold text-warning-500">See an Example</h2>
    {/snippet}
    
    {#snippet children()}
      <p class="mb-4">
        Explore what an active reputation community looks like with sample data and interactions.
      </p>
      <button 
        class="btn preset-filled-warning-500"
        onclick={() => navigateToTag('preview-mode')}
      >
        View Example Community
      </button>
    {/snippet}
  </BaseCard>

  <!-- Tags Grid -->
  {#if loading}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each Array(6) as _}
        <div class="placeholder animate-pulse h-48 rounded"></div>
      {/each}
    </div>
  {:else if tags.length === 0}
    <BaseCard>
      {#snippet header()}
        <h2 class="text-lg font-bold">No Tags Found</h2>
      {/snippet}
      
      {#snippet children()}
        <p class="text-center opacity-70 py-8">
          No community tags have been created yet. 
          {#if $authUserDoc}
            Be the first to create one!
          {:else}
            Log in to create the first tag.
          {/if}
        </p>
        {#if $authUserDoc}
          <div class="text-center">
            <button 
              class="btn preset-filled-primary-500"
              onclick={() => goto('/tag/create')}
            >
              Create First Tag
            </button>
          </div>
        {/if}
      {/snippet}
    </BaseCard>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each tags as tag (tag.key)}
        <BaseCard classes="cursor-pointer hover:scale-105 transition-transform">
          {#snippet header()}
            <div class="flex items-center gap-2">
              <Hash size={20} class="text-primary-500" />
              <h3 class="text-lg font-bold">{tag.data.tag_handle}</h3>
            </div>
          {/snippet}
          
          {#snippet children()}
            <div onclick={() => navigateToTag(tag.data.tag_handle)}>
              {#if tag.data.description}
                <p class="text-sm opacity-80 mb-4 line-clamp-3">
                  {tag.data.description}
                </p>
              {:else}
                <p class="text-sm opacity-50 mb-4 italic">No description provided</p>
              {/if}
              
              <div class="grid grid-cols-3 gap-2 mb-4">
                <div class="text-center">
                  <Users size={20} class="mx-auto mb-1 text-surface-500" />
                  <p class="text-xs opacity-70">Total Users</p>
                  {#if tag.stats?.loading}
                    <div class="placeholder animate-pulse h-4 w-8 mx-auto rounded"></div>
                  {:else}
                    <p class="font-semibold text-sm">{tag.stats?.totalUsers ?? 0}</p>
                  {/if}
                </div>
                <div class="text-center">
                  							<ShieldCheck size={20} class="mx-auto mb-1 text-surface-500" />
                  <p class="text-xs opacity-70">Trusted Users</p>
                  {#if tag.stats?.loading}
                    <div class="placeholder animate-pulse h-4 w-8 mx-auto rounded"></div>
                  {:else}
                    <p class="font-semibold text-sm">{tag.stats?.trustedUsers ?? 0}</p>
                  {/if}
                </div>
                <div class="text-center">
                  <Send size={20} class="mx-auto mb-1 text-surface-500" />
                  <p class="text-xs opacity-70">Total Votes</p>
                  {#if tag.stats?.loading}
                    <div class="placeholder animate-pulse h-4 w-8 mx-auto rounded"></div>
                  {:else}
                    <p class="font-semibold text-sm">{tag.stats?.totalVotes ?? 0}</p>
                  {/if}
                </div>
              </div>
              
              <button class="btn preset-filled-primary-500 w-full">
                Explore Tag
              </button>
            </div>
          {/snippet}
        </BaseCard>
      {/each}
    </div>
  {/if}
</div> 