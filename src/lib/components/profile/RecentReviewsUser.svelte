<script lang="ts">
import BaseCard from '$lib/components/common/BaseCard.svelte';
import { Star } from 'lucide-svelte';
import { ThumbsUp, ThumbsDown } from 'lucide-svelte';

interface Review {
  type: 'vote' | 'received';
  target: string;
  value: number;
  tag: string;
  date: string;
  message: string;
}

export let reviews: Review[];
</script>

<BaseCard>
  <div class="flex items-center gap-2 mb-4">
    <Star class="text-primary-500" size={20} />
    <h2 class="text-xl font-bold">Recent Reviews</h2>
  </div>

  <div class="space-y-4">
    {#each reviews as review}
      <div class="flex items-start gap-3">
        <div class="mt-1">
          {#if review.type === 'vote'}
            {#if review.value > 0}
              <ThumbsUp class="text-success-500" size={16} />
            {:else}
              <ThumbsDown class="text-error-500" size={16} />
            {/if}
          {:else}
            {#if review.value > 0}
              <ThumbsUp class="text-success-500" size={16} />
            {:else}
              <ThumbsDown class="text-error-500" size={16} />
            {/if}
          {/if}
        </div>
        <div class="flex-1">
          <div class="flex items-center gap-2">
            <span class="font-medium">@{review.target}</span>
            <span class="text-sm opacity-60">{review.date}</span>
          </div>
          <div class="text-sm opacity-80">{review.message}</div>
          <div class="text-xs opacity-60 mt-1">in {review.tag}</div>
        </div>
      </div>
    {/each}
  </div>
</BaseCard> 