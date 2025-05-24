<script lang="ts">
  import { Plus, Mail, BarChart, Vote } from 'lucide-svelte';
  import { slide } from 'svelte/transition';
  import { fade } from 'svelte/transition';

  const quickActions = [
    { name: 'Create Tag', icon: Plus },
    { name: 'Invite User', icon: Mail },
    { name: 'View Reports', icon: BarChart },
    { name: 'Vote', icon: Vote }
  ];

  let activeAction: string | null = null;

  function handleActionClick(action: string) {
    activeAction = activeAction === action ? null : action;
  }
</script>

<div class="flex flex-col">
  <!-- Buttons Container -->
  <div class="card shadow bg-surface-100-900 border border-surface-200-800 p-3">
    <div class="grid grid-cols-4 gap-2">
      {#each quickActions as action}
        <button 
          class="btn preset-tonal-primary flex flex-col items-center p-2 transition-all duration-200"
          class:ring-2={activeAction === action.name}
          class:ring-primary-500={activeAction === action.name}
          on:click={() => handleActionClick(action.name)}
        >
          <svelte:component this={action.icon} size={24} class="mb-1" />
          <span class="text-xs">{action.name}</span>
        </button>
      {/each}
    </div>
  </div>

  <!-- Expandable Content Area -->
  {#if activeAction}
    <div 
      class="px-2"
      transition:slide={{ duration: 200 }}
    >
      <div class="bg-surface-100-900/95 backdrop-blur-sm rounded-b-lg shadow-lg">
        <div class="p-4" transition:fade={{ duration: 150 }}>
          {#if activeAction === 'Vote'}
            <div class="text-sm">
              Vote content goes here
            </div>
          {:else if activeAction === 'Create Tag'}
            <div class="text-sm">
              Create Tag content goes here
            </div>
          {:else if activeAction === 'Invite User'}
            <div class="text-sm">
              Invite User content goes here
            </div>
          {:else if activeAction === 'View Reports'}
            <div class="text-sm">
              View Reports content goes here
            </div>
          {/if}
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  /* Ensure the expandable content appears above other elements */
  .relative {
    z-index: 10;
  }
</style> 