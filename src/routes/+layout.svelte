<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import Header from '$lib/Header.svelte';
	import { initJuno } from '$lib/juno';
	import { authSubscribe, getDoc, type User } from '@junobuild/core';
	import { goto } from '$app/navigation';
	import { Toaster } from '@skeletonlabs/skeleton-svelte';
	import { toaster } from '$lib/toaster-skeleton';
	import { authUser, authUserDoneInitializing } from '$lib/stores/authUser';
	import { page } from '$app/stores';
	import type { UserData } from '$lib/types';

	let user: User | null = null;
	let checkedOnboarding = false;
	$: currentPath = $page.url.pathname;

	// List of paths that don't require user document check
	const EXEMPT_PATHS = ['/onboarding', '/', '/login'];

	onMount(async () => {
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
					if (!userDoc || !userDoc.data?.username) {
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
</script>

<!-- Global Skeleton Toaster for toast notifications -->
<Toaster {toaster} />

{#if import.meta.env.DEV}
	<div class="container mx-auto p-4">
		<div class="bg-yellow-100 border border-yellow-400 text-yellow-700 px-4 py-3 rounded relative mb-4" role="alert">
			<strong class="font-bold">Local Development Mode</strong>
		</div>
	</div>
{/if}

<Header />
<main class="container mx-auto px-4 py-8">
    <slot />
</main>
