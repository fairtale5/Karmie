<script lang="ts">
import { page } from '$app/stores';
import type { UserDocument } from '$lib/types';
import ProfileHeader from '$lib/components/profile/ProfileHeader.svelte';
import TrustedCommunities from '$lib/components/profile/TrustedCommunities.svelte';
import ReputationOverview from '$lib/components/profile/ReputationOverview.svelte';
import ActiveReputations from '$lib/components/profile/ActiveReputations.svelte';
import RecentActivity from '$lib/components/profile/RecentActivity.svelte';
import { dummyProfileData } from '$lib/data/dummyProfileData';

// Get handle from URL
const handle = $page.params.handle;
</script>

<div class="p-4">
  <!-- Main Grid Layout -->
  <div class="grid grid-cols-1 lg:grid-cols-2 2xl:grid-cols-3 gap-6">
    <!-- Left Column -->
    <div class="space-y-6">
      <ProfileHeader user={dummyProfileData.user} stats={dummyProfileData.communityStats} />
      <TrustedCommunities communities={dummyProfileData.trustedCommunities} />
    </div>

    <!-- Middle Column -->
    <div class="space-y-6">
      <ReputationOverview stats={dummyProfileData.reputationStats} />
      <ActiveReputations reputations={dummyProfileData.activeReputations} />
    </div>

    <!-- Right Column -->
    <div class="space-y-6">
      <RecentActivity activities={dummyProfileData.recentActivity} />
    </div>
  </div>
</div> 