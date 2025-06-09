<script lang="ts">
    // Import query helper for document operations
    import { queryDocsByKey } from '$lib/docs-crud/query_by_key';
    // Import type definitions that enforce data structure compliance
    import type { VoteDocument, UserDocument, UserData, VoteData } from '$lib/types';
    // Import Svelte's lifecycle hook for component initialization
    import { onMount } from 'svelte';
    // Import Avatar component
    import { Avatar, Popover } from '@skeletonlabs/skeleton-svelte';
    // Import icons
    import { Expand, Activity, X, CirclePlus, CircleMinus } from 'lucide-svelte';
    // Import dummy data for demo user
    import { dummyProfileData } from '$lib/data/dummyProfileData';
    // Import BaseCard component
    import BaseCard from '$lib/components/common/BaseCard.svelte';

    // --- Component Interface Definition ---
    // These props define the component's external interface and data requirements
    const { user, limit = 20 } = $props<{
        user: UserDocument;  // User whose votes to display
        limit?: number;      // Maximum number of votes to display
    }>();

    // --- Internal State Management ---
    // These variables maintain the component's internal state using runes
    let votes = $state<VoteDocument[]>([]);             // Array of vote documents from Juno
    let loading = $state(true);                         // Loading state flag for UI feedback
    let error = $state<string | null>(null);            // Error state container
    let userData = $state<Map<string, UserDocument>>(new Map());  // Cache for user documents
    // Individual toggle filters - all enabled by default
    let showIncoming = $state(true);                    // Show votes received by user
    let showOutgoing = $state(true);                    // Show votes cast by user  
    let showPositive = $state(true);                    // Show positive votes
    let showNegative = $state(true);                    // Show negative votes
    
    // Popover state for expand icon
    let expandPopoverOpen = $state(false);

    // Helper function to get user initials from handle
    function getInitials(handle: string): string {
        return handle.slice(0, 2).toUpperCase();
    }

    // Helper function to get avatar URL
    function getAvatarUrl(ulid: string): string {
        return `https://images.unsplash.com/photo-1617296538902-887900d9b592?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzExMDB8&ixlib=rb-4.0.3&w=128&h=128&auto=format&fit=crop`;
    }

    // Helper function to close expand popover
    function closeExpandPopover() {
        expandPopoverOpen = false;
    }

    // Helper function to filter votes based on toggle states
    function filterVotes(votes: VoteDocument[]): VoteDocument[] {
        return votes.filter(vote => {
            // Check direction filters
            const isIncoming = vote.data.target_ulid === user.data.user_ulid;
            const isOutgoing = vote.data.owner_ulid === user.data.user_ulid;
            const directionMatch = (isIncoming && showIncoming) || (isOutgoing && showOutgoing);
            
            // Check value filters
            const voteValue = vote.data.value ?? 0;
            const isPositive = voteValue > 0;
            const isNegative = voteValue < 0;
            const valueMatch = (isPositive && showPositive) || (isNegative && showNegative);
            
            return directionMatch && valueMatch;
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
        // Guard clause: exit if no user context is available
        if (!user?.data?.user_ulid) return;
        
        // Reset component state for new fetch operation
        loading = true;
        error = null;
        
        try {
            // Special case: Demo user - use dummy data
            if (user.data.user_handle === 'demo_user') {
                // Use dummy vote data
                votes = dummyProfileData.recentVotes.slice(0, limit);
                
                // Pre-populate userData with dummy users
                dummyProfileData.dummyUsers.forEach(dummyUser => {
                    userData.set(dummyUser.data.user_ulid, dummyUser);
                });
                
                // Ensure the demo user themselves has an avatar by adding them to userData
                // Create a demo user document with avatar if not already present
                if (!userData.has(user.data.user_ulid)) {
                    const demoUserWithAvatar = {
                        ...user,
                        data: {
                            ...user.data,
                            avatar_url: user.data.avatar_url || getAvatarUrl(user.data.user_ulid)
                        }
                    };
                    userData.set(user.data.user_ulid, demoUserWithAvatar);
                }
                
                loading = false;
                return;
            }

            // Log query parameters for debugging and monitoring
            console.log('Fetching user votes with params:', {
                userUlid: user.data.user_ulid,
                keyPattern: `${user.data.user_ulid}_`
            });

            // Query votes where this user is either the voter or the target
            // The query uses just the user ULID to find all votes involving this user
            const result = await queryDocsByKey<VoteData>('votes', `${user.data.user_ulid}_`);
            
            // Sort by creation date descending and limit results
            const sortedVotes = result.items.sort((a, b) => {
                const timeA = a.created_at || 0n;
                const timeB = b.created_at || 0n;
                return timeA < timeB ? 1 : timeA > timeB ? -1 : 0;
            });
            
            // Apply limit to keep only the most recent votes
            votes = sortedVotes.slice(0, limit);

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
            console.error('Error fetching user votes:', e);
        } finally {
            // Reset loading state regardless of success/failure
            loading = false;
        }
    }

    // --- Reactive Data Flow ---
    $effect(() => {
        // Only fetch if we have a valid user with a user_ulid
        if (user?.data?.user_ulid) {
            fetchRecentVotes();
        }
    });
</script>

<!-- --- Component Template --- -->
    <BaseCard 
        header={headerSnippet}
        actions={actionsSnippet}
        children={contentSnippet}
    >
    </BaseCard>

{#snippet headerSnippet()}
    <Activity class="text-primary-500" size={20} />
    <h2 class="text-lg font-bold">Recent Votes</h2>
{/snippet}

{#snippet actionsSnippet()}
    <!-- Filter Controls -->
    <div class="flex gap-2">
        <!-- Direction filters (closer spacing) -->
        <div class="flex gap-1">
            <button type="button" class="chip text-xs px-2 py-0.5 w-8 {showIncoming ? 'preset-filled-secondary-500' : 'preset-tonal-surface'}" onclick={() => showIncoming = !showIncoming}>In</button>
            <button type="button" class="chip text-xs px-2 py-0.5 w-8 {showOutgoing ? 'preset-filled-tertiary-500' : 'preset-tonal-surface'}" onclick={() => showOutgoing = !showOutgoing}>Out</button>
        </div>
        <!-- Value filters (closer spacing) -->
        <div class="flex gap-1">
            <button type="button" class="chip text-xs px-1 py-0.5 w-6 flex justify-center items-center {showPositive ? 'preset-filled-success-500' : 'preset-tonal-surface'}" onclick={() => showPositive = !showPositive}>
                <CirclePlus size={14} />
            </button>
            <button type="button" class="chip text-xs px-1 py-0.5 w-6 flex justify-center items-center {showNegative ? 'preset-filled-error-500' : 'preset-tonal-surface'}" onclick={() => showNegative = !showNegative}>
                <CircleMinus size={14} />
            </button>
        </div>
    </div>
    <!-- Expand Icon with Popover -->
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
                    This feature isn't available yet. In the future, you'll be able to view a comprehensive list of all votes for this user, with advanced filtering, search, and sorting capabilities.
                </p>
            </article>
        {/snippet}
    </Popover>
{/snippet}

{#snippet contentSnippet()}
    <!-- Content Section -->
    <div class="max-h-64 overflow-y-auto">
        {#if loading}
            <!-- Loading State: Skeleton UI -->
            <div class="space-y-2">
                {#each Array(5) as _}
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
                <div class="table-wrap">
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
                            <tr>
                                <td>
                                    {#if vote.data.owner_ulid && userData.get(vote.data.owner_ulid)}
                                        {@const ownerUser = userData.get(vote.data.owner_ulid)!}
                                        <div class="flex items-center gap-2">
                                            <Avatar 
                                                name={ownerUser.data.user_handle}
                                                src={ownerUser.data.avatar_url || getAvatarUrl(ownerUser.data.user_ulid)} 
                                                size="w-6"
                                                rounded="rounded-full"
                                                background="bg-transparent"
                                            >
                                                {getInitials(ownerUser.data.user_handle)}
                                            </Avatar>
                                            <span>{ownerUser.data.user_handle}</span>
                                        </div>
                                    {:else}
                                        <span class="font-mono text-xs">{vote.data.owner_ulid}</span>
                                    {/if}
                                </td>
                                <td>
                                    {#if vote.data.target_ulid && userData.get(vote.data.target_ulid)}
                                        {@const targetUser = userData.get(vote.data.target_ulid)!}
                                        <div class="flex items-center gap-2">
                                            <Avatar 
                                                name={targetUser.data.user_handle}
                                                src={targetUser.data.avatar_url || getAvatarUrl(targetUser.data.user_ulid)} 
                                                size="w-6"
                                                rounded="rounded-full"
                                                background="bg-transparent"
                                            >
                                                {getInitials(targetUser.data.user_handle)}
                                            </Avatar>
                                            <span>{targetUser.data.user_handle}</span>
                                        </div>
                                    {:else}
                                        <span class="font-mono text-xs">{vote.data.target_ulid}</span>
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
                </div>
            {:else}
                <p class="text-center opacity-70">No votes match the selected filter.</p>
            {/if}
        {:else}
            <!-- Empty State: No Votes Message -->
            <p class="text-center opacity-70">No recent votes to display for this user.</p>
        {/if}
    </div>
{/snippet} 