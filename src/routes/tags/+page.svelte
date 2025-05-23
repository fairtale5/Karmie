<script lang="ts">
// --- Skeleton v3 Toasts: Ensure <Toaster /> is present in your root layout (e.g., +layout.svelte) ---
import { onMount, tick } from 'svelte';
import { listDocs, type Doc } from '@junobuild/core';
import { toaster } from '$lib/skeletonui/toaster-skeleton';
import { goto } from '$app/navigation';
// import sigma.js for future graph integration (placeholder for now)
// import Sigma from 'sigma';
// import SkeletonLoader from '$lib/components/common/SkeletonLoader.svelte'; // We'll create inline placeholders
import { initJuno } from '$lib/juno';
import { Avatar } from '@skeletonlabs/skeleton-svelte';
import { UserRoundPen, Expand, BookOpen, SlidersHorizontal, Orbit } from 'lucide-svelte';
import NotLoggedInAlert from '$lib/components/common/NotLoggedInAlert.svelte';
import { authUserDoc } from '$lib/stores/authUserDoc';
import { Tabs } from '@skeletonlabs/skeleton-svelte';

// --- State ---
let pageLoading = $state(true); // True when initially loading tags list or when fetching specific tag data
let initialTagsLoading = $state(true); // Specifically for the first load of the tags list
let error = $state<string | null>(null);
let tags: Doc<any>[] = $state([]); // Initialize as empty array, make reactive for #each
let selectedTagKey = $state(''); // Make reactive
let selectedTag = $state<Doc<any> | null>(null); // Make reactive
let userReputation: any = $state(null); // Will be populated later, can add placeholders if complex
let topUsers: any[] = $state([]); // Make reactive for #each
let recentVotes: any[] = $state([]); // Make reactive for #each
let userRecentActivity: any[] = $state([]); // Ensure reactivity if used with placeholders
let selectedPeriod = '24h';
let activeTab = $state('about');
let userActivityFilter = $state('both'); // 'both', 'in', 'out'

// Dummy stats data (can be replaced with placeholders if fetched)
let stats = {
	totalUsers: 1234,
	verifiedUsers: 567,
	activeUsers: 89
};

// --- Fetch Data ---
onMount(async () => {
	await initJuno();
	initialTagsLoading = true;
	pageLoading = true;
	error = null;
	try {
		const tagsList = await listDocs({ collection: 'tags' });
		tags = tagsList.items;
		if (tags.length > 0) {
			selectedTagKey = tags[0].key;
		} else {
			// If no tags, no specific tag data to load, so pageLoading can be false
			pageLoading = false; 
		}
	} catch (e) {
		error = e instanceof Error ? e.message : 'Failed to load initial tag list.';
		toaster.error({ title: error });
		pageLoading = false; // Error occurred, nothing more to load for the page
	} finally {
		initialTagsLoading = false;
	}
});

// Reactive effect to fetch tag data when selectedTagKey changes or tags list gets populated
$effect(() => {
	const keyToFetch = selectedTagKey;
	const currentTags = tags; // $state vars are reactive by themselves

	if (keyToFetch && currentTags.length > 0) {
		fetchTagData(keyToFetch);
	} else {
		// This handles deselection or if no tags were loaded initially and selectedTagKey remains empty
		selectedTag = null;
		userReputation = null;
		topUsers = [];
		recentVotes = [];
		userRecentActivity = [];
		// Only set pageLoading to false if we are not in the initial tags loading phase.
		// If initialTagsLoading is true, onMount's logic will handle pageLoading.
		if (!initialTagsLoading) {
			pageLoading = false;
		}
	}
});

