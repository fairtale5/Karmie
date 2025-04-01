<script lang="ts">
	import { onMount } from 'svelte';
	import { initJuno } from '$lib/juno';
	import Header from '$lib/Header.svelte';

	onMount(async () => {
		await initJuno();
	});
</script>

{#if import.meta.env.DEV}
	<div class="container mx-auto p-4">
		<div class="bg-yellow-100 border border-yellow-400 text-yellow-700 px-4 py-3 rounded relative mb-4" role="alert">
			<strong class="font-bold">Local Development Mode</strong>
		</div>
	</div>
{/if}

<Header />
<main class="container mx-auto px-4 py-8">
    <slot />
</main>
