import { createToaster } from '@skeletonlabs/skeleton-svelte';

/**
 * Shared Skeleton toaster instance for toast notifications.
 * Import this in any Svelte component to trigger toasts.
 *
 * Example usage:
 *   import { toaster } from '$lib/toaster-skeleton';
 *   toaster.error({ title: 'Something went wrong!' });
 */
export const toaster = createToaster(); 