async function fetchTagData(tagKey: string) {
	pageLoading = true;
	error = null;
	selectedTag = null; // Clear previous selected tag
	userReputation = null;
	topUsers = [];
	recentVotes = [];
	userRecentActivity = [];
	try {
		const foundTag = tags.find((t) => t.key === tagKey);
		if (!foundTag) {
			throw new Error(`Tag with key ${tagKey} not found.`);
		}
		selectedTag = foundTag;

		// Simulate fetching other data for this tag for now
		// In a real app, these would be actual async calls based on selectedTag.key

		// Only fetch user-specific data if logged in
		if ($authUserDoc) {
			userReputation = { score: 123, rank: 5, badges: ['Active', 'Top Voter'] };
			userRecentActivity = [ { target: 'bob', value: 1, date: '2024-06-01' }, { target: 'carol', value: -1, date: '2024-05-30' }];
		} else {
			// Already nullified above
		}
		topUsers = [ { username: 'alice', score: 200, bar: 1 }, { username: 'bob', score: 180, bar: 0.9 }, { username: 'carol', score: 150, bar: 0.75 } ];
		recentVotes = [ { author: 'alice', target: 'bob', value: 1 }, { author: 'carol', target: 'alice', value: -1 } ];

	} catch (e) {
		error = e instanceof Error ? e.message : 'Failed to load data for the selected tag.';
		toaster.error({ title: error });
		// selectedTag is already nullified at the start of this function
	} finally {
		pageLoading = false;
	}
}

function onTagChange(event: Event) {
	const newKey = (event.target as HTMLSelectElement).value;
	// The $effect will pick up the change to selectedTagKey
	selectedTagKey = newKey; 
}
// Removed console.log statements from Tabs onValueChange for cleaner code
</script>

<NotLoggedInAlert />

