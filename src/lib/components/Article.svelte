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

<a
	{href}
	rel="noreferrer noopener"
	target="_blank"
	aria-label={ariaLabel}
	class="group dark:border-lavender-blue-500 hover:bg-lavender-blue-200 active:bg-lavender-blue-400 flex flex-col rounded-sm border-[3px] border-black bg-white px-4 py-3 shadow-[8px_8px_0px_rgba(0,0,0,1)] transition-all active:translate-x-[8px] active:translate-y-[8px] active:shadow-none dark:bg-black dark:text-white dark:shadow-[8px_8px_0px_#7888FF] dark:hover:border-white dark:hover:bg-black dark:hover:shadow-[8px_8px_0px_#fff] dark:active:bg-black"
>
	<h4
		class="dark:text-lavender-blue-500 mb-1 font-extrabold break-words sm:text-lg dark:group-hover:text-white"
	>
		{title}
	</h4>

	<p class="mb-2">{@render children()}</p>
</a>
