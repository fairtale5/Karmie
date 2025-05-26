<script lang="ts">
  import { Plus, Mail, BarChart, Vote, LoaderCircle, CheckCircle, XCircle, ThumbsUp, ThumbsDown } from 'lucide-svelte';
  import { slide } from 'svelte/transition';
  import { fade } from 'svelte/transition';
  import { listDocs } from '@junobuild/core';
  import { queryDocsByKey } from '$lib/docs-crud/query_by_key';
  import { authUserDoneInitializing, authUser } from '$lib/stores/authUser';
  import type { TagDocument, UserDocument, ReputationDocument, TagData, UserData, ReputationData } from '$lib/types';
  import { createVoteDoc } from '$lib/docs-crud/vote_create';
  import { toaster } from '$lib/skeletonui/toaster-skeleton';
  import { authUserDoc } from '$lib/stores/authUserDoc';
  import { isValid } from 'ulid';

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
  let selectedUser: UserDocument | null = null;
  let tags: TagDocument[] = [];
  let users: UserDocument[] = [];
  let suggestedTags: TagDocument[] = []; // Tags sorted by user's reputation
  let isLoading = true;
  let error: string | null = null;
  let tagSearchQuery = '';
  let userSearchQuery = '';
  let tagSearchStatus: 'idle' | 'loading' | 'found' | 'not_found' | 'error' = 'idle';
  let userSearchStatus: 'idle' | 'loading' | 'found' | 'not_found' | 'error' = 'idle';
  let tagSearchResults: TagDocument[] = [];
  let userSearchResults: UserDocument[] = [];
  let tagDebounceTimer: ReturnType<typeof setTimeout>;
  let userDebounceTimer: ReturnType<typeof setTimeout>;
  let currentFocus: 'tag' | 'user' | 'vote' = 'tag';
  let selectedVoteValue: number | null = null;
  let isVoting = false;

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
          .map(rep => tags.find(tag => tag.key === rep.data.tag_ulid))
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
    if (!activeAction) {
      // Reset state when closing
      selectedTag = null;
      selectedUser = null;
      currentFocus = 'tag';
      tagSearchQuery = '';
      userSearchQuery = '';
      tagSearchResults = [];
      userSearchResults = [];
    }
  }

  /**
   * Searches tags based on user input.
   * Currently performs frontend search on pre-fetched tags.
   */
  async function searchTags(query: string) {
    console.log('Searching tags with query:', query);
    tagSearchStatus = 'loading';
    currentFocus = 'tag';
    
    try {
      // Clear previous timer
      clearTimeout(tagDebounceTimer);
      
      // Set new timer for debouncing
      tagDebounceTimer = setTimeout(async () => {
        if (!query.trim()) {
          tagSearchStatus = 'idle';
          tagSearchResults = [];
          return;
        }

        console.log('Executing tag search for:', query);
        
        // Search in the loaded tags (frontend search)
        const results = tags.filter(tag => 
          tag.data.tag_handle?.toLowerCase().includes(query.toLowerCase())
        );

        console.log('Search results:', results);
        
        tagSearchResults = results;
        tagSearchStatus = results.length > 0 ? 'found' : 'not_found';
      }, 300); // 300ms debounce
    } catch (e) {
      console.error('Error searching tags:', e);
      tagSearchStatus = 'error';
    }
  }

  /**
   * Searches users based on user input.
   * Searches across any part of the user's key (username, handle, principal, ulid).
   */
  async function searchUsers(query: string) {
    console.log('Searching users with query:', query);
    userSearchStatus = 'loading';
    currentFocus = 'user';
    
    try {
      // Clear previous timer
      clearTimeout(userDebounceTimer);
      
      // Set new timer for debouncing
      userDebounceTimer = setTimeout(async () => {
        if (!query.trim()) {
          userSearchStatus = 'idle';
          userSearchResults = [];
          return;
        }

        console.log('Executing user search for:', query);
        
        // Search users by any part of their key
        const results = await queryDocsByKey<UserData>(
          'users',
          query.toLowerCase() // Search across any part of the key
        );

        console.log('User search results:', results);
        
        userSearchResults = results.items;
        userSearchStatus = results.items.length > 0 ? 'found' : 'not_found';
      }, 300); // 300ms debounce
    } catch (e) {
      console.error('Error searching users:', e);
      userSearchStatus = 'error';
    }
  }

  function handleTagSelect(tag: TagDocument) {
    console.log('Tag selected:', tag);
    selectedTag = tag;
    tagSearchQuery = tag.data.tag_handle || '';
    tagSearchStatus = 'found';
    tagSearchResults = [];
    currentFocus = 'user';
  }

  function handleUserSelect(user: UserDocument) {
    console.log('User selected:', user);
    selectedUser = user;
    userSearchQuery = user.data.user_handle;
    userSearchStatus = 'found';
    userSearchResults = [];
    currentFocus = 'vote';
  }

  async function handleVote(value: number) {
    if (!selectedTag || !selectedUser) return;
    selectedVoteValue = value;
  }

  async function confirmVote() {
    try {
        // Check if all required selections exist in memory
        if (!selectedTag || !selectedUser || selectedVoteValue === undefined || !$authUserDoc) {
            throw new Error('Please select a tag, user, and vote value');
        }

        // Debug logging
        console.log('[QuickActions] Auth User Doc:', $authUserDoc);
        console.log('[QuickActions] Selected User:', selectedUser);
        console.log('[QuickActions] Selected Tag:', selectedTag);

        // Create vote document with empty key to satisfy TypeScript
        const voteDoc = {
            key: '', // Empty key string to satisfy TypeScript
            data: {
                owner_ulid: $authUserDoc.data.user_ulid,
                target_ulid: selectedUser.data.user_ulid,
                tag_ulid: selectedTag.data.tag_ulid || selectedTag.key,
                value: Number(selectedVoteValue), // Ensure value is a number
                weight: 1 // Default weight
            }
        };

        // Log the vote document for debugging
        console.log('[QuickActions] Vote document before sending:', voteDoc);

        // Validate ULID format using ulid package's isValid function
        if (!isValid(voteDoc.data.owner_ulid)) {
            throw new Error('Invalid owner ULID format');
        }
        if (!isValid(voteDoc.data.target_ulid)) {
            throw new Error('Invalid target ULID format');
        }
        if (!isValid(voteDoc.data.tag_ulid)) {
            throw new Error('Invalid tag ULID format');
        }

        // Use toaster.promise() for consistent handling
        await toaster.promise(
            (async () => {
                // Call vote_create.ts to create the vote document
                await createVoteDoc(voteDoc);
                // Add a small delay to ensure toast is visible
                await new Promise(resolve => setTimeout(resolve, 1000));
            })(),
            {
                loading: {
                    title: 'Recording Vote on the Blockchain',
                    description: 'Please wait while we store your vote on the ICP blockchain...'
                },
                success: () => ({
                    title: 'Vote Recorded!',
                    description: 'Your vote has been stored on-chain.'
                }),
                error: (e) => ({
                    title: 'Failed to Record Vote',
                    description: e instanceof Error ? e.message : 'An unknown error occurred'
                })
            }
        );

        // Reset form after successful vote
        selectedTag = null;
        selectedUser = null;
        selectedVoteValue = null;
        tagSearchQuery = '';
        userSearchQuery = '';
        currentFocus = 'tag';
    } catch (error) {
        console.error('Error creating vote:', error);
    }
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
            <div class="text-sm">
              {#if !$authUserDoneInitializing}
                <div class="text-center">Initializing...</div>
              {:else if isLoading}
                <div class="text-center">Loading tags...</div>
              {:else if error}
                <div class="text-error-500">{error}</div>
              {:else}
                <div class="grid grid-cols-2 gap-4">
                  <!-- Left Column: Input Fields -->
                  <div class="space-y-4">
                    <!-- Tag Search -->
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
                          bind:value={tagSearchQuery}
                          on:input={(e) => searchTags(e.currentTarget.value)}
                          class="input pr-10 border-primary-300-700 focus:border-primary-500 focus:ring-primary-500 bg-surface-50-950 w-full"
                          placeholder="Search for a tag..."
                        />
                        <span class="absolute right-2 top-1/2 -translate-y-1/2" aria-live="polite">
                          {#if tagSearchStatus === 'loading'}
                            <LoaderCircle class="animate-spin text-gray-400" />
                          {:else if tagSearchStatus === 'found'}
                            <CheckCircle class="text-success-500" />
                          {:else if tagSearchStatus === 'not_found'}
                            <XCircle class="text-error-500" />
                          {/if}
                        </span>
                      </div>
                      {#if tagSearchStatus === 'not_found'}
                        <span class="text-error-500 text-xs mt-1">No tags found matching "{tagSearchQuery}"</span>
                      {:else if tagSearchStatus === 'found' && !selectedTag}
                        <span class="text-success-500 text-xs mt-1">Found {tagSearchResults.length} matching tags</span>
                      {:else if selectedTag}
                        <span class="text-success-500 text-xs mt-1">Selected #{selectedTag.data.tag_handle}</span>
                      {/if}
                    </div>

                    <!-- User Search -->
                    {#if selectedTag}
                      <div>
                        <label class="label" for="user-search">Select a User</label>
                        <div class="relative">
                          <input
                            type="text"
                            id="user-search"
                            bind:value={userSearchQuery}
                            on:input={(e) => searchUsers(e.currentTarget.value)}
                            class="input pr-10 border-primary-300-700 focus:border-primary-500 focus:ring-primary-500 bg-surface-50-950 w-full"
                            placeholder="Search for a user..."
                          />
                          <span class="absolute right-2 top-1/2 -translate-y-1/2" aria-live="polite">
                            {#if userSearchStatus === 'loading'}
                              <LoaderCircle class="animate-spin text-gray-400" />
                            {:else if userSearchStatus === 'found'}
                              <CheckCircle class="text-success-500" />
                            {:else if userSearchStatus === 'not_found'}
                              <XCircle class="text-error-500" />
                            {/if}
                          </span>
                        </div>
                        {#if userSearchStatus === 'not_found'}
                          <span class="text-error-500 text-xs mt-1">No users found matching "{userSearchQuery}"</span>
                        {:else if userSearchStatus === 'found' && !selectedUser}
                          <span class="text-success-500 text-xs mt-1">Found {userSearchResults.length} matching users</span>
                        {:else if selectedUser}
                          <span class="text-success-500 text-xs mt-1">Selected @{selectedUser.data.user_handle}</span>
                        {/if}
                      </div>
                    {/if}
                  </div>

                  <!-- Right Column: Search Results or Vote Options -->
                  <div class="space-y-4">
                    {#if currentFocus === 'tag' && tagSearchResults.length > 0}
                      <div class="space-y-2 max-h-[300px] overflow-y-auto">
                        {#each tagSearchResults as tag}
                          <button
                            class="card shadow bg-surface-100-900 border border-surface-200-800 p-3 w-full text-left hover:preset-tonal-primary transition-colors duration-200"
                            on:click={() => handleTagSelect(tag)}
                          >
                            <div class="flex items-center gap-2">
                              <span class="i-lucide-tag text-primary-500"></span>
                              <span class="font-bold">#{tag.data.tag_handle}</span>
                            </div>
                          </button>
                        {/each}
                      </div>
                    {:else if currentFocus === 'user' && userSearchResults.length > 0}
                      <div class="space-y-2 max-h-[300px] overflow-y-auto">
                        {#each userSearchResults as user}
                          <button
                            class="card shadow bg-surface-100-900 border border-surface-200-800 p-3 w-full text-left hover:preset-tonal-primary transition-colors duration-200"
                            on:click={() => handleUserSelect(user)}
                          >
                            <div class="flex items-center gap-3">
                              <figure class="overflow-hidden isolate bg-surface-400-600 size-10 rounded-full">
                                {#if user.data.avatar_url}
                                  <img src={user.data.avatar_url} alt="" class="w-full object-cover" />
                                {:else}
                                  <span class="w-full h-full flex justify-center items-center text-surface-700">
                                    {user.data.display_name?.[0]?.toUpperCase() || '?'}
                                  </span>
                                {/if}
                              </figure>
                              <div>
                                <p class="font-bold">{user.data.display_name}</p>
                                <p class="opacity-60 text-xs">@{user.data.user_handle}</p>
                              </div>
                            </div>
                          </button>
                        {/each}
                      </div>
                    {:else if currentFocus === 'vote' && selectedTag && selectedUser}
                      <div class="flex flex-col items-center justify-center h-full gap-4">
                        <p class="text-center opacity-70">
                          Vote for <span class="font-bold">@{selectedUser.data.user_handle}</span>
                          <br>in <span class="font-bold">#{selectedTag.data.tag_handle}</span>
                        </p>
                        <div class="flex gap-4">
                          <button
                            class="btn preset-filled-success-500 p-4 transition-all duration-200"
                            class:border-2={selectedVoteValue === 1}
                            class:border-success-500={selectedVoteValue === 1}
                            class:opacity-75={selectedVoteValue === -1}
                            on:click={() => handleVote(1)}
                            disabled={isVoting}
                          >
                            <ThumbsUp size={24} />
                          </button>
                          <button
                            class="btn preset-filled-error-500 p-4 transition-all duration-200"
                            class:border-2={selectedVoteValue === -1}
                            class:border-error-500={selectedVoteValue === -1}
                            class:opacity-75={selectedVoteValue === 1}
                            on:click={() => handleVote(-1)}
                            disabled={isVoting}
                          >
                            <ThumbsDown size={24} />
                          </button>
                        </div>
                        <!-- Placeholder div to prevent layout shift -->
                        <div class="h-[40px] mt-4">
                          {#if selectedVoteValue !== null}
                            <button
                              class="btn preset-filled-primary-500 w-full"
                              on:click={confirmVote}
                              disabled={isVoting}
                            >
                              {#if isVoting}
                                <LoaderCircle class="animate-spin mr-2" />
                                Recording Vote...
                              {:else}
                                Confirm Vote
                              {/if}
                            </button>
                          {/if}
                        </div>
                      </div>
                    {:else}
                      <div class="flex items-center justify-center h-full">
                        <p class="text-center opacity-70">
                          {#if !selectedTag}
                            Search for a tag to begin
                          {:else if !selectedUser}
                            Search for a user to vote on
                          {/if}
                        </p>
                      </div>
                    {/if}
                  </div>
                </div>
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