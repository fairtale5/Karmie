<script lang="ts">
	import { onMount } from 'svelte';
	import { authSubscribe, signIn, signOut, type User } from '@junobuild/core';
	import { goto } from '$app/navigation';
	import { initJuno } from '$lib/juno';
	import { page } from '$app/stores';
	import { authUserDoc } from '$lib/stores/authUserDoc';
	import { authUserDoneInitializing } from '$lib/stores/authUser';
	import { handleLogin, handleLogout } from '$lib/login';
	import { setPageMeta } from '$lib/stores/page';
	
	/**
	 * Component State
	 * - initialized: Tracks if Juno has been initialized
	 * - user: Local reference to current auth state
	 * - error: Holds any auth-related errors
	 * - currentPath: Reactive reference to current URL path
	 */
	let initialized = false;
	let user: User | null = null;
	let error: string | null = null;
	$: currentPath = $page.url.pathname;
	
	// Placeholder image imports (replace with your preferred images from /img/landing_page)
	const features = [
		{
			title: 'Natively Bots-Resistant',
			text: `Uses a system in which only the votes of those with good reputation count. In other words: for a user's votes to count, they must earn reputation first. Only votes from a user who has already gained reputation in a topic (say #trust) can influence voting on others in that same topic. This makes it really hard for bots to enter the system, since their spamming or voting won't carry any weight until they earn it.`,
			img: '/img/landing_page/u6389832795_imagine_a_sketch_of_peopel_uniting_to_build_great_21ea2e1f-e07f-45f7-b1ac-0d7bf9e6c0d3_0.png',
			imgAlt: 'Sketch of people uniting to build',
		},
		{
			title: 'Easy to Purge Bots',
			text: `If a bad actor is downvoted, all their past and future votes are affected. Votes carry weight based on the author's reputation. If a bad actor infiltrates the community and starts spamming or acting in bad faith, others can downvote their account, which will affect not only new votes but also any votes they did in the past. This brings accountability and keeps the system bot and bad actor free, without KYC or infringing on user anonymity.`,
			img: '/img/landing_page/u6389832795_imagine_a_sketch_of_hundreds_of_people_all_contri_513e73f8-2993-480f-a765-2c6788a028c9_3-removebg-preview.png',
			imgAlt: 'Sketch of many people contributing',
		},
		{
			title: 'Disencourages Spamming',
			text: `An author's reputation is split across all votes they cast. If a user with 100 reputation casts one vote, that vote will have 100 weight, but if they cast 5 votes, those 100 will be divided by 5, carrying 20 each. This ensures spamming doesn't give anyone an unfair advantage.`,
			img: '/img/landing_page/—Pngtree—orange technology data ring_4863566.png',
			imgAlt: 'Orange technology data ring',
		},
		{
			title: 'Flexible #Tags and Use Cases',
			text: `Each community, app, developer, or game can create one or many #tags to track reputations. Each tag works as an independent reputation community. Use cases include games (e.g., #skill, #fun, #friendly), online stores (like eBay/Amazon reviews, but immune to bots), and even "reputation unions" for cross-community trust.`,
			img: '/img/landing_page/vecteezy_3d-abstract-digital-technology-yellow-orange-light-particles_26914816.png',
			imgAlt: 'Abstract digital technology',
		},
	];
	
	/**
	 * Component Initialization
	 * - Initializes Juno on mount
	 * - No auth subscription needed here as it's handled in +layout.svelte
	 */
	onMount(() => {
		// Set page title
		setPageMeta({ title: 'Home' });
		
		(async () => {
			await initJuno();
			initialized = true;
		})();
	});

	/**
	 * Handles user login and post-login navigation
	 * 
	 * Flow:
	 * 1. Calls Juno's signIn()
	 * 2. If not on homepage, stops here (no redirect)
	 * 3. If on homepage, waits for:
	 *    a. Auth state to be set
	 *    b. User document to be fetched (via authUserDoneInitializing)
	 * 4. Redirects based on user document status:
	 *    - No document/missing fields -> /onboarding
	 *    - Complete document -> /dashboard
	 * 
	 * Note: Main onboarding check is in +layout.svelte, this only handles
	 * the initial login redirect from homepage.
	 */
	async function login() {
		try {
			error = '';
			await signIn();
			
			// Only handle redirects if user is on the homepage
			if (currentPath !== '/') {
				return;
			}

			// Wait for auth state and user document to be ready
			await new Promise<void>((resolve) => {
				const unsubscribe = authSubscribe(async (state) => {
					if (state) {
						// Wait until layout has finished checking user document
						// This prevents race conditions with the layout's auth handling
						if (!$authUserDoneInitializing) {
							return; // Keep subscription active
						}
						
						// Now safe to check user document as layout has processed it
						const userDoc = $authUserDoc;
						const hasRequiredFields = userDoc && userDoc.data.user_handle && userDoc.data.display_name;
						
						// Redirect based on user document status
						if (!userDoc || !hasRequiredFields) {
							goto('/new/user');
						} else {
							goto('/dashboard');
						}
						
						// Clean up subscription and resolve promise
						unsubscribe();
						resolve();
					}
				});
			});
		} catch (e) {
			console.error('Login failed:', e);
			error = e instanceof Error ? e.message : 'Login failed';
		}
	}

	/**
	 * Handles user logout
	 * - Calls Juno's signOut()
	 * - Error handling included
	 * - No redirect needed as +layout.svelte handles auth state changes
	 */
	async function logout() {
		try {
			error = '';
			await signOut();
		} catch (e) {
			console.error('Logout failed:', e);
			error = e instanceof Error ? e.message : 'Logout failed';
		}
	}
