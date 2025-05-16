<script lang="ts">
	export let title = 'Page Title';
	import { Switch } from '@skeletonlabs/skeleton-svelte';
	import { Sun, Moon } from 'lucide-svelte';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { authUser } from '$lib/stores/authUser';
	import { signIn, signOut, getDoc } from '@junobuild/core';
	import { goto } from '$app/navigation';
	import { LOGIN_REDIRECT_URL, LOGOUT_REDIRECT_URL } from '$lib/settings';
	import { toaster } from '$lib/skeletonui/toaster-skeleton';
	import type { UserData } from '$lib/types';

	let checked = false;
	$: currentPath = $page.url.pathname;
	let error: string | null = null;
	
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
	 * Handles login and redirects based on user document existence.
	 * If user has no document, redirects to onboarding.
	 * If user has document, redirects to reputations.
	 */
	async function handleLogin() {
		try {
			// First attempt login
			await toaster.promise(
				signIn(),
				{
					loading: { title: 'Logging in...' },
					success: { title: 'Login successful!' },
					error: { title: 'Login failed', description: 'Please try again.' }
				}
			);

			// After successful login, check for user document
			const user = $authUser;
			if (!user) {
				throw new Error('Login succeeded but user state is not available');
			}

			try {
				const userDoc = await getDoc<UserData>({ collection: 'users', key: user.key });
				if (!userDoc || !userDoc.data?.user_handle) {
					// No user document or incomplete - redirect to onboarding
					goto('/onboarding');
				} else {
					// User document exists - redirect to reputations
					goto(LOGIN_REDIRECT_URL);
				}
			} catch (e) {
				// If we can't check user document, default to onboarding
				console.error('Failed to check user document:', e);
				goto('/onboarding');
			}
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

<header class="bg-[var(--color-surface-50-950)] border-b border-[color-mix(in oklab,var(--color-surface-500)20%,transparent)]">
	<div class="container mx-auto p-3 flex justify-between items-center">
		<div class="flex items-center gap-4">
			<a href="/" class="text-2xl font-bold text-[var(--color-primary-500)]">Reputator</a>
			<span class="ml-2 text-xl font-semibold text-primary-700-300">{title}</span>
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
