<script lang="ts">
  import { TriangleAlert } from 'lucide-svelte';
</script>

<div class="card preset-outlined-warning-500 grid grid-cols-1 items-center gap-4 p-4 lg:grid-cols-[auto_1fr_auto] my-4">
  <TriangleAlert class="text-warning-500" />
  <div>
    <p class="font-bold">Dashboard Under Development</p>
    <p class="text-xs opacity-60">This dashboard is not fully implemented yet and is currently displaying dummy data for demonstration purposes.</p>
  </div>
</div>