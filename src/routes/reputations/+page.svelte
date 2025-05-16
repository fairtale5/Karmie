<script lang="ts">
// --- Skeleton v3 Toasts: Ensure <Toaster /> is present in your root layout (e.g., +layout.svelte) ---
import { onMount } from 'svelte';
import { listDocs, type Doc, authSubscribe, type User } from '@junobuild/core';
import { toaster } from '$lib/skeletonui/toaster-skeleton';
// import sigma.js for future graph integration (placeholder for now)
// import Sigma from 'sigma';
import SkeletonLoader from '$lib/components/common/SkeletonLoader.svelte';
import { initJuno } from '$lib/juno';
import { Avatar } from '@skeletonlabs/skeleton-svelte';
import { UserRoundPen } from 'lucide-svelte';
import NotLoggedInAlert from '$lib/components/common/NotLoggedInAlert.svelte';

// --- State ---
let loading = true;
let error: string | null = null;
let user: User | null = null;
let tags: Doc<any>[] = [];
let selectedTagKey = '';
let selectedTag: Doc<any> | null = null;
let userReputation: any = null;
let topUsers: any[] = [];
let recentVotes: any[] = [];
let userRecentActivity: any[] = [];

// --- Fetch Data ---
onMount(async () => {
	// Ensure Juno is initialized before any backend calls (see docs/README.md#juno-integration)
	await initJuno();
	try {
		loading = true;
		error = null;
		// Subscribe to auth state
		authSubscribe((state) => {
			user = state;
			if (!user) {
				error = 'You must be logged in to view this page.';
				loading = false;
			}
		});
		// Fetch tags (reputation communities)
		const tagsList = await listDocs({ collection: 'tags' });
		tags = tagsList.items;
		if (tags.length > 0) {
			selectedTagKey = tags[0].key;
			selectedTag = tags[0];
			await fetchTagData();
		}
	} catch (e) {
		error = e instanceof Error ? e.message : 'Failed to load data.';
		toaster.error({ title: error });
	} finally {
		loading = false;
	}
});

// --- Fetch Data for Selected Tag ---
async function fetchTagData() {
	try {
		loading = true;
		error = null;
		selectedTag = tags.find((t) => t.key === selectedTagKey) || null;
		if (!selectedTag) return;
		// Fetch user's reputation in this tag (stub, replace with real call)
		userReputation = {
			score: 123,
			rank: 5,
			badges: ['Active', 'Top Voter']
		};
		// Fetch top users (stub, replace with real call)
		topUsers = [
			{ username: 'alice', score: 200, bar: 1 },
			{ username: 'bob', score: 180, bar: 0.9 },
			{ username: 'carol', score: 150, bar: 0.75 },
			// ...
		];
		// Fetch most recent votes (stub, replace with real call)
		recentVotes = [
			{ author: 'alice', target: 'bob', value: 1 },
			{ author: 'carol', target: 'alice', value: -1 },
			// ...
		];
		// Fetch user's recent activity (stub, replace with real call)
		userRecentActivity = [
			{ target: 'bob', value: 1, date: '2024-06-01' },
			{ target: 'carol', value: -1, date: '2024-05-30' },
			// ...
		];
	} catch (e) {
		error = e instanceof Error ? e.message : 'Failed to load tag data.';
		toaster.error({ title: error });
	} finally {
		loading = false;
	}
}

// --- Handle Tag Change ---
function onTagChange(event: Event) {
	selectedTagKey = (event.target as HTMLSelectElement).value;
	fetchTagData();
}
</script>

<!-- Show warning if not logged in -->
<NotLoggedInAlert />

