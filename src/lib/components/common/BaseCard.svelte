<script lang="ts">
  import { Construction, X } from 'lucide-svelte';
  import { Popover } from '@skeletonlabs/skeleton-svelte';
  
  const { classes = '', underConstruction = false, header, actions, children } = $props<{
    classes?: string;
    underConstruction?: boolean;
    header?: import('svelte').Snippet;
    actions?: import('svelte').Snippet;
    children?: import('svelte').Snippet;
  }>();
  
  let constructionPopoverOpen = $state(false);

  function closeConstructionPopover() {
    constructionPopoverOpen = false;
  }
</script>

<div class={`card shadow bg-surface-100-900 p-4 ${underConstruction ? 'preset-outlined-error-500' : 'border border-surface-200-800'} ${classes}`}>
  <!-- Standardized Header -->
  {#if header}
    <div class="flex justify-between items-start mb-4">
      <div class="flex items-center gap-2">
        {@render header()}
      </div>
      <div class="flex items-center gap-2">
        {#if actions}
          {@render actions()}
        {/if}
        {#if underConstruction}
          <Popover
            open={constructionPopoverOpen}
            onOpenChange={(e) => (constructionPopoverOpen = e.open)}
            positioning={{ placement: 'top', flip: true }}
            triggerBase="chip-icon preset-tonal-surface"
            contentBase="card bg-surface-200-800 p-4 space-y-4 max-w-[320px]"
            arrow
            arrowBackground="!bg-surface-200 dark:!bg-surface-800"
          >
            {#snippet trigger()}
              <Construction class="text-error-500" size={16}/>
            {/snippet}
            {#snippet content()}
              <header class="flex justify-between">
                <p class="font-bold">Under Construction</p>
                <button class="btn-icon hover:preset-tonal" onclick={closeConstructionPopover}>
                  <X class="w-4 h-4" />
                </button>
              </header>
              <article>
                <p class="opacity-60">
                  This feature is coming soon and currently does not use real data. It's a preview of the planned functionality.
                </p>
              </article>
            {/snippet}
          </Popover>
        {/if}
      </div>
    </div>
  {/if}
  
  <!-- Main Content -->
  <div>
    {#if children}
      {@render children()}
    {/if}
  </div>
</div> 