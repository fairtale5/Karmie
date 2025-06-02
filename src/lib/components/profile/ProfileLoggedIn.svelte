<script lang="ts">
import { onMount } from 'svelte';
import { page } from '$app/stores';
import { authUserDoc } from '$lib/stores/authUserDoc';
import { Avatar } from '@skeletonlabs/skeleton-svelte';
import { UserRoundPen, Expand, BookOpen, SlidersHorizontal, Orbit, Star, Trophy, MessageSquare, ThumbsUp, ThumbsDown, Construction } from 'lucide-svelte';
import { Tabs } from '@skeletonlabs/skeleton-svelte';
import { toaster } from '$lib/skeletonui/toaster-skeleton';
import type { UserDocument } from '$lib/types';

// Props
const { user, loading } = $props<{
  user: UserDocument | null;
  loading: boolean;
}>();

// State
let error = $state<string | null>(null);
let activeTab = $state('about');
let isOwnProfile = $state(false);

// Helper function for construction warning
function showConstructionWarning() {
  toaster.warning({
    title: 'Under Construction',
    description: 'This feature is not yet implemented with real data.'
  });
}

// Initialize
onMount(() => {
  try {
    isOwnProfile = $authUserDoc?.data.user_handle === user?.data.user_handle;
  } catch (e) {
    error = e instanceof Error ? e.message : 'Failed to check profile ownership';
    toaster.error({ title: error });
  }
});
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
              src={user?.data.avatar_url} 
              size="xl" 
              name={user?.data.display_name ?? 'User'} 
            />
          </div>
          <h1 class="text-2xl font-bold">{user?.data.display_name}</h1>
          <div class="opacity-60 mb-4">@{user?.data.user_handle}</div>

          <!-- Main Reputation Score -->
          <div class="mt-6 p-4 bg-surface-200-800 rounded-lg w-full">
            <div class="text-sm opacity-70">Main Reputation Score</div>
            <div class="text-3xl font-bold text-primary-500">850</div>
            <div class="text-xs opacity-60">Aggregate of all reputations</div>
          </div>

          <!-- Quick Stats -->
          <div class="grid grid-cols-2 gap-4 w-full mt-4">
            <div class="p-3 bg-surface-200-800 rounded">
              <div class="text-sm opacity-70">Votes Given</div>
              <div class="text-xl font-bold">156</div>
            </div>
            <div class="p-3 bg-surface-200-800 rounded">
              <div class="text-sm opacity-70">Votes Received</div>
              <div class="text-xl font-bold">89</div>
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
          <div class="p-3 bg-surface-200-800 rounded flex justify-between items-center">
            <div>
              <div class="font-bold">#ICP</div>
              <div class="text-sm opacity-70">Rank #5</div>
            </div>
            <div class="text-right">
              <div class="text-lg font-bold text-primary-500">850</div>
              <div class="badge preset-filled-success-500">Trusted</div>
            </div>
          </div>
          <div class="p-3 bg-surface-200-800 rounded flex justify-between items-center">
            <div>
              <div class="font-bold">#Rust</div>
              <div class="text-sm opacity-70">Rank #12</div>
            </div>
            <div class="text-right">
              <div class="text-lg font-bold text-primary-500">720</div>
              <div class="badge preset-filled-success-500">Trusted</div>
            </div>
          </div>
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
            <div class="text-xl font-bold">2</div>
            <div class="text-xs opacity-60">out of 4</div>
          </div>
          <div class="p-3 bg-surface-200-800 rounded">
            <div class="text-sm opacity-70">Active In</div>
            <div class="text-xl font-bold">4</div>
            <div class="text-xs opacity-60">communities</div>
          </div>
        </div>
      </div>

      <!-- Active Reputations -->
      <div class="card shadow bg-surface-100-900 border border-surface-200-800 p-6">
        <div class="flex justify-between items-center mb-4">
          <h2 class="text-lg font-bold">Active Reputations</h2>
          <button class="chip-icon preset-tonal-surface" title="View All" on:click={showConstructionWarning}>
            <Expand size={16} />
          </button>
        </div>
        <div class="space-y-2">
          <div class="p-3 bg-surface-200-800 rounded">
            <div class="flex justify-between items-center mb-2">
              <div class="font-bold">#ICP</div>
              <div class="text-lg font-bold text-primary-500">850</div>
            </div>
            <div class="flex justify-between items-center text-sm">
              <div class="opacity-70">Rank #5</div>
              <div class="opacity-70">Progress: 100%</div>
            </div>
            <div class="mt-2 h-1 w-full bg-surface-300-700 rounded-full overflow-hidden">
              <div class="h-full bg-primary-500" style="width: 100%"></div>
            </div>
          </div>
          <div class="p-3 bg-surface-200-800 rounded">
            <div class="flex justify-between items-center mb-2">
              <div class="font-bold">#Rust</div>
              <div class="text-lg font-bold text-primary-500">720</div>
            </div>
            <div class="flex justify-between items-center text-sm">
              <div class="opacity-70">Rank #12</div>
              <div class="opacity-70">Progress: 100%</div>
            </div>
            <div class="mt-2 h-1 w-full bg-surface-300-700 rounded-full overflow-hidden">
              <div class="h-full bg-primary-500" style="width: 100%"></div>
            </div>
          </div>
          <div class="p-3 bg-surface-200-800 rounded">
            <div class="flex justify-between items-center mb-2">
              <div class="font-bold">#Svelte</div>
              <div class="text-lg font-bold text-primary-500">450</div>
            </div>
            <div class="flex justify-between items-center text-sm">
              <div class="opacity-70">Rank #25</div>
              <div class="opacity-70">Progress: 75%</div>
            </div>
            <div class="mt-2 h-1 w-full bg-surface-300-700 rounded-full overflow-hidden">
              <div class="h-full bg-primary-500" style="width: 75%"></div>
            </div>
          </div>
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
              <div class="font-bold">156</div>
            </div>
          </div>
          <div class="p-3 bg-surface-200-800 rounded">
            <div class="flex justify-between items-center">
              <div class="text-sm opacity-70">Total Votes Received</div>
              <div class="font-bold">89</div>
            </div>
          </div>
          <div class="p-3 bg-surface-200-800 rounded">
            <div class="flex justify-between items-center">
              <div class="text-sm opacity-70">Average Score</div>
              <div class="font-bold">600</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div> 