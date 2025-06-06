<script lang="ts">
import BaseCard from '$lib/components/common/BaseCard.svelte';
import { Shield } from 'lucide-svelte';

interface Community {
  tag: string;
  score: number;
  rank: number;
  isTrusted: boolean;
  progress: number;
}

export let communities: Community[];
</script>

<BaseCard>
  <div class="flex items-center gap-2 mb-4">
    <Shield class="text-primary-500" size={20} />
    <h2 class="text-xl font-bold">Trusted In</h2>
  </div>

  <div class="space-y-4">
    {#each communities as community}
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <div class="w-8 h-8 rounded-full bg-primary-500 flex items-center justify-center text-white font-bold">
            {community.tag[0]}
          </div>
          <div>
            <div class="font-medium">{community.tag}</div>
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
</BaseCard> 