</script>

<section class="section flex flex-col items-center justify-center min-h-[60vh] py-12 space-y-8">
	<div class="container max-w-2xl mx-auto text-center space-y-6">
		<h1 class="preset-typo-title text-5xl font-bold mb-2">Karmie dApp</h1>
		<h2 class="preset-typo-title text-2xl text-secondary-500 mb-4">Decentralized On-chain Reputation System</h2>
		<p class="preset-typo-body-1 text-lg mb-6">
			Karmie is a tool to track reputations (Karma) across any app, store, community, or game. Any dev can integrate it easily into their app with just a few API calls. Everything is handled on-chain, so not even the owners of those apps can adulterate or censor any votes. It gives true transparency and power to users.
		</p>
		{#if !user}
			<div class="grid grid-cols-1 sm:grid-cols-2 gap-4 w-full max-w-xl mx-auto mt-4">
				<!-- Top row -->
				<button on:click={() => handleLogin(currentPath)} class="btn preset-filled-primary-500 w-full text-lg">
					Login with Internet Identity
				</button>
				<a
					href="https://internetcomputer.org/internet-identity"
					target="_blank"
					rel="noopener"
					class="btn preset-filled-secondary-500 w-full text-lg"
				>
					What is an Internet Identity?
				</a>

				<!-- Bottom row -->
				<!--
				<button class="btn bg-surface-200 text-surface-500 w-full text-lg flex flex-col items-center leading-tight" disabled>
					Create & Track a Reputation
					<span class="text-xs">(coming soon)</span>
				</button>
				<button class="btn bg-surface-200 text-surface-500 w-full text-lg flex flex-col items-center leading-tight" disabled>
					Integrate in Your App
					<span class="text-xs">(coming soon)</span>
				</button>
				-->
			</div>
			{#if error}
				<div class="alert alert-error mt-2">{error}</div>
			{/if}
		{:else}
			<div class="flex flex-col items-center gap-2 mb-4">
				<span class="text-success-500 font-semibold">You are logged in.</span>
				<button on:click={handleLogout} class="btn preset-outlined-primary-500 w-full sm:w-auto">Logout</button>
			</div>
		{/if}
	</div>
</section>

<!-- Features Sections -->
{#each features as {title, text, img, imgAlt}, i}
	<section class="section py-8">
		<div class="container mx-auto">
			<div class="card shadow bg-surface-100-900 border border-surface-200-800 flex flex-col md:flex-row {i % 2 === 1 ? 'md:flex-row-reverse' : ''} items-center gap-8 p-6">
				<img src={img} alt={imgAlt} class="w-full md:w-1/2 rounded-lg shadow mb-6 md:mb-0" />
				<div class="md:w-1/2">
					<h2 class="preset-typo-title text-2xl mb-2">{title}</h2>
					<p class="preset-typo-body-1">{text}</p>
				</div>
			</div>
		</div>
	</section>
{/each}

<!-- Final CTA Section -->
<section class="section py-16">
	<div class="container mx-auto text-center space-y-6">
		<h2 class="preset-typo-title text-3xl mb-4">Ready to build trust in your community?</h2>
		<div class="flex flex-col sm:flex-row gap-4 justify-center mt-4">
			<a href="/new/tag" class="btn preset-filled-secondary-500 w-full sm:w-auto text-lg">Register Your Community</a>
			<button class="btn preset-filled-tertiary-500 w-full sm:w-auto text-lg" disabled>Integrate in Your App (coming soon)</button>
		</div>
	</div>
</section>

<style>
	/* Add any component-specific styles here */
</style>
