<script lang="ts">
import { page } from '$app/stores';
import type { UserDocument } from '$lib/types';
import ProfileHeader from '$lib/components/profile/ProfileHeader.svelte';
import TrustedCommunities from '$lib/components/profile/TrustedCommunities.svelte';
import ReputationOverview from '$lib/components/profile/ReputationOverview.svelte';
import ActiveReputations from '$lib/components/profile/ActiveReputations.svelte';
import RecentActivity from '$lib/components/profile/RecentActivity.svelte';
import { onMount } from 'svelte';
import { initJuno } from '$lib/juno';
import { toaster } from '$lib/skeletonui/toaster-skeleton';
import type { PageData } from './$types';

// Get handle from URL
const handle = $page.params.handle;
let { data } = $props<{ data: PageData }>();

let loading = $state(true);
let error = $state<string | null>(null);
let userData = $state<any>(null);

onMount(async () => {
    try {
        await initJuno();
        
        // If we have a fetchUserData function, use it
        if (data.fetchUserData) {
            userData = await data.fetchUserData();
        } else {
            // Otherwise use the data directly (demo or current user case)
            userData = data;
        }
    } catch (e) {
        error = e instanceof Error ? e.message : 'Failed to load user data';
        toaster.error({ title: error });
    } finally {
        loading = false;
    }
});
</script>

<div class="p-4">
  {#if error}
    <div class="alert alert-error mb-6">{error}</div>
  {/if}

  {#if loading}
    <div class="placeholder animate-pulse">
      <div class="h-32 bg-surface-200-800 rounded mb-6"></div>
      <div class="grid grid-cols-1 lg:grid-cols-2 2xl:grid-cols-3 gap-6">
        <div class="h-64 bg-surface-200-800 rounded"></div>
        <div class="h-64 bg-surface-200-800 rounded"></div>
        <div class="h-64 bg-surface-200-800 rounded"></div>
      </div>
    </div>
  {:else if userData}
    <!-- Main Grid Layout -->
    <div class="grid grid-cols-1 lg:grid-cols-2 2xl:grid-cols-3 gap-6">
      <!-- Left Column -->
      <div class="space-y-6">
        <ProfileHeader user={userData.user} stats={userData.stats} />
        <TrustedCommunities communities={userData.trustedCommunities} />
      </div>

      <!-- Middle Column -->
      <div class="space-y-6">
        <ReputationOverview stats={userData.reputationStats} />
        <ActiveReputations reputations={userData.activeReputations} />
      </div>

      <!-- Right Column -->
      <div class="space-y-6">
        <RecentActivity activities={userData.recentActivity} />
      </div>
    </div>
  {/if}
</div> 