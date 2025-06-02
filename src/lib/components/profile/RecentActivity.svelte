<script lang="ts">
import BaseCard from '$lib/components/common/BaseCard.svelte';
import { History } from 'lucide-svelte';
import { ThumbsUp, ThumbsDown } from 'lucide-svelte';

interface Activity {
  type: 'vote' | 'received';
  target: string;
  value: number;
  tag: string;
  date: string;
  message: string;
}

export let activities: Activity[];
</script>

<BaseCard>
  <div class="flex items-center gap-2 mb-4">
    <History class="text-primary-500" size={20} />
    <h2 class="text-xl font-bold">Recent Activity</h2>
  </div>

  <div class="space-y-4">
    {#each activities as activity}
      <div class="flex items-start gap-3">
        <div class="mt-1">
          {#if activity.type === 'vote'}
            {#if activity.value > 0}
              <ThumbsUp class="text-success-500" size={16} />
            {:else}
              <ThumbsDown class="text-error-500" size={16} />
            {/if}
          {:else}
            {#if activity.value > 0}
              <ThumbsUp class="text-success-500" size={16} />
            {:else}
              <ThumbsDown class="text-error-500" size={16} />
            {/if}
          {/if}
        </div>
        <div class="flex-1">
          <div class="flex items-center gap-2">
            <span class="font-medium">@{activity.target}</span>
            <span class="text-sm opacity-60">{activity.date}</span>
          </div>
          <div class="text-sm opacity-80">{activity.message}</div>
          <div class="text-xs opacity-60 mt-1">in {activity.tag}</div>
        </div>
      </div>
    {/each}
  </div>
</BaseCard> 