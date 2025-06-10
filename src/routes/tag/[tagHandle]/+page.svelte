<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import type { PageData } from './$types';
  import { initJuno } from '$lib/juno';
  import { listDocs } from '@junobuild/core';
  import { setPageMeta } from '$lib/stores/page';
  import { authUserDoc } from '$lib/stores/authUserDoc';
  import NotLoggedInAlert from '$lib/components/common/NotLoggedInAlert.svelte';
  import TagAboutCard from '$lib/components/tags/TagAboutCard.svelte';
  import QuickActionsTags from '$lib/components/tags/QuickActionsTags.svelte';
  import TagUserReputationCard from '$lib/components/tags/TagUserReputationCard.svelte';
  import TagGraphCard from '$lib/components/tags/TagGraphCard.svelte';
  import TagStatsCard from '$lib/components/tags/TagStatsCard.svelte';
  import RecentVotesTag from '$lib/components/tags/RecentVotesTag.svelte';
  import TagTopUsersCard from '$lib/components/tags/TagTopUsersCard.svelte';
  import { ArrowLeft } from 'lucide-svelte';

  let { data }: { data: PageData } = $props();
  
  let tagData = $state<any>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let allTags = $state<any[]>([]);

  // Time filter state
  let cutoffTimestamp = $state(BigInt(new Date('2025-01-01T00:00:00Z').getTime()) * BigInt(1_000_000));
  
  const timePeriods = [
    { label: '24h', ms: 24 * 60 * 60 * 1000 },
    { label: '7d', ms: 7 * 24 * 60 * 60 * 1000 },
    { label: '30d', ms: 30 * 24 * 60 * 60 * 1000 },
    { label: '90d', ms: 90 * 24 * 60 * 60 * 1000 },
    { label: '1y', ms: 365 * 24 * 60 * 60 * 1000 },
    { label: 'All', ms: null }
  ];
  
  let selectedPeriod = $state(timePeriods[timePeriods.length - 1]);
  const ALL_TIME_CUTOFF = BigInt(new Date('2025-01-01T00:00:00Z').getTime()) * BigInt(1_000_000);

  function setPeriod(period: { label: string; ms: number | null }) {
    selectedPeriod = period;
    if (period.label === 'All') {
      cutoffTimestamp = ALL_TIME_CUTOFF;
    } else if (period.ms !== null) {
      cutoffTimestamp = BigInt(Date.now() - period.ms) * BigInt(1_000_000);
    }
  }

  async function loadTagData() {
    try {
      loading = true;
      error = null;
      
      await initJuno();
      
      // Load all tags for dropdown
      const allTagsResult = await listDocs({ collection: 'tags' });
      allTags = allTagsResult.items;
      
      // Handle different data loading scenarios
      if (data.isPreview) {
        // Preview mode - data is already available
        tagData = data;
        setPageMeta({ title: `Tag: ${data.tag.data.tag_handle}` });
      } else if (data.fetchTagData) {
        // Dynamic data loading
        tagData = await data.fetchTagData();
        
        // Check if URL needs normalization (case mismatch)
        if (tagData && tagData.tag) {
          const realHandle = tagData.tag.data.tag_handle;
          const urlHandle = data.tagHandle;
          
          if (realHandle && realHandle !== urlHandle) {
            // Mismatch detected! Normalize URL
            goto(`/tag/${realHandle}`, { replaceState: true });
            return; // Exit early, the goto will trigger effect again
          }
        }
        
        setPageMeta({ title: `Tag: ${tagData.tag.data.tag_handle}` });
      } else {
        // Fallback - should not happen
        throw new Error('No data available');
      }
    } catch (e) {
      console.error('Failed to load tag data:', e);
      error = e instanceof Error ? e.message : 'Failed to load tag data';
    } finally {
      loading = false;
    }
  }

  onMount(loadTagData);
  
  // React to data changes (e.g., when navigating between tags)
  $effect(() => {
    if (data.tagHandle) {
      loadTagData();
    }
  });
  
  function onTagChange(event: Event) {
    const newHandle = (event.target as HTMLSelectElement).value;
    if (newHandle && newHandle !== tagData?.tag?.data?.tag_handle) {
      // Force reload by setting loading state and clearing data
      loading = true;
      tagData = null;
      error = null;
      goto(`/tag/${newHandle}`);
    }
  }
