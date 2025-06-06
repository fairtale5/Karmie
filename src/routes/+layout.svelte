<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';
	import { initJuno } from '$lib/juno';
	import { authSubscribe, type User } from '@junobuild/core';
	import { goto } from '$app/navigation';
	import { Toaster } from '@skeletonlabs/skeleton-svelte';
	import { toaster } from '$lib/skeletonui/toaster-skeleton';
	import { authUser, authUserDoneInitializing, loginInProgress } from '$lib/stores/authUser';
	import { authUserDoc } from '$lib/stores/authUserDoc';
	import { page } from '$app/stores';
	import type { UserData } from '$lib/types';
	import AppShell from '$lib/components/layout/AppShell.svelte';
	import { setPageMeta, page as pageStore } from '$lib/stores/page';
	import { queryDocsByKey } from '$lib/docs-crud/query_by_key';
	import { themeStore } from '$lib/stores/theme';

	let user: User | null = null;
	let checkedOnboarding = false;
	$: currentPath = $page.url.pathname;

	// List of paths that don't require user document check
	const EXEMPT_PATHS = ['/new/user', '/', '/login'];

	// Function to fetch user document
	async function fetchUserDoc(principal: string) {
		try {
			const keyPattern = `_prn_${principal}_`;
			const results = await queryDocsByKey<UserData>('users', keyPattern);
			const userDoc = results.items[0] || null; // Store the full document
			authUserDoc.set(userDoc);
			return userDoc;
		} catch (e) {
			console.error('Error fetching user document:', e);
			authUserDoc.set(null);
			return null;
		}
	}

	// HOW: Reactive variable that updates whenever theme store changes
	$: currentTheme = $themeStore;

	// HOW: Apply theme data-mode attribute to document root whenever theme changes
	// This triggers CSS custom property updates across the entire app via [data-mode="dark"] selectors
	$: if (typeof document !== 'undefined') {
		document.documentElement.setAttribute('data-mode', currentTheme);
	}

	onMount(async () => {
		if (import.meta.env.DEV) {
			toaster.info({
				title: 'Development Mode',
				description: 'You are running the app in local development mode. Some features may be unstable.',
				closable: true,
				duration: 0
			});
		}

		// Initialize Juno with auth worker enabled
		await initJuno();

		// Handle session expiration
		document.addEventListener("junoSignOutAuthTimer", () => {
			toaster.warning({
				title: 'Session Expired',
				description: 'Your session has expired. Please sign in again.'
			});
			authUserDoc.set(null);
			goto('/login');
		});

		// Handle remaining session time
		document.addEventListener("junoDelegationRemainingTime", ((event: Event) => {
			const customEvent = event as CustomEvent<number>;
			// Optional: Show warning when session is about to expire
			if (customEvent.detail < 5 * 60 * 1000) { // 5 minutes
				toaster.warning({
					title: 'Session Expiring Soon',
					description: 'Your session will expire in 5 minutes. Please save your work.'
				});
			}
		}) as EventListener);

		// Subscribe to auth state changes
		authSubscribe(async (state) => {
			user = state;
			authUser.set(state);

			// Clear user document when logged out
			if (!state) {
				authUserDoc.set(null);
				authUserDoneInitializing.set(true);
				loginInProgress.set(false);
				return;
			}

			// Always check user document on login
			try {
				const userDoc = await fetchUserDoc(state.key);
				const hasRequiredFields = userDoc && userDoc.data.user_handle && userDoc.data.display_name;
				
				// Store user document regardless of completeness
				authUserDoc.set(userDoc);
				
				// Handle redirects based on login context
				const isActiveLogin = $loginInProgress;
				const isHomepage = currentPath === '/';
				
				console.log('Layout: Auth state changed', {
					isActiveLogin,
					isHomepage,
					hasRequiredFields: !!hasRequiredFields,
					currentPath
				});
				
				if (isActiveLogin && isHomepage) {
					// Active login from homepage - redirect based on user document
					if (hasRequiredFields) {
						console.log('Layout: Active login with complete user doc - redirecting to dashboard');
						goto('/dashboard');
					} else {
						console.log('Layout: Active login with incomplete user doc - redirecting to new user');
						goto('/new/user');
					}
					loginInProgress.set(false);
				} else if (!hasRequiredFields && !EXEMPT_PATHS.includes(currentPath)) {
					// New user or incomplete profile on non-exempt page - redirect to 'new/user'
					console.log('Layout: Incomplete user doc on non-exempt page - redirecting to new user');
					goto('/new/user');
				}

				// Mark auth initialization as complete
				authUserDoneInitializing.set(true);
			} catch (e) {
				console.error('Error checking user document:', e);
				authUserDoc.set(null);
				if (!EXEMPT_PATHS.includes(currentPath)) {
				goto('/new/user');
				}
				authUserDoneInitializing.set(true);
				loginInProgress.set(false);
			}
		});

		// HOW: Initialize theme from saved preference or system detection
		// Must happen after component mounts to ensure browser APIs are available
		themeStore.init();
	});

	// Only set fallback title if no page has set one
	// Individual pages should set their own titles using setPageMeta()
	$: {
		// Simple fallback - pages should handle their own titles
		if (!$pageStore.title) {
			setPageMeta({ title: 'Reputator' });
		}
	}
	$: meta = $pageStore;
</script>

<!-- 
Flash Prevention Script and Page Meta
HOW: This executes before the page renders, preventing theme flash
- Immediately checks localStorage for saved theme
- Applies theme class to document root before any content shows
- Fallback to system preference if no saved theme exists
-->
<svelte:head>
	<title>{meta.title ? `${meta.title} | Reputator` : 'Reputator'}</title>
	{#if meta.description}
		<meta name="description" content={meta.description} />
	{/if}
	<script>
		// HOW: Same flash prevention as original Header - use 'mode' key
		(function() {
			const stored = localStorage.getItem('mode');
			if (stored) {
				document.documentElement.setAttribute('data-mode', stored);
			} else {
				const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
				document.documentElement.setAttribute('data-mode', prefersDark ? 'dark' : 'light');
			}
		})();
	</script>
</svelte:head>

<!-- Global Skeleton Toaster for toast notifications -->
<Toaster {toaster} />

<AppShell title="Reputator">
	<slot />
</AppShell>
