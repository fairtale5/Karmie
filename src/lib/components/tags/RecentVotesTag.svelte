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
    // Import icons
    import { Expand, CirclePlus, CircleMinus, X } from 'lucide-svelte';

    // --- Preview Data Constants ---
    const PREVIEW_TAG_KEY = '___PREVIEW_DATA___';
    
    // Preview votes data
    const previewVotes = [
        {
            key: 'preview_vote_1',
            data: {
                owner_ulid: 'demo_user_1',
                target_ulid: 'demo_user_2',
                value: 1,
                tag_ulid: PREVIEW_TAG_KEY
            }
        },
        {
            key: 'preview_vote_2',
            data: {
                owner_ulid: 'demo_user_3',
                target_ulid: 'demo_user_1',
                value: -1,
                tag_ulid: PREVIEW_TAG_KEY
            }
        },
        {
            key: 'preview_vote_3',
            data: {
                owner_ulid: 'demo_user_2',
                target_ulid: 'demo_user_3',
                value: 1,
                tag_ulid: PREVIEW_TAG_KEY
            }
        }
    ];

    // Preview user data
    const previewUserData = new Map([
        ['demo_user_1', {
            key: 'usr_demo_user_1',
            data: {
                user_handle: 'alice',
                user_ulid: 'demo_user_1',
                display_name: 'Alice Demo',
                avatar_url: ''
            }
        }],
        ['demo_user_2', {
            key: 'usr_demo_user_2',
            data: {
                user_handle: 'bob',
                user_ulid: 'demo_user_2',
                display_name: 'Bob Demo',
                avatar_url: ''
            }
        }],
        ['demo_user_3', {
            key: 'usr_demo_user_3',
            data: {
                user_handle: 'carol',
                user_ulid: 'demo_user_3',
                display_name: 'Carol Demo',
                avatar_url: ''
            }
        }]
    ]);

    // --- Component Interface Definition ---
    // These props define the component's external interface and data requirements
    const { selectedTag, cutoffTimestamp, limit = 250 } = $props<{
        selectedTag: TagDocument | null;  // Current tag context from parent
        cutoffTimestamp: bigint;         // Time boundary for vote filtering
        limit?: number;                   // Maximum number of votes to display
    }>();

    // --- Internal State Management ---
    // These variables maintain the component's internal state using runes
    let votes = $state<VoteDocument[]>([]);             // Array of vote documents from Juno
    let loading = $state(true);                         // Loading state flag for UI feedback
    let error = $state<string | null>(null);            // Error state container
    let userData = $state<Map<string, UserDocument>>(new Map());  // Cache for user documents

    // Popover state for expand icon
    let expandPopoverOpen = $state(false);

    // Helper function to close expand popover
    function closeExpandPopover() {
        expandPopoverOpen = false;
    }

    // Helper function to get user initials from handle
    function getInitials(handle: string): string {
        return handle.slice(0, 2).toUpperCase();
    }

    // Helper function to get avatar URL
    function getAvatarUrl(ulid: string): string {
        return `https://images.unsplash.com/photo-1617296538902-887900d9b592?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzExMDB8&ixlib=rb-4.0.3&w=128&h=128&auto=format&fit=crop`;
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
        // Handle preview mode
        if (selectedTag?.key === PREVIEW_TAG_KEY) {
            loading = false;
            error = null;
            votes = previewVotes as VoteDocument[];
            userData = previewUserData;
            return;
        }
        
        // Only fetch if we have a valid tag with a tag_ulid
        if (selectedTag?.data?.tag_ulid) {
            fetchRecentVotes();
        } else {
            // Clear data if no valid tag
            loading = false;
            votes = [];
            userData.clear();
        }
    });
</script>

<!-- --- Component Template --- -->
<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
    <!-- Header Section -->
    <div class="flex justify-between items-center mb-4">
        <h2 class="text-lg font-bold {((!selectedTag) ? 'opacity-50' : '')}">Recent Votes</h2>
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
            <table class="table caption-bottom">
                <thead>
                    <tr>
                        <th>From</th>
                        <th>To</th>
                        <th class="text-right flex justify-end">Value</th>
                    </tr>
                </thead>
                <tbody class="[&>tr]:hover:preset-tonal-primary">
                    {#each votes as vote (vote.key)}
                        <tr>
                            <td>
                                {#if vote.data.owner_ulid && userData.get(vote.data.owner_ulid)}
                                    {@const user = userData.get(vote.data.owner_ulid)!}
                                    <div class="flex items-center gap-2">
                                        <Avatar 
                                            name={user.data.user_handle}
                                            src={user.data.avatar_url || getAvatarUrl(user.data.user_ulid)} 
                                            size="w-6"
                                            rounded="rounded-full"
                                            background="bg-transparent"
                                        >
                                            {getInitials(user.data.user_handle)}
                                        </Avatar>
                                        <span>{user.data.user_handle}</span>
                                    </div>
                                {:else}
                                    <span class="font-mono">{vote.data.owner_ulid}</span>
                                {/if}
                            </td>
                            <td>
                                {#if vote.data.target_ulid && userData.get(vote.data.target_ulid)}
                                    {@const user = userData.get(vote.data.target_ulid)!}
                                    <div class="flex items-center gap-2">
                                        <Avatar 
                                            name={user.data.user_handle}
                                            src={user.data.avatar_url || getAvatarUrl(user.data.user_ulid)} 
                                            size="w-6"
                                            rounded="rounded-full"
                                            background="bg-transparent"
                                        >
                                            {getInitials(user.data.user_handle)}
                                        </Avatar>
                                        <span>{user.data.user_handle}</span>
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
            <!-- Empty State: No Votes Message -->
            <p class="text-center opacity-70">No recent votes to display for this tag.</p>
        {/if}
    </div>
</div> 