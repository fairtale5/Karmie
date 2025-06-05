<script lang="ts">
import { Avatar } from '@skeletonlabs/skeleton-svelte';
import type { UserDocument } from '$lib/types';
import BaseCard from '$lib/components/common/BaseCard.svelte';
import { dummyProfileData } from '$lib/data/dummyProfileData';

interface CommunityStats {
  totalVotesGiven: number;
  totalVotesReceived: number;
  trustedCommunities: number;
  activeCommunities: number;
  averageScore: number;
}

export let user: UserDocument;

// For now, use dummy stats. Later this should fetch real community stats based on user
let stats: CommunityStats = dummyProfileData.communityStats;
</script>

<BaseCard>
  <div class="flex flex-col items-center">
    <div class="mb-4 w-24 h-24">
      <Avatar 
        src={user.data.avatar_url} 
        size="xl" 
        name={user.data.display_name}
        classes="w-full h-full object-cover"
      />
    </div>
    <h1 class="text-2xl font-bold">{user.data.display_name}</h1>
    <div class="opacity-60 mb-4">@{user.data.user_handle}</div>

    <!-- Main Reputation Score -->
    <div class="mt-6 p-4 bg-surface-200-800 rounded-lg w-full">
      <div class="text-sm opacity-70">Main Reputation Score</div>
      <div class="text-3xl font-bold text-primary-500">{stats.averageScore}</div>
      <div class="text-xs opacity-60">Aggregate of all reputations</div>
    </div>

    <!-- Community Stats -->
    <div class="grid grid-cols-2 gap-4 w-full mt-4">
      <div class="p-3 bg-surface-200-800 rounded">
        <div class="text-sm opacity-70">Votes Given</div>
        <div class="text-xl font-bold text-primary-500">{stats.totalVotesGiven}</div>
      </div>
      <div class="p-3 bg-surface-200-800 rounded">
        <div class="text-sm opacity-70">Votes Received</div>
        <div class="text-xl font-bold text-primary-500">{stats.totalVotesReceived}</div>
      </div>
      <div class="p-3 bg-surface-200-800 rounded">
        <div class="text-sm opacity-70">Trusted Communities</div>
        <div class="text-xl font-bold text-primary-500">{stats.trustedCommunities}</div>
      </div>
      <div class="p-3 bg-surface-200-800 rounded">
        <div class="text-sm opacity-70">Active Communities</div>
        <div class="text-xl font-bold text-primary-500">{stats.activeCommunities}</div>
      </div>
    </div>
  </div>
</BaseCard> 