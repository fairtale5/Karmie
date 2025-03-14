<script lang="ts">
	import { onMount } from 'svelte';
	import { initSatellite } from '@junobuild/core';

	onMount(
		async () =>
			await initSatellite({
				workers: {
					auth: true
				}
			})
	);
</script>

<main>
	<slot />
</main>
