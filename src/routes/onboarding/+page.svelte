<script lang="ts">
  import { setDoc, getDoc } from '@junobuild/core';
  import { goto } from '$app/navigation';
  import type { UserData } from '$lib/types';
  import { FileUpload } from '@skeletonlabs/skeleton-svelte';
  import { authUser, authUserDoneInitializing } from '$lib/stores/authUser';
  import { toaster } from '$lib/toaster-skeleton';
  import NotLoggedInAlert from '$lib/components/NotLoggedInAlert.svelte';

  let user_handle = '';
  let displayName = '';
  let avatarUrl = '';
  let loading = false;
  let userDocFetched = false;

  // Only fetch user doc if authenticated and initialized
  $: if ($authUserDoneInitializing && $authUser && !userDocFetched) {
    (async () => {
      try {
        const userDoc = await getDoc({ collection: 'users', key: $authUser.key });
        const data = userDoc?.data as UserData | undefined;
        if (userDoc) {
          user_handle = data?.user_handle || '';
          displayName = data?.display_name || '';
        }
        userDocFetched = true;
      } catch (e) {
        // Ignore errors here
      }
    })();
  }

  async function handleSubmit(event: Event) {
    event.preventDefault();
    if (!$authUser) {
      toaster.error({ title: 'You must be logged in to set up your profile.' });
      return;
    }
    // ... existing logic ...
  }

  async function saveProfile() {
    loading = true;
    try {
      if (!user_handle.trim()) {
        toaster.error({ title: 'Validation Error', description: 'Username/handle is required.' });
        loading = false;
        return;
      }
      if (!$authUser) {
        toaster.error({ title: 'User not authenticated.', description: 'Please log in to set up your profile.' });
        loading = false;
        return;
      }
      await setDoc({
        collection: 'users',
        doc: {
          key: $authUser.key,
          data: {
            user_handle,
            display_name: displayName,
            user_key: $authUser.key,
            avatar_url: avatarUrl
          }
        }
      });
      toaster.success({ title: 'Profile saved!', description: 'Your profile has been updated.' });
      goto('/reputations');
    } catch (e) {
      toaster.error({ title: 'Failed to save profile.', description: e instanceof Error ? e.message : 'Unknown error.' });
    } finally {
      loading = false;
    }
  }
</script>

{#if !$authUserDoneInitializing}
  <!-- Loading placeholder for onboarding card -->
  <div class="card shadow bg-surface-100-900 border border-surface-200-800 p-5 space-y-5 max-w-md mx-auto mt-10 animate-pulse">
    <div class="h-6 bg-surface-300-700 rounded w-1/2 mb-4"></div>
    <div class="h-4 bg-surface-200-800 rounded w-3/4 mb-2"></div>
    <div class="h-4 bg-surface-200-800 rounded w-2/3 mb-2"></div>
    <div class="h-10 bg-surface-200-800 rounded w-full mb-4"></div>
    <div class="h-10 bg-surface-200-800 rounded w-full"></div>
  </div>
{:else}
  {#if $authUserDoneInitializing && !$authUser}
    <NotLoggedInAlert />
  {/if}

  <form class="card shadow bg-surface-100-900 border border-surface-200-800 p-5 space-y-5 max-w-md mx-auto mt-10"
        on:submit|preventDefault={handleSubmit}>
    <fieldset class="space-y-2">
      <h2 class="h2">Set Up Your Profile</h2>
      <p class="opacity-60">Choose a username and display name. You can add an avatar later.</p>
    </fieldset>
    <fieldset class="space-y-2">
      <label class="label">
        <span class="label-text">Username</span>
        <input type="text" bind:value={user_handle} class="input" required autocomplete="off" disabled={!$authUser} />
      </label>
      <label class="label">
        <span class="label-text">Display Name</span>
        <input type="text" bind:value={displayName} class="input" required autocomplete="off" disabled={!$authUser} />
      </label>
      <label class="label">
        <span class="label-text">Avatar (optional)</span>
        <FileUpload
          name="avatar"
          accept={{
            "image/png": [".png"],
            "image/jpeg": [".jpg", ".jpeg"],
            "image/webp": [".webp"],
            "image/svg+xml": [".svg"],
            "image/gif": [".gif"]
          }}
          maxFiles={1}
          classes="w-full"
          disabled={!$authUser}
        />
      </label>
    </fieldset>
    <fieldset>
      <button type="submit" class="btn preset-filled-primary-500 w-full" disabled={loading || !$authUser}>
        {loading ? 'Saving...' : 'Save Profile'}
      </button>
    </fieldset>
  </form>
{/if} 