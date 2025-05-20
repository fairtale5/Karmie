<script lang="ts">
	import { Switch } from '@skeletonlabs/skeleton-svelte';
	import { Sun, Moon } from 'lucide-svelte';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { authUser } from '$lib/stores/authUser';
	import { signIn, signOut } from '@junobuild/core';
	import { goto } from '$app/navigation';
	import { LOGIN_REDIRECT_URL, LOGOUT_REDIRECT_URL } from '$lib/settings';
	import { toaster } from '$lib/skeletonui/toaster-skeleton';
	import type { UserData } from '$lib/types';
	import { page as pageStore, type PageMeta } from '$lib/stores/page';

	let checked = false;
	$: currentPath = $page.url.pathname;
	let error: string | null = null;
	let meta: PageMeta = {};
	$: meta = $pageStore;
	
	onMount(() => {
		// Handle theme dark vs light mode:
		// 1. Check if user has a stored preference ("last used on last visit")
		const stored = localStorage.getItem('mode');
		if (stored) {
			// 1.1 Use stored preference
			checked = stored === 'light';
			document.documentElement.setAttribute('data-mode', stored);
		} else {
			// 1.2 Use system preference
			const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
			checked = !prefersDark;
			document.documentElement.setAttribute('data-mode', prefersDark ? 'dark' : 'light');
		}
	});

	function onCheckedChange(event: { checked: boolean }) {
		const mode = event.checked ? 'light' : 'dark';
		document.documentElement.setAttribute('data-mode', mode);
		localStorage.setItem('mode', mode);
		checked = event.checked;
	}

	/**
	 * Handles login. All onboarding and redirect logic is now centralized in +layout.svelte.
	 */
	async function handleLogin() {
		try {
			await toaster.promise(
				signIn(),
				{
					loading: { title: 'Logging in...' },
					success: { title: 'Login successful!' },
					error: { title: 'Login failed', description: 'Please try again.' }
				}
			);
			// All post-login checks and redirects are handled in +layout.svelte
		} catch (e) {
			// Other errors are already handled by the toaster.promise above
		}
	}

	/**
	 * Handles logout and redirects on success, with Skeleton toast notifications.
	 */
	async function handleLogout() {
		await toaster.promise(
			signOut(),
			{
				loading: { title: 'Logging out...' },
				success: { title: 'Logged out' },
				error: { title: 'Logout failed', description: 'Please try again.' }
			}
		);
	}
</script>

<svelte:head>
	<script>
		const stored = localStorage.getItem('mode');
		if (stored) {
			document.documentElement.setAttribute('data-mode', stored);
		} else {
			const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
			document.documentElement.setAttribute('data-mode', prefersDark ? 'dark' : 'light');
		}
	</script>
</svelte:head>

<header class="bg-surface-50-950 border-b border-surface-200-800/80">
	<div class="container mx-auto p-3 flex justify-between items-center">
		<div class="flex items-center gap-4">
			<span class="ml-2 text-xl font-semibold text-primary-700-300">{meta.title ?? ''}</span>
		</div>
		<div class="flex items-center gap-4">
			<Switch 
				name="mode" 
				controlActive="bg-[var(--color-surface-200-800)]" 
				checked={checked} 
				{onCheckedChange}
			>
				{#snippet inactiveChild()}<Moon size={14} />{/snippet}
				{#snippet activeChild()}<Sun size={14} />{/snippet}
			</Switch>
			{#if $authUser === null}
				<button
					type="button"
					class="btn preset-filled-primary-500"
					on:click={handleLogin}
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
</header>
