<script lang="ts">
import BaseCard from '$lib/components/common/BaseCard.svelte';
import { Activity } from 'lucide-svelte';
import { Progress } from '@skeletonlabs/skeleton-svelte';

interface Reputation {
  tag: string;
  score: number;
  rank: number;
  isTrusted: boolean;
  progress: number;
}

export let reputations: Reputation[];
</script>

<BaseCard>
  <div class="flex items-center gap-2 mb-4">
    <Activity class="text-primary-500" size={20} />
    <h2 class="text-xl font-bold">Active In</h2>
  </div>

  <div class="space-y-4">
    {#each reputations as rep}
      <div class="p-4 bg-surface-200-800 rounded-lg">
        <div class="flex items-center justify-between mb-3">
          <div class="flex items-center gap-2">
            <div class="w-8 h-8 rounded-full bg-surface-300-700 flex items-center justify-center font-bold">
              {rep.tag[0]}
            </div>
            <div>
              <div class="font-medium">#{rep.tag}</div>
              <div class="text-sm opacity-60">Rank #{rep.rank}</div>
            </div>
          </div>
          <div class="text-right">
            <div class="text-xl font-bold text-primary-500">{rep.score}</div>
            <div class="text-sm opacity-60">0-100%</div>
          </div>
        </div>
        <Progress 
          value={rep.progress} 
          max={100} 
          height="h-1"
          meterBg="bg-primary-500"
        />
      </div>
    {/each}
  </div>
</BaseCard> 