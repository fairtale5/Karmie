<script lang="ts">
import BaseCard from '$lib/components/common/BaseCard.svelte';
import { Star, Expand, X } from 'lucide-svelte';
import { ThumbsUp, ThumbsDown } from 'lucide-svelte';
import { Popover } from '@skeletonlabs/skeleton-svelte';

interface Review {
  type: 'vote' | 'received';
  target: string;
  value: number;
  tag: string;
  date: string;
  message: string;
}

const { reviews } = $props<{
  reviews: Review[];
}>();

// Popover state
let expandPopoverOpen = $state(false);

function closeExpandPopover() {
  expandPopoverOpen = false;
}
</script>

<BaseCard underConstruction={true}>
  <div class="flex justify-between items-start mb-4">
    <div class="flex items-center gap-2">
      <Star class="text-primary-500" size={20} />
      <h2 class="text-xl font-bold">Recent Reviews</h2>
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
          <p class="font-bold">See More Reviews</p>
          <button class="btn-icon hover:preset-tonal" onclick={closeExpandPopover}><X class="w-4 h-4" /></button>
        </header>
        <article>
          <p class="opacity-60">
            This feature isn't available yet. In the future, you'll be able to view a comprehensive list of all reviews for this user, with advanced filtering and search capabilities.
          </p>
        </article>
      {/snippet}
    </Popover>
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