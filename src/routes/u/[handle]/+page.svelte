<script lang="ts">
import { onMount } from 'svelte';
import { page } from '$app/stores';
import { authUserDoc } from '$lib/stores/authUserDoc';
import { Avatar } from '@skeletonlabs/skeleton-svelte';
import { UserRoundPen, Expand, BookOpen, SlidersHorizontal, Orbit, Star, Trophy, MessageSquare, ThumbsUp, ThumbsDown } from 'lucide-svelte';
import NotLoggedInAlert from '$lib/components/common/NotLoggedInAlert.svelte';
import { Tabs } from '@skeletonlabs/skeleton-svelte';
import { toaster } from '$lib/skeletonui/toaster-skeleton';
import { goto } from '$app/navigation';
import type { UserDocument } from '$lib/types';

// --- Dummy Data ---
const dummyUser: UserDocument = {
  key: 'user_123',
  data: {
    user_handle: 'johndoe',
    display_name: 'John Doe',
    user_ulid: 'user_123',
    avatar_url: 'https://i.pravatar.cc/100?img=3'
  }
};

const dummyReputations = [
  { tag: 'ICP', score: 850, rank: 5, isTrusted: true, progress: 100 },
  { tag: 'Rust', score: 720, rank: 12, isTrusted: true, progress: 100 },
  { tag: 'Svelte', score: 450, rank: 25, isTrusted: false, progress: 75 },
  { tag: 'TypeScript', score: 380, rank: 30, isTrusted: false, progress: 60 }
];

const dummyRecentActivity = [
  { type: 'vote', target: 'alice', value: 1, tag: 'ICP', date: '2h ago', message: 'Great contribution to the community!' },
  { type: 'vote', target: 'bob', value: -1, tag: 'Rust', date: '5h ago', message: 'Incorrect information provided' },
  { type: 'received', from: 'carol', value: 1, tag: 'Svelte', date: '1d ago', message: 'Helpful explanation' }
];

const dummyStats = {
  totalVotesGiven: 156,
  totalVotesReceived: 89,
  trustedCommunities: 2,
  activeCommunities: 4,
  averageScore: 600
};

// --- State Management ---
let user = $state<UserDocument | null>(null);
let loading = $state(true);
let error = $state<string | null>(null);
let activeTab = $state('about');
let isOwnProfile = $state(false);

// --- Initialization ---
onMount(async () => {
  try {
    // TODO: Fetch user data based on handle
    user = dummyUser;
    isOwnProfile = $authUserDoc?.data.user_handle === $page.params.handle;
  } catch (e) {
    error = e instanceof Error ? e.message : 'Failed to load profile';
    toaster.error({ title: error });
  } finally {
    loading = false;
  }
});
</script>

<NotLoggedInAlert />

