<script lang="ts">
import { onMount } from 'svelte';
import { setDoc, type Doc } from '@junobuild/core';
import { goto } from '$app/navigation';
import { REPUTATION_SETTINGS } from '$lib/settings';
import { formatTagKey, createUlid } from '$lib/keys/mod.js';
import type { TagData } from '$lib/types';
import { toaster } from '$lib/skeletonui/toaster-skeleton';
import { CircleHelp } from 'lucide-svelte';
import { authUserDoc } from '$lib/stores/authUserDoc';
import { Popover } from '@skeletonlabs/skeleton-svelte';
import { X } from 'lucide-svelte';
import { queryDocsByKey } from '$lib/docs-crud/query_by_key';
import { LoaderCircle, CheckCircle, XCircle } from 'lucide-svelte';
import { createTagDoc } from '$lib/docs-crud/tag_create';

// Tag form state
let tagBeingEdited = $state<Doc<TagData>>({
  key: '',
  data: {
    user_key: '',
    tag_handle: '',
    description: '',
    time_periods: [...REPUTATION_SETTINGS.DEFAULT_TIME_PERIODS] as Array<{ months: number; multiplier: number }>,
    reputation_threshold: REPUTATION_SETTINGS.DEFAULT_TAG.REPUTATION_THRESHOLD,
    vote_reward: REPUTATION_SETTINGS.DEFAULT_TAG.VOTE_REWARD,
    min_users_for_threshold: REPUTATION_SETTINGS.DEFAULT_TAG.MIN_USERS_FOR_THRESHOLD
  }
});

let loading = $state(false);
let errorGlobal = $state('');

// Popover states
let reputationPopoverOpen = $state(false);
let voteRewardPopoverOpen = $state(false);
let minUsersPopoverOpen = $state(false);
let tagNamePopoverOpen = $state(false);
let descriptionPopoverOpen = $state(false);
let timePeriodsPopoverOpen = $state(false);

// Tag name checking state
let tagNameStatus = $state<'idle' | 'loading' | 'available' | 'taken' | 'error' | 'invalid'>('idle');
let tagNameError = $state<string>('');
let debounceTimer: ReturnType<typeof setTimeout>;

function validateTagName(name: string): { isValid: boolean; error?: string } {
  if (!name) return { isValid: false, error: 'Tag name is required' };
  
  // Check for spaces
  if (name.includes(' ')) {
    return { isValid: false, error: 'No spaces allowed' };
  }

  // Check for special characters and validate format
  const validFormat = /^[a-zA-Z0-9]+(?:-[a-zA-Z0-9]+)*$/;
  if (!validFormat.test(name)) {
    return { 
      isValid: false, 
      error: 'Only letters, numbers, and single dashes between words allowed' 
    };
  }

  // Check for consecutive dashes
  if (name.includes('--')) {
    return { isValid: false, error: 'No consecutive dashes allowed' };
  }

  // Check for dashes at start or end
  if (name.startsWith('-') || name.endsWith('-')) {
    return { isValid: false, error: 'Dashes not allowed at start or end' };
  }

  return { isValid: true };
}

function debounce(fn: () => void, delay: number) {
  clearTimeout(debounceTimer);
  debounceTimer = setTimeout(fn, delay);
}

async function checkTagName() {
  const tagHandle = tagBeingEdited.data.tag_handle;
  
  // Reset status
  tagNameStatus = 'idle';
  tagNameError = '';

  // Basic length check
  if (!tagHandle || tagHandle.length < 3) {
    tagNameStatus = 'idle';
    return;
  }

  // Validate format
  const validation = validateTagName(tagHandle);
  if (!validation.isValid) {
    tagNameStatus = 'invalid';
    tagNameError = validation.error || 'Invalid tag name format';
    return;
  }

  // If valid, check availability
  tagNameStatus = 'loading';
  try {
    const normalizedTagName = tagHandle.toLowerCase();
    const existingTags = await queryDocsByKey('tags', `hdl_${normalizedTagName}_`);
    tagNameStatus = existingTags.items.length > 0 ? 'taken' : 'available';
  } catch (e) {
    tagNameStatus = 'error';
  }
}

$effect(() => {
  const tagHandle = tagBeingEdited.data.tag_handle;
  debounce(checkTagName, 300);
});

function closeReputationPopover() {
  reputationPopoverOpen = false;
}

function closeVoteRewardPopover() {
  voteRewardPopoverOpen = false;
}

