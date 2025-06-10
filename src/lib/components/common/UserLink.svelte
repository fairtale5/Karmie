<script lang="ts">
	import { goto } from '$app/navigation';
	import type { ComponentProps } from 'svelte';

	interface Props {
		/** The user handle to display and link to */
		handle: string;
		/** Optional display name to show instead of handle */
		displayName?: string;
		/** Additional CSS classes to apply */
		class?: string;
		/** Whether to show the @ symbol before the handle */
		showAt?: boolean;
		/** Whether to show as a link (default: true) */
		clickable?: boolean;
		/** Optional click handler - if provided, overrides default navigation */
		onclick?: (handle: string) => void;
	}

	let {
		handle,
		displayName,
		class: className = '',
		showAt = false,
		clickable = true,
		onclick
	}: Props = $props();

	/**
	 * Navigate to the user's profile page
	 * Uses the standard /u/[userhandle] route pattern
	 */
	function handleClick() {
		if (onclick) {
			onclick(handle);
		} else {
			goto(`/u/${handle}`);
		}
	}

	/**
	 * Derived display text based on props
	 */
	const displayText = $derived(
		showAt ? `@${displayName || handle}` : (displayName || handle)
	);
</script>

{#if clickable}
	<button 
		type="button"
		class="text-left hover:text-primary-500 hover:underline transition-colors duration-200 {className}"
		onclick={handleClick}
	>
		{displayText}
	</button>
{:else}
	<span class={className}>
		{displayText}
	</span>
{/if} 