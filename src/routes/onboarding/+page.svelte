<script lang="ts">
  import { setDoc, uploadFile } from '@junobuild/core';
  import { goto } from '$app/navigation';
  import type { UserData } from '$lib/types';
  import { authUser, authUserDoneInitializing } from '$lib/stores/authUser';
  import { authUserDoc } from '$lib/stores/authUserDoc';
  import { toaster } from '$lib/skeletonui/toaster-skeleton';
  import NotLoggedInAlert from '$lib/components/common/NotLoggedInAlert.svelte';
  import { createUserDoc } from '$lib/docs-crud/user_create';
  import { queryDocsByKey } from '$lib/docs-crud/query_by_key';
  import { LoaderCircle, CheckCircle, XCircle } from 'lucide-svelte';
  import AvatarCropper from '$lib/components/onboarding/AvatarCropper.svelte';

  let user_handle = '';
  let displayName = '';
  let avatarUrl = '';
  let loading = false;
  let userDocFetched = false;
  let usernameStatus: 'idle' | 'loading' | 'available' | 'taken' | 'error' = 'idle';
  let lastCheckedHandle = '';
  let principalString = '';
  let croppingInProgress = false;
  let avatarUploadPromise: Promise<any> | null = null;
  let avatarUrlToSave: string = '';
  let avatarUploadComplete = false;
  let saveProfileRequested = false;

  /**
   * Single source of truth for the avatar file.
   * This variable is shared between the upload area and the cropper.
   * When set/cleared in one place, the other updates automatically.
   */
  let avatarFile: File | null = null;

  /**
   * Utility: debounce
   *
   * Returns a debounced version of the provided function, ensuring it is only invoked
   * after the specified delay has elapsed since the last call. Used to limit the rate
   * of backend queries for username availability as the user types in the onboarding form.
   *
   * @template T - The function type to debounce
   * @param fn - The function to debounce
   * @param delay - The debounce delay in milliseconds
   * @returns A debounced function with the same parameters as the original
   *
   * Usage: Used for debouncing username availability checks to avoid excessive backend requests.
   */
  function debounce<T extends (...args: any[]) => void>(fn: T, delay: number): (...args: Parameters<T>) => void {
    let timeout: ReturnType<typeof setTimeout>;
    return (...args: Parameters<T>) => {
      clearTimeout(timeout);
      timeout = setTimeout(() => fn(...args), delay);
    };
  }

  const checkUsername = debounce(async (handle: string) => {
    if (!handle || handle.length < 3) {
      usernameStatus = 'idle';
      return;
    }
    usernameStatus = 'loading';
    lastCheckedHandle = handle;
    try {
      const normalized = handle.trim().toLowerCase();
      const keyPattern = `hdl_${normalized}_`;
      const results = await queryDocsByKey('users', keyPattern);
      // Only update if the input hasn't changed since the request was sent
      if (lastCheckedHandle === handle) {
        usernameStatus = results.items.length > 0 ? 'taken' : 'available';
      }
    } catch (e) {
      if (lastCheckedHandle === handle) {
        usernameStatus = 'error';
      }
    }
  }, 350);

  // Only fetch user doc if authenticated and initialized
  $: if ($authUserDoneInitializing && $authUser && !userDocFetched) {
    if ($authUserDoc) {
      user_handle = $authUserDoc.data.user_handle || '';
      displayName = $authUserDoc.data.display_name || '';
      avatarUrl = $authUserDoc.data.avatar_url || '';
    }
    userDocFetched = true;
  }

  // Ensure principal is always a string for avatar filename
  $: principalString = typeof $authUser?.key === 'string' ? $authUser.key : '';

  async function handleSubmit(event: Event) {
    event.preventDefault();
    if (!$authUser) {
      toaster.error({ title: 'You must be logged in to set up your profile.' });
      return;
    }
    await saveProfile();
  }

  /**
   * Handles the avatar cropping and upload process.
   *
   * This function is called when the user clicks 'Crop' in the avatar cropper.
   * It immediately starts uploading the cropped avatar image to the backend (Juno Storage).
   * The upload result is logged to the console so we can inspect the returned object and determine the correct URL property.
   *
   * If the user clicks 'Save Profile' before the upload is complete, the save handler will wait for this promise to resolve.
   *
   * @param blob - The cropped avatar image as a Blob (or null if removed)
   */
  async function handleCrop(blob: Blob | null) {
    if (!blob) {
      // If the user removes the avatar, reset all upload state
      avatarUploadPromise = null;
      avatarUrlToSave = '';
      avatarUploadComplete = false;
      avatarFile = null;
      return;
    }
    avatarUploadComplete = false;
    // Use the user's principal (key) for the filename, so it is unique and stable
    const file = new File([blob], `avatar_${principalString}.webp`, { type: 'image/webp' });
    avatarUploadPromise = uploadFile({
      data: file, // Pass the File object so the backend gets a name
      collection: 'user_avatars',
      filename: file.name // Explicitly set the filename for clarity
    })
      .then(result => {
        // Log the upload result so we can inspect the returned object
        console.log('Avatar upload result:', result);
        // Use downloadUrl or fullPath for the avatar URL
        avatarUrlToSave = result.downloadUrl || result.fullPath || '';
        avatarUploadComplete = true;
        avatarFile = file;
      })
      .catch(err => {
        // Show an error toast if the upload fails
        toaster.error({ title: 'Avatar upload failed', description: err.message });
        avatarUrlToSave = '';
        avatarUploadComplete = false;
        avatarFile = null;
      });
  }

  /**
   * Handles the profile save process.
   *
   * This function is called when the user clicks 'Save Profile'.
   * If an avatar upload is in progress, it waits for the upload to finish before saving the user document.
   * The avatar URL (if available) is included in the user document.
   *
   * Error handling ensures the user is notified if validation fails or if the upload/save fails.
   */
  async function saveProfile() {
    loading = true;
    try {
      // Validate username
      if (!user_handle.trim()) {
        toaster.error({ title: 'Validation Error', description: 'You must enter a username.' });
        loading = false;
        return;
      }
      // Validate display name
      if (!displayName.trim()) {
        toaster.error({ title: 'Validation Error', description: 'You must enter a display name.' });
        loading = false;
        return;
      }
      // Validate authentication
      if (!$authUser) {
        toaster.error({ title: 'User not authenticated.', description: 'Please log in to set up your profile.' });
        loading = false;
        return;
      }
      saveProfileRequested = true;
      // If avatar upload is in progress, wait for it to finish
      if (avatarUploadPromise) {
        await avatarUploadPromise;
      }
      // Use the uploaded avatar URL if available, otherwise fallback to previous avatarUrl
      const finalAvatarUrl = avatarUrlToSave || avatarUrl || '';
      // Save the user document with the avatar URL
      await createUserDoc({
        user_handle: user_handle.trim(),
        display_name: displayName.trim() || ' ',
        avatar_url: finalAvatarUrl
      });

      // Fetch the newly created document to ensure it's in the store
      const keyPattern = `_prn_${$authUser.key}_`;
      const results = await queryDocsByKey<UserData>('users', keyPattern);
      const userDoc = results.items[0];
      if (userDoc) {
        authUserDoc.set(userDoc);
        toaster.success({ title: 'Profile saved!', description: 'Your profile has been updated.' });
        goto('/dashboard');
      } else {
        throw new Error('Failed to fetch created user document');
      }
    } catch (e) {
      // Show an error toast if saving fails
      toaster.error({ title: 'Failed to save profile.', description: e instanceof Error ? e.message : 'Unknown error.' });
    } finally {
      loading = false;
      saveProfileRequested = false;
    }
  }

  $: if (user_handle && user_handle.length >= 3) checkUsername(user_handle);
  $: if (!user_handle || user_handle.length < 3) usernameStatus = 'idle';
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
        <div class="relative w-full">
          <input
            type="text"
            bind:value={user_handle}
            class="input pr-10"
            required
            autocomplete="off"
            aria-describedby="username-status"
            disabled={!$authUser}
          />
          <span class="absolute right-2 top-1/2 -translate-y-1/2" aria-live="polite" id="username-status">
            {#if usernameStatus === 'loading'}
              <LoaderCircle class="animate-spin text-gray-400" />
            {:else if usernameStatus === 'available'}
              <CheckCircle class="text-success-500" />
            {:else if usernameStatus === 'taken'}
              <XCircle class="text-error-500" />
            {:else if usernameStatus === 'error'}
              <XCircle class="text-error-500" />
            {/if}
          </span>
        </div>
        {#if usernameStatus === 'taken'}
          <span class="text-error-500 text-xs mt-1">Username is already taken.</span>
        {:else if usernameStatus === 'available'}
          <span class="text-success-500 text-xs mt-1">Username is available!</span>
        {:else if user_handle && user_handle.length > 0 && user_handle.length < 3}
          <span class="text-error-500 text-xs mt-1">Username must be at least 3 characters.</span>
        {/if}
      </label>
      <label class="label">
        <span class="label-text">Display Name</span>
        <input type="text" bind:value={displayName} class="input" autocomplete="off" disabled={!$authUser} required />
      </label>
      <div class="label">
        <span class="label-text">Avatar (optional)</span>
        {#if $authUser && $authUser.key}
          <AvatarCropper
            initialUrl={avatarUrl}
            bind:avatarFile={avatarFile}
            cropped={handleCrop}
            change={(url) => avatarUrl = url}
            croppingChange={v => croppingInProgress = v}
          />
        {/if}
      </div>
    </fieldset>
    <fieldset>
      <div style="min-height:1.2em;display:flex;align-items:center;justify-content:center;">
        {#if croppingInProgress}
          <span class="text-error-500 text-sm font-medium">Crop your image or remove it to save profile</span>
        {/if}
      </div>
      <button type="submit" class="btn preset-filled-primary-500 w-full" disabled={loading || !$authUser || croppingInProgress}>
        {loading ? 'Saving...' : 'Save Profile'}
      </button>
    </fieldset>
  </form>
{/if}