function closeMinUsersPopover() {
  minUsersPopoverOpen = false;
}

function closeTagNamePopover() {
  tagNamePopoverOpen = false;
}

function closeDescriptionPopover() {
  descriptionPopoverOpen = false;
}

function closeTimePeriodsPopover() {
  timePeriodsPopoverOpen = false;
}

async function saveTag() {
  errorGlobal = '';
  loading = true;
  try {
    if (!tagBeingEdited.data.tag_handle || !tagBeingEdited.data.description) {
      errorGlobal = 'Please fill in all required fields.';
      loading = false;
      return;
    }

    const userDoc = $authUserDoc;
    if (!userDoc || !userDoc.data.user_key) {
      errorGlobal = 'You must be logged in to create a tag.';
      loading = false;
      return;
    }

    // Set the user key from the auth doc
    tagBeingEdited.data.user_key = userDoc.data.user_key;

    // Show loading toast
    toaster.loading({
      title: 'Creating Tag on the Blockchain',
      description: 'Please wait while we create your tag...'
    });

    // Create the tag using our utility function
    await createTagDoc(tagBeingEdited);
    
    toaster.success({
      title: 'Tag Created!',
      description: 'Your tag was created successfully. Redirecting to the tags page...'
    });
    
    goto('/tags');
  } catch (e) {
    errorGlobal = e instanceof Error ? e.message : 'Failed to create tag.';
    toaster.error({
      title: 'Error Creating Tag',
      description: e instanceof Error ? e.message : 'Failed to create tag.'
    });
  } finally {
    loading = false;
  }
}

function addTimePeriod() {
  const periods = tagBeingEdited.data.time_periods as Array<{ months: number; multiplier: number }>;
  tagBeingEdited.data.time_periods = [
    ...periods,
    { months: 12, multiplier: 1.0 }
  ];
}

function removeTimePeriod(i: number) {
  const periods = tagBeingEdited.data.time_periods as Array<{ months: number; multiplier: number }>;
  tagBeingEdited.data.time_periods = periods.filter((_, idx) => idx !== i);
}
</script>

