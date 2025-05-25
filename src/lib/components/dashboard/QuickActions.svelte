<script lang="ts">
  import { Plus, Mail, BarChart, Vote, LoaderCircle, CheckCircle, XCircle } from 'lucide-svelte';
  import { slide } from 'svelte/transition';
  import { fade } from 'svelte/transition';
  import { listDocs } from '@junobuild/core';
  import { queryDocsByKey } from '$lib/docs-crud/query_by_key';
  import { authUserDoneInitializing, authUser } from '$lib/stores/authUser';
  import type { TagDocument, ReputationDocument, TagData, ReputationData } from '$lib/types';

  // Quick action buttons configuration
  const quickActions = [
    { name: 'Vote', icon: Vote },
    { name: 'Create Tag', icon: Plus },
    { name: 'Invite User', icon: Mail },
    { name: 'View Reports', icon: BarChart }
  ];

  // Component state
  let activeAction: string | null = null;
  let selectedTag: TagDocument | null = null;
  let tags: TagDocument[] = [];
  let suggestedTags: TagDocument[] = []; // Tags sorted by user's reputation
  let isLoading = true;
  let error: string | null = null;
  let searchQuery = '';
  let searchStatus: 'idle' | 'loading' | 'found' | 'not_found' | 'error' = 'idle';
  let searchResults: TagDocument[] = [];
  let debounceTimer: ReturnType<typeof setTimeout>;

  // Only load tags when auth is initialized
  $: if ($authUserDoneInitializing) {
    loadTags();
  }

  /**
   * Loads all tags and user reputation data.
   * This is called when auth is initialized to pre-fetch data for better UX.
   */
  async function loadTags() {
    try {
      isLoading = true;
      error = null;
      
      console.log('Starting tag load...');
      
      // Fetch all tags from the backend - use base data type for API call
      const tagsResult = await listDocs<TagData>({
        collection: 'tags'
      });
      
      console.log('Fetched tags:', tagsResult.items);
      
      // Store the complete tag documents
      tags = tagsResult.items;

      // Only fetch user's reputation if logged in
      if ($authUser) {
        console.log('User is logged in, fetching reputation documents...');
        
        // Query reputation documents for this user - use base data type for API call
        const userReputations = await queryDocsByKey<ReputationData>(
          'reputations',
          `usr_${$authUser.key}_` // Use actual user key from auth store
        );

        console.log('Fetched reputation documents:', userReputations.items);

        // Get top 5 tags by reputation_total_effective
        const topReputations = userReputations.items
          .sort((a, b) => b.data.reputation_total_effective - a.data.reputation_total_effective)
          .slice(0, 5);

        console.log('Top reputation documents:', topReputations);

        // Match reputation documents with their corresponding tags
        suggestedTags = topReputations
          .map(rep => tags.find(tag => tag.key === rep.data.tag_key))
          .filter((tag): tag is TagDocument => tag !== null);

        console.log('Suggested tags:', suggestedTags);
      } else {
        console.log('User is not logged in, skipping reputation fetch');
        suggestedTags = [];
      }
    } catch (e) {
      error = 'Failed to load tags';
      console.error('Error loading tags:', e);
    } finally {
      isLoading = false;
    }
  }

  function handleActionClick(action: string) {
    activeAction = activeAction === action ? null : action;
  }

  /**
   * Searches tags based on user input.
   * Currently performs frontend search on pre-fetched tags.
   * TODO: Move this to backend search for better performance with large datasets.
   */
  async function searchTags(query: string) {
    console.log('Searching tags with query:', query);
    searchStatus = 'loading';
    
    try {
      // Clear previous timer
      clearTimeout(debounceTimer);
      
      // Set new timer for debouncing
      debounceTimer = setTimeout(async () => {
        if (!query.trim()) {
          searchStatus = 'idle';
          searchResults = [];
          return;
        }

        console.log('Executing tag search for:', query);
        
        // Search in the loaded tags (frontend search)
        const results = tags.filter(tag => 
          tag.data.tag_handle?.toLowerCase().includes(query.toLowerCase())
        );

        console.log('Search results:', results);
        
        searchResults = results;
        searchStatus = results.length > 0 ? 'found' : 'not_found';
      }, 300); // 300ms debounce
    } catch (e) {
      console.error('Error searching tags:', e);
      searchStatus = 'error';
    }
  }

  function handleTagSelect(tag: TagDocument) {
    console.log('Tag selected:', tag);
    selectedTag = tag;
    searchQuery = tag.data.tag_handle || '';
    searchStatus = 'found';
  }
</script>

