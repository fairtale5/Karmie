<script lang="ts">
    // Import core Juno functionality for document operations
    import { listDocs } from '@junobuild/core';
    // Import type definitions that enforce data structure compliance
    import type { VoteDocument, TagDocument, UserDocument, UserData } from '$lib/types';
    // Import Svelte's lifecycle hook for component initialization
    import { onMount } from 'svelte';
    // Import query helper
    import { queryDocsByKey } from '$lib/docs-crud/query_by_key';
    // Import Avatar component
    import { Avatar, Popover } from '@skeletonlabs/skeleton-svelte';
import { getUserAvatar } from '$lib/utils/avatar';
    // Import UserLink component for clickable usernames
    import UserLink from '$lib/components/common/UserLink.svelte';
    // Import icons
    import { Expand, CirclePlus, CircleMinus, X } from 'lucide-svelte';
    // Import centralized dummy data
    import { dummyProfileData } from '$lib/data/dummyProfileData';

    // --- Preview Data Constants ---
    const PREVIEW_TAG_KEY = '___PREVIEW_DATA___';

    // --- Component Interface Definition ---
    // These props define the component's external interface and data requirements
    const { selectedTag, cutoffTimestamp, limit = 250, refreshKey = 0 } = $props<{
        selectedTag: TagDocument | null;  // Current tag context from parent
        cutoffTimestamp: bigint;         // Time boundary for vote filtering
        limit?: number;                   // Maximum number of votes to display
        refreshKey?: number;              // Refresh trigger for re-fetching data
    }>();

    // --- Internal State Management ---
    // These variables maintain the component's internal state using runes
    let votes = $state<VoteDocument[]>([]);             // Array of vote documents from Juno
    let loading = $state(true);                         // Loading state flag for UI feedback
    let error = $state<string | null>(null);            // Error state container
    let userData = $state<Map<string, UserDocument>>(new Map());  // Cache for user documents

    // Filter states - toggle filters for better user control
    let showPositive = $state(true);                    // Show positive votes
    let showNegative = $state(true);                    // Show negative votes

    // Popover state for expand icon
    let expandPopoverOpen = $state(false);

    // Helper function to close expand popover
    function closeExpandPopover() {
        expandPopoverOpen = false;
    }



    // Filter votes based on toggle states
    function filterVotes(votes: VoteDocument[]): VoteDocument[] {
        return votes.filter(vote => {
            // For tag-level votes, we only filter by vote value (positive/negative)
            // Direction filters don't make sense without a specific user context
            const voteValue = vote.data.value ?? 0;
            const isPositive = voteValue > 0;
            const isNegative = voteValue < 0;
            
            // Apply value filters - show vote if it matches either positive or negative filter
            return (isPositive && showPositive) || (isNegative && showNegative);
        });
    }

    // Helper function to fetch user data
    async function fetchUserData(ulid: string) {
        if (userData.has(ulid)) return; // Skip if already cached
        
        try {
            const keyPattern = `usr_${ulid}_`;
            const results = await queryDocsByKey<UserData>('users', keyPattern);
            
            // Ensure we have exactly one result
            if (results.items.length === 0) {
                console.warn(`No user found for ULID: ${ulid}`);
                return;
            }
            if (results.items.length > 1) {
                throw new Error(`Multiple users found for ULID: ${ulid}`);
            }
            
            // Store the user document
            userData.set(ulid, results.items[0] as UserDocument);
        } catch (e) {
            console.error(`Failed to fetch user data for ${ulid}:`, e);
        }
    }

    // --- Data Fetching Logic ---
    // This function handles the asynchronous data retrieval from Juno
    async function fetchRecentVotes() {
        // Guard clause: exit if no tag context is available
        if (!selectedTag) return;
        
        // Reset component state for new fetch operation
        loading = true;
        error = null;
        
        try {
            // Log query parameters for debugging and monitoring
            console.log('Fetching votes with params:', {
                tagUlid: selectedTag.data.tag_ulid,
                cutoffTimestamp: cutoffTimestamp.toString(),
                keyPattern: `tag_${selectedTag.data.tag_ulid}`
            });

            // Construct and execute Juno query
            // The query uses a composite key pattern and timestamp filter to
            // query votes for this tag, ordered by creation date descending
            const result = await listDocs({
                collection: 'votes',
                filter: {
                    matcher: {
                        // Composite key pattern: 'tag_{tag_ulid}' 
                        // This creates a unique namespace for votes within a tag
                        key: `tag_${selectedTag.data.tag_ulid}`,
                        // Timestamp filter using bigint comparison
                        // Ensures votes are newer than the cutoff point
                        createdAt: {
                            matcher: "greaterThan",
                            timestamp: cutoffTimestamp
                        }
                    },
                    // Pagination control to limit result set size
                    paginate: {
                        limit: limit
                    },
                    // Sort configuration for chronological ordering
                    order: {
                        field: 'created_at',
                        desc: true
                    }
                }
            });
            
            // Type assertion to ensure data structure compliance
            votes = result.items as VoteDocument[];

            // Fetch user data for all unique users in the votes
            const uniqueUsers = new Set<string>();
            votes.forEach(vote => {
                if (vote.data.owner_ulid) uniqueUsers.add(vote.data.owner_ulid);
                if (vote.data.target_ulid) uniqueUsers.add(vote.data.target_ulid);
            });

            // Fetch user data for all unique users
            await Promise.all(Array.from(uniqueUsers).map(fetchUserData));
            
        } catch (e) {
            // Error handling with type checking
            error = e instanceof Error ? e.message : 'Failed to fetch recent votes';
            console.error('Error fetching votes:', e);
        } finally {
            // Reset loading state regardless of success/failure
            loading = false;
        }
    }

    // --- Reactive Data Flow ---
    $effect(() => {
        // Track refreshKey to ensure effect re-runs when data needs to be refreshed
        refreshKey;
        
        // Handle preview mode
        if (selectedTag?.key === PREVIEW_TAG_KEY) {
            loading = false;
            error = null;
            
            // Use the master vote list (same as TagUserReputationCard)
            votes = dummyProfileData.recentVotes;
            
            // Create new Map to avoid reactivity issues
            const newUserData = new Map();
            
            // Use the master user list (same as TagUserReputationCard)
            dummyProfileData.dummyUsers.forEach(dummyUser => {
                newUserData.set(dummyUser.data.user_ulid, dummyUser);
            });
            
            // Ensure the demo user themselves has an avatar by adding them to userData
            if (!newUserData.has('demo_user')) {
                const demoUserWithAvatar = {
                    key: 'demo_user',
                    data: {
                        user_handle: 'demo_user',
                        user_ulid: 'demo_user',
                        display_name: 'Demo User',
                        avatar_url: 'https://i.pravatar.cc/100?img=3'
                    }
                };
                newUserData.set('demo_user', demoUserWithAvatar);
            }
            
            // Set userData last to prevent reactivity loops
            userData = newUserData;
            return;
        }
        
        // Only fetch if we have a valid tag with a tag_ulid
        if (selectedTag?.data?.tag_ulid) {
            fetchRecentVotes();
        } else {
            // Clear data if no valid tag
            loading = false;
            votes = [];
            userData = new Map(); // Create new Map instead of clearing
        }
    });
