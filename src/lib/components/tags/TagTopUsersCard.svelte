<script lang="ts">
  import { Expand, UserRoundPen, X, ShieldCheck } from 'lucide-svelte';
  import { Avatar, Popover } from '@skeletonlabs/skeleton-svelte';
  import BaseCard from '$lib/components/common/BaseCard.svelte';
  import UserLink from '$lib/components/common/UserLink.svelte';
  import type { TagDocument, UserDocument, ReputationDocument, UserData } from '$lib/types';
  import { queryDocsByKey } from '$lib/docs-crud/query_by_key';
  import { dummyProfileData } from '$lib/data/dummyProfileData';
  import { REPUTATION_SETTINGS } from '$lib/settings';

  const { 
    tag, 
    loading = false,
    isPreview = false
  }: { 
    tag: TagDocument | null; 
    loading?: boolean;
    isPreview?: boolean;
  } = $props();

  // Preview data constants
  const PREVIEW_TAG_KEY = '___PREVIEW_DATA___';

  // Internal state for real data fetching
  let topUsers = $state<Array<{
    userDocument: UserDocument;
    reputationDocument: ReputationDocument;
    score: number;
    isTrusted: boolean;
  }>>([]);
  let componentLoading = $state(false);
  let error = $state<string | null>(null);
  let userData = $state<Map<string, UserDocument>>(new Map());

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

  // Helper function to format reputation score
  function formatScore(score: number): string {
    const { DECIMAL_PLACES, WHOLE_NUMBERS } = REPUTATION_SETTINGS.UI;
    
    if (WHOLE_NUMBERS) {
      return Math.round(score).toString();
    } else {
      return score.toFixed(DECIMAL_PLACES);
    }
  }

  // Helper function to fetch user data (following pattern from RecentVotes components)
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

  // Main data fetching function
  async function fetchTopUsers() {
    if (!tag?.data?.tag_ulid) return;
    
    componentLoading = true;
    error = null;
    
    try {
      // Step 1: Get all reputation documents for this tag
      const reputationResults = await queryDocsByKey('reputations', `tag_${tag.data.tag_ulid}_`);
      const reputationDocs = reputationResults.items as ReputationDocument[];
      
      if (reputationDocs.length === 0) {
        topUsers = [];
        return;
      }

      // Step 2: Extract unique user ULIDs and fetch user data in parallel
      const uniqueUserUlids = new Set<string>();
      reputationDocs.forEach(rep => {
        if (rep.data.owner_ulid) {
          uniqueUserUlids.add(rep.data.owner_ulid);
        }
      });

      // Fetch all user data in parallel (following RecentVotes pattern)
      await Promise.all(Array.from(uniqueUserUlids).map(fetchUserData));

      // Step 3: Combine reputation and user data, then sort by score
      const combinedData = reputationDocs
        .map(rep => {
          const userDoc = userData.get(rep.data.owner_ulid);
          if (!userDoc) return null;
          
          return {
            userDocument: userDoc,
            reputationDocument: rep,
            score: rep.data.reputation_total_effective || 0,
            isTrusted: rep.data.has_voting_power === true
          };
        })
        .filter(item => item !== null)
        .sort((a, b) => b.score - a.score) // Sort by score descending
        .slice(0, 250); // Limit to top 250 users

      topUsers = combinedData;
      
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to fetch top users';
      console.error('Error fetching top users:', e);
    } finally {
      componentLoading = false;
    }
  }

  // Track the current tag ULID to prevent unnecessary re-fetches
  let currentTagUlid = $state<string | null>(null);

  // Reactive data fetching when tag changes
  $effect(() => {
    // Handle preview mode
    if (tag?.key === PREVIEW_TAG_KEY) {
      currentTagUlid = PREVIEW_TAG_KEY;
      componentLoading = false;
      error = null;
      
      // Generate topUsers from master user list
      topUsers = dummyProfileData.dummyUsers.slice(0, 10).map((user, index) => ({
        userDocument: user,
        reputationDocument: {
          key: `rep_${user.data.user_ulid}`,
          data: {
            owner_ulid: user.data.user_ulid,
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 100 - index * 10,
            has_voting_power: index < 5
          }
        } as ReputationDocument,
        score: 100 - index * 10,
        isTrusted: index < 5
      }));
      
      return;
    }
    
    const tagUlid = tag?.data?.tag_ulid;
    
    if (tagUlid && tagUlid !== currentTagUlid) {
      currentTagUlid = tagUlid;
      fetchTopUsers();
    } else if (!tagUlid && currentTagUlid) {
      currentTagUlid = null;
      topUsers = [];
      error = null;
    }
  });
</script>

<BaseCard>
  {#snippet header()}
    <h2 class="text-lg font-bold {(!tag || loading) ? 'opacity-50' : ''}">Top Users</h2>
  {/snippet}
  
  {#snippet actions()}
    {#if tag && !isPreview}
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
            <p class="font-bold">See More Users</p>
            <button class="btn-icon hover:preset-tonal" onclick={closeExpandPopover}><X class="w-4 h-4" /></button>
          </header>
          <article>
            <p class="opacity-60">
              This feature isn't available yet. In the future, you'll be able to view a comprehensive list of all users in this tag, with their reputation scores, rankings, and activity metrics.
            </p>
          </article>
        {/snippet}
      </Popover>
    {/if}
  {/snippet}
  
  {#snippet children()}
    {#if loading || componentLoading}
      <div class="space-y-2">
        {#each Array(3) as _}
          <div class="flex items-center gap-2 placeholder animate-pulse h-10 rounded"></div>
        {/each}
      </div>
    {:else if error}
      <p class="text-center text-error-500">{error}</p>
    {:else if tag && topUsers.length > 0}
      <div class="table-wrap">
        <table class="table caption-bottom">
          <thead>
            <tr>
              <th>User</th>
              <th class="text-right">Score</th>
            </tr>
          </thead>
          <tbody class="[&>tr]:hover:preset-tonal-primary">
            {#each topUsers as user, i (user.userDocument.key)}
              <tr>
                <td>
                  <div class="flex items-center gap-2">
                    <Avatar 
                      name={user.userDocument.data.user_handle}
                      src={user.userDocument.data.avatar_url || getAvatarUrl(user.userDocument.data.user_ulid)}
                      size="w-8 h-8"
                      rounded="rounded-full"
                      background="bg-transparent"
                    >
                      {getInitials(user.userDocument.data.user_handle)}
                    </Avatar>
                    <div class="flex flex-col">
                      <span class="font-bold text-sm">{user.userDocument.data.display_name}</span>
                      <UserLink handle={user.userDocument.data.user_handle} showAt={true} class="text-xs opacity-60" />
                    </div>
                    <div class="flex items-center gap-1">
                      {#if i === 0}
                        <span class="text-yellow-500">🥇</span>
                      {:else if i === 1}
                        <span class="text-gray-400">🥈</span>
                      {:else if i === 2}
                        <span class="text-orange-700">🥉</span>
                      {/if}
                      {#if user.isTrusted}
                        <ShieldCheck class="w-4 h-4 text-success-500" />
                      {/if}
                    </div>
                  </div>
                </td>
                <td class="text-right">
                  <span class="badge preset-filled-secondary-500">{formatScore(user.score)}</span>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {:else if tag && topUsers.length === 0}
      <p class="text-center opacity-70">No top users to display for this tag.</p>
    {:else}
      <p class="text-center opacity-70">Loading users...</p>
    {/if}
  {/snippet}
</BaseCard> 