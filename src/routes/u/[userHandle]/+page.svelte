<script lang="ts">
let { data } = $props<{ data: { handle: string } }>();
import type { UserDocument } from '$lib/types';
import ProfileHeader from '$lib/components/profile/ProfileHeader.svelte';
import TrustedCommunities from '$lib/components/profile/TrustedCommunities.svelte';

import ActiveReputations from '$lib/components/profile/ActiveReputations.svelte';
import RecentReviewsUser from '$lib/components/profile/RecentReviewsUser.svelte';
import RecentVotesUser from '$lib/components/profile/RecentVotesUser.svelte';
import { onMount } from 'svelte';
import { initJuno } from '$lib/juno';
import { toaster } from '$lib/skeletonui/toaster-skeleton';
import { queryDocsByKey } from '$lib/docs-crud/query_by_key';
import { authUserDoc } from '$lib/stores/authUserDoc';
import { authUser } from '$lib/stores/authUser';
import { dummyProfileData } from '$lib/data/dummyProfileData';
import { setPageMeta } from '$lib/stores/page';
import { goto } from '$app/navigation';

let junoInitialized = $state(false);
let loading = $state(true);
let error_state = $state<string | null>(null);
let userDocument = $state<UserDocument | null>(null);
let currentHandle = $state<string>('');
let isNormalizing = $state(false); // Flag to prevent re-fetch during URL normalization

// Initialize Juno on mount
onMount(async () => {
  try {
    await initJuno();
    junoInitialized = true;
  } catch (e) {
    error_state = e instanceof Error ? e.message : 'Failed to initialize Juno';
    toaster.error({ title: error_state });
    loading = false;
  }
});

// Fetch user document for a specific handle
async function fetchUserDocument(handle: string): Promise<UserDocument> {
  // Case 1: Demo user - return dummy data
  if (handle === 'demo_user') {
    return dummyProfileData.user;
  }

  // Case 2: Current logged-in user - use existing store
  if ($authUserDoc && handle.toLowerCase() === $authUserDoc.data.user_handle.toLowerCase()) {
    return $authUserDoc;
  }

  // Case 3: Other user - fetch from database
  // Normalize handle to lowercase to match database storage format
  const normalizedHandle = handle.toLowerCase();
  const results = await queryDocsByKey('users', `hdl_${normalizedHandle}_`);
  if (!results.items.length) {
    throw new Error('User not found');
  }

  return results.items[0] as UserDocument;
}

// React to URL parameter changes
$effect(() => {
  const handle = data.handle;
  
  // Skip if we're currently normalizing the URL to prevent re-fetch
  if (isNormalizing) {
    isNormalizing = false; // Reset flag
    return;
  }
  
  // Only proceed if Juno is initialized and handle has changed
  if (!junoInitialized || handle === currentHandle) {
    return;
  }

  // For logged-in users accessing their own profile, wait for authUserDoc to be populated
  // This prevents race conditions when accessing profile directly via URL
  if ($authUser && !$authUserDoc && handle !== 'demo_user') {
    // Auth user exists but user doc not loaded yet - wait for layout to populate it
    return;
  }

  // Update current handle and start loading
  currentHandle = handle;
  loading = true;
  error_state = null;

  // Handle async operation inside the effect
  (async () => {
    try {
      userDocument = await fetchUserDocument(handle);
      
      // Check if URL needs normalization (case mismatch)
      if (userDocument) {
        const realHandle = userDocument.data.user_handle;
        const urlHandle = handle;
        
        if (realHandle !== urlHandle) {
          // Mismatch detected! Normalize URL without re-fetching data
          // Set page title before normalizing since we'll skip title setting in the next effect run
          const browserTitle = `@${userDocument.data.user_handle}`;
          const headerTitle = `${userDocument.data.display_name}`;
          setPageMeta({ 
            title: browserTitle,
            headerTitle: headerTitle
          });
          
          isNormalizing = true; // Set flag BEFORE goto to prevent effect re-run
          goto(`/u/${realHandle}`, { replaceState: true });
          return; // Exit early, the goto will trigger effect again but flag will prevent re-fetch
        }
        
        // No mismatch, set page title normally
        const browserTitle = `@${userDocument.data.user_handle}`;
        const headerTitle = `${userDocument.data.display_name}`;
        setPageMeta({ 
          title: browserTitle,
          headerTitle: headerTitle
        });
      }
    } catch (e) {
      error_state = e instanceof Error ? e.message : 'Failed to load user';
      toaster.error({ title: error_state });
      userDocument = null;
    } finally {
      loading = false;
    }
  })();
});
</script>

<div class="p-4">
  {#if error_state}
    <div class="alert alert-error mb-6">{error_state}</div>
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
  {:else if userDocument}
    <!-- Main Grid Layout -->
    <div class="grid grid-cols-1 lg:grid-cols-2 2xl:grid-cols-3 gap-6">
      <!-- Left Column -->
      <div class="space-y-6">
        <ProfileHeader user={userDocument} />
        <TrustedCommunities communities={dummyProfileData.trustedCommunities} />
      </div>

      <!-- Middle Column -->
      <div class="space-y-6">
        <RecentVotesUser user={userDocument} />
        <RecentReviewsUser reviews={dummyProfileData.recentReviews} />
      </div>

      <!-- Right Column -->
      <div class="space-y-6">
        <ActiveReputations reputations={dummyProfileData.activeReputations} />
      </div>
    </div>
  {/if}
</div> 