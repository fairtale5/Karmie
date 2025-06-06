<script lang="ts">
import { Avatar, Popover } from '@skeletonlabs/skeleton-svelte';
import type { UserDocument } from '$lib/types';
import BaseCard from '$lib/components/common/BaseCard.svelte';
import { dummyProfileData } from '$lib/data/dummyProfileData';
import { authUserDoc } from '$lib/stores/authUserDoc';
import { Edit3, Save, X, CircleHelp, Trash2, LoaderCircle } from 'lucide-svelte';
import { updateUserDoc } from '$lib/docs-crud/user_update';
import { deleteUserDoc } from '$lib/docs-crud/user_delete';
import { toaster } from '$lib/skeletonui/toaster-skeleton';
import { goto } from '$app/navigation';
import { signOut } from '@junobuild/core';

interface CommunityStats {
  totalVotesGiven: number;
  totalVotesReceived: number;
  trustedCommunities: number;
  activeCommunities: number;
  averageScore: number;
}

const { user } = $props<{ user: UserDocument }>();

// For now, use dummy stats. Later this should fetch real community stats based on user
let stats: CommunityStats = dummyProfileData.communityStats;

// Edit mode state
let editMode = $state(false);
let loading = $state(false);
let deleteConfirmOpen = $state(false);

// Form fields for editing
let editDisplayName = $state('');
let editDescription = $state('');
let editAvatarUrl = $state('');

// Popover states
let handleHelpOpen = $state(false);

// Check if current user owns this profile
const isOwner = $derived($authUserDoc?.key === user.key);

// Initialize edit fields when entering edit mode
function startEdit() {
  editDisplayName = user.data.display_name || '';
  editDescription = user.data.description || '';
  editAvatarUrl = user.data.avatar_url || '';
  editMode = true;
}

function cancelEdit() {
  editMode = false;
  editDisplayName = '';
  editDescription = '';
  editAvatarUrl = '';
}

async function saveProfile() {
  if (!user.key || user.version === undefined) {
    toaster.error({ title: 'Error', description: 'Invalid user document data' });
    return;
  }

  loading = true;
  try {
    await toaster.promise(
      updateUserDoc({
        key: user.key,
        version: user.version,
        data: {
          user_handle: user.data.user_handle, // Keep the same handle
          display_name: editDisplayName.trim(),
          description: editDescription.trim(),
          avatar_url: editAvatarUrl.trim(),
          user_ulid: user.data.user_ulid // Keep the same ULID
        }
      }),
      {
        loading: {
          title: 'Updating Profile',
          description: 'Saving your changes to the blockchain...'
        },
        success: () => ({
          title: 'Profile Updated!',
          description: 'Your profile has been successfully updated.'
        }),
        error: (e) => ({
          title: 'Failed to Update Profile',
          description: e instanceof Error ? e.message : 'An unknown error occurred.'
        })
      }
    );

    // Update the local user object with new values
    user.data.display_name = editDisplayName.trim();
    user.data.description = editDescription.trim();
    user.data.avatar_url = editAvatarUrl.trim();
    
    editMode = false;
  } catch (e) {
    console.error('Failed to update profile:', e);
  } finally {
    loading = false;
  }
}

async function deleteAccount() {
  if (!user.key || user.version === undefined) {
    toaster.error({ title: 'Error', description: 'Invalid user document data' });
    return;
  }

  loading = true;
  try {
    await toaster.promise(
      (async () => {
        await deleteUserDoc(user.key, user.version);
        // Sign out the user after successful deletion
        await signOut();
        // Navigate to home page
        goto('/');
      })(),
      {
        loading: {
          title: 'Deleting Account',
          description: 'Removing your account from the blockchain...'
        },
        success: () => ({
          title: 'Account Deleted',
          description: 'Your account has been successfully deleted.'
        }),
        error: (e) => ({
          title: 'Failed to Delete Account',
          description: e instanceof Error ? e.message : 'An unknown error occurred.'
        })
      }
    );
  } catch (e) {
    console.error('Failed to delete account:', e);
    loading = false;
  }
}

function closeHandleHelp() {
  handleHelpOpen = false;
}
</script>

