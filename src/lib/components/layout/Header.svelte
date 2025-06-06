<script lang="ts">
	import { Switch } from '@skeletonlabs/skeleton-svelte';
	import { Sun, Moon } from 'lucide-svelte';
	import { page } from '$app/stores';
	import { authUser } from '$lib/stores/authUser';
	import { page as pageStore, type PageMeta } from '$lib/stores/page';
	import { handleLogin, handleLogout } from '$lib/login';
	import { themeStore } from '$lib/stores/theme';

	/**
	 * Theme Toggle UI Component
	 * 
	 * HOW IT WORKS:
	 * - Reads current theme from central theme store
	 * - Converts theme state to toggle switch position
	 * - Delegates theme changes to theme store
	 * - No localStorage logic - all handled by theme store
	 */

	$: currentPath = $page.url.pathname;
	let meta: PageMeta = {};
	$: meta = $pageStore;
	
	// HOW: Convert theme store state to toggle switch position
	// Theme store uses 'light'/'dark', switch uses boolean (true = light, false = dark)
	$: themeToggleChecked = $themeStore === 'light';

	/**
	 * Handle theme toggle switch changes
	 * 
	 * HOW:
	 * - Switch component calls this when user clicks toggle
	 * - Delegates actual theme change to theme store
	 * - Theme store handles localStorage persistence and document updates
	 */
	function onThemeToggleChange(event: { checked: boolean }) {
		// HOW: Let theme store handle the toggle logic and persistence
		// This keeps UI component focused only on UI interaction
		themeStore.toggle();
	}
</script>

<header class="bg-surface-50-950 border-b border-surface-200-800/80">
	<div class="p-4">
		<div class="px-4 flex justify-between items-center">
			<div class="flex items-center gap-4">
				<span class="text-xl font-semibold text-primary-700-300">{meta.headerTitle ?? meta.title ?? ''}</span>
			</div>
			<div class="flex items-center gap-4">
				<!-- Theme Toggle Switch -->
				<Switch 
					name="themeToggle" 
					controlActive="bg-[var(--color-surface-200-800)]" 
					checked={themeToggleChecked} 
					onCheckedChange={onThemeToggleChange}
				>
					{#snippet inactiveChild()}<Moon size={14} />{/snippet}
					{#snippet activeChild()}<Sun size={14} />{/snippet}
				</Switch>
				{#if $authUser === null}
					<button
						type="button"
						class="btn preset-filled-primary-500"
						on:click={() => handleLogin(currentPath)}
						aria-label="Login with Internet Identity"
					>
						Login
					</button>
				{:else}
					<button
						type="button"
						class="btn preset-outlined-primary-500"
						on:click={handleLogout}
						aria-label="Logout"
					>
						Logout
					</button>
				{/if}
			</div>
		</div>
	</div>
</header>
