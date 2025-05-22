<script lang="ts">
// --- Skeleton v3 Toasts: Ensure <Toaster /> is present in your root layout (e.g., +layout.svelte) ---
import { onMount } from 'svelte';
import { listDocs, type Doc } from '@junobuild/core';
import { toaster } from '$lib/skeletonui/toaster-skeleton';
import { goto } from '$app/navigation';
// import sigma.js for future graph integration (placeholder for now)
// import Sigma from 'sigma';
import SkeletonLoader from '$lib/components/common/SkeletonLoader.svelte';
import { initJuno } from '$lib/juno';
import { Avatar } from '@skeletonlabs/skeleton-svelte';
import { UserRoundPen, Expand } from 'lucide-svelte';
import NotLoggedInAlert from '$lib/components/common/NotLoggedInAlert.svelte';
import { authUserDoc } from '$lib/stores/authUserDoc';

// --- State ---
let loading = true;
let error: string | null = null;
let tags: Doc<any>[] = [];
let selectedTagKey = '';
let selectedTag: Doc<any> | null = null;
let userReputation: any = null;
let topUsers: any[] = [];
let recentVotes: any[] = [];
let userRecentActivity: any[] = [];
let selectedPeriod = '24h';

// Dummy stats data
let stats = {
	totalUsers: 1234,
	verifiedUsers: 567,
	activeUsers: 89
};

