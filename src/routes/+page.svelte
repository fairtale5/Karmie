<script lang="ts">
	import { onMount } from 'svelte';
	import { initJuno } from '$lib/juno';
	
	let initialized = false;
	let error: string | null = null;
	
	onMount(async () => {
		try {
			await initJuno();
			initialized = true;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to initialize Juno';
		}
	});
</script>

<div class="container mx-auto p-4">
	{#if error}
		<div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
			<strong class="font-bold">Error:</strong>
			<span class="block sm:inline">{error}</span>
		</div>
	{:else if !initialized}
		<div class="text-center">
			<p>Initializing Juno...</p>
		</div>
	{:else}
		<h1 class="text-4xl font-bold mb-4">Welcome to Reputator</h1>
		<p class="text-lg">Your decentralized reputation management system</p>
	{/if}
</div>

<style>
	/* Add any component-specific styles here */
</style>