<!-- Main Container -->
<div class="container mx-auto p-4">
	{#if error && !pageLoading } <!-- Show general error if not also loading -->
		<div class="alert alert-error mb-6">{error}</div>
	{/if}

	<!-- Header Section -->
		<div class="flex flex-row items-center justify-between gap-4 mb-6">
			<!-- Left side: Context text and Tag Selector -->
			<div class="flex items-center gap-4">
				<span class="text-lg text-surface-500">You are exploring:</span>
				<select 
					class="input input-lg"
					bind:value={selectedTagKey} 
					on:change={onTagChange}
					disabled={initialTagsLoading || tags.length === 0}
				>
					{#if initialTagsLoading}
						<option value="" disabled selected>Loading tags...</option>
					{:else if tags.length === 0}
						<option value="" disabled selected>No tags available</option>
					{:else}
						<option value="" disabled={selectedTagKey !== ''}>Select a tag...</option>
						{#each tags as tag (tag.key)}
							<option value={tag.key}>{tag.data.tag_handle}</option>
						{/each}
					{/if}
				</select>
				
				{#if !initialTagsLoading && tags.length > 0 && !selectedTagKey && !pageLoading} 
					<h1 class="text-2xl font-bold text-error-500 ml-4">Select a tag</h1>
				{/if}
			</div>

			<!-- Right side: Global Time Filter -->
			<div class="flex gap-2">
				{#each ['24h', '7d', '30d', '90d', '1y'] as period}
					<button 
						class="btn preset-tonal-primary text-xs"
						class:preset-filled-primary-500={selectedPeriod === period && !pageLoading && selectedTag}
						class:bg-surface-500_10={pageLoading || !selectedTagKey || (Boolean(selectedTagKey) && !selectedTag)} 
						class:text-surface-500_50={pageLoading || !selectedTagKey || (Boolean(selectedTagKey) && !selectedTag)} 
						on:click={() => selectedPeriod = period}
						disabled={pageLoading || !selectedTagKey || (Boolean(selectedTagKey) && !selectedTag)}
					>
						{period}
					</button>
				{/each}
			</div>
		</div>

		<!-- Tag Info & Settings & New Activity Card -->
		<div class="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-6">
			<!-- About (#tag_handle) & Settings Tabs -->
			<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6 col-span-1">
				<Tabs value={activeTab} onValueChange={async (e) => { activeTab = e.value; await tick();}}>
					{#snippet list()}
						<Tabs.Control value="about" disabled={(initialTagsLoading && !selectedTagKey) || (Boolean(selectedTagKey) && pageLoading && !selectedTag?.data?.description)}>
							{#snippet lead()}<Orbit size={20} />{/snippet}
							{#if selectedTag}#{selectedTag.data.tag_handle}{:else}About{/if}
						</Tabs.Control>
						<Tabs.Control value="settings" disabled={(initialTagsLoading && !selectedTagKey) || (Boolean(selectedTagKey) && pageLoading && !selectedTag?.data)}>
							{#snippet lead()}<SlidersHorizontal size={20} />{/snippet}
							Settings
						</Tabs.Control>
					{/snippet}
					{#snippet content()}
						<Tabs.Panel value="about">
							<div class="pt-4 max-h-60 overflow-y-auto">
								{#if (initialTagsLoading && !selectedTagKey) || (Boolean(selectedTagKey) && pageLoading && !selectedTag?.data?.description) }
									<div class="placeholder animate-pulse w-full h-24 rounded"></div>
								{:else if selectedTag?.data?.description}
									<p class="whitespace-pre-line opacity-80">{selectedTag.data.description}</p>
								{:else if selectedTag && !selectedTag.data?.description}
									<p class="opacity-50 text-sm">No description available for this tag.</p>
								{:else if !initialTagsLoading && tags.length > 0 && !selectedTagKey}
									<p class="text-center opacity-70">Select a tag to see its details.</p>
								{:else if !initialTagsLoading && tags.length === 0}
									<p class="text-center opacity-70">No tags found to display.</p>
								{/if}
							</div>
						</Tabs.Panel>
						<Tabs.Panel value="settings">
							<div class="pt-4">
								{#if (initialTagsLoading && !selectedTagKey) || (Boolean(selectedTagKey) && pageLoading && !selectedTag?.data)}
									<div class="placeholder animate-pulse w-1/2 h-8 rounded mb-4"></div>
									<div class="grid grid-cols-2 gap-4">
										<div class="p-3 bg-surface-200-800 rounded placeholder animate-pulse h-16"></div>
										<div class="p-3 bg-surface-200-800 rounded placeholder animate-pulse h-16"></div>
										<div class="p-3 bg-surface-200-800 rounded placeholder animate-pulse h-16"></div>
									</div>
								{:else if selectedTag?.data}
									<div class="flex justify-between items-center mb-4">
										{#if $authUserDoc?.data.user_key === selectedTag.data.user_key}
											<button class="btn preset-tonal-primary" on:click={() => goto(`/tag/edit/${selectedTagKey}`)}>
												Edit Settings
											</button>
										{/if}
									</div>
									<div class="grid grid-cols-2 gap-4">
										<div class="p-3 bg-surface-200-800 rounded">
											<span class="text-sm opacity-70">Reputation Threshold</span>
											<p class="font-mono text-lg">{selectedTag.data.reputation_threshold ?? 'N/A'}</p>
										</div>
										<div class="p-3 bg-surface-200-800 rounded">
											<span class="text-sm opacity-70">Vote Reward</span>
											<p class="font-mono text-lg">{selectedTag.data.vote_reward ?? 'N/A'}</p>
										</div>
										<div class="p-3 bg-surface-200-800 rounded">
											<span class="text-sm opacity-70">Min Users</span>
											<p class="font-mono text-lg">{selectedTag.data.min_users_for_threshold ?? 'N/A'}</p>
										</div>
									</div>
									<hr class="my-4 border-surface-300-700" />
									<div>
										<h4 class="text-md font-semibold mb-2">Decay Rules</h4>
										<p class="text-sm opacity-70">
											{selectedTag.data.decay_rules_description || 'Decay rules for this tag are not currently specified. Reputation may be subject to periodic adjustments based on overall network activity or specific tag settings that are not detailed here.'}
										</p>
										{#if selectedTag.data.decay_percentage && selectedTag.data.decay_period_days}
											<div class="mt-2 p-2 bg-surface-200-800 rounded text-xs">
												Reputation score decays by <span class="font-semibold">{selectedTag.data.decay_percentage}%</span> every <span class="font-semibold">{selectedTag.data.decay_period_days} days</span> if no new positive reputation is gained.
											</div>
										{/if}
									</div>
								{:else if !initialTagsLoading && tags.length > 0 && !selectedTagKey}
									<p class="text-center opacity-70">Select a tag to see its settings.</p>
								{:else if !initialTagsLoading && tags.length === 0}
									<p class="text-center opacity-70">No tags found to display settings for.</p>
								{/if}
							</div>
						</Tabs.Panel>
					{/snippet}
				</Tabs>
			</div>

			<!-- User Activity & Reputation Card (New) -->
			<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6 col-span-1">
				<div class="flex justify-between items-center mb-4">
					<h2 class="text-lg font-bold {((!selectedTag && !initialTagsLoading && !pageLoading && tags.length > 0 && !selectedTagKey) || (initialTagsLoading && !selectedTagKey) || !$authUserDoc) ? 'opacity-50' : ''}">Your Activity</h2>
					{#if $authUserDoc && selectedTag}
						<button class="btn btn-sm preset-tonal-primary" on:click={() => goto(`/tags/${selectedTagKey}/reputation`)} disabled={!selectedTagKey || !$authUserDoc || initialTagsLoading || (Boolean(selectedTagKey) && pageLoading && !userReputation)}>
							<Expand class="w-3 h-3 mr-1" /> Full Details
						</button>
					{/if}
				</div>

				{#if !$authUserDoc}
					<p class="text-center opacity-60 py-10">Log in to see your activity and reputation for this tag.</p>
				{:else if (initialTagsLoading && !selectedTagKey) || (Boolean(selectedTagKey) && pageLoading && !userReputation)}
					<div class="placeholder animate-pulse w-full h-40 rounded"></div>
				{:else if selectedTag && userReputation}
					<div class="grid grid-cols-2 gap-4 mb-4">
						<div class="p-3 bg-surface-200-800 rounded">
							<span class="text-sm opacity-70">Score</span>
							<p class="text-2xl font-bold">{userReputation.score}</p>
						</div>
						<div class="p-3 bg-surface-200-800 rounded">
							<span class="text-sm opacity-70">Rank</span>
							<p class="text-2xl font-bold">#{userReputation.rank}</p>
						</div>
					</div>
					<div class="mb-3">
						<div class="flex justify-start gap-1 mb-2">
							<button class="btn btn-xs preset-tonal-primary {userActivityFilter === 'both' ? 'preset-filled-primary-500' : ''}" on:click={() => userActivityFilter = 'both'}>All</button>
							<button class="btn btn-xs preset-tonal-success {userActivityFilter === 'in' ? 'preset-filled-success-500' : ''}" on:click={() => userActivityFilter = 'in'}>Votes Received</button>
							<button class="btn btn-xs preset-tonal-warning {userActivityFilter === 'out' ? 'preset-filled-warning-500' : ''}" on:click={() => userActivityFilter = 'out'}>Votes Cast</button>
						</div>
						<div class="max-h-48 overflow-y-auto bg-surface-200-800 rounded p-2 space-y-1">
							{#if userRecentActivity.length > 0}
								{#each userRecentActivity.filter(activity => userActivityFilter === 'both' || (userActivityFilter === 'in' && activity.value > 0 ) || (userActivityFilter === 'out' && activity.value < 0)) as activity (activity.date + activity.target)}
									<div class="text-xs p-1 rounded {activity.value > 0 ? 'bg-success-500/10' : 'bg-error-500/10'}">
										{activity.value > 0 ? 'Received' : 'Cast'} vote on <strong>{activity.target}</strong> ({new Date(activity.date).toLocaleDateString()})
									</div>
								{:else}
									<p class="text-center text-xs opacity-50 py-2">No activities match the filter.</p>
								{/each}
							{:else}
								<p class="text-center text-xs opacity-50 py-2">No recent activity for this tag.</p>
							{/if}
						</div>
					</div>
				{:else if !initialTagsLoading && tags.length > 0 && !selectedTagKey}
					<p class="text-center opacity-70 py-10">Select a tag to see your activity.</p>
				{:else if !initialTagsLoading && tags.length === 0}
					<p class="text-center opacity-70 py-10">No tags available to show activity for.</p>
				{/if}
			</div>

			<!-- Graph Preview -->
			<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6 col-span-1">
				<div class="flex justify-between items-center mb-4">
					<h2 class="text-lg font-bold {((!selectedTag && !initialTagsLoading && !pageLoading && tags.length > 0 && !selectedTagKey) || (initialTagsLoading && !selectedTagKey)) ? 'opacity-50' : ''}">Graph Overview</h2>
					<button class="btn preset-tonal-primary" on:click={() => goto(`/tags/${selectedTagKey}/graph`)} disabled={!selectedTagKey || initialTagsLoading || (Boolean(selectedTagKey) && pageLoading && !selectedTag)}>
						<Expand class="w-4 h-4 mr-2" />
						View Full Graph
					</button>
				</div>
				<div class="w-full h-64 bg-surface-200-800 rounded flex items-center justify-center">
					{#if (initialTagsLoading && !selectedTagKey) || (pageLoading && selectedTagKey && !selectedTag) }
						<div class="placeholder animate-pulse w-3/4 h-8 rounded"></div>
					{:else if selectedTag}
						<span class="opacity-50">Graph visualization coming soon…</span>
					{:else if !initialTagsLoading && tags.length > 0 && !selectedTagKey}
						<span class="opacity-50">Select a tag to see graph overview.</span>
					{:else if !initialTagsLoading && tags.length === 0}
						<span class="opacity-50">No tags available for graph.</span>
					{/if}
				</div>
			</div>
		</div>

		<!-- Stats Overview: Assuming stats are always available or have their own fine-grained loading not tied to selectedTag -->
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
			{#each ['Total Users', 'Verified Users', 'Active Users'] as statItem, i}
				<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6" 
					 class:opacity-50={ (initialTagsLoading && !selectedTagKey) || (pageLoading && selectedTagKey && !selectedTag) } >
					<h3 class="text-sm opacity-70">{statItem}</h3>
					<p class="text-2xl font-bold">
						{#if statItem === 'Total Users'}{stats.totalUsers}{/if}
						{#if statItem === 'Verified Users'}{stats.verifiedUsers}{/if}
						{#if statItem === 'Active Users'}{stats.activeUsers}{/if}
					</p>
					<div class="mt-2 h-1 w-full bg-surface-200-800 rounded-full overflow-hidden">
						<div 
							class="h-full {statItem === 'Total Users' ? 'bg-primary-500' : statItem === 'Verified Users' ? 'bg-success-500' : 'bg-warning-500'}" 
							style="width: {statItem === 'Total Users' ? 100 : statItem === 'Verified Users' ? (stats.verifiedUsers / stats.totalUsers * 100) : (stats.activeUsers / stats.totalUsers * 100)}%"
						></div>
					</div>
				</div>
			{/each}
		</div>

		<!-- Activity Sections -->
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
			<!-- Recent Votes -->
			<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
				<div class="flex justify-between items-center mb-4">
					<h2 class="text-lg font-bold {((!selectedTag && !initialTagsLoading && !pageLoading && tags.length > 0 && !selectedTagKey) || (initialTagsLoading && !selectedTagKey)) ? 'opacity-50' : ''}">Recent Votes</h2>
					<button class="btn preset-tonal-primary" on:click={() => goto(`/tags/${selectedTagKey}/votes`)} disabled={!selectedTagKey || initialTagsLoading || (Boolean(selectedTagKey) && pageLoading && recentVotes.length === 0 && !selectedTag && tags.length > 0)}>
						<Expand class="w-4 h-4 mr-2" />
						See More
					</button>
				</div>
				{#if (initialTagsLoading && !selectedTagKey) || (pageLoading && selectedTagKey && recentVotes.length === 0 && !selectedTag && tags.length > 0) }
					<div class="space-y-2">
						{#each Array(3) as _}
							<div class="flex justify-between items-center placeholder animate-pulse h-8 rounded"></div>
						{/each}
					</div>
				{:else if selectedTag && recentVotes.length > 0}
					<div class="table-wrap">
						<table class="table caption-bottom">
							<thead><tr><th>From</th><th>To</th><th class="text-right">Value</th></tr></thead>
							<tbody class="[&>tr]:hover:preset-tonal-primary">
								{#each recentVotes as vote (vote.author + vote.target + (vote.date || Math.random()))}
									<tr>
										<td class="font-mono">{vote.author}</td>
										<td class="font-mono">{vote.target}</td>
										<td class="text-right"><span class="badge preset-filled-{vote.value > 0 ? 'success' : 'error'}-500">{vote.value > 0 ? '+1' : '-1'}</span></td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				{:else if selectedTag && recentVotes.length === 0}
					<p class="text-center opacity-70">No recent votes to display for this tag.</p>
				{:else if !initialTagsLoading && tags.length > 0 && !selectedTagKey}
					<p class="text-center opacity-70">Select a tag to see recent votes.</p>
				{:else if !initialTagsLoading && tags.length === 0}
					<p class="text-center opacity-70">No tags available.</p>
				{/if}
			</div>

			<!-- Top Users -->
			<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
				<div class="flex justify-between items-center mb-4">
					<h2 class="text-lg font-bold {((!selectedTag && !initialTagsLoading && !pageLoading && tags.length > 0 && !selectedTagKey) || (initialTagsLoading && !selectedTagKey)) ? 'opacity-50' : ''}">Top Users</h2>
					<button class="btn preset-tonal-primary" on:click={() => goto(`/tags/${selectedTagKey}/users`)} disabled={!selectedTagKey || initialTagsLoading || (Boolean(selectedTagKey) && pageLoading && topUsers.length === 0 && !selectedTag && tags.length > 0)}>
						<Expand class="w-4 h-4 mr-2" />
						See More
					</button>
				</div>
				{#if (initialTagsLoading && !selectedTagKey) || (pageLoading && selectedTagKey && topUsers.length === 0 && !selectedTag && tags.length > 0) }
					<div class="space-y-2">
						{#each Array(3) as _}
							<div class="flex items-center gap-2 placeholder animate-pulse h-10 rounded"></div>
						{/each}
					</div>
				{:else if selectedTag && topUsers.length > 0}
					<div class="table-wrap">
						<table class="table caption-bottom">
							<thead><tr><th>User</th><th class="text-right">Score</th></tr></thead>
							<tbody class="[&>tr]:hover:preset-tonal-primary">
								{#each topUsers as user, i (user.username)}
									<tr>
										<td><div class="flex items-center gap-2"><Avatar name={user.username}><UserRoundPen class="w-6 h-6 text-surface-700" /></Avatar><span class="font-bold">{user.username}</span>{#if i === 0}<span class="text-yellow-500">🥇</span>{:else if i === 1}<span class="text-gray-400">🥈</span>{:else if i === 2}<span class="text-orange-700">🥉</span>{/if}</div></td>
										<td class="text-right"><span class="badge preset-filled-primary-500">{user.score} points</span></td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				{:else if selectedTag && topUsers.length === 0}
					<p class="text-center opacity-70">No top users to display for this tag.</p>
				{:else if !initialTagsLoading && tags.length > 0 && !selectedTagKey}
					<p class="text-center opacity-70">Select a tag to see top users.</p>
				{:else if !initialTagsLoading && tags.length === 0}
					<p class="text-center opacity-70">No tags available.</p>
				{/if}
			</div>
		</div>

		<!-- Call to Action -->
		<div class="mb-6">
			{#if (initialTagsLoading && !selectedTagKey) || (pageLoading && selectedTagKey && !selectedTag) }
				<div class="placeholder animate-pulse w-full h-12 rounded"></div>
			{:else if selectedTag}
				<button class="btn preset-filled-primary-500 w-full">
					{userReputation && userReputation.score > 0 ? 'Contribute' : 'Join Community'}
				</button>
			{:else if !initialTagsLoading && tags.length > 0 && !selectedTagKey}
				<button class="btn preset-filled-primary-500 w-full" disabled>
					Select a Tag
				</button>
			{:else if !initialTagsLoading && tags.length === 0}
				<button class="btn preset-filled-primary-500 w-full" disabled>
					No Tags Available
				</button>
			{/if}
		</div>
</div>