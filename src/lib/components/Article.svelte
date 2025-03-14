<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		href: string;
		ariaLabel: string;
		title: string;
		children: Snippet;
	}

	let { href, ariaLabel, title, children }: Props = $props();
</script>

<div>
</div>