// --- Fetch Data ---
onMount(async () => {
	// Ensure Juno is initialized before any backend calls
	await initJuno();
	try {
		loading = true;
		error = null;

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

		// Only fetch user-specific data if logged in
		if ($authUserDoc) {
			// Fetch user's reputation in this tag (stub, replace with real call)
			userReputation = {
				score: 123,
				rank: 5,
				badges: ['Active', 'Top Voter']
			};
			// Fetch user's recent activity (stub, replace with real call)
			userRecentActivity = [
				{ target: 'bob', value: 1, date: '2024-06-01' },
				{ target: 'carol', value: -1, date: '2024-05-30' },
				// ...
			];
		} else {
			userReputation = null;
			userRecentActivity = [];
		}

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
		<!-- Header Section Skeleton -->
		<div class="flex flex-col gap-4 mb-6">
			<div class="flex items-center gap-4">
				<div class="placeholder animate-pulse w-64 h-12 rounded"></div>
				<div class="placeholder animate-pulse w-32 h-8 rounded"></div>
			</div>
			<div class="flex gap-2">
				{#each Array(5) as _}
					<div class="placeholder animate-pulse w-16 h-8 rounded"></div>
				{/each}
			</div>
		</div>

		<!-- Tag Info & Settings Skeleton -->
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
			<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
				<div class="placeholder animate-pulse w-24 h-6 mb-4 rounded"></div>
				<div class="placeholder animate-pulse w-full h-24 rounded"></div>
			</div>
			<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
				<div class="flex justify-between items-center mb-4">
					<div class="placeholder animate-pulse w-24 h-6 rounded"></div>
					<div class="placeholder animate-pulse w-32 h-10 rounded"></div>
				</div>
				<div class="grid grid-cols-2 gap-4">
					{#each Array(3) as _}
						<div class="p-3 bg-surface-200-800 rounded">
							<div class="placeholder animate-pulse w-32 h-4 mb-2 rounded"></div>
							<div class="placeholder animate-pulse w-16 h-8 rounded"></div>
						</div>
					{/each}
				</div>
			</div>
		</div>

		<!-- Stats Overview Skeleton -->
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
			{#each Array(3) as _}
				<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
					<div class="placeholder animate-pulse w-24 h-4 mb-2 rounded"></div>
					<div class="placeholder animate-pulse w-16 h-8 mb-2 rounded"></div>
					<div class="placeholder animate-pulse w-full h-1 rounded"></div>
				</div>
			{/each}
		</div>

		<!-- User's Tag Info Skeleton -->
		<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6 mb-6">
			<div class="flex justify-between items-center mb-4">
				<div class="placeholder animate-pulse w-32 h-6 rounded"></div>
				<div class="placeholder animate-pulse w-24 h-10 rounded"></div>
			</div>
			<div class="grid grid-cols-3 gap-4">
				{#each Array(3) as _}
					<div class="p-3 bg-surface-200-800 rounded">
						<div class="placeholder animate-pulse w-16 h-4 mb-2 rounded"></div>
						<div class="placeholder animate-pulse w-12 h-8 rounded"></div>
					</div>
				{/each}
			</div>
		</div>

		<!-- Activity Sections Skeleton -->
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
			{#each Array(2) as _}
				<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
					<div class="flex justify-between items-center mb-4">
						<div class="placeholder animate-pulse w-32 h-6 rounded"></div>
						<div class="placeholder animate-pulse w-24 h-10 rounded"></div>
					</div>
					<div class="space-y-2">
						{#each Array(3) as _}
							<div class="flex justify-between items-center">
								<div class="placeholder animate-pulse w-24 h-6 rounded"></div>
								<div class="placeholder animate-pulse w-16 h-6 rounded"></div>
							</div>
						{/each}
					</div>
				</div>
			{/each}
		</div>

		<!-- Graph Preview Skeleton -->
		<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6 mb-6">
			<div class="flex justify-between items-center mb-4">
				<div class="placeholder animate-pulse w-32 h-6 rounded"></div>
				<div class="placeholder animate-pulse w-32 h-10 rounded"></div>
			</div>
			<div class="placeholder animate-pulse w-full h-64 rounded"></div>
		</div>

		<!-- Call to Action Skeleton -->
		<div class="mb-6">
			<div class="placeholder animate-pulse w-full h-12 rounded"></div>
		</div>
	{:else if error}
		<!-- Error State -->
		<div class="alert alert-error">{error}</div>
	{:else}
		<!-- Header Section -->
		<div class="flex flex-col gap-4 mb-6">
			<!-- Tag Selector & Title -->
			<div class="flex items-center gap-4">
				<select class="input input-lg" bind:value={selectedTagKey} on:change={onTagChange}>
					{#each tags as tag}
						<option value={tag.key}>{tag.data.tag_handle}</option>
					{/each}
				</select>
				<h1 class="text-2xl font-bold">#{selectedTag?.data.tag_handle}</h1>
			</div>

			<!-- Global Time Filter -->
			<div class="flex gap-2">
				{#each ['24h', '7d', '30d', '90d', '1y'] as period}
					<button 
						class="btn preset-tonal-primary text-xs" 
						class:preset-filled-primary-500={selectedPeriod === period}
						on:click={() => selectedPeriod = period}
					>
						{period}
					</button>
				{/each}
			</div>
		</div>

		<!-- Tag Info & Settings -->
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
			<!-- Tag Description -->
			<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
				<h2 class="text-lg font-bold mb-2">About</h2>
				<p class="whitespace-pre-line opacity-80">{selectedTag?.data.description}</p>
			</div>

			<!-- Tag Settings -->
			<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
				<div class="flex justify-between items-center mb-4">
					<h2 class="text-lg font-bold">Settings</h2>
					{#if $authUserDoc?.data.user_key === selectedTag?.data.user_key}
						<button class="btn preset-tonal-primary" on:click={() => goto(`/tag/edit/${selectedTagKey}`)}>
							Edit Settings
						</button>
					{/if}
				</div>
				<div class="grid grid-cols-2 gap-4">
					<div class="p-3 bg-surface-200-800 rounded">
						<span class="text-sm opacity-70">Reputation Threshold</span>
						<p class="font-mono text-lg">{selectedTag?.data.reputation_threshold}</p>
					</div>
					<div class="p-3 bg-surface-200-800 rounded">
						<span class="text-sm opacity-70">Vote Reward</span>
						<p class="font-mono text-lg">{selectedTag?.data.vote_reward}</p>
					</div>
					<div class="p-3 bg-surface-200-800 rounded">
						<span class="text-sm opacity-70">Min Users</span>
						<p class="font-mono text-lg">{selectedTag?.data.min_users_for_threshold}</p>
					</div>
				</div>
			</div>
		</div>

		<!-- Stats Overview -->
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
			<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
				<h3 class="text-sm opacity-70">Total Users</h3>
				<p class="text-2xl font-bold">{stats.totalUsers}</p>
				<div class="mt-2 h-1 w-full bg-surface-200-800 rounded-full overflow-hidden">
					<div class="h-full bg-primary-500" style="width: 100%"></div>
				</div>
			</div>
			<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
				<h3 class="text-sm opacity-70">Verified Users</h3>
				<p class="text-2xl font-bold">{stats.verifiedUsers}</p>
				<div class="mt-2 h-1 w-full bg-surface-200-800 rounded-full overflow-hidden">
					<div class="h-full bg-success-500" style="width: {stats.verifiedUsers / stats.totalUsers * 100}%"></div>
				</div>
			</div>
			<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
				<h3 class="text-sm opacity-70">Active Users</h3>
				<p class="text-2xl font-bold">{stats.activeUsers}</p>
				<div class="mt-2 h-1 w-full bg-surface-200-800 rounded-full overflow-hidden">
					<div class="h-full bg-warning-500" style="width: {stats.activeUsers / stats.totalUsers * 100}%"></div>
				</div>
			</div>
		</div>

		<!-- User's Tag Info -->
		{#if userReputation}
			<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6 mb-6">
				<div class="flex justify-between items-center mb-4">
					<h2 class="text-lg font-bold">Your Reputation</h2>
					<button class="btn preset-tonal-primary" on:click={() => goto(`/tags/${selectedTagKey}/reputation`)}>
						<Expand class="w-4 h-4 mr-2" />
						See More
					</button>
				</div>
				<div class="grid grid-cols-3 gap-4">
					<div class="p-3 bg-surface-200-800 rounded">
						<span class="text-sm opacity-70">Score</span>
						<p class="text-2xl font-bold">{userReputation.score}</p>
					</div>
					<div class="p-3 bg-surface-200-800 rounded">
						<span class="text-sm opacity-70">Rank</span>
						<p class="text-2xl font-bold">#{userReputation.rank}</p>
					</div>
					<div class="p-3 bg-surface-200-800 rounded">
						<span class="text-sm opacity-70">Recent Activity</span>
						<p class="text-2xl font-bold">{userRecentActivity.length}</p>
					</div>
				</div>
			</div>
		{/if}

		<!-- Activity Sections -->
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
			<!-- Recent Votes -->
			<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
				<div class="flex justify-between items-center mb-4">
					<h2 class="text-lg font-bold">Recent Votes</h2>
					<button class="btn preset-tonal-primary" on:click={() => goto(`/tags/${selectedTagKey}/votes`)}>
						<Expand class="w-4 h-4 mr-2" />
						See More
					</button>
				</div>
				<div class="table-wrap">
					<table class="table caption-bottom">
						<thead>
							<tr>
								<th>From</th>
								<th>To</th>
								<th class="text-right">Value</th>
							</tr>
						</thead>
						<tbody class="[&>tr]:hover:preset-tonal-primary">
							{#each recentVotes as vote}
								<tr>
									<td class="font-mono">{vote.author}</td>
									<td class="font-mono">{vote.target}</td>
									<td class="text-right">
										<span class="badge preset-filled-{vote.value > 0 ? 'success' : 'error'}-500">
											{vote.value > 0 ? '+1' : '-1'}
										</span>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			</div>

			<!-- Top Users -->
			<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
				<div class="flex justify-between items-center mb-4">
					<h2 class="text-lg font-bold">Top Users</h2>
					<button class="btn preset-tonal-primary" on:click={() => goto(`/tags/${selectedTagKey}/users`)}>
						<Expand class="w-4 h-4 mr-2" />
						See More
					</button>
				</div>
				<div class="table-wrap">
					<table class="table caption-bottom">
						<thead>
							<tr>
								<th>User</th>
								<th class="text-right">Score</th>
							</tr>
						</thead>
						<tbody class="[&>tr]:hover:preset-tonal-primary">
							{#each topUsers as user, i}
								<tr>
									<td>
										<div class="flex items-center gap-2">
											<Avatar name={user.username}>
												<UserRoundPen class="w-6 h-6 text-surface-700" />
											</Avatar>
											<span class="font-bold">{user.username}</span>
											{#if i === 0}
												<span class="text-yellow-500">🥇</span>
											{:else if i === 1}
												<span class="text-gray-400">🥈</span>
											{:else if i === 2}
												<span class="text-orange-700">🥉</span>
											{/if}
										</div>
									</td>
									<td class="text-right">
										<span class="badge preset-filled-primary-500">{user.score} points</span>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			</div>
		</div>

		<!-- Graph Preview -->
		<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6 mb-6">
			<div class="flex justify-between items-center mb-4">
				<h2 class="text-lg font-bold">Graph Overview</h2>
				<button class="btn preset-tonal-primary" on:click={() => goto(`/tags/${selectedTagKey}/graph`)}>
					<Expand class="w-4 h-4 mr-2" />
					View Full Graph
				</button>
			</div>
			<div class="w-full h-64 bg-surface-200-800 rounded flex items-center justify-center">
				<span class="opacity-50">Graph visualization coming soon…</span>
			</div>
		</div>

		<!-- Call to Action -->
		<div class="mb-6">
			{#if selectedTag}
				<button class="btn preset-filled-primary-500 w-full">
					{userReputation && userReputation.score > 0 ? 'Contribute' : 'Join Community'}
				</button>
			{/if}
		</div>
	{/if}
</div>