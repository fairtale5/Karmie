<script lang="ts">
/**
 * SkeletonLoader.svelte
 * A reusable skeleton loading component for cards, lists, or sections.
 * Follows Skeleton UI and project coding standards.
 * @param {number} count - Number of skeleton items to render (default: 1)
 * @param {string} variant - Optional variant for different shapes (e.g., 'card', 'list', 'avatar')
 * @example <SkeletonLoader count={3} variant="card" />
 */
export let count: number = 1;
export let variant: 'card' | 'list' | 'avatar' = 'card';
</script>

<!--
  SkeletonLoader: Renders animated skeleton placeholders for loading states.
  - Uses only necessary custom styles (theme extension), otherwise Skeleton UI defaults.
  - Accessible: aria-busy and role attributes.
-->
<div aria-busy="true" role="status">
  {#each Array(count) as _, i}
    {#if variant === 'card'}
      <div class="skeleton h-32 w-full rounded mb-4" data-testid="skeleton-card-{i}"></div>
    {:else if variant === 'list'}
      <div class="skeleton h-8 w-full rounded mb-2" data-testid="skeleton-list-{i}"></div>
    {:else if variant === 'avatar'}
      <div class="skeleton h-12 w-12 rounded-full mb-2" data-testid="skeleton-avatar-{i}"></div>
    {/if}
  {/each}
</div>

<style>
/*
  Skeleton animation and color follow Skeleton UI best practices and your theme.
  If you have a custom gradient, extend only the background as needed.
*/
.skeleton {
  animation: skeleton-loading 1s linear infinite alternate;
  background-color: var(--color-surface-200, hsl(200, 20%, 80%));
}
@keyframes skeleton-loading {
  0% {
    background-color: var(--color-surface-200, hsl(200, 20%, 80%));
  }
  100% {
    background-color: var(--color-surface-100, hsl(200, 20%, 95%));
  }
}
</style> 