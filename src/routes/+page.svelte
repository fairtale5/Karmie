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
	import { Accordion } from '@skeletonlabs/skeleton-svelte';
	
	/**
	 * Component State
	 * - initialized: Tracks if Juno has been initialized
	 * - user: Local reference to current auth state
	 * - error: Holds any auth-related errors
	 * - currentPath: Reactive reference to current URL path
	 * - useCasesValue: Controls which accordion items are open
	 */
	let initialized = false;
	let user: User | null = null;
	let error = $state<string | null>(null);
	const currentPath = $derived($page.url.pathname);
	
	// Accordion state - start with first item open to draw attention
	let useCasesValue = $state(['ecommerce']);

	// Use case examples for the accordion
	const useCases = [
		{
			id: 'ecommerce',
			title: '🛒 E-commerce Marketplaces',
			subtitle: 'Like eBay or Shopify, but completely bot-proof',
			content: `Create marketplace platforms where buyer and seller reputations actually matter. Unlike traditional reviews that can be faked with bots, Karmie ensures only trusted community members can influence reputation scores. Perfect for:
			
			• **Online marketplaces** - Track seller reliability and buyer behavior
			• **Peer-to-peer trading** - Build trust in decentralized exchanges
			• **Service platforms** - Rate freelancers, contractors, and service providers
			
			**Example**: A seller with 95% positive #trustworthy votes from established community members carries far more weight than thousands of bot reviews.`
		},
		{
			id: 'airdrops',
			title: '🪂 Token Distribution & Airdrops',
			subtitle: 'Ensure rewards reach real humans, not bots',
			content: `Stop wasting tokens on bot farms and Sybil attacks. Use reputation as a filter to identify genuine community members who deserve rewards.
			
			• **Airdrop campaigns** - Only send tokens to users with #genuine reputation
			• **Community rewards** - Distribute based on contribution history
			• **Incentive programs** - Reward real engagement, not spam
			
			**Example**: Instead of dropping tokens to 10,000 wallets (90% bots), drop to 1,000 wallets with proven #community reputation - reaching 10x more real humans.`
		},
		{
			id: 'social',
			title: '🗣️ Web3 Social Media',
			subtitle: 'Where expertise determines influence, not follower count',
			content: `Build social platforms where domain experts have more influence in their areas of expertise, creating higher quality discussions and content curation.
			
			• **Topic-based influence** - Developers have more weight in #programming discussions
			• **Expert curation** - Environmentalists shape #climate content
			• **Quality over quantity** - Thoughtful contributions matter more than volume
			
			**Example**: A post about DeFi protocols gets more visibility when upvoted by users with high #defi reputation, while meme posts are boosted by #entertainment experts.`
		},
		{
			id: 'gaming',
			title: '🎮 Gaming Communities',
			subtitle: 'Track player skills, teamwork, and community behavior',
			content: `Create gaming environments where reputation follows players across games and platforms, building long-term community trust and skill recognition.
			
			• **Skill tracking** - Build #skill reputation through gameplay
			• **Behavior monitoring** - Track #sportsmanship and #teamwork
			• **Cross-game reputation** - Carry trust between different games
			
			**Example**: A player with high #leadership reputation in strategy games can be auto-nominated as team captain in new games, while toxic players lose influence over time.`
		},
		{
			id: 'professional',
			title: '💼 Professional Networks',
			subtitle: 'Build verifiable expertise without traditional credentials',
			content: `Enable professional communities where expertise is earned through peer recognition rather than just formal education or job titles.
			
			• **Skill validation** - Peers vouch for each other's #technical abilities
			• **Project collaboration** - Find teammates based on reputation history
			• **Knowledge sharing** - Experts gain influence in their domains
			
			**Example**: A developer gains #rust reputation by consistently providing helpful solutions, making their future advice carry more weight in Rust discussions.`
		},
		{
			id: 'governance',
			title: '🏛️ Community Governance',
			subtitle: 'Democratic decision-making with weighted expertise',
			content: `Create governance systems where voting power is earned through community contribution and expertise, not just token holdings.
			
			• **Proposal voting** - Weight votes by relevant expertise
			• **Moderation decisions** - Trusted members help govern content
			• **Resource allocation** - Community decides funding based on contribution history
			
			**Example**: A vote on platform technical improvements weighs #development reputation more heavily, while #design reputation influences UI/UX decisions.`
		}
	];

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
				<button onclick={() => login()} class="btn preset-filled-primary-500 w-full text-lg">
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
				<button onclick={logout} class="btn preset-outlined-primary-500 w-full sm:w-auto">Logout</button>
			</div>
		{/if}
	</div>
</section>

<!-- Use Cases Section -->
<section class="section py-16">
	<div class="container mx-auto max-w-4xl">
		<div class="text-center mb-12">
			<h2 class="preset-typo-title text-4xl font-bold mb-4">Real-World Use Cases</h2>
			<p class="preset-typo-body-1 text-lg text-surface-600-400 max-w-2xl mx-auto">
				See how Karmie's bot-resistant reputation system can transform your platform, community, or application.
			</p>
		</div>
		
		<div class="card shadow bg-surface-50-950 border border-surface-200-800 p-6">
			<Accordion value={useCasesValue} onValueChange={(e) => (useCasesValue = e.value)} multiple>
				{#each useCases as useCase, i}
					<Accordion.Item value={useCase.id}>
						{#snippet control()}
							<div class="text-left">
								<h3 class="preset-typo-title text-xl mb-1">{useCase.title}</h3>
								<p class="text-sm text-surface-600-400">{useCase.subtitle}</p>
							</div>
						{/snippet}
						{#snippet panel()}
							<div class="prose max-w-none text-surface-700-300">
								{@html useCase.content.replace(/\n/g, '<br>').replace(/•/g, '&bull;')}
							</div>
						{/snippet}
					</Accordion.Item>
					{#if i < useCases.length - 1}
						<hr class="hr my-2" />
					{/if}
				{/each}
			</Accordion>
		</div>
		
		<div class="text-center mt-8">
			<p class="text-sm text-surface-600-400 mb-4">
				These are just a few examples. Any platform that needs to distinguish real users from bots can benefit from Karmie.
			</p>
			<button class="btn preset-filled-tertiary-500" disabled>
				View Integration Guide (coming soon)
			</button>
		</div>
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
