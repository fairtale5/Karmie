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
import QuickActionsTags from '$lib/components/tags/QuickActionsTags.svelte';
import RecentVotes from '$lib/components/tags/RecentVotes.svelte';
import type { TagDocument } from '$lib/types';

// --- Preview Data Constants ---
const PREVIEW_TAG_KEY = '___PREVIEW_DATA___';
const previewTagForList: TagDocument = {
	key: PREVIEW_TAG_KEY,
	data: {
		tag_handle: '✨ Preview Mode ✨',
		description: 'Currently displaying sample data. Select or create a real tag to see live information and interact with the platform.',
		reputation_threshold: 10,
		vote_reward: 0.1,
		min_users_for_threshold: 5,
		time_periods: [
			{ months: 1, multiplier: 0.95 },  // 5% decay after 1 month
			{ months: 3, multiplier: 0.90 },  // 10% decay after 3 months
			{ months: 6, multiplier: 0.80 }   // 20% decay after 6 months
		]
	}
};

const initialUserReputationPreview = { score: 123, rank: 5, badges: ['Active', 'Top Voter'] };
const initialTopUsersPreview = [
	{ username: 'alice', score: 200, bar: 1 },
	{ username: 'bob', score: 180, bar: 0.9 },
	{ username: 'carol', score: 150, bar: 0.75 }
];
const initialRecentVotesPreview = [
	{ author: 'alice', target: 'bob', value: 1, date: new Date().toISOString() },
	{ author: 'carol', target: 'alice', value: -1, date: new Date(Date.now() - 86400000).toISOString() } // 1 day ago
];
function generateInitialUserActivityPreview(): any[] {
	const activities = [];
	const peerNames = ['alpha', 'bravo', 'charlie', 'delta', 'echo', 'foxtrot', 'golf', 'hotel', 'india', 'juliet'];
	for (let i = 0; i < 5; i++) { // Reduced count for brevity in preview
		activities.push({
			id: `cast-preview-${i}`, type: 'cast', peerName: peerNames[i % peerNames.length],
			value: i < 3 ? 1 : -1, date: new Date(Date.now() - Math.random() * 1000000000).toISOString()
		});
		activities.push({
			id: `received-preview-${i}`, type: 'received', peerName: peerNames[(i + 2) % peerNames.length], // different peers
			value: i < 2 ? 1 : -1, date: new Date(Date.now() - Math.random() * 1000000000).toISOString()
		});
	}
	return activities.sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());
}

// --- State Management ---
let tags = $state<TagDocument[]>([]);
let selectedTag = $state<TagDocument | null>(null);
let loading = $state(true);
let error = $state<string | null>(null);
let userReputation = $state<any>(JSON.parse(JSON.stringify(initialUserReputationPreview)));
let topUsers = $state<any[]>(JSON.parse(JSON.stringify(initialTopUsersPreview)));
let recentVotes = $state<any[]>(JSON.parse(JSON.stringify(initialRecentVotesPreview)));
let userRecentActivity = $state<any[]>(generateInitialUserActivityPreview());
let cutoffTimestamp = $state(BigInt(new Date('2025-01-01T00:00:00Z').getTime()) * BigInt(1_000_000));
let activeTab = $state('about');
let userActivityFilter = $state('all');

// Dummy stats data (can be replaced with placeholders if fetched)
let stats = $state({
	totalUsers: 1234,
	verifiedUsers: 567,
	activeUsers: 89
});

// Define a cutoff far in the past for 'All' (2025-01-01T00:00:00Z)
const ALL_TIME_CUTOFF = BigInt(new Date('2025-01-01T00:00:00Z').getTime()) * BigInt(1_000_000);

