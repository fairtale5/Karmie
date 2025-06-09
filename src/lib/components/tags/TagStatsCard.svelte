<script lang="ts">
  import BaseCard from '$lib/components/common/BaseCard.svelte';
  import type { TagDocument } from '$lib/types';

  const { 
    tag, 
    stats, 
    loading = false
  }: { 
    tag: TagDocument | null; 
    stats: { totalUsers: number; verifiedUsers: number; activeUsers: number };
    loading?: boolean;
  } = $props();

  const statItems = [
    { name: 'Total Users', value: stats.totalUsers, color: 'bg-primary-500' },
    { name: 'Verified Users', value: stats.verifiedUsers, color: 'bg-success-500' },
    { name: 'Active Users', value: stats.activeUsers, color: 'bg-warning-500' }
  ];
</script>

{#each statItems as statItem, i}
  <BaseCard classes={loading || !tag ? 'opacity-50' : ''}>
    {#snippet header()}
      <h3 class="text-sm opacity-70">{statItem.name}</h3>
    {/snippet}
    
    {#snippet children()}
      <p class="text-2xl font-bold mb-2">{statItem.value}</p>
      <div class="h-1 w-full bg-surface-200-800 rounded-full overflow-hidden">
        <div 
          class="h-full {statItem.color}" 
          style="width: {statItem.name === 'Total Users' ? 100 : (statItem.value / stats.totalUsers * 100)}%"
        ></div>
      </div>
    {/snippet}
  </BaseCard>
{/each} 