<script lang="ts">
import { Avatar } from '@skeletonlabs/skeleton-svelte';
import { UserRoundPen, Expand, BookOpen, SlidersHorizontal, Orbit, Star, Trophy, MessageSquare, ThumbsUp, ThumbsDown, Construction } from 'lucide-svelte';
import { toaster } from '$lib/skeletonui/toaster-skeleton';
import type { UserDocument } from '$lib/types';
import BaseCard from '$lib/components/common/BaseCard.svelte';

// Props
const { user, loading } = $props<{
  user: UserDocument | null;
  loading: boolean;
}>();

// State
let error = $state<string | null>(null);

// Helper function for construction warning
function showConstructionWarning() {
  toaster.warning({
    title: 'Under Construction',
    description: 'This feature is not yet implemented with real data.'
  });
}

// Dummy data for demo
const dummyUser: UserDocument = {
  data: {
    user_handle: 'demo_user',
    display_name: 'Demo User',
    user_ulid: 'demo_ulid',
    avatar_url: 'https://api.dicebear.com/7.x/avataaars/svg?seed=demo'
  }
} as UserDocument;

const dummyReputations = [
  { tag: 'ICP', score: 850, rank: 5, isTrusted: true, progress: 100 },
  { tag: 'Rust', score: 720, rank: 12, isTrusted: true, progress: 100 },
  { tag: 'Svelte', score: 450, rank: 25, isTrusted: false, progress: 75 },
  { tag: 'TypeScript', score: 380, rank: 30, isTrusted: false, progress: 60 }
];

const dummyStats = {
  totalVotesGiven: 156,
  totalVotesReceived: 89,
  trustedCommunities: 2,
  activeCommunities: 4,
  averageScore: 600
};

// Use dummy data if no real user data
const displayUser = user || dummyUser;
</script>

<div class="p-4">
  {#if error}
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
              src={displayUser.data.avatar_url} 
              size="xl" 
              name={displayUser.data.display_name} 
            />
          </div>
          <h1 class="text-2xl font-bold">{displayUser.data.display_name}</h1>
          <div class="opacity-60 mb-4">@{displayUser.data.user_handle}</div>

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
          <button class="chip-icon preset-tonal-surface" title="View All" on:click={showConstructionWarning}>
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

      <!-- Active Reputations -->
      <BaseCard underConstruction={false}>
        <div class="flex justify-between items-center mb-4">
          <h2 class="text-lg font-bold">Active Reputations</h2>
          <button class="chip-icon preset-tonal-surface" title="View All" on:click={showConstructionWarning}>
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
      </BaseCard>
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
    </div>
  </div>
</div> 