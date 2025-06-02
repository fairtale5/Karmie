<script lang="ts">
import { onMount } from 'svelte';
import { page } from '$app/stores';
import { authUser } from '$lib/stores/authUser';
import { goto } from '$app/navigation';
import type { UserDocument } from '$lib/types';
import ProfileLoggedIn from '$lib/components/profile/ProfileLoggedIn.svelte';
import ProfileLoggedOut from '$lib/components/profile/ProfileLoggedOut.svelte';

// --- Dummy Data ---
const dummyUser: UserDocument = {
  key: 'user_123',
  data: {
    user_handle: 'demo_user',
    display_name: 'Demo User',
    user_ulid: 'user_123',
    avatar_url: 'https://i.pravatar.cc/100?img=3'
  }
};

// --- State Management ---
let user = $state<UserDocument | null>(null);
let loading = $state(true);
let error = $state<string | null>(null);

// --- Initialization ---
onMount(async () => {
  try {
    // If logged in, redirect to real profile
    if ($authUser) {
      goto('/u/' + $authUser.key);
      return;
    }

    // Use dummy data for demo
    user = dummyUser;
  } catch (e) {
    error = e instanceof Error ? e.message : 'Failed to load profile';
  } finally {
    loading = false;
  }
});
</script>

<ProfileLoggedOut {user} {loading} /> 