<div class="min-h-screen flex items-center justify-center py-8">
  <div class="card bg-surface-100-900 border border-surface-200-800 shadow-xl max-w-lg w-full p-8">
    <h1 class="text-2xl font-bold mb-6 text-primary-700-300">Create New Tag</h1>
    <form onsubmit={(e) => { e.preventDefault(); saveTag(); }} class="space-y-5">
      <div>
        <label class="label">
          <div class="flex items-center gap-1">
            <span class="label-text text-base font-medium opacity-70">Tag Name</span>
            <Popover
              open={tagNamePopoverOpen}
              onOpenChange={(e) => (tagNamePopoverOpen = e.open)}
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
                  <p class="font-bold">Creating a Reputation Tag</p>
                  <button class="btn-icon hover:preset-tonal" onclick={closeTagNamePopover}><X class="w-4 h-4" /></button>
                </header>
                <article>
                  <p class="opacity-60">
                    Tags represent unique reputations that users can track. Each tag must have a unique name without the leading # symbol. The tag name will be used to identify and reference this reputation throughout the platform. Choose a clear, descriptive name that represents the reputation you want to track.
                  </p>
                </article>
              {/snippet}
            </Popover>
          </div>
          <div class="relative w-full">
            <input
              type="text"
              bind:value={tagBeingEdited.data.tag_handle}
              class="input pr-10 border-primary-300-700 focus:border-primary-500 focus:ring-primary-500 bg-surface-50-950"
              required
              autocomplete="off"
              aria-describedby="tagname-status"
              disabled={loading}
              placeholder="Enter a unique tag name"
            />
            <span class="absolute right-2 top-1/2 -translate-y-1/2" aria-live="polite" id="tagname-status">
              {#if tagNameStatus === 'loading'}
                <LoaderCircle class="animate-spin text-gray-400" />
              {:else if tagNameStatus === 'available'}
                <CheckCircle class="text-success-500" />
              {:else if tagNameStatus === 'taken' || tagNameStatus === 'error' || tagNameStatus === 'invalid'}
                <XCircle class="text-error-500" />
              {/if}
            </span>
          </div>
          {#if tagNameStatus === 'taken'}
            <span class="text-error-500 text-xs mt-1">Tag name is already taken.</span>
          {:else if tagNameStatus === 'available'}
            <span class="text-success-500 text-xs mt-1">Tag name is available!</span>
          {:else if tagBeingEdited.data.tag_handle && tagBeingEdited.data.tag_handle.length > 0 && tagBeingEdited.data.tag_handle.length < 3}
            <span class="text-error-500 text-xs mt-1">Tag name must be at least 3 characters.</span>
          {:else if tagNameStatus === 'invalid'}
            <span class="text-error-500 text-xs mt-1">{tagNameError}</span>
          {/if}
        </label>
      </div>
      <div>
        <label class="label">
          <div class="flex items-center gap-1">
            <span class="label-text text-base font-medium opacity-70">Description</span>
            <Popover
              open={descriptionPopoverOpen}
              onOpenChange={(e) => (descriptionPopoverOpen = e.open)}
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
                  <p class="font-bold">Tag Description</p>
                  <button class="btn-icon hover:preset-tonal" onclick={closeDescriptionPopover}><X class="w-4 h-4" /></button>
                </header>
                <article>
                  <p class="opacity-60">
                    Provide a clear and detailed description of what this reputation tag represents and how users should be rated. Define what constitutes valuable contributions in this community and what doesn't. For example, in a technical community, emphasize the importance of helpful technical answers over humorous content. This helps maintain the quality and focus of the reputation system.
                  </p>
                </article>
              {/snippet}
            </Popover>
          </div>
          <textarea
            id="tagDescription"
            bind:value={tagBeingEdited.data.description}
            class="input input-lg w-full border-primary-300-700 focus:border-primary-500 focus:ring-primary-500 bg-surface-50-950"
            placeholder="Describe what this tag represents"
            rows="3"
            required
            maxlength="1024"
          ></textarea>
          {#if tagBeingEdited.data.description && tagBeingEdited.data.description.length > 1024}
            <span class="text-error-500 text-xs mt-1">Description cannot exceed 1024 characters</span>
          {/if}
        </label>
      </div>
      <div>
        <fieldset>
          <legend class="flex items-center gap-1 mb-1">
            <span class="block text-base font-medium opacity-70">Set vote decay rules</span>
            <Popover
              open={timePeriodsPopoverOpen}
              onOpenChange={(e) => (timePeriodsPopoverOpen = e.open)}
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
                  <p class="font-bold">Vote Decay Rules</p>
                  <button class="btn-icon hover:preset-tonal" onclick={closeTimePeriodsPopover}><X class="w-4 h-4" /></button>
                </header>
                <article>
                  <p class="opacity-60">
                    Define how votes lose their impact over time. Each period represents a time window where votes are weighted by their multiplier. For example, votes from the last 3 months might count fully (1.0x), while older votes gradually lose their impact. This helps ensure that reputation reflects recent contributions while still considering historical activity.
                  </p>
                </article>
              {/snippet}
            </Popover>
          </legend>
          <div class="space-y-2">
            <table class="w-full border-collapse">
              <thead class="opacity-70 text-center">
                <tr>
                  <th class="font-normal text-left py-2">Period</th>
                  <th class="font-normal py-2">Duration</th>
                  <th class="font-normal py-2">Multiplier</th>
                  <th class="font-normal py-2">Actions</th>
                </tr>
              </thead>
              <tbody>
                {#each (tagBeingEdited.data.time_periods as Array<{ months: number; multiplier: number }>) as period, i}
                  <tr>
                    <td class="border p-2 bg-surface-50-950 text-center">{i + 1}</td>
                    <td class="border p-2 bg-surface-50-950">
                      <input
                        type="number"
                        bind:value={period.months}
                        class="input input-sm w-full border-secondary-300-700 focus:border-secondary-500 bg-surface-100-900 text-right"
                        min="1"
                        max={i === (tagBeingEdited.data.time_periods as Array<{ months: number; multiplier: number }>).length - 1 ? 999 : 12}
                        required
                      />
                    </td>
                    <td class="border p-2 bg-surface-50-950">
                      <input
                        type="number"
                        bind:value={period.multiplier}
                        class="input input-sm w-full border-tertiary-300-700 focus:border-tertiary-500 bg-surface-100-900 text-center"
                        min="0"
                        max="10"
                        step="0.05"
                        required
                      />
                    </td>
                    <td class="border p-2 text-center bg-surface-50-950">
                      {#if i === (tagBeingEdited.data.time_periods as Array<{ months: number; multiplier: number }>).length - 1}
                        <button type="button" onclick={addTimePeriod} class="btn btn-xs preset-filled-secondary-500">Add</button>
                      {:else}
                        <button type="button" onclick={() => removeTimePeriod(i)} class="btn btn-xs preset-filled-error-500">Remove</button>
                      {/if}
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </fieldset>
      </div>
      <div class="space-y-4">
        <div class="flex items-center justify-between gap-4">
          <div class="flex items-center gap-1">
            <label for="reputation_threshold" class="text-base font-medium opacity-70 whitespace-nowrap">Reputation Threshold</label>
            <Popover
              open={reputationPopoverOpen}
              onOpenChange={(e) => (reputationPopoverOpen = e.open)}
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
                  <p class="font-bold">Set the barrier to entry</p>
                  <button class="btn-icon hover:preset-tonal" onclick={closeReputationPopover}><X class="w-4 h-4" /></button>
                </header>
                <article>
                  <p class="opacity-60">
                    Sets the minimum reputation score required for the user's votes to start counting, and for the user to start earning voting rewards. Use this to prevent bots and bad actors from gaming the system.
                  </p>
                </article>
              {/snippet}
            </Popover>
          </div>
          <input
            type="number"
            id="reputation_threshold"
            bind:value={tagBeingEdited.data.reputation_threshold}
            step="1"
            min="0"
            class="input input-sm w-24 text-right border-primary-300-700 focus:border-primary-500 bg-surface-50-950"
            required
          />
        </div>
        <div class="flex items-center justify-between gap-4">
          <div class="flex items-center gap-1">
            <label for="vote_reward" class="text-base font-medium opacity-70 whitespace-nowrap">Vote Reward</label>
            <Popover
              open={voteRewardPopoverOpen}
              onOpenChange={(e) => (voteRewardPopoverOpen = e.open)}
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
                  <p class="font-bold">Rewards for Voting</p>
                  <button class="btn-icon hover:preset-tonal" onclick={closeVoteRewardPopover}><X class="w-4 h-4" /></button>
                </header>
                <article>
                  <p class="opacity-60">
                    Amount of reputation points awarded for each vote on this tag. Only applies to users who have reached the reputation threshold.
                  </p>
                </article>
              {/snippet}
            </Popover>
          </div>
          <input
            type="number"
            id="vote_reward"
            bind:value={tagBeingEdited.data.vote_reward}
            step="0.1"
            min="0"
            class="input input-sm w-24 text-right border-secondary-300-700 focus:border-secondary-500 bg-surface-50-950"
            required
          />
        </div>
        <div class="flex items-center justify-between gap-4">
          <div class="flex items-center gap-1">
            <label for="min_users_for_threshold" class="text-base font-medium opacity-70 whitespace-nowrap">Min Users for Threshold</label>
            <Popover
              open={minUsersPopoverOpen}
              onOpenChange={(e) => (minUsersPopoverOpen = e.open)}
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
                  <p class="font-bold">Minimum Users to activate bot-protection</p>
                  <button class="btn-icon hover:preset-tonal" onclick={closeMinUsersPopover}><X class="w-4 h-4" /></button>
                </header>
                <article>
                  <p class="opacity-60">
                    Minimum number of users required before reputation threshold takes effect. During the early bootstrapping phase of a new reputation, everyone earns voting rewards. This is done so the early users can earn enough reputation. But once this amount of users have reached the threshold, the bot-protection is activated.
                  </p>
                </article>
              {/snippet}
            </Popover>
          </div>
          <input
            type="number"
            id="min_users_for_threshold"
            bind:value={tagBeingEdited.data.min_users_for_threshold}
            step="1"
            min="1"
            class="input input-sm w-24 text-right border-tertiary-300-700 focus:border-tertiary-500 bg-surface-50-950"
            required
          />
        </div>
      </div>
      {#if errorGlobal}
        <div class="alert alert-error preset-filled-error-500 text-white">{errorGlobal}</div>
      {/if}
      <div class="flex gap-4 justify-end mt-6">
        <button type="submit" class="btn preset-filled-primary-500" disabled={loading}>
          {#if loading}
            <LoaderCircle class="animate-spin mr-2" />
            Creating...
          {:else}
            Create Tag
          {/if}
        </button>
        <button type="button" onclick={() => goto('/tags')} class="btn preset-tonal-secondary">
          Cancel
        </button>
      </div>
    </form>
  </div>
</div> 