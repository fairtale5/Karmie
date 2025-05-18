<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { initJuno } from '$lib/juno';
	import { authSubscribe, getDoc, type User } from '@junobuild/core';
	import { goto } from '$app/navigation';
	import { Toaster } from '@skeletonlabs/skeleton-svelte';
	import { toaster } from '$lib/skeletonui/toaster-skeleton';
	import { authUser, authUserDoneInitializing } from '$lib/stores/authUser';
	import { page } from '$app/stores';
	import type { UserData } from '$lib/types';
	import AppShell from '$lib/components/layout/AppShell.svelte';
	import { setPageMeta, page as pageStore } from '$lib/stores/page';

	let user: User | null = null;
	let checkedOnboarding = false;
	$: currentPath = $page.url.pathname;

	// List of paths that don't require user document check
	const EXEMPT_PATHS = ['/onboarding', '/', '/login'];

	onMount(async () => {
		if (import.meta.env.DEV) {
			toaster.info({
				title: 'Development Mode',
				description: 'You are running the app in local development mode. Some features may be unstable.',
				closable: true,
				duration: 0
			});
		}

		await initJuno();
		authSubscribe(async (state) => {
			user = state;
			// Only check user document if:
			// 1. User is logged in
			// 2. We haven't checked onboarding yet
			// 3. Current path requires a check
			if (user && !checkedOnboarding && !EXEMPT_PATHS.includes(currentPath)) {
				try {
					const userDoc = await getDoc<UserData>({ collection: 'users', key: user.key });
					if (!userDoc || !userDoc.data?.user_handle) {
						checkedOnboarding = true;
						goto('/onboarding');
					}
				} catch (e) {
					// If backend fails, still try to show onboarding
					checkedOnboarding = true;
					goto('/onboarding');
				}
			}
		});
	});

	setPageMeta({
		title: 'Home',
		description: 'Welcome to Reputator.'
	});
	$: meta = $pageStore;
</script>

<svelte:head>
	<title>{meta.title ? `${meta.title} | Reputator` : 'Reputator'}</title>
	{#if meta.description}
		<meta name="description" content={meta.description} />
	{/if}
</svelte:head>

<!-- Global Skeleton Toaster for toast notifications -->
<Toaster {toaster} />

<AppShell title="Reputator">
	<slot />
</AppShell>
