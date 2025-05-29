<script lang="ts">
    // Import core Juno functionality for document operations
    import { listDocs } from '@junobuild/core';
    // Import type definitions that enforce data structure compliance
    import type { VoteDocument, TagDocument } from '$lib/types';
    // Import Svelte's lifecycle hook for component initialization
    import { onMount } from 'svelte';

    // --- Component Interface Definition ---
    // These props define the component's external interface and data requirements
    const { selectedTag, cutoffTimestamp, limit = 20 } = $props<{
        selectedTag: TagDocument | null;  // Current tag context from parent
        cutoffTimestamp: bigint;         // Time boundary for vote filtering
        limit?: number;                   // Maximum number of votes to display
    }>();

    // --- Internal State Management ---
    // These variables maintain the component's internal state using runes
    let votes = $state<VoteDocument[]>([]);             // Array of vote documents from Juno
    let loading = $state(true);                         // Loading state flag for UI feedback
    let error = $state<string | null>(null);            // Error state container

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
            
            // Log query results for debugging and monitoring
            console.log('Query result:', {
                itemsCount: result.items.length,
                firstItem: result.items[0],
                matchesLength: result.matches_length.toString()
            });
            
            // Type assertion to ensure data structure compliance
            votes = result.items as VoteDocument[];
        } catch (e) {
            // Error handling with type checking
            // Converts unknown error to string message
            error = e instanceof Error ? e.message : 'Failed to fetch recent votes';
            console.error('Error fetching votes:', e);
        } finally {
            // Reset loading state regardless of success/failure
            loading = false;
        }

    }

    // --- Reactive Data Flow ---
    $effect(() => {
        // Only fetch if we have a valid tag with a tag_ulid
        if (selectedTag?.data?.tag_ulid) {
            fetchRecentVotes();
        }
    });
</script>

<!-- --- Component Template --- -->
<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
    <!-- Header Section -->
    <div class="flex justify-between items-center mb-4">
        <h2 class="text-lg font-bold {((!selectedTag) ? 'opacity-50' : '')}">Recent Votes</h2>
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
                        <th class="text-right">Value</th>
                    </tr>
                </thead>
                <tbody class="[&>tr]:hover:preset-tonal-primary">
                    {#each votes as vote (vote.key)}
                        <tr>
                            <td class="font-mono">{vote.data.owner_ulid}</td>
                            <td class="font-mono">{vote.data.target_ulid}</td>
                            <td class="text-right">
                                <span class="badge preset-filled-{(vote.data.value ?? 0) > 0 ? 'success' : 'error'}-500">
                                    {(vote.data.value ?? 0) > 0 ? '+1' : '-1'}
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