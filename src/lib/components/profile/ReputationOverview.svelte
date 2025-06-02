<script lang="ts">
import BaseCard from '$lib/components/common/BaseCard.svelte';
import { Star } from 'lucide-svelte';

interface ReputationStats {
  trustedIn: number;
  totalCommunities: number;
  activeIn: number;
}

export let stats: ReputationStats;
</script>

<BaseCard>
  <div class="flex items-center gap-2 mb-4">
    <Star class="text-primary-500" size={20} />
    <h2 class="text-xl font-bold">Reputation Overview</h2>
  </div>

  <div class="grid grid-cols-3 gap-4">
    <div class="text-center p-4 bg-surface-200-800 rounded-lg">
      <div class="text-2xl font-bold text-primary-500">{stats.trustedIn}</div>
      <div class="text-sm opacity-60">Trusted In</div>
    </div>
    <div class="text-center p-4 bg-surface-200-800 rounded-lg">
      <div class="text-2xl font-bold text-primary-500">{stats.totalCommunities}</div>
      <div class="text-sm opacity-60">Total Communities</div>
    </div>
    <div class="text-center p-4 bg-surface-200-800 rounded-lg">
      <div class="text-2xl font-bold text-primary-500">{stats.activeIn}</div>
      <div class="text-sm opacity-60">Active In</div>
    </div>
  </div>
</BaseCard> 