<div class="flex flex-col">
  <!-- Buttons Container -->
  <div class="card shadow bg-surface-100-900 border border-surface-200-800 p-3">
    <div class="grid grid-cols-4 gap-2">
      {#each quickActions as action}
        <button 
          class="btn preset-outlined-primary-500 flex flex-col items-center p-2 transition-all duration-200"
          class:preset-tonal-primary={activeAction === action.name}
          class:!border-0={activeAction === action.name}
          on:click={() => handleActionClick(action.name)}
        >
          <svelte:component this={action.icon} size={24} class="mb-1" />
          <span class="text-xs">{action.name}</span>
        </button>
      {/each}
    </div>
  </div>

  <!-- Expandable Content Area -->
  {#if activeAction}
    <div 
      class="px-2"
      transition:slide={{ duration: 200 }}
    >
      <div class="bg-surface-100-900/95 backdrop-blur-sm rounded-b-lg shadow-lg">
        <div class="p-4" transition:fade={{ duration: 150 }}>
          {#if activeAction === 'Vote'}
            <div class="text-sm space-y-4">
              {#if !$authUserDoneInitializing}
                <div class="text-center">Initializing...</div>
              {:else if isLoading}
                <div class="text-center">Loading tags...</div>
              {:else if error}
                <div class="text-error-500">{error}</div>
              {:else}
                <div>
                  <div class="flex items-center gap-2 mb-2">
                    <label class="label" for="tag-search">Select a Tag</label>
                    {#if suggestedTags.length > 0}
                      <div class="flex flex-wrap gap-2">
                        {#each suggestedTags as tag}
                          <button
                            class="btn btn-sm preset-tonal-primary"
                            on:click={() => handleTagSelect(tag)}
                          >
                            {tag.data.tag_handle}
                          </button>
                        {/each}
                      </div>
                    {/if}
                  </div>
                  <div class="relative">
                    <input
                      type="text"
                      id="tag-search"
                      bind:value={searchQuery}
                      on:input={(e) => searchTags(e.currentTarget.value)}
                      class="input pr-10 border-primary-300-700 focus:border-primary-500 focus:ring-primary-500 bg-surface-50-950 w-full"
                      placeholder="Search for a tag..."
                    />
                    <span class="absolute right-2 top-1/2 -translate-y-1/2" aria-live="polite">
                      {#if searchStatus === 'loading'}
                        <LoaderCircle class="animate-spin text-gray-400" />
                      {:else if searchStatus === 'found'}
                        <CheckCircle class="text-success-500" />
                      {:else if searchStatus === 'not_found'}
                        <XCircle class="text-error-500" />
                      {/if}
                    </span>
                  </div>
                  {#if searchStatus === 'not_found'}
                    <span class="text-error-500 text-xs mt-1">No tags found matching "{searchQuery}"</span>
                  {:else if searchStatus === 'found'}
                    <span class="text-success-500 text-xs mt-1">Found {searchResults.length} matching tags</span>
                  {/if}
                  
                  {#if searchResults.length > 0}
                    <div class="mt-4">
                      <h3 class="text-sm font-medium mb-2">Search Results</h3>
                      <div class="flex flex-wrap gap-2">
                        {#each searchResults as tag}
                          <button
                            class="chip preset-tonal-primary hover:scale-105 transition-transform duration-200 flex items-center gap-1.5"
                            on:click={() => handleTagSelect(tag)}
                          >
                            <span class="i-lucide-tag text-xs" />
                            #{tag.data.tag_handle}
                          </button>
                        {/each}
                      </div>
                    </div>
                  {/if}
                </div>

                {#if selectedTag}
                  <div class="mt-4 p-3 bg-surface-200-800 rounded-lg">
                    <h3 class="text-sm font-medium mb-2">Selected Tag</h3>
                    <div class="flex items-center gap-2">
                      <span class="chip preset-filled-primary-500 flex items-center gap-1.5">
                        <span class="i-lucide-check text-xs" />
                        #{selectedTag.data.tag_handle}
                      </span>
                      <button 
                        class="btn btn-sm variant-ghost-surface"
                        on:click={() => {
                          selectedTag = null;
                          searchQuery = '';
                          searchStatus = 'idle';
                        }}
                      >
                        Change
                      </button>
                    </div>
                  </div>
                {/if}
              {/if}
            </div>
          {:else if activeAction === 'Create Tag'}
            <div class="text-sm">
              Create Tag content goes here
            </div>
          {:else if activeAction === 'Invite User'}
            <div class="text-sm">
              Invite User content goes here
            </div>
          {:else if activeAction === 'View Reports'}
            <div class="text-sm">
              View Reports content goes here
            </div>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  /* Ensure the expandable content appears above other elements */
  .relative {
    z-index: 10;
  }
</style> 