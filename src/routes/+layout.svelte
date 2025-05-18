<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { initJuno } from '$lib/juno';
	import { authSubscribe, type User } from '@junobuild/core';
	import { goto } from '$app/navigation';
	import { Toaster } from '@skeletonlabs/skeleton-svelte';
	import { toaster } from '$lib/skeletonui/toaster-skeleton';
	import { authUser, authUserDoneInitializing } from '$lib/stores/authUser';
	import { page } from '$app/stores';
	import type { UserData } from '$lib/types';
	import AppShell from '$lib/components/layout/AppShell.svelte';
	import { setPageMeta, page as pageStore } from '$lib/stores/page';
	import { queryDocsByKey } from '$lib/docs-crud/query_by_key';

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
					// Use principal-based key pattern to find user doc
					const principal = user.key;
					const keyPattern = `_prn_${principal}_`;
					const results = await queryDocsByKey<UserData>('users', keyPattern);
					const userDoc = results.items[0]?.data;
					// TODO: Centralize required fields in a shared constant or type for maintainability
					const hasRequiredFields = userDoc && userDoc.user_handle && userDoc.display_name;
					if (!userDoc || !hasRequiredFields) {
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

	// Set page metadata based on current route
	$: {
		if ($page.url.pathname === '/') {
			setPageMeta({ title: 'Home' });
		} else if ($page.url.pathname === '/dashboard') {
			setPageMeta({ title: 'Dashboard' });
		} else if ($page.url.pathname === '/tags-hub') {
			setPageMeta({ title: 'Tags Hub' });
		} else if ($page.url.pathname === '/tag/new') {
			setPageMeta({ title: 'Create Tag' });
		} else if ($page.url.pathname === '/user/me') {
			setPageMeta({ title: 'Profile' });
		} else if ($page.url.pathname === '/user') {
			setPageMeta({ title: 'Users' });
		} else if ($page.url.pathname === '/onboarding') {
			setPageMeta({ title: 'Onboarding' });
		} else if ($page.url.pathname === '/admin') {
			setPageMeta({ title: 'Admin' });
		} else {
			setPageMeta({ title: 'Reputator' }); // fallback
		}
	}
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
