<script lang="ts">
  import { Expand, X, CirclePlus, CircleMinus, Activity } from 'lucide-svelte';
  import BaseCard from '$lib/components/common/BaseCard.svelte';
  import UserLink from '$lib/components/common/UserLink.svelte';
  import { authUserDoc } from '$lib/stores/authUserDoc';
  import type { TagDocument, VoteDocument, UserDocument, VoteData, UserData, ReputationDocument } from '$lib/types';
  import { Popover, Avatar } from '@skeletonlabs/skeleton-svelte';
  import { queryDocsByKey } from '$lib/docs-crud/query_by_key';
  import { dummyProfileData } from '$lib/data/dummyProfileData';
  import { REPUTATION_SETTINGS } from '$lib/settings';
  import { getUserAvatar } from '$lib/utils/avatar';

  const { 
    tag, 
    loading = false,
    isPreview = false,
    cutoffTimestamp,
    refreshKey = 0
  }: { 
    tag: TagDocument | null; 
    loading?: boolean;
    isPreview?: boolean;
    cutoffTimestamp: bigint;
    refreshKey?: number;
  } = $props();

  // Preview data constants
  const PREVIEW_TAG_KEY = '___PREVIEW_DATA___';
  
  // Internal state for votes and user data
  let votes = $state<VoteDocument[]>([]);
  let votesLoading = $state(false);
  let votesError = $state<string | null>(null);
  let userData = $state<Map<string, UserDocument>>(new Map());
  let userReputationScore = $state<number>(0);
  let userRank = $state<number>(0);

  // Filter states - similar to RecentVotesUser but as individual toggles
  let showIncoming = $state(true);
  let showOutgoing = $state(true);
  let showPositive = $state(true);
  let showNegative = $state(true);

  // Popover state for expand icon
  let expandPopoverOpen = $state(false);

  // Helper functions
  function closeExpandPopover() {
    expandPopoverOpen = false;
  }



  // Helper function to format reputation score (same as TopUsersCard)
  function formatScore(score: number): string {
    const { DECIMAL_PLACES, WHOLE_NUMBERS } = REPUTATION_SETTINGS.UI;
    
    if (WHOLE_NUMBERS) {
      return Math.round(score).toString();
    } else {
      return score.toFixed(DECIMAL_PLACES);
    }
  }

  // Filter votes based on toggle states and time
  function filterVotes(votes: VoteDocument[]): VoteDocument[] {
    // In preview mode, use demo_user as the active user
    const activeUserUlid = tag?.key === PREVIEW_TAG_KEY ? 'demo_user' : $authUserDoc?.data?.user_ulid;
    
    if (!activeUserUlid) return [];
    
    return votes.filter(vote => {
      // Check direction filters
      const isIncoming = vote.data.target_ulid === activeUserUlid;
      const isOutgoing = vote.data.owner_ulid === activeUserUlid;
      const directionMatch = (isIncoming && showIncoming) || (isOutgoing && showOutgoing);
      
      // Check value filters
      const voteValue = vote.data.value ?? 0;
      const isPositive = voteValue > 0;
      const isNegative = voteValue < 0;
      const valueMatch = (isPositive && showPositive) || (isNegative && showNegative);
      
      // Check time filter (skip time filter in preview mode)
      const timeMatch = tag?.key === PREVIEW_TAG_KEY || !vote.created_at || vote.created_at >= cutoffTimestamp;
      
      return directionMatch && valueMatch && timeMatch;
    });
  }

  // Fetch user data for a given ULID
  async function fetchUserData(ulid: string) {
    if (userData.has(ulid)) return;
    
    try {
      const keyPattern = `usr_${ulid}_`;
      const results = await queryDocsByKey<UserData>('users', keyPattern);
      
      if (results.items.length === 0) {
        console.warn(`No user found for ULID: ${ulid}`);
        return;
      }
      if (results.items.length > 1) {
        throw new Error(`Multiple users found for ULID: ${ulid}`);
      }
      
      userData.set(ulid, results.items[0] as UserDocument);
    } catch (e) {
      console.error(`Failed to fetch user data for ${ulid}:`, e);
    }
  }

  // Fetch user's actual reputation document for this tag
  async function fetchUserReputation(userUlid: string, tagUlid: string): Promise<{ score: number; rank: number }> {
    try {
      // Query the user's reputation document for this tag
      // Pattern: usr_{userUlid}_tag_{tagUlid}_
      const reputationPattern = `usr_${userUlid}_tag_${tagUlid}_`;
      const reputationResults = await queryDocsByKey('reputations', reputationPattern);
      
      if (reputationResults.items.length === 0) {
        console.log('No reputation document found for user in this tag');
        return { score: 0, rank: 0 };
      }
      
      if (reputationResults.items.length > 1) {
        console.warn('Multiple reputation documents found for user in tag, using first one');
      }
      
      const reputationDoc = reputationResults.items[0] as ReputationDocument;
      const score = reputationDoc.data.reputation_total_effective || 0;
      
      // To calculate rank, we need to get all users' scores in this tag
      const allReputationResults = await queryDocsByKey('reputations', `tag_${tagUlid}_`);
      const allScores = allReputationResults.items
        .map(doc => (doc as ReputationDocument).data.reputation_total_effective || 0)
        .sort((a, b) => b - a); // Sort descending
      
      // Find this user's rank (1-based)
      const rank = allScores.findIndex(s => s <= score) + 1;
      
      return { score, rank: rank || 0 };
    } catch (e) {
      console.error('Error fetching user reputation:', e);
      return { score: 0, rank: 0 };
    }
  }

  // Main data fetching function
  async function fetchUserVotesInTag() {
    if (!tag?.data?.tag_ulid || !$authUserDoc?.data?.user_ulid) {
      votes = [];
      return;
    }

    votesLoading = true;
    votesError = null;

    try {
      const userUlid = $authUserDoc.data.user_ulid;
      const tagUlid = tag.data.tag_ulid;

      console.log('Fetching user votes in tag with params:', {
        userUlid,
        tagUlid,
        cutoffTimestamp: cutoffTimestamp.toString()
      });

      // Query votes cast by the user in this tag: "usr_{userULID}_tag_{tagULID}_"
      const outgoingPattern = `usr_${userUlid}_tag_${tagUlid}_`;
      const outgoingResults = await queryDocsByKey<VoteData>('votes', outgoingPattern);

      // Query votes cast on the user in this tag: "tag_{tagULID}_tar_{userULID}_"
      const incomingPattern = `tag_${tagUlid}_tar_${userUlid}_`;
      const incomingResults = await queryDocsByKey<VoteData>('votes', incomingPattern);

      // Combine and deduplicate votes
      const allVotes = [...outgoingResults.items, ...incomingResults.items];
      const uniqueVotes = Array.from(
        new Map(allVotes.map(vote => [vote.key, vote])).values()
      ) as VoteDocument[];

      // Sort by creation date descending
      votes = uniqueVotes.sort((a, b) => {
        const timeA = a.created_at || 0n;
        const timeB = b.created_at || 0n;
        return timeA < timeB ? 1 : timeA > timeB ? -1 : 0;
      });

      // Fetch actual reputation from database
      const reputation = await fetchUserReputation(userUlid, tagUlid);
      userReputationScore = reputation.score;
      userRank = reputation.rank;

      // Fetch user data for all unique users in the votes
      const uniqueUsers = new Set<string>();
      votes.forEach(vote => {
        if (vote.data.owner_ulid) uniqueUsers.add(vote.data.owner_ulid);
        if (vote.data.target_ulid) uniqueUsers.add(vote.data.target_ulid);
      });

      await Promise.all(Array.from(uniqueUsers).map(fetchUserData));

    } catch (e) {
      votesError = e instanceof Error ? e.message : 'Failed to fetch user votes';
      console.error('Error fetching user votes in tag:', e);
    } finally {
      votesLoading = false;
    }
  }

  // Reactive effect to fetch data when tag, user, or cutoff changes
  $effect(() => {
    // Track refreshKey to ensure effect re-runs when data needs to be refreshed
    refreshKey;
    
    // Handle preview mode
    if (tag?.key === PREVIEW_TAG_KEY) {
      votesLoading = false;
      votesError = null;
      votes = dummyProfileData.recentVotes;
      
      // Create new Map to avoid reactivity issues
      const newUserData = new Map();
      
      // Pre-populate userData with dummy users (exact same approach as RecentVotesUser)
      dummyProfileData.dummyUsers.forEach(dummyUser => {
        newUserData.set(dummyUser.data.user_ulid, dummyUser);
      });
      
      // Set userData last to prevent reactivity loops
      userData = newUserData;
      userReputationScore = 123;
      userRank = 5;
      return;
    }
    
    if ($authUserDoc && tag?.data?.tag_ulid) {
      fetchUserVotesInTag();
    } else {
      votes = [];
      userReputationScore = 0;
      userRank = 0;
    }
  });
