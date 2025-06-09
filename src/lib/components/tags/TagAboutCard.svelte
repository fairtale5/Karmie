<script lang="ts">
  import { tick } from 'svelte';
  import { goto } from '$app/navigation';
  import { Tabs } from '@skeletonlabs/skeleton-svelte';
  import { Orbit, SlidersHorizontal } from 'lucide-svelte';
  import BaseCard from '$lib/components/common/BaseCard.svelte';
  import { authUserDoc } from '$lib/stores/authUserDoc';
  import type { TagDocument } from '$lib/types';

  const { tag, loading = false }: { tag: TagDocument | null; loading?: boolean } = $props();
  
  let activeTab = $state('about');
</script>

<BaseCard classes="h-[400px]">
  {#snippet header()}
    <h2 class="text-lg font-bold">Tag Details</h2>
  {/snippet}
  
  {#snippet children()}
    <div class="h-full flex flex-col">
      <Tabs value={activeTab} onValueChange={async (e) => { activeTab = e.value; await tick();}}>
        {#snippet list()}
          <Tabs.Control value="about" disabled={loading || !tag}>
            {#snippet lead()}<Orbit size={20} />{/snippet}
            {#if tag}#{tag.data.tag_handle}{:else}About{/if}
          </Tabs.Control>
          <Tabs.Control value="settings" disabled={loading || !tag}>
            {#snippet lead()}<SlidersHorizontal size={20} />{/snippet}
            Settings
          </Tabs.Control>
        {/snippet}
        {#snippet content()}
          <div class="h-[288px] overflow-y-auto">
            <Tabs.Panel value="about">
              {#if loading}
                <div class="placeholder animate-pulse w-full h-24 rounded"></div>
              {:else if tag?.data?.description}
                <p class="whitespace-pre-line opacity-80">{tag.data.description}</p>
              {:else if tag}
                <p class="opacity-50 text-sm">No description available for this tag.</p>
              {:else}
                <p class="text-center opacity-70">Loading tag details...</p>
              {/if}
            </Tabs.Panel>
            <Tabs.Panel value="settings">
              {#if loading}
                <div class="placeholder animate-pulse w-1/2 h-8 rounded mb-4"></div>
                <div class="grid grid-cols-2 gap-4">
                  <div class="p-3 bg-surface-200-800 rounded placeholder animate-pulse h-16"></div>
                  <div class="p-3 bg-surface-200-800 rounded placeholder animate-pulse h-16"></div>
                  <div class="p-3 bg-surface-200-800 rounded placeholder animate-pulse h-16"></div>
                </div>
              {:else if tag?.data}
                <div class="flex justify-between items-center mb-4">
                  {#if $authUserDoc?.data.user_ulid === tag?.data?.owner_ulid}
                    <button class="btn preset-tonal-primary" onclick={() => goto(`/tag/edit/${tag?.key}`)}>
                      Edit Settings
                    </button>
                  {/if}
                </div>
                <div class="grid grid-cols-2 gap-4">
                  <div class="p-3 bg-surface-200-800 rounded">
                    <span class="text-sm opacity-70">Reputation Threshold</span>
                    <p class="font-mono text-lg">{tag?.data?.reputation_threshold ?? 'N/A'}</p>
                  </div>
                  <div class="p-3 bg-surface-200-800 rounded">
                    <span class="text-sm opacity-70">Vote Reward</span>
                    <p class="font-mono text-lg">{tag?.data?.vote_reward ?? 'N/A'}</p>
                  </div>
                  <div class="p-3 bg-surface-200-800 rounded">
                    <span class="text-sm opacity-70">Min Users</span>
                    <p class="font-mono text-lg">{tag?.data?.min_users_for_threshold ?? 'N/A'}</p>
                  </div>
                </div>
                <hr class="my-4 border-surface-300-700" />
                <div>
                  <h4 class="text-md font-semibold mb-2">Decay Rules</h4>
                  {#if tag?.data?.time_periods?.length > 0}
                    <div class="space-y-2">
                      {#each tag.data.time_periods as period}
                        <div class="p-2 bg-surface-200-800 rounded text-xs">
                          After <span class="font-semibold">{period.months} months</span>:
                          {#if period.multiplier < 1}
                            Reputation decays by <span class="font-semibold text-error-500">{((1 - period.multiplier) * 100).toFixed(1)}%</span>
                          {:else if period.multiplier === 1}
                            <span class="font-semibold text-surface-500">No change</span> to reputation
                          {:else}
                            Reputation increases by <span class="font-semibold text-success-500">{((period.multiplier - 1) * 100).toFixed(1)}%</span>
                          {/if}
                        </div>
                      {/each}
                    </div>
                  {:else}
                    <p class="text-sm opacity-70">No decay rules specified for this tag. Reputation scores will remain constant over time.</p>
                  {/if}
                </div>
              {:else}
                <p class="text-center opacity-70">Loading tag settings...</p>
              {/if}
            </Tabs.Panel>
          </div>
        {/snippet}
      </Tabs>
    </div>
  {/snippet}
</BaseCard> 