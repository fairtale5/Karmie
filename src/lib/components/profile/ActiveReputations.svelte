<script lang="ts">
import BaseCard from '$lib/components/common/BaseCard.svelte';
import { Orbit } from 'lucide-svelte';
import { Progress } from '@skeletonlabs/skeleton-svelte';
import { queryDocsByKey } from '$lib/docs-crud/query_by_key';
import { authUserDoc } from '$lib/stores/authUserDoc';
import { toaster } from '$lib/skeletonui/toaster-skeleton';

interface Reputation {
  tag: string;
  score: number;
  rank: number;
  isTrusted: boolean;
  progress: number;
}

const { reputations: dummyReputations } = $props<{ reputations: Reputation[] }>();

// Real active reputations data
let activeReputations = $state<Reputation[]>([]);
let loading = $state(true);

// Fetch real active reputations data
async function fetchActiveReputations() {
  if (!$authUserDoc?.data?.user_ulid) {
    activeReputations = [];
    loading = false;
    return;
  }

  try {
    loading = true;
    const userUlid = $authUserDoc.data.user_ulid;
    
    // 1. Get all reputation documents for this user
    const reputationsResults = await queryDocsByKey('reputations', `usr_${userUlid}_`);
    
    // 2. For each reputation, fetch tag data and calculate progress
    const reputations: Reputation[] = [];
    
    for (const rep of reputationsResults.items) {
      try {
        const tagUlid = (rep as any).data.tag_ulid;
        const userScore = (rep as any).data.reputation_total_effective || 0;
        const hasVotingPower = (rep as any).data.has_voting_power || false;
        
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
        
        reputations.push({
          tag: tagHandle,
          score: Math.round(userScore * 10) / 10,
          rank: rank || 0,
          isTrusted: hasVotingPower,
          progress: Math.round(progress)
        });
        
      } catch (tagError) {
        console.warn(`Failed to fetch tag data for ULID ${(rep as any).data.tag_ulid}:`, tagError);
      }
    }
    
    // Sort by score descending
    activeReputations = reputations.sort((a, b) => b.score - a.score);
    
  } catch (error) {
    console.error('Failed to fetch active reputations:', error);
    toaster.error({ 
      title: 'Failed to load active reputations', 
      description: 'Could not load reputation data' 
    });
    activeReputations = [];
  } finally {
    loading = false;
  }
}

// Fetch data when user changes
$effect(() => {
  fetchActiveReputations();
});
</script>

<BaseCard 
  header={headerSnippet}
  children={contentSnippet}
>
</BaseCard>

{#snippet headerSnippet()}
  <Orbit class="text-primary-500" size={20} />
  <h2 class="text-xl font-bold">Active In</h2>
{/snippet}

{#snippet contentSnippet()}
  {#if !$authUserDoc}
    <p class="text-center opacity-60 py-8">Log in to see your active reputations.</p>
  {:else if loading}
    <div class="space-y-4">
      {#each Array(3) as _}
        <div class="p-4 bg-surface-200-800 rounded-lg">
          <div class="flex items-center justify-between mb-3">
            <div class="flex items-center gap-2">
              <div class="w-8 h-8 bg-surface-300-700 animate-pulse rounded-full"></div>
              <div class="space-y-1">
                <div class="h-4 w-20 bg-surface-300-700 animate-pulse rounded"></div>
                <div class="h-3 w-16 bg-surface-300-700 animate-pulse rounded"></div>
              </div>
            </div>
            <div class="text-right space-y-1">
              <div class="h-6 w-12 bg-surface-300-700 animate-pulse rounded"></div>
              <div class="h-3 w-16 bg-surface-300-700 animate-pulse rounded"></div>
            </div>
          </div>
          <div class="h-1 w-full bg-surface-300-700 animate-pulse rounded"></div>
        </div>
      {/each}
    </div>
  {:else if activeReputations.length > 0}
    <div class="space-y-4">
      {#each activeReputations as rep}
        <div class="p-4 bg-surface-200-800 rounded-lg">
          <div class="flex items-center justify-between mb-3">
            <div class="flex items-center gap-2">
              <div class="w-8 h-8 rounded-full {rep.isTrusted ? 'bg-primary-500' : 'bg-surface-300-700'} flex items-center justify-center font-bold text-white">
                {rep.tag[0]}
              </div>
              <div>
                <div class="font-medium">#{rep.tag}</div>
                <div class="text-sm opacity-60">Rank #{rep.rank} {rep.isTrusted ? '• Trusted' : ''}</div>
              </div>
            </div>
            <div class="text-right">
              <div class="text-xl font-bold text-primary-500">{rep.score}</div>
              <div class="text-sm opacity-60">{rep.progress}% to trusted</div>
            </div>
          </div>
          <Progress 
            value={rep.progress} 
            max={100} 
            height="h-1"
            meterBg={rep.isTrusted ? "bg-success-500" : "bg-primary-500"}
          />
        </div>
      {/each}
    </div>
  {:else}
    <p class="text-center opacity-60 py-8">No active reputations yet. Start voting to build reputation!</p>
  {/if}
{/snippet} 