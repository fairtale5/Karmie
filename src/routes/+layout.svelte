<script lang="ts">
	import { onMount, type Snippet } from 'svelte';
	import { initSatellite } from '@junobuild/core';
	import Footer from '$lib/components/Footer.svelte';
	import Background from '$lib/components/Background.svelte';
	import '../app.css';

	interface Props {
		children?: Snippet;
	}

	let { children }: Props = $props();

	onMount(
		async () =>
			await initSatellite({
				workers: {
					auth: true
				}
			})
	);
</script>

<div class="relative isolate min-h-[100dvh]">
	<main
		class="mx-auto max-w-(--breakpoint-2xl) px-8 py-16 md:px-24 [@media(min-height:800px)]:min-h-[calc(100dvh-128px)]"
	>
		<h1 class="text-5xl font-extrabold md:pt-16 md:text-6xl dark:text-white">Welcome to Juno</h1>

		<div class="mt-8 grid w-full max-w-2xl grid-cols-2 gap-8">
			{@render children?.()}
		</div>
	</main>

	<Footer />

	<Background />
</div>