</script>

<!-- --- Component Template --- -->
<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
    <!-- Header Section -->
    <div class="flex justify-between items-center mb-4">
        <h2 class="text-lg font-bold {((!selectedTag) ? 'opacity-50' : '')}">Recent Votes</h2>
        <div class="flex items-center gap-2">
            <!-- Filter Controls -->
            <div class="flex gap-2">
                <!-- Value filters -->
                <div class="flex gap-1">
                    <button type="button" class="chip text-xs px-1 py-0.5 w-6 flex justify-center items-center {showPositive ? 'preset-filled-success-500' : 'preset-tonal-surface'}" onclick={() => showPositive = !showPositive}>
                        <CirclePlus size={14} />
                    </button>
                    <button type="button" class="chip text-xs px-1 py-0.5 w-6 flex justify-center items-center {showNegative ? 'preset-filled-error-500' : 'preset-tonal-surface'}" onclick={() => showNegative = !showNegative}>
                        <CircleMinus size={14} />
                    </button>
                </div>
            </div>
            <!-- Expand Popover -->
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
                        <p class="font-bold">See More Votes</p>
                        <button class="btn-icon hover:preset-tonal" onclick={closeExpandPopover}><X class="w-4 h-4" /></button>
                    </header>
                    <article>
                        <p class="opacity-60">
                            This feature isn't available yet. In the future, you'll be able to view a comprehensive list of all votes for this tag, with advanced filtering, search, and sorting capabilities.
                        </p>
                    </article>
                {/snippet}
            </Popover>
        </div>
    </div>

    <!-- Content Section -->
    <div class="table-wrap">
        {#if loading}
            <!-- Loading State: Skeleton UI -->
            <div class="space-y-2">
                {#each Array(3) as _}
                    <div class="flex justify-between items-center placeholder animate-pulse h-8 rounded"></div>
                {/each}
            </div>
        {:else if error}
            <!-- Error State: Error Message Display -->
            <p class="text-center text-error-500">{error}</p>
        {:else if votes.length > 0}
            <!-- Success State: Vote Table -->
            {@const filteredVotes = filterVotes(votes)}
            {#if filteredVotes.length > 0}
                <table class="table caption-bottom">
                    <thead>
                        <tr>
                            <th>From</th>
                            <th>To</th>
                            <th class="text-right flex justify-end">Value</th>
                        </tr>
                    </thead>
                    <tbody class="[&>tr]:hover:preset-tonal-primary">
                        {#each filteredVotes as vote (vote.key)}
                        {@const voteValue = vote.data.value ?? 0}
                        {@const isPositive = voteValue > 0}
                        <tr class="{isPositive ? 'bg-success-50/30 dark:bg-success-500/5' : 'bg-error-50/30 dark:bg-error-500/5'}">
                            <td class="border-l-4 {isPositive ? 'border-success-500' : 'border-error-500'}">
                                {#if vote.data.owner_ulid && userData.get(vote.data.owner_ulid)}
                                    {@const user = userData.get(vote.data.owner_ulid)!}
                                    {@const userAvatar = getUserAvatar(user)}
                                    <div class="flex items-center gap-2">
                                        <Avatar 
                                            name={userAvatar.name}
                                            src={userAvatar.src} 
                                            size="w-6"
                                            rounded="rounded-full"
                                            background="bg-transparent"
                                        >
                                            {userAvatar.initials}
                                        </Avatar>
                                        <UserLink handle={user.data.user_handle} />
                                    </div>
                                {:else}
                                    <span class="font-mono">{vote.data.owner_ulid}</span>
                                {/if}
                            </td>
                            <td>
                                {#if vote.data.target_ulid && userData.get(vote.data.target_ulid)}
                                    {@const user = userData.get(vote.data.target_ulid)!}
                                    {@const userAvatar = getUserAvatar(user)}
                                    <div class="flex items-center gap-2">
                                        <Avatar 
                                            name={userAvatar.name}
                                            src={userAvatar.src} 
                                            size="w-6"
                                            rounded="rounded-full"
                                            background="bg-transparent"
                                        >
                                            {userAvatar.initials}
                                        </Avatar>
                                        <UserLink handle={user.data.user_handle} />
                                    </div>
                                {:else}
                                    <span class="font-mono">{vote.data.target_ulid}</span>
                                {/if}
                            </td>
                            <td class="text-right">
                                <span class="chip-icon preset-filled-{(vote.data.value ?? 0) > 0 ? 'success' : 'error'}-500 w-5 h-5">
                                    {#if (vote.data.value ?? 0) > 0}
                                        <CirclePlus size={19} />
                                    {:else}
                                        <CircleMinus size={19} />
                                    {/if}
                                </span>
                            </td>
                        </tr>
                        {/each}
                    </tbody>
                </table>
            {:else}
                <p class="text-center opacity-70">No votes match the selected filters.</p>
            {/if}
        {:else}
            <!-- Empty State: No Votes Message -->
            <p class="text-center opacity-70">No recent votes to display for this tag.</p>
        {/if}
    </div>
</div> 