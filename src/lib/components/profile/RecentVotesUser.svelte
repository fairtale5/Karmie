<script lang="ts">
    // Import query helper for document operations
    import { queryDocsByKey } from '$lib/docs-crud/query_by_key';
    // Import type definitions that enforce data structure compliance
    import type { VoteDocument, UserDocument, UserData, VoteData, TagDocument, TagData } from '$lib/types';
    // Import Svelte's lifecycle hook for component initialization
    import { onMount } from 'svelte';
    // Import Avatar component
    import { Avatar, Popover } from '@skeletonlabs/skeleton-svelte';
    import { getUserAvatar } from '$lib/utils/avatar';
    // Import icons
    import { Expand, Activity, X, CirclePlus, CircleMinus } from 'lucide-svelte';
    // Import dummy data for demo user
    import { dummyProfileData } from '$lib/data/dummyProfileData';
    // Import BaseCard component
    import BaseCard from '$lib/components/common/BaseCard.svelte';
    // Import UserLink component for clickable usernames
    import UserLink from '$lib/components/common/UserLink.svelte';

    // --- Component Interface Definition ---
    // These props define the component's external interface and data requirements
    const { user, limit = 250 } = $props<{
        user: UserDocument;  // User whose votes to display
        limit?: number;      // Maximum number of votes to display
    }>();

    // --- Internal State Management ---
    // These variables maintain the component's internal state using runes
    let votes = $state<VoteDocument[]>([]);             // Array of vote documents from Juno
    let loading = $state(true);                         // Loading state flag for UI feedback
    let error = $state<string | null>(null);            // Error state container
    let userData = $state<Map<string, UserDocument>>(new Map());  // Cache for user documents
    let tagData = $state<Map<string, TagDocument>>(new Map());    // Cache for tag documents
    // Individual toggle filters - all enabled by default
    let showIncoming = $state(true);                    // Show votes received by user
    let showOutgoing = $state(true);                    // Show votes cast by user  
    let showPositive = $state(true);                    // Show positive votes
    let showNegative = $state(true);                    // Show negative votes
    
    // Popover state for expand icon
    let expandPopoverOpen = $state(false);





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

    // Helper function to fetch tag data
    async function fetchTagData(tagUlid: string) {
        if (tagData.has(tagUlid)) return; // Skip if already cached
        
        try {
            const keyPattern = `tag_${tagUlid}_`;
            const results = await queryDocsByKey<TagData>('tags', keyPattern);
            
            // Ensure we have exactly one result
            if (results.items.length === 0) {
                console.warn(`No tag found for ULID: ${tagUlid}`);
                return;
            }
            if (results.items.length > 1) {
                throw new Error(`Multiple tags found for ULID: ${tagUlid}`);
            }
            
            // Store the tag document
            tagData.set(tagUlid, results.items[0] as TagDocument);
        } catch (e) {
            console.error(`Failed to fetch tag data for ${tagUlid}:`, e);
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
                
                // Pre-populate tagData with demo tag
                const demoTag: TagDocument = {
                    key: '___PREVIEW_DATA___',
                    data: {
                        tag_ulid: '___PREVIEW_DATA___',
                        tag_handle: 'demo',
                        description: 'Demo tag for preview purposes',
                        owner_ulid: 'demo_user',
                        time_periods: [],
                        reputation_threshold: 10,
                        vote_reward: 1,
                        min_users_for_threshold: 3
                    }
                };
                tagData.set('___PREVIEW_DATA___', demoTag);
                
                // Ensure the demo user themselves has an avatar by adding them to userData
                // Create a demo user document with avatar if not already present
                if (!userData.has(user.data.user_ulid)) {
                    const demoUserWithAvatar = {
                        ...user,
                        data: {
                            ...user.data,
                            avatar_url: user.data.avatar_url
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
            const uniqueTags = new Set<string>();
            votes.forEach(vote => {
                if (vote.data.owner_ulid) uniqueUsers.add(vote.data.owner_ulid);
                if (vote.data.target_ulid) uniqueUsers.add(vote.data.target_ulid);
                if (vote.data.tag_ulid) uniqueTags.add(vote.data.tag_ulid);
            });

            // Fetch user data and tag data in parallel
            await Promise.all([
                ...Array.from(uniqueUsers).map(fetchUserData),
                ...Array.from(uniqueTags).map(fetchTagData)
            ]);
            
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
                                <th>Tag</th>
                            </tr>
                        </thead>
                        <tbody class="[&>tr]:hover:preset-tonal-primary">
                            {#each filteredVotes as vote (vote.key)}
                            {@const voteValue = vote.data.value ?? 0}
                            {@const isPositive = voteValue > 0}
                            <tr class="{isPositive ? 'bg-success-50/30 dark:bg-success-500/5' : 'bg-error-50/30 dark:bg-error-500/5'}">
                                <td class="border-l-4 {isPositive ? 'border-success-500' : 'border-error-500'}">
                                    {#if vote.data.owner_ulid && userData.get(vote.data.owner_ulid)}
                                        {@const ownerUser = userData.get(vote.data.owner_ulid)!}
                                        {@const ownerAvatar = getUserAvatar(ownerUser)}
                                        <div class="flex items-center gap-2">
                                            <Avatar
                                                name={ownerAvatar.name}
                                                src={ownerAvatar.src}
                                                size="w-6"
                                                rounded="rounded-full"
                                                background="bg-transparent"
                                            >
                                                {ownerAvatar.initials}
                                            </Avatar>
                                            <UserLink handle={ownerUser.data.user_handle} />
                                        </div>
                                    {:else}
                                        <span class="font-mono text-xs">{vote.data.owner_ulid}</span>
                                    {/if}
                                </td>
                                <td>
                                    {#if vote.data.target_ulid && userData.get(vote.data.target_ulid)}
                                        {@const targetUser = userData.get(vote.data.target_ulid)!}
                                        {@const targetAvatar = getUserAvatar(targetUser)}
                                        <div class="flex items-center gap-2">
                                            <Avatar 
                                                name={targetAvatar.name}
                                                src={targetAvatar.src} 
                                                size="w-6"
                                                rounded="rounded-full"
                                                background="bg-transparent"
                                            >
                                                {targetAvatar.initials}
                                            </Avatar>
                                            <UserLink handle={targetUser.data.user_handle} />
                                        </div>
                                    {:else}
                                        <span class="font-mono text-xs">{vote.data.target_ulid}</span>
                                    {/if}
                                </td>
                                <td>
                                    {#if vote.data.tag_ulid && tagData.get(vote.data.tag_ulid)}
                                        {@const tag = tagData.get(vote.data.tag_ulid)!}
                                        <span class="chip variant-soft-primary text-xs px-2 py-1">
                                            #{tag.data.tag_handle}
                                        </span>
                                    {:else}
                                        <span class="font-mono text-xs opacity-60">{vote.data.tag_ulid}</span>
                                    {/if}
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