<BaseCard>
  <div class="flex flex-col items-center">
    <!-- Edit button - only visible to profile owner -->
    {#if isOwner && !editMode}
      <div class="self-end mb-2">
        <button 
          class="btn btn-sm preset-tonal-primary" 
          onclick={startEdit}
          disabled={loading}
        >
          <Edit3 class="w-4 h-4 mr-1" />
          Edit Profile
        </button>
      </div>
    {/if}

    <!-- Avatar section -->
    <div class="mb-4 w-24 h-24">
      {#if editMode}
        <div class="relative">
          <Avatar 
            src={editAvatarUrl} 
            size="xl" 
            name={editDisplayName || user.data.display_name}
            classes="w-full h-full object-cover"
          />
          <div class="absolute inset-0 flex items-center justify-center bg-black bg-opacity-50 rounded-full opacity-0 hover:opacity-100 transition-opacity cursor-pointer">
            <Edit3 class="w-6 h-6 text-white" />
          </div>
        </div>
      {:else}
        <Avatar 
          src={user.data.avatar_url} 
          size="xl" 
          name={user.data.display_name}
          classes="w-full h-full object-cover"
        />
      {/if}
    </div>

    <!-- Display name section -->
    {#if editMode}
      <input
        type="text"
        bind:value={editDisplayName}
        class="input text-2xl font-bold text-center border-primary-300-700 focus:border-primary-500 mb-2"
        placeholder="Display name"
        disabled={loading}
      />
    {:else}
      <h1 class="text-2xl font-bold">{user.data.display_name}</h1>
    {/if}

    <!-- Handle section with help icon in edit mode -->
    <div class="flex items-center gap-1 opacity-60 mb-4">
      <span>@{user.data.user_handle}</span>
      {#if editMode}
        <Popover
          open={handleHelpOpen}
          onOpenChange={(e) => (handleHelpOpen = e.open)}
          positioning={{ placement: 'top', flip: true }}
          triggerBase="btn-icon variant-ghost"
          contentBase="card bg-surface-200-800 p-4 space-y-4 max-w-[320px]"
          arrow
          arrowBackground="!bg-surface-200 dark:!bg-surface-800"
        >
          {#snippet trigger()}
            <CircleHelp class="w-4 h-4 opacity-70" />
          {/snippet}
          {#snippet content()}
            <header class="flex justify-between">
              <p class="font-bold">Handle Cannot Be Changed</p>
              <button class="btn-icon hover:preset-tonal" onclick={closeHandleHelp}>
                <X class="w-4 h-4" />
              </button>
            </header>
            <article>
              <p class="opacity-60">
                You can delete this account and create a new one, or request the feature to change your handle on <a href="https://github.com/junobuild/junobuild/issues/new" target="_blank" class="text-primary-500 hover:underline">GitHub</a>.
              </p>
            </article>
          {/snippet}
        </Popover>
      {/if}
    </div>

    <!-- Description section -->
    {#if editMode}
      <textarea
        bind:value={editDescription}
        class="input w-full text-center border-secondary-300-700 focus:border-secondary-500 mb-4"
        placeholder="Tell us about yourself..."
        rows="3"
        disabled={loading}
      ></textarea>
    {:else if user.data.description}
      <p class="text-center opacity-80 mb-4">{user.data.description}</p>
    {/if}

    <!-- Avatar URL field in edit mode -->
    {#if editMode}
      <div class="w-full mb-4">
        <label class="label">
          <span class="label-text text-sm opacity-70">Avatar URL (optional)</span>
          <input
            type="url"
            bind:value={editAvatarUrl}
            class="input border-tertiary-300-700 focus:border-tertiary-500"
            placeholder="https://example.com/avatar.jpg"
            disabled={loading}
          />
        </label>
      </div>
    {/if}

    <!-- Edit mode action buttons -->
    {#if editMode}
      <div class="flex gap-2 mb-4">
        <button 
          class="btn preset-filled-primary-500" 
          onclick={saveProfile}
          disabled={loading || !editDisplayName.trim()}
        >
          {#if loading}
            <LoaderCircle class="animate-spin mr-2 w-4 h-4" />
            Saving...
          {:else}
            <Save class="w-4 h-4 mr-1" />
            Save Changes
          {/if}
        </button>
        <button 
          class="btn preset-tonal-secondary" 
          onclick={cancelEdit}
          disabled={loading}
        >
          <X class="w-4 h-4 mr-1" />
          Cancel
        </button>
      </div>

      <!-- Delete account section -->
      <div class="w-full border-t border-surface-300-700 pt-4 mt-4">
        <details class="space-y-2">
          <summary class="cursor-pointer text-error-500 text-sm font-medium">
            Danger Zone
          </summary>
          <div class="space-y-2">
            <p class="text-xs opacity-60">
              Want to change your handle? You'll need to delete your account and create a new one.
            </p>
            {#if !deleteConfirmOpen}
              <button 
                class="btn btn-sm preset-filled-error-500 w-full" 
                onclick={() => deleteConfirmOpen = true}
                disabled={loading}
              >
                <Trash2 class="w-4 h-4 mr-1" />
                Delete Account
              </button>
            {:else}
              <div class="space-y-2">
                <p class="text-error-500 text-sm font-bold">Are you sure? This cannot be undone!</p>
                <div class="flex gap-2">
                  <button 
                    class="btn btn-sm preset-filled-error-500 flex-1" 
                    onclick={deleteAccount}
                    disabled={loading}
                  >
                    {#if loading}
                      <LoaderCircle class="animate-spin w-4 h-4" />
                    {:else}
                      Yes, Delete
                    {/if}
                  </button>
                  <button 
                    class="btn btn-sm preset-tonal-secondary flex-1" 
                    onclick={() => deleteConfirmOpen = false}
                    disabled={loading}
                  >
                    Cancel
                  </button>
                </div>
              </div>
            {/if}
          </div>
        </details>
      </div>
    {/if}

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