</script>

<!-- Main Container -->
<div class="p-4">
  <NotLoggedInAlert />
  
  {#if error}
    <div class="alert alert-error mb-6">{error}</div>
  {/if}

  <!-- Header Section -->
  <div class="flex flex-row items-center justify-between flex-wrap gap-4 mb-6">
    <!-- Left side: Back button and Tag info -->
    <div class="flex items-center gap-4">
      <button 
        class="btn preset-tonal-surface"
        onclick={() => goto('/tag')}
        title="Back to Tags"
      >
        <ArrowLeft size={20} />
      </button>
      <span class="text-lg whitespace-nowrap">You are exploring:</span>
      {#if loading}
        <div class="placeholder animate-pulse w-48 h-8 rounded"></div>
      {:else if tagData?.tag}
        <select 
          class="input input-lg"
          value={tagData.tag.data.tag_handle}
          onchange={onTagChange}
          disabled={loading}
        >
          {#if tagData.isPreview}
            <option value="preview-mode" selected>✨ Preview Mode ✨</option>
          {/if}
          {#each allTags as tag (tag.key)}
            <option value={tag.data.tag_handle}>{tag.data.tag_handle}</option>
          {/each}
        </select>
        {#if tagData.isPreview}
          <span class="badge preset-filled-warning-500">Preview Mode</span>
        {/if}
      {:else}
        <h1 class="text-2xl font-bold text-error-500">Tag not found</h1>
      {/if}
    </div>

    <!-- Right side: Global Time Filter -->
    <div class="flex gap-2">
      {#each timePeriods as period}
        <button
          class={`btn text-xs ${selectedPeriod.label === period.label ? 'preset-filled-primary-500' : 'preset-tonal-primary'}`}
          onclick={() => setPeriod(period)}
          disabled={loading}
        >
          {period.label}
        </button>
      {/each}
    </div>
  </div>

  <!-- Main Grid Layout -->
  <div class="grid grid-cols-1 lg:grid-cols-2 2xl:grid-cols-3 gap-6">
    <!-- Left Column: About/Settings and Quick Actions -->
    <div class="flex flex-col gap-6">
      <!-- About & Settings -->
      <TagAboutCard tag={tagData?.tag} loading={loading} />

      <!-- Quick Actions -->
      <QuickActionsTags selectedTag={tagData?.tag} />
    </div>

    <!-- User Activity -->
    <TagUserReputationCard 
      tag={tagData?.tag} 
      loading={loading}
      isPreview={tagData?.isPreview}
      cutoffTimestamp={cutoffTimestamp}
    />

    <!-- Graph Preview -->
    <TagGraphCard 
      tag={tagData?.tag} 
      loading={loading}
      isPreview={tagData?.isPreview}
    />
  </div>

  <!-- Stats Overview -->
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 mt-6">
    <TagStatsCard 
      tag={tagData?.tag} 
      loading={loading}
    />
  </div>

  <!-- Activity Sections -->
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mt-6">
    <!-- Recent Votes -->
    <RecentVotesTag 
      selectedTag={tagData?.tag} 
      cutoffTimestamp={cutoffTimestamp} 
    />

    <!-- Top Users -->
    <TagTopUsersCard 
      tag={tagData?.tag} 
      loading={loading}
      isPreview={tagData?.isPreview}
    />
  </div>

  <!-- Call to Action -->
  <div class="mt-6">
    {#if loading}
      <div class="placeholder animate-pulse w-full h-12 rounded"></div>
    {:else if tagData?.isPreview}
      <button class="btn preset-filled-primary-500 w-full" disabled>
        Currently in Preview Mode
      </button>
    {:else if tagData?.tag}
      <button 
        class="btn preset-filled-primary-500 w-full"
        onclick={() => {
          if ($authUserDoc) {
            // Navigate to contribute/vote page
            goto(`/tag/${tagData.tag.data.tag_handle}/vote`);
          } else {
            // Navigate to login
            goto('/login');
          }
        }}
      >
        To join, start voting!
      </button>
    {:else}
      <button class="btn preset-filled-primary-500 w-full" disabled>
        Tag Not Available
      </button>
    {/if}
  </div>
</div> 