<!-- Main Container -->
<div class="container mx-auto p-4">
	{#if loading}
		<!-- Loading State: Use reusable SkeletonLoader for cards and lists -->
		<SkeletonLoader count={1} variant="card" />
		<SkeletonLoader count={1} variant="list" />
		<SkeletonLoader count={1} variant="card" />
	{:else if error}
		<!-- Error State -->
		<div class="alert alert-error">{error}</div>
	{:else}
		<!-- Reputation Tag Selector -->
		<div class="mb-6">
			<label for="reputation-select" class="block mb-2 text-lg font-bold">Select Reputation Community</label>
			<select id="reputation-select" class="input input-lg w-full" bind:value={selectedTagKey} on:change={onTagChange}>
				{#each tags as tag}
					<option value={tag.key}>{tag.data.name}</option>
				{/each}
			</select>
		</div>

		<!-- Community Description -->
		{#if selectedTag}
			<div class="card p-4 mb-6 bg-surface-100-900">
				<div class="font-bold text-xl mb-2">{selectedTag.data.name}</div>
				<div class="text-base opacity-80">{selectedTag.data.description}</div>
			</div>
		{/if}

		<!-- User's Reputation in Selected Tag -->
		{#if userReputation}
			<div class="card preset-tonal-primary grid grid-cols-1 items-center gap-4 p-4 mb-6 lg:grid-cols-[1fr_auto]">
				<div>
					<div class="font-bold text-lg">Your Reputation</div>
					<div>Score: <span class="font-mono">{userReputation.score}</span></div>
					<div>Rank: <span class="font-mono">#{userReputation.rank}</span></div>
					<div class="flex gap-2">
						{#each userReputation.badges as badge}
							<span class="badge badge-success">{badge}</span>
						{/each}
					</div>
				</div>
			</div>
		{/if}

		<!-- User's Recent Activity -->
		<div class="mb-6">
			<div class="font-bold mb-2">Your Recent Activity</div>
			{#if userRecentActivity.length === 0}
				<div class="opacity-60">No recent activity.</div>
			{:else}
				<ul class="space-y-1">
					{#each userRecentActivity as activity}
						<li class="flex items-center gap-2">
							<span class={activity.value > 0 ? 'text-success-500' : 'text-error-500'}>
								{activity.value > 0 ? '+1' : '-1'}
							</span>
							<span>to {activity.target}</span>
							<span class="text-xs opacity-60">({activity.date})</span>
						</li>
					{/each}
				</ul>
			{/if}
		</div>

		<!-- Top Users in Reputation -->
		<div class="mb-6">
			<div class="font-bold mb-2">Top Users</div>
			{#if topUsers.length === 0}
				<div class="opacity-60">No users yet.</div>
			{:else}
				<ul class="space-y-2">
					{#each topUsers as user, i}
						<li class="card shadow bg-surface-100-900 border border-surface-200-800 grid grid-cols-[auto_1fr_auto] items-center gap-4 p-4 relative">
							<!-- Avatar with fallback icon -->
							<Avatar name={user.username}>
								<UserRoundPen class="w-8 h-8 text-surface-700" />
							</Avatar>
							<!-- User info -->
							<div>
								<p class="font-bold">{user.username}</p>
								<p class="opacity-60 text-xs">@{user.username}</p>
							</div>
							<!-- Score and bar graph -->
							<div class="flex flex-col items-end z-10">
								<span class="font-bold">{user.score}</span>
								<span class="absolute left-0 bottom-0 h-2 rounded bg-primary-500 opacity-20" style="width: {user.bar * 100}%;"></span>
								{#if i === 0}
									<span class="ml-2 text-yellow-500">🥇</span>
								{:else if i === 1}
									<span class="ml-2 text-gray-400">🥈</span>
								{:else if i === 2}
									<span class="ml-2 text-orange-700">🥉</span>
								{/if}
							</div>
						</li>
					{/each}
				</ul>
			{/if}
		</div>

		<!-- Most Recent Votes -->
		<div class="mb-6">
			<div class="font-bold mb-2">Most Recent Votes</div>
			{#if recentVotes.length === 0}
				<div class="opacity-60">No votes yet.</div>
			{:else}
				<ul class="space-y-1">
					{#each recentVotes as vote}
						<li class="flex items-center gap-2">
							<span class="font-mono">{vote.author}</span>
							<span>→</span>
							<span class="font-mono">{vote.target}</span>
							<span class={vote.value > 0 ? 'text-success-500' : 'text-error-500'}>
								{vote.value > 0 ? '+1' : '-1'}
							</span>
						</li>
					{/each}
				</ul>
			{/if}
		</div>

		<!-- Graph Overview (sigma.js placeholder) -->
		<div class="mb-6">
			<div class="font-bold mb-2">Graph Overview</div>
			<div class="w-full h-64 bg-surface-200-800 rounded flex items-center justify-center opacity-50">
				<!-- TODO: Integrate sigma.js graph here -->
				<span>Graph visualization coming soon…</span>
			</div>
		</div>

		<!-- Call to Action -->
		<div class="mb-6">
			{#if user && selectedTag}
				<button class="btn preset-filled-primary-500 w-full">
					{userReputation && userReputation.score > 0 ? 'Contribute' : 'Join Community'}
				</button>
			{/if}
		</div>
	{/if}
</div> 