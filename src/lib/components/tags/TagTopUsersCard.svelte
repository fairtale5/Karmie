<script lang="ts">
  import { goto } from '$app/navigation';
  import { Expand, UserRoundPen } from 'lucide-svelte';
  import { Avatar } from '@skeletonlabs/skeleton-svelte';
  import BaseCard from '$lib/components/common/BaseCard.svelte';
  import type { TagDocument } from '$lib/types';

  const { 
    tag, 
    topUsers, 
    loading = false,
    isPreview = false
  }: { 
    tag: TagDocument | null; 
    topUsers: any[];
    loading?: boolean;
    isPreview?: boolean;
  } = $props();
</script>

<BaseCard>
  {#snippet header()}
    <h2 class="text-lg font-bold {(!tag || loading) ? 'opacity-50' : ''}">Top Users</h2>
  {/snippet}
  
  {#snippet actions()}
    {#if tag && !isPreview}
      <button 
        type="button" 
        class="chip-icon preset-tonal-surface" 
        onclick={() => goto(`/tag/${tag?.data.tag_handle}/users`)} 
        disabled={loading || topUsers.length === 0} 
        title="See More Users"
      >
        <Expand size={16} />
      </button>
    {/if}
  {/snippet}
  
  {#snippet children()}
    {#if loading}
      <div class="space-y-2">
        {#each Array(3) as _}
          <div class="flex items-center gap-2 placeholder animate-pulse h-10 rounded"></div>
        {/each}
      </div>
    {:else if tag && topUsers.length > 0}
      <div class="table-wrap">
        <table class="table caption-bottom">
          <thead>
            <tr>
              <th>User</th>
              <th class="text-right">Score</th>
            </tr>
          </thead>
          <tbody class="[&>tr]:hover:preset-tonal-primary">
            {#each topUsers as user, i (user.username)}
              <tr>
                <td>
                  <div class="flex items-center gap-2">
                    <Avatar name={user.username}>
                      <UserRoundPen class="w-6 h-6 text-surface-700" />
                    </Avatar>
                    <span class="font-bold">{user.username}</span>
                    {#if i === 0}
                      <span class="text-yellow-500">🥇</span>
                    {:else if i === 1}
                      <span class="text-gray-400">🥈</span>
                    {:else if i === 2}
                      <span class="text-orange-700">🥉</span>
                    {/if}
                  </div>
                </td>
                <td class="text-right">
                  <span class="badge preset-filled-secondary-500">{user.score} points</span>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {:else if tag && topUsers.length === 0}
      <p class="text-center opacity-70">No top users to display for this tag.</p>
    {:else}
      <p class="text-center opacity-70">Loading users...</p>
    {/if}
  {/snippet}
</BaseCard> 