<!-- Main Container -->
<div class="p-4">
  {#if error && !loading}
    <div class="alert alert-error mb-6">{error}</div>
  {/if}

  <!-- Main Grid Layout -->
  <div class="grid grid-cols-1 lg:grid-cols-2 2xl:grid-cols-3 gap-6">
    <!-- Left Column: Profile Card -->
    <div class="space-y-6">
      <!-- Profile Card -->
      <div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
        <div class="flex flex-col items-center">
          <div class="mb-4">
            <Avatar 
              src={user?.data.avatar_url} 
              size="xl" 
              name={user?.data.display_name ?? 'User'} 
            />
          </div>
          <h1 class="text-2xl font-bold">{user?.data.display_name}</h1>
          <div class="opacity-60 mb-4">@{user?.data.user_handle}</div>
          
          {#if isOwnProfile}
            <button class="btn preset-tonal-primary" onclick={() => goto('/settings/profile')}>
              <UserRoundPen size={16} class="mr-2" />
              Edit Profile
            </button>
          {/if}

          <!-- Main Reputation Score -->
          <div class="mt-6 p-4 bg-surface-200-800 rounded-lg w-full">
            <div class="text-sm opacity-70">Main Reputation Score</div>
            <div class="text-3xl font-bold text-primary-500">{dummyStats.averageScore}</div>
            <div class="text-xs opacity-60">Aggregate of all reputations</div>
          </div>

          <!-- Quick Stats -->
          <div class="grid grid-cols-2 gap-4 w-full mt-4">
            <div class="p-3 bg-surface-200-800 rounded">
              <div class="text-sm opacity-70">Votes Given</div>
              <div class="text-xl font-bold">{dummyStats.totalVotesGiven}</div>
            </div>
            <div class="p-3 bg-surface-200-800 rounded">
              <div class="text-sm opacity-70">Votes Received</div>
              <div class="text-xl font-bold">{dummyStats.totalVotesReceived}</div>
            </div>
          </div>
        </div>
      </div>

      <!-- Trusted Communities -->
      <div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
        <div class="flex justify-between items-center mb-4">
          <h2 class="text-lg font-bold">Trusted User Status</h2>
          <button class="chip-icon preset-tonal-surface" title="View All">
            <Expand size={16} />
          </button>
        </div>
        <div class="space-y-2">
          {#each dummyReputations.filter(r => r.isTrusted) as rep}
            <div class="p-3 bg-surface-200-800 rounded flex justify-between items-center">
              <div>
                <div class="font-bold">#{rep.tag}</div>
                <div class="text-sm opacity-70">Rank #{rep.rank}</div>
              </div>
              <div class="text-right">
                <div class="text-lg font-bold text-primary-500">{rep.score}</div>
                <div class="badge preset-filled-success-500">Trusted</div>
              </div>
            </div>
          {/each}
        </div>
      </div>
    </div>

    <!-- Middle Column: Active Reputations -->
    <div class="space-y-6">
      <!-- Reputation Overview -->
      <div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
        <h2 class="text-lg font-bold mb-4">Reputation Overview</h2>
        <div class="grid grid-cols-2 gap-4">
          <div class="p-3 bg-surface-200-800 rounded">
            <div class="text-sm opacity-70">Trusted In</div>
            <div class="text-xl font-bold">{dummyStats.trustedCommunities}</div>
            <div class="text-xs opacity-60">out of {dummyStats.activeCommunities}</div>
          </div>
          <div class="p-3 bg-surface-200-800 rounded">
            <div class="text-sm opacity-70">Active In</div>
            <div class="text-xl font-bold">{dummyStats.activeCommunities}</div>
            <div class="text-xs opacity-60">communities</div>
          </div>
        </div>
      </div>

      <!-- Recent Activity -->
      <div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
        <div class="flex justify-between items-center mb-4">
          <h2 class="text-lg font-bold">Recent Activity</h2>
          <button class="chip-icon preset-tonal-surface" title="View All">
            <Expand size={16} />
          </button>
        </div>
        <div class="table-wrap">
          {#if loading}
            <!-- Loading State: Skeleton UI -->
            <div class="space-y-2">
              {#each Array(3) as _}
                <div class="flex justify-between items-center placeholder animate-pulse h-8 rounded"></div>
              {/each}
            </div>
          {:else if error}
            <!-- Error State: Error Message Display -->
            <p class="text-center text-error-500">{error}</p>
          {:else}
            <!-- Success State: Activity Table -->
            <table class="table caption-bottom">
              <thead>
                <tr>
                  <th>From</th>
                  <th>To</th>
                  <th class="text-right">Value</th>
                </tr>
              </thead>
              <tbody class="[&>tr]:hover:preset-tonal-primary">
                {#each dummyRecentActivity as activity}
                  <tr>
                    <td>
                      <div class="flex items-center gap-2">
                        <Avatar 
                          name={activity.type === 'vote' ? 'You' : (activity.from ?? 'Unknown')}
                          src={activity.type === 'vote' ? (user?.data.avatar_url ?? '') : 'https://i.pravatar.cc/100?img=1'}
                          size="w-6"
                          rounded="rounded-full"
                          background="bg-transparent"
                        >
                          {activity.type === 'vote' ? 'Y' : (activity.from?.[0]?.toUpperCase() ?? '?')}
                        </Avatar>
                        <span>{activity.type === 'vote' ? 'You' : (activity.from ?? 'Unknown')}</span>
                      </div>
                    </td>
                    <td>
                      <div class="flex items-center gap-2">
                        <Avatar 
                          name={activity.type === 'vote' ? (activity.target ?? 'Unknown') : 'You'}
                          src={activity.type === 'vote' ? 'https://i.pravatar.cc/100?img=2' : (user?.data.avatar_url ?? '')}
                          size="w-6"
                          rounded="rounded-full"
                          background="bg-transparent"
                        >
                          {activity.type === 'vote' ? (activity.target?.[0]?.toUpperCase() ?? '?') : 'Y'}
                        </Avatar>
                        <span>{activity.type === 'vote' ? (activity.target ?? 'Unknown') : 'You'}</span>
                      </div>
                    </td>
                    <td class="text-right">
                      <span class="badge preset-filled-{activity.value > 0 ? 'success' : 'error'}-500">
                        {activity.value > 0 ? '+1' : '-1'}
                      </span>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          {/if}
        </div>
      </div>

      <!-- Reviews (formerly Recent Activity) -->
      <div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
        <div class="flex justify-between items-center mb-4">
          <h2 class="text-lg font-bold">Reviews</h2>
          <button class="chip-icon preset-tonal-surface" title="View All">
            <Expand size={16} />
          </button>
        </div>
        <div class="space-y-2">
          {#each dummyRecentActivity as activity}
            <div class="p-3 bg-surface-200-800 rounded">
              <div class="flex justify-between items-start">
                <div>
                  <div class="font-bold">
                    {#if activity.type === 'vote'}
                      Voted on @{activity.target}
                    {:else}
                      Received vote from @{activity.from}
                    {/if}
                  </div>
                  <div class="text-sm opacity-70">{activity.message}</div>
                </div>
                <div class="text-right">
                  <div class="badge preset-filled-{activity.value > 0 ? 'success' : 'error'}-500">
                    {activity.value > 0 ? '+' : ''}{activity.value}
                  </div>
                  <div class="text-xs opacity-60 mt-1">#{activity.tag}</div>
                </div>
              </div>
              <div class="text-xs opacity-60 mt-2">{activity.date}</div>
            </div>
          {/each}
        </div>
      </div>
    </div>

    <!-- Right Column: Stats and Favorites -->
    <div class="space-y-6">
      <!-- Community Stats -->
      <div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
        <h2 class="text-lg font-bold mb-4">Community Stats</h2>
        <div class="space-y-2">
          <div class="p-3 bg-surface-200-800 rounded">
            <div class="flex justify-between items-center">
              <div class="text-sm opacity-70">Total Votes Given</div>
              <div class="font-bold">{dummyStats.totalVotesGiven}</div>
            </div>
          </div>
          <div class="p-3 bg-surface-200-800 rounded">
            <div class="flex justify-between items-center">
              <div class="text-sm opacity-70">Total Votes Received</div>
              <div class="font-bold">{dummyStats.totalVotesReceived}</div>
            </div>
          </div>
          <div class="p-3 bg-surface-200-800 rounded">
            <div class="flex justify-between items-center">
              <div class="text-sm opacity-70">Average Score</div>
              <div class="font-bold">{dummyStats.averageScore}</div>
            </div>
          </div>
        </div>
      </div>

      <!-- Active Reputations -->
      <div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
        <div class="flex justify-between items-center mb-4">
          <h2 class="text-lg font-bold">Active Reputations</h2>
          <button class="chip-icon preset-tonal-surface" title="View All">
            <Expand size={16} />
          </button>
        </div>
        <div class="space-y-2">
          {#each dummyReputations as rep}
            <div class="p-3 bg-surface-200-800 rounded">
              <div class="flex justify-between items-center mb-2">
                <div class="font-bold">#{rep.tag}</div>
                <div class="text-lg font-bold text-primary-500">{rep.score}</div>
              </div>
              <div class="flex justify-between items-center text-sm">
                <div class="opacity-70">Rank #{rep.rank}</div>
                <div class="opacity-70">Progress: {rep.progress}%</div>
              </div>
              <div class="mt-2 h-1 w-full bg-surface-300-700 rounded-full overflow-hidden">
                <div class="h-full bg-primary-500" style="width: {rep.progress}%"></div>
              </div>
            </div>
          {/each}
        </div>
      </div>

      <!-- Favorite Reputations (Placeholder) -->
      <div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
        <div class="flex justify-between items-center mb-4">
          <h2 class="text-lg font-bold">Favorite Reputations</h2>
          <button class="chip-icon preset-tonal-surface" title="Coming Soon">
            <Star size={16} />
          </button>
        </div>
        <div class="text-center opacity-60 py-4">
          Coming soon: Mark your favorite reputations
        </div>
      </div>
    </div>
  </div>
</div> 