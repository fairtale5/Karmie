<script lang="ts">
	import { onMount } from 'svelte';
	import { initSatelliteConnection } from '$lib/junoInit';
	import { junoStatus } from '$lib/stores/junoStore';

	onMount(async () => {
		await initSatelliteConnection();
	});
</script>

{#if $junoStatus.error}
	<div class="container mx-auto p-4">
		<div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative mb-4" role="alert">
			<strong class="font-bold">Connection Error: </strong>
			<span class="block sm:inline">{$junoStatus.error}</span>
			<p class="mt-2 text-sm">Please make sure the Juno development server is running with <code class="bg-red-50 px-1">juno dev start</code></p>
		</div>
	</div>
{:else if !$junoStatus.initialized}
	<div class="container mx-auto p-4">
		<div class="bg-blue-100 border border-blue-400 text-blue-700 px-4 py-3 rounded relative mb-4" role="alert">
			<p>Connecting to satellite...</p>
		</div>
	</div>
{/if}

<slot />

<style>
	code {
		font-family: monospace;
	}
</style>