// Define time periods, including "All"
const timePeriods = [
	{ label: '24h', ms: 24 * 60 * 60 * 1000 },
	{ label: '7d', ms: 7 * 24 * 60 * 60 * 1000 },
	{ label: '30d', ms: 30 * 24 * 60 * 60 * 1000 },
	{ label: '90d', ms: 90 * 24 * 60 * 60 * 1000 },
	{ label: '1y', ms: 365 * 24 * 60 * 60 * 1000 },
	{ label: 'All', ms: null }
];

// Default to last period (e.g., 'All' if last)
let selectedPeriod = $state(timePeriods[timePeriods.length - 1]);

// Set cutoffTimestamp based on selection
function setPeriod(period: { label: string; ms: number | null }) {
	selectedPeriod = period;
	if (period.label === 'All') {
		cutoffTimestamp = ALL_TIME_CUTOFF;
	} else if (period.ms !== null) {
		cutoffTimestamp = BigInt(Date.now() - period.ms) * BigInt(1_000_000);
	}
}

// --- Initialization ---
onMount(async () => {
	try {
		await initJuno();
		const result = await listDocs({ collection: 'tags' });
		const fetchedTags = result.items as TagDocument[];

		// Set tags list based on auth state
		tags = $authUserDoc 
			? [...fetchedTags, previewTagForList as TagDocument]
			: [previewTagForList as TagDocument, ...fetchedTags];

		// Set initial selected tag - default to preview mode if not logged in
		selectedTag = $authUserDoc 
			? (fetchedTags.length > 0 ? fetchedTags[0] : previewTagForList as TagDocument)
			: previewTagForList as TagDocument;
	} catch (e) {
		console.error('Failed to load tags:', e);
		error = e instanceof Error ? e.message : 'Failed to load tags';
		tags = [previewTagForList as TagDocument];
		selectedTag = previewTagForList as TagDocument;
	} finally {
		loading = false;
	}
});

// --- Tag Selection Effect ---
$effect(() => {
	if (!selectedTag) return;
	
	if (selectedTag.key === PREVIEW_TAG_KEY) {
		userReputation = JSON.parse(JSON.stringify(initialUserReputationPreview));
		topUsers = JSON.parse(JSON.stringify(initialTopUsersPreview));
		recentVotes = JSON.parse(JSON.stringify(initialRecentVotesPreview));
		userRecentActivity = generateInitialUserActivityPreview();
		return;
	}

	fetchTagData(selectedTag.key);
});

async function fetchTagData(tagKey: string) {
	loading = true;
	error = null;
	// Don't reset selectedTag - we already have the correct one
	userReputation = null;
	topUsers = [];
	recentVotes = [];
	userRecentActivity = [];
	
	try {
		const foundTag = tags.find((t) => t.key === tagKey);
		if (!foundTag) {
			throw new Error(`Tag with key ${tagKey} not found.`);
		}
		// No need to set selectedTag again - it's already set correctly
		// Just verify it matches what we expect
		if (foundTag.key !== selectedTag?.key) {
			throw new Error('Tag mismatch - this should never happen');
		}

		// Only fetch user-specific data if logged in
		if ($authUserDoc) {
			userReputation = { score: 123, rank: 5, badges: ['Active', 'Top Voter'] };
			// Generate diverse dummy data for userRecentActivity
			const activities = [];
			const peerNames = ['alpha', 'bravo', 'charlie', 'delta', 'echo', 'foxtrot', 'golf', 'hotel', 'india', 'juliet'];
			for (let i = 0; i < 10; i++) {
				// Cast votes (by current user)
				activities.push({
					id: `cast-${i}`,
					type: 'cast',
					peerName: peerNames[i % peerNames.length],
					value: i < 5 ? 1 : -1, // 5 positive, 5 negative
					date: new Date(Date.now() - Math.random() * 1000000000).toISOString()
				});
				// Received votes (by current user)
				activities.push({
					id: `received-${i}`,
					type: 'received',
					peerName: peerNames[(i + 5) % peerNames.length], // different peers for variety
					value: i < 5 ? 1 : -1, // 5 positive, 5 negative
					date: new Date(Date.now() - Math.random() * 1000000000).toISOString()
				});
			}
			userRecentActivity = activities.sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());
		}
		topUsers = [ { username: 'alice', score: 200, bar: 1 }, { username: 'bob', score: 180, bar: 0.9 }, { username: 'carol', score: 150, bar: 0.75 } ];
		recentVotes = [ { author: 'alice', target: 'bob', value: 1 }, { author: 'carol', target: 'alice', value: -1 } ];

	} catch (e) {
		error = e instanceof Error ? e.message : 'Failed to load data for the selected tag.';
		toaster.error({ title: error });
	} finally {
		loading = false;
	}
}

