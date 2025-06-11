<script lang="ts">
import BaseCard from '$lib/components/common/BaseCard.svelte';
import { Shield } from 'lucide-svelte';
import { queryDocsByKey } from '$lib/docs-crud/query_by_key';
import { authUserDoc } from '$lib/stores/authUserDoc';
import { toaster } from '$lib/skeletonui/toaster-skeleton';

interface Community {
  tag: string;
  score: number;
  rank: number;
  isTrusted: boolean;
  progress: number;
}

const { communities: dummyCommunities } = $props<{ communities: Community[] }>();

// Real trusted communities data
let trustedCommunities = $state<Community[]>([]);
let loading = $state(true);

// Fetch real trusted communities data
async function fetchTrustedCommunities() {
  if (!$authUserDoc?.data?.user_ulid) {
    trustedCommunities = [];
    loading = false;
    return;
  }

  try {
    loading = true;
    const userUlid = $authUserDoc.data.user_ulid;
    
    // 1. Get all reputation documents for this user
    const reputationsResults = await queryDocsByKey('reputations', `usr_${userUlid}_`);
    
    // 2. Filter for trusted communities (has_voting_power = true)
    const trustedReps = reputationsResults.items.filter((rep: any) => rep.data.has_voting_power);
    
    // 3. For each trusted reputation, fetch tag data and calculate rank
    const communities: Community[] = [];
    
    for (const rep of trustedReps) {
      try {
        const tagUlid = (rep as any).data.tag_ulid;
        const userScore = (rep as any).data.reputation_total_effective || 0;
        
        // Fetch tag document to get tag_handle and threshold
        const tagResults = await queryDocsByKey('tags', `tag_${tagUlid}_`);
        if (tagResults.items.length === 0) continue;
        
        const tagDoc = tagResults.items[0] as any;
        const tagHandle = tagDoc.data.tag_handle;
        const threshold = tagDoc.data.reputation_threshold || 10;
        
        // Calculate rank by getting all reputations for this tag and sorting
        const allTagRepsResults = await queryDocsByKey('reputations', `tag_${tagUlid}_`);
        const allScores = allTagRepsResults.items
          .map((doc: any) => (doc as any).data.reputation_total_effective || 0)
          .sort((a: number, b: number) => b - a); // Sort descending
        
        const rank = allScores.findIndex(score => score <= userScore) + 1;
        const progress = Math.min((userScore / threshold) * 100, 100);
        
        communities.push({
          tag: tagHandle,
          score: Math.round(userScore * 10) / 10,
          rank: rank || 0,
          isTrusted: true,
          progress: Math.round(progress)
        });
        
      } catch (tagError) {
        console.warn(`Failed to fetch tag data for ULID ${(rep as any).data.tag_ulid}:`, tagError);
      }
    }
    
    // Sort by score descending
    trustedCommunities = communities.sort((a, b) => b.score - a.score);
    
  } catch (error) {
    console.error('Failed to fetch trusted communities:', error);
    toaster.error({ 
      title: 'Failed to load trusted communities', 
      description: 'Could not load community data' 
    });
    trustedCommunities = [];
  } finally {
    loading = false;
  }
}

// Fetch data when user changes
$effect(() => {
  fetchTrustedCommunities();
});
</script>

<BaseCard 
  header={headerSnippet}
  children={contentSnippet}
>
</BaseCard>

{#snippet headerSnippet()}
  <Shield class="text-primary-500" size={20} />
  <h2 class="text-xl font-bold">Trusted In</h2>
{/snippet}

{#snippet contentSnippet()}
  {#if !$authUserDoc}
    <p class="text-center opacity-60 py-8">Log in to see your trusted communities.</p>
  {:else if loading}
    <div class="space-y-4">
      {#each Array(3) as _}
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <div class="w-8 h-8 bg-surface-300-700 animate-pulse rounded-full"></div>
            <div class="space-y-1">
              <div class="h-4 w-20 bg-surface-300-700 animate-pulse rounded"></div>
              <div class="h-3 w-16 bg-surface-300-700 animate-pulse rounded"></div>
            </div>
          </div>
          <div class="text-right space-y-1">
            <div class="h-6 w-12 bg-surface-300-700 animate-pulse rounded"></div>
            <div class="h-3 w-10 bg-surface-300-700 animate-pulse rounded"></div>
          </div>
        </div>
      {/each}
    </div>
  {:else if trustedCommunities.length > 0}
    <div class="space-y-4">
      {#each trustedCommunities as community}
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <div class="w-8 h-8 rounded-full bg-primary-500 flex items-center justify-center text-white font-bold">
              {community.tag[0]}
            </div>
            <div>
              <div class="font-medium">#{community.tag}</div>
              <div class="text-sm opacity-60">Rank #{community.rank}</div>
            </div>
          </div>
          <div class="text-right">
            <div class="text-xl font-bold text-primary-500">{community.score}</div>
            <div class="text-sm opacity-60">Score</div>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <p class="text-center opacity-60 py-8">Not trusted in any communities yet.<br>Vote and engage to build reputation!</p>
  {/if}
{/snippet} 