</script>

<BaseCard classes="max-h-[600px] flex flex-col">
  {#snippet header()}
    <Activity class="text-primary-500" size={20} />
    <h2 class="text-lg font-bold {(!tag || !$authUserDoc) ? 'opacity-50' : ''}">
      Your Reputation in {tag?.data.tag_handle || '...'}
    </h2>
  {/snippet}
  
  {#snippet actions()}
    {#if $authUserDoc && tag && !isPreview}
      <!-- Filter Controls -->
      <div class="flex gap-2">
        <!-- Direction filters -->
        <div class="flex gap-1">
          <button type="button" class="chip text-xs px-2 py-0.5 w-8 {showIncoming ? 'preset-filled-secondary-500' : 'preset-tonal-surface'}" onclick={() => showIncoming = !showIncoming}>In</button>
          <button type="button" class="chip text-xs px-2 py-0.5 w-8 {showOutgoing ? 'preset-filled-tertiary-500' : 'preset-tonal-surface'}" onclick={() => showOutgoing = !showOutgoing}>Out</button>
        </div>
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
            <p class="font-bold">Reputation Details</p>
            <button class="btn-icon hover:preset-tonal" onclick={closeExpandPopover}><X class="w-4 h-4" /></button>
          </header>
          <article>
            <p class="opacity-60">
              This feature isn't available yet. In the future, you'll be able to view a detailed breakdown of your reputation history, voting patterns, trust metrics, and contribution analytics for this tag.
            </p>
          </article>
        {/snippet}
      </Popover>
    {/if}
  {/snippet}
  
  {#snippet children()}
    <div class="flex flex-col h-full">
      {#if !$authUserDoc}
        <p class="text-center opacity-60 py-10">Log in to see your activity and reputation for this tag.</p>
      {:else if loading || votesLoading}
        <div class="space-y-4">
          <!-- Reputation Stats Skeleton -->
          <div class="grid grid-cols-2 gap-4">
            <div class="placeholder animate-pulse h-16 rounded"></div>
            <div class="placeholder animate-pulse h-16 rounded"></div>
          </div>
          <!-- Votes List Skeleton -->
          <div class="space-y-2">
            {#each Array(5) as _}
              <div class="placeholder animate-pulse h-8 rounded"></div>
            {/each}
          </div>
        </div>
      {:else if votesError}
        <p class="text-center text-error-500">{votesError}</p>
      {:else if tag}
        <!-- Reputation Stats -->
        <div class="grid grid-cols-2 gap-4 mb-4">
          <div class="p-3 bg-surface-200-800 rounded">
            <span class="text-sm opacity-70">Your Score</span>
            <p class="text-2xl font-bold">{formatScore(userReputationScore)}</p>
          </div>
          <div class="p-3 bg-surface-200-800 rounded">
            <span class="text-sm opacity-70">Rank</span>
            <p class="text-2xl font-bold">#{userRank || '-'}</p>
          </div>
        </div>

                 <!-- Votes Section -->
         <div class="flex-1 flex flex-col min-h-0">
           <h3 class="text-sm font-semibold mb-2 opacity-70">Recent Votes</h3>
           <div class="flex-1 overflow-y-auto max-h-80">
            {#if votes && votes.length > 0}
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
                                <UserLink handle={ownerUser.data.user_handle} class="text-sm" />
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
                                <UserLink handle={targetUser.data.user_handle} class="text-sm" />
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
                <p class="text-center opacity-70 text-sm">No votes match the selected filters.</p>
              {/if}
            {:else}
              <p class="text-center opacity-70 text-sm">No votes found for this tag.</p>
            {/if}
          </div>
        </div>
      {:else}
        <p class="text-center opacity-70 py-10">Select a tag to see your activity.</p>
      {/if}
    </div>
  {/snippet}
</BaseCard> 