function onTagChange(event: Event) {
	const newKey = (event.target as HTMLSelectElement).value;
	// The $effect will pick up the change to selectedTag
	selectedTag = tags.find(t => t.key === newKey) || null; 
}
// Removed console.log statements from Tabs onValueChange for cleaner code
</script>

<NotLoggedInAlert />

<!-- Main Container -->
<div class="p-4">
	{#if error && !loading } <!-- Show general error if not also loading -->
		<div class="alert alert-error mb-6">{error}</div>
	{/if}

	<!-- Header Section -->
	<div class="flex flex-row items-center justify-between flex-wrap gap-4 mb-6">
		<!-- Left side: Context text and Tag Selector -->
		<div class="flex items-center gap-4">
			<span class="text-lg text-surface-500 whitespace-nowrap">You are exploring:</span>
			<select 
				class="input input-lg"
				value={selectedTag?.key ?? ''}
				onchange={onTagChange}
				disabled={loading || tags.length === 0}
			>
				{#if loading}
					<option value="" disabled selected>Loading tags...</option>
				{:else if tags.length === 0}
					<option value="" disabled selected>No tags available</option>
				{:else if tags.length === 1 && tags[0].key === PREVIEW_TAG_KEY && !$authUserDoc}
					<option value={PREVIEW_TAG_KEY} selected>✨ Preview Mode ✨</option>
					<option value="" disabled>No other tags found. Create one to get started!</option>
				{:else}
					<option value="" disabled={selectedTag?.key !== ''}>Select a tag...</option>
					{#each tags as tag (tag.key)}
						<option value={tag.key}>{tag.data.tag_handle}</option>
					{/each}
				{/if}
			</select>
			
			{#if !loading && tags.length > 0 && !selectedTag && !loading} 
				<h1 class="text-2xl font-bold text-error-500 ml-4">Select a tag</h1>
			{/if}
		</div>

		<!-- Right side: Global Time Filter -->
		<div class="flex gap-2">
			{#each timePeriods as period}
				<button
					class={`btn text-xs ${selectedPeriod.label === period.label ? 'preset-filled-primary-500' : 'preset-tonal-primary'}`}
					onclick={() => setPeriod(period)}
				>
					{period.label}
				</button>
			{/each}
		</div>
	</div>

	<!-- Main Grid Layout -->
	<div class="grid grid-cols-1 lg:grid-cols-2 2xl:grid-cols-3 gap-6">
		<!-- Left Column: About/Settings and Quick Actions -->
		<div class="flex flex-col gap-6">
			<!-- About & Settings -->
			<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6 h-[400px]">
				<div class="h-full flex flex-col">
					<Tabs value={activeTab} onValueChange={async (e) => { activeTab = e.value; await tick();}}>
						{#snippet list()}
							<Tabs.Control value="about" disabled={Boolean(loading && !selectedTag) || Boolean(selectedTag && loading && !selectedTag?.data?.description)}>
								{#snippet lead()}<Orbit size={20} />{/snippet}
								{#if selectedTag}#{selectedTag.data.tag_handle}{:else}About{/if}
							</Tabs.Control>
							<Tabs.Control value="settings" disabled={Boolean(loading && !selectedTag) || Boolean(selectedTag && loading && !selectedTag?.data)}>
								{#snippet lead()}<SlidersHorizontal size={20} />{/snippet}
								Settings
							</Tabs.Control>
						{/snippet}
						{#snippet content()}
							<div class="h-[288px] overflow-y-auto">
								<Tabs.Panel value="about">
									{#if (loading && !selectedTag) || (selectedTag && Boolean(selectedTag) && loading && !selectedTag?.data?.description) }
										<div class="placeholder animate-pulse w-full h-24 rounded"></div>
									{:else if selectedTag?.data?.description}
										<p class="whitespace-pre-line opacity-80">{selectedTag.data.description}</p>
									{:else if selectedTag && !selectedTag?.data?.description && selectedTag && !selectedTag?.key}
										<p class="opacity-50 text-sm">No description available for this tag.</p>
									{:else if !loading && tags.length > 0 && !selectedTag}
										<p class="text-center opacity-70">Select a tag to see its details.</p>
									{:else if !loading && tags.length === 0}
										<p class="text-center opacity-70">No tags found to display.</p>
									{/if}
								</Tabs.Panel>
								<Tabs.Panel value="settings">
									{#if (loading && !selectedTag) || (selectedTag && Boolean(selectedTag) && loading && !selectedTag?.data)}
										<div class="placeholder animate-pulse w-1/2 h-8 rounded mb-4"></div>
										<div class="grid grid-cols-2 gap-4">
											<div class="p-3 bg-surface-200-800 rounded placeholder animate-pulse h-16"></div>
											<div class="p-3 bg-surface-200-800 rounded placeholder animate-pulse h-16"></div>
											<div class="p-3 bg-surface-200-800 rounded placeholder animate-pulse h-16"></div>
										</div>
									{:else if selectedTag?.data}
										<div class="flex justify-between items-center mb-0">
											{#if $authUserDoc?.data.user_ulid === selectedTag?.data?.owner_ulid && selectedTag && !selectedTag?.key}
												<button class="btn preset-tonal-primary" onclick={() => goto(`/tag/edit/${selectedTag?.key}`)}>
													Edit Settings
												</button>
											{/if}
										</div>
										<div class="grid grid-cols-2 gap-4">
											<div class="p-3 bg-surface-200-800 rounded">
												<span class="text-sm opacity-70">Reputation Threshold</span>
												<p class="font-mono text-lg">{selectedTag?.data?.reputation_threshold ?? 'N/A'}</p>
											</div>
											<div class="p-3 bg-surface-200-800 rounded">
												<span class="text-sm opacity-70">Vote Reward</span>
												<p class="font-mono text-lg">{selectedTag?.data?.vote_reward ?? 'N/A'}</p>
											</div>
											<div class="p-3 bg-surface-200-800 rounded">
												<span class="text-sm opacity-70">Min Users</span>
												<p class="font-mono text-lg">{selectedTag?.data?.min_users_for_threshold ?? 'N/A'}</p>
											</div>
										</div>
										<hr class="my-4 border-surface-300-700" />
										<div>
											<h4 class="text-md font-semibold mb-2">Decay Rules</h4>
											{#if selectedTag?.data?.time_periods?.length > 0}
												<div class="space-y-2">
													{#each selectedTag.data.time_periods as period}
														<div class="p-2 bg-surface-200-800 rounded text-xs">
															After <span class="font-semibold">{period.months} months</span>:
															{#if period.multiplier < 1}
																Reputation decays by <span class="font-semibold text-error-500">{((1 - period.multiplier) * 100).toFixed(1)}%</span>
															{:else if period.multiplier === 1}
																<span class="font-semibold text-surface-500">No change</span> to reputation
															{:else}
																Reputation increases by <span class="font-semibold text-success-500">{((period.multiplier - 1) * 100).toFixed(1)}%</span>
															{/if}
														</div>
													{/each}
												</div>
											{:else if selectedTag && !selectedTag?.key}
												<p class="text-sm opacity-70">Preview decay rules description.</p>
											{:else}
												<p class="text-sm opacity-70">No decay rules specified for this tag. Reputation scores will remain constant over time.</p>
											{/if}
										</div>
									{:else if !loading && tags.length > 0 && !selectedTag}
										<p class="text-center opacity-70">Select a tag to see its settings.</p>
									{:else if !loading && tags.length === 0}
										<p class="text-center opacity-70">No tags found to display settings for.</p>
									{/if}
								</Tabs.Panel>
							</div>
						{/snippet}
					</Tabs>
				</div>
			</div>

			<!-- Quick Actions -->
			<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
				<QuickActionsTags selectedTag={selectedTag} />
			</div>
		</div>

		<!-- User Activity -->
		<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6 h-[400px] flex flex-col">
			<div class="flex justify-between items-center mb-4">
				<h2 class="text-lg font-bold {((!selectedTag && !loading && !loading && tags.length > 0 && !selectedTag) || (loading && !selectedTag) || !$authUserDoc) ? 'opacity-50' : ''}">Your Reputation in {selectedTag?.data.tag_handle || '...'}</h2>
				{#if $authUserDoc && selectedTag}
					<button type="button" class="chip-icon preset-tonal-surface" onclick={() => goto(`/tags/${selectedTag?.key}/reputation`)} disabled={!selectedTag || selectedTag?.key === PREVIEW_TAG_KEY || !$authUserDoc || loading || (selectedTag && Boolean(selectedTag) && loading && !userReputation)} title="View Full Details">
						<Expand size={16} />
					</button>
				{/if}
			</div>

			{#if !$authUserDoc}
				<p class="text-center opacity-60 py-10">Log in to see your activity and reputation for this tag.</p>
			{:else if (loading && !selectedTag) || (selectedTag && Boolean(selectedTag) && loading && !userReputation)}
				<div class="placeholder animate-pulse w-full h-40 rounded"></div>
			{:else if selectedTag && userReputation}
				<div class="grid grid-cols-2 gap-4 mb-4">
					<div class="p-3 bg-surface-200-800 rounded">
						<span class="text-sm opacity-70">Your Score</span>
						<p class="text-2xl font-bold">{userReputation.score}</p>
					</div>
					<div class="p-3 bg-surface-200-800 rounded">
						<span class="text-sm opacity-70">Rank</span>
						<p class="text-2xl font-bold">#{userReputation.rank}</p>
					</div>
				</div>
				<div class="flex-1 flex flex-col min-h-0">
					<div class="flex justify-start border-b-[1px] border-surface-200-800 mb-0 gap-2">
						<button type="button" class="chip text-xs {userActivityFilter === 'all' ? 'preset-filled-primary-500' : 'preset-tonal-surface'}" onclick={() => userActivityFilter = 'all'}>All</button>
						<button type="button" class="chip text-xs {userActivityFilter === 'in' ? 'preset-filled-secondary-500' : 'preset-tonal-surface'}" onclick={() => userActivityFilter = 'in'}>In</button>
						<button type="button" class="chip text-xs {userActivityFilter === 'out' ? 'preset-filled-tertiary-500' : 'preset-tonal-surface'}" onclick={() => userActivityFilter = 'out'}>Out</button>
						<button type="button" class="chip text-xs {userActivityFilter === 'positive' ? 'preset-filled-success-500' : 'preset-tonal-surface'}" onclick={() => userActivityFilter = 'positive'}>+</button>
						<button type="button" class="chip text-xs {userActivityFilter === 'negative' ? 'preset-filled-error-500' : 'preset-tonal-surface'}" onclick={() => userActivityFilter = 'negative'}>-</button>
					</div>
					<div class="flex-1 overflow-y-auto bg-surface-200-800 rounded p-2 space-y-1">
						{#if userRecentActivity.length > 0}
							{#each userRecentActivity.filter(activity => { 
								if (userActivityFilter === 'all') return true; 
								if (userActivityFilter === 'in') return activity.type === 'received'; 
								if (userActivityFilter === 'out') return activity.type === 'cast'; 
								if (userActivityFilter === 'positive') return activity.value > 0; 
								if (userActivityFilter === 'negative') return activity.value < 0; 
								return true; 
							}) as activity (activity.id || (activity.date + (activity.target || activity.peerName)))} 
								<div class="text-xs p-1 rounded {activity.value > 0 ? 'bg-success-500/10' : 'bg-error-500/10'}">
									{#if activity.type === 'received'}
										Received <span class="font-semibold">{activity.value > 0 ? `+${activity.value}` : activity.value}</span> vote from <strong>{activity.peerName}</strong>
									{:else if activity.type === 'cast'}
										Cast <span class="font-semibold">{activity.value > 0 ? `+${activity.value}` : activity.value}</span> vote to <strong>{activity.peerName}</strong>
									{:else}
										Vote: <span class="font-semibold">{activity.value > 0 ? `+${activity.value}` : activity.value}</span> regarding <strong>{activity.target || activity.peerName}</strong>
									{/if}
									({new Date(activity.date).toLocaleDateString()})
								</div>
							{:else}
								<p class="text-center text-xs opacity-50 py-2">No activities match the filter.</p>
							{/each}
						{:else}
							<p class="text-center text-xs opacity-50 py-2">No recent activity for this tag.</p>
						{/if}
					</div>
				</div>
			{:else if !loading && tags.length > 0 && !selectedTag}
				<p class="text-center opacity-70 py-10">Select a tag to see your activity.</p>
			{:else if !loading && tags.length === 0}
				<p class="text-center opacity-70 py-10">No tags available to show activity for.</p>
			{/if}
		</div>

		<!-- Graph Preview -->
		<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6 2xl:col-span-1 lg:col-span-2">
			<div class="flex justify-between items-center mb-4">
				<h2 class="text-lg font-bold {((!selectedTag && !loading && !loading && tags.length > 0 && !selectedTag) || (loading && !selectedTag)) ? 'opacity-50' : ''}">Graph Overview</h2>
				<button type="button" class="chip-icon preset-tonal-surface" onclick={() => goto(`/tags/${selectedTag?.key}/graph`)} disabled={!selectedTag || selectedTag?.key === PREVIEW_TAG_KEY || loading || (selectedTag && Boolean(selectedTag) && loading && !selectedTag)} title="View Full Graph">
					<Expand size={16} />
				</button>
			</div>
			<div class="w-full h-64 bg-surface-200-800 rounded flex items-center justify-center">
				{#if (loading && !selectedTag) || (loading && selectedTag && selectedTag.key && selectedTag.key !== PREVIEW_TAG_KEY && !selectedTag)}
					<div class="placeholder animate-pulse w-3/4 h-8 rounded"></div>
				{:else if selectedTag && selectedTag.key === PREVIEW_TAG_KEY}
					<span class="opacity-50">Graph visualization for Preview Mode.</span>
				{:else if selectedTag}
					<span class="opacity-50">Graph visualization coming soon…</span>
				{:else if !loading && tags.length > 0 && !selectedTag}
					<span class="opacity-50">Select a tag to see graph overview.</span>
				{:else if !loading && tags.length === 0}
					<span class="opacity-50">No tags available for graph.</span>
				{/if}
			</div>
		</div>
	</div>

	<!-- Stats Overview -->
	<div class="grid grid-cols-1 lg:grid-cols-3 gap-6 mt-6">
		{#each ['Total Users', 'Verified Users', 'Active Users'] as statItem, i}
			<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6" 
				 class:opacity-50={ (loading && !selectedTag) || (loading && selectedTag && !selectedTag) } >
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
	<div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mt-6">
		<!-- Recent Votes (Old Implementation) -->
		<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
			<div class="flex justify-between items-center mb-4">
				<h2 class="text-lg font-bold {((!selectedTag && !loading && !loading && tags.length > 0 && !selectedTag) || (loading && !selectedTag)) ? 'opacity-50' : ''}">Recent Votes (Old)</h2>
				<button type="button" class="chip-icon preset-tonal-surface" onclick={() => goto(`/tags/${selectedTag?.key}/votes`)} disabled={!selectedTag || selectedTag?.key === PREVIEW_TAG_KEY || loading || (selectedTag && Boolean(selectedTag) && loading && recentVotes.length === 0 && !selectedTag && tags.length > 0)} title="See More Votes">
					<Expand size={16} />
				</button>
			</div>
			{#if (loading && !selectedTag) || (loading && selectedTag && selectedTag.key && selectedTag.key !== PREVIEW_TAG_KEY && recentVotes.length === 0 && !selectedTag && tags.length > 0) }
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
			{:else if !loading && tags.length > 0 && !selectedTag}
				<p class="text-center opacity-70">Select a tag to see recent votes.</p>
			{:else if !loading && tags.length === 0}
				<p class="text-center opacity-70">No tags available.</p>
			{/if}
		</div>

		<!-- Top Users -->
		<div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
			<div class="flex justify-between items-center mb-4">
				<h2 class="text-lg font-bold {((!selectedTag && !loading && !loading && tags.length > 0 && !selectedTag) || (loading && !selectedTag)) ? 'opacity-50' : ''}">Top Users</h2>
				<button type="button" class="chip-icon preset-tonal-surface" onclick={() => goto(`/tags/${selectedTag?.key}/users`)} disabled={!selectedTag || selectedTag?.key === PREVIEW_TAG_KEY || loading || (selectedTag && Boolean(selectedTag) && loading && topUsers.length === 0 && !selectedTag && tags.length > 0)} title="See More Users">
					<Expand size={16} />
				</button>
			</div>
			{#if (loading && !selectedTag) || (loading && selectedTag && selectedTag.key && selectedTag.key !== PREVIEW_TAG_KEY && topUsers.length === 0 && !selectedTag && tags.length > 0) }
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
									<td class="text-right"><span class="badge preset-filled-secondary-500">{user.score} points</span></td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			{:else if selectedTag && topUsers.length === 0}
				<p class="text-center opacity-70">No top users to display for this tag.</p>
			{:else if !loading && tags.length > 0 && !selectedTag}
				<p class="text-center opacity-70">Select a tag to see top users.</p>
			{:else if !loading && tags.length === 0}
				<p class="text-center opacity-70">No tags available.</p>
			{/if}
		</div>
	</div>

	<!-- New Implementation Section -->
	<div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mt-6">
		<!-- Recent Votes (New Implementation) -->
		<RecentVotes 
			selectedTag={selectedTag} 
			cutoffTimestamp={cutoffTimestamp} 
		/>
	</div>

	<!-- Call to Action -->
	<div class="mt-6">
		{#if (loading && !selectedTag) || (loading && selectedTag && selectedTag.key && selectedTag.key !== PREVIEW_TAG_KEY && !selectedTag)}
			<div class="placeholder animate-pulse w-full h-12 rounded"></div>
		{:else if selectedTag && selectedTag.key === PREVIEW_TAG_KEY}
			<button class="btn preset-filled-primary-500 w-full" disabled>
				Currently in Preview Mode
			</button>
		{:else if selectedTag}
			<button class="btn preset-filled-primary-500 w-full">
				{userReputation && userReputation.score > 0 ? 'Contribute' : 'Join Community'}
			</button>
		{:else if !loading && tags.length > 0 && !selectedTag}
			<button class="btn preset-filled-primary-500 w-full" disabled>
				Select a Tag
			</button>
		{:else if !loading && tags.length === 0}
			<button class="btn preset-filled-primary-500 w-full" disabled>
				No Tags Available
			</button>
		{/if}
	</div>
</div>