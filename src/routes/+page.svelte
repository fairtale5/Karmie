<script lang="ts">
	import { junoStatus } from '$lib/stores/juno';
</script>

<div class="container mx-auto p-4">
	{#if $junoStatus.initialized}
		<div class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded relative mb-4" role="alert">
			<p>✓ Connected to Juno satellite</p>
		</div>
	{/if}

	<h1 class="text-4xl font-bold mb-4">Welcome to Reputator</h1>
	<p class="text-lg">Your decentralized reputation management system</p>
</div>

<style>
	code {
		font-family: monospace;
	}
</style>
