<script lang="ts">
import { onMount } from 'svelte';
import { page } from '$app/stores';
import { authUserDoc } from '$lib/stores/authUserDoc';
import { authUser } from '$lib/stores/authUser';
import { Avatar } from '@skeletonlabs/skeleton-svelte';
import { UserRoundPen, Expand, BookOpen, SlidersHorizontal, Orbit, Star, Trophy, MessageSquare, ThumbsUp, ThumbsDown } from 'lucide-svelte';
import NotLoggedInAlert from '$lib/components/common/NotLoggedInAlert.svelte';
import { Tabs } from '@skeletonlabs/skeleton-svelte';
import { toaster } from '$lib/skeletonui/toaster-skeleton';
import { goto } from '$app/navigation';
import type { UserDocument } from '$lib/types';
import ProfileLoggedIn from '$lib/components/profile/ProfileLoggedIn.svelte';
import ProfileLoggedOut from '$lib/components/profile/ProfileLoggedOut.svelte';
import { queryDocsByKey } from '$lib/docs-crud/query_by_key';
import { Construction } from 'lucide-svelte';

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
let isOwnProfile = $state(false);

// --- Initialization ---
onMount(async () => {
  try {
    const handle = $page.params.handle;
    
    // Check if user is logged in
    if ($authUserDoc) {
      // If viewing own profile, use authUserDoc
      if ($authUserDoc.data.user_handle === handle) {
        user = $authUserDoc;
        isOwnProfile = true;
      } else {
        // Query database for other user's profile
        // TODO: Implement database query
        error = 'User not found';
      }
    } else {
      // If not logged in and trying to view a real profile, redirect to demo
      if (handle !== 'demo_user') {
        goto('/u/demo_user');
        return;
      }
      // Use dummy data for demo user
      user = {
        data: {
          user_handle: 'demo_user',
          display_name: 'Demo User',
          user_ulid: 'demo_ulid',
          avatar_url: 'https://api.dicebear.com/7.x/avataaars/svg?seed=demo'
        }
      } as UserDocument;
    }
  } catch (e) {
    error = e instanceof Error ? e.message : 'Failed to load profile';
  } finally {
    loading = false;
  }
});
</script>

<NotLoggedInAlert />

{#if $authUserDoc}
  <ProfileLoggedIn {user} {loading} />
{:else}
  <ProfileLoggedOut {user} {loading} />
{/if} 