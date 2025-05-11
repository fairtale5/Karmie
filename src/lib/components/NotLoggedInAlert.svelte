<script lang="ts">
  import { TriangleAlert } from 'lucide-svelte';
  import { authUser, authUserDoneInitializing } from '$lib/stores/authUser';
</script>

{#if $authUserDoneInitializing && !$authUser}
  <div class="card preset-outlined-warning-500 grid grid-cols-1 items-center gap-4 p-4 lg:grid-cols-[auto_1fr_auto] my-4">
    <TriangleAlert class="text-warning-500" />
    <div>
      <p class="font-bold">Warning</p>
      <p class="text-xs opacity-60">You are not logged in. You can browse, but you cannot make changes.</p>
    </div>
  </div>
{/if} 