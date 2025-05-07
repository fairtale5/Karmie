<script lang="ts">
	import { Switch } from '@skeletonlabs/skeleton-svelte';
	import { Sun, Moon } from 'lucide-svelte';
	import { onMount } from 'svelte';

	let checked = false;

	onMount(() => {

        // Handle theme dark vs light mode:
		// 1. Check if user has a stored preference ("last used on last visit")
		const stored = localStorage.getItem('mode');
		if (stored) {
			// 1.1 Use stored preference
			checked = stored === 'dark';
		} else {
			// 1.2 Use system preference
			const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
			checked = prefersDark;
			document.documentElement.setAttribute('data-mode', prefersDark ? 'dark' : 'light');
		}
	});

	function onCheckedChange(event: { checked: boolean }) {
		const mode = event.checked ? 'dark' : 'light';
		document.documentElement.setAttribute('data-mode', mode);
		localStorage.setItem('mode', mode);
		checked = event.checked;
	}
</script>

<svelte:head>
	<script>
		const stored = localStorage.getItem('mode');
		if (stored) {
			document.documentElement.setAttribute('data-mode', stored);
		} else {
			const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
			document.documentElement.setAttribute('data-mode', prefersDark ? 'dark' : 'light');
		}
	</script>
</svelte:head>

<header class="bg-surface-100-800-token border-b border-surface-300-700-token">
	<div class="container mx-auto p-4 flex justify-between items-center">
		<div class="flex items-center gap-8">
			<a href="/" class="text-2xl font-bold text-primary-700-300-token">Reputator</a>
			<nav class="hidden md:flex gap-6">
				<a href="/" class="text-surface-700-300-token hover:text-primary-500 transition-colors">Home</a>
				<a href="/admin" class="text-surface-700-300-token hover:text-primary-500 transition-colors">Admin</a>
				<a href="/profile" class="text-surface-700-300-token hover:text-primary-500 transition-colors">Profile</a>
			</nav>
		</div>
		<div class="flex items-center gap-4">
			<Switch 
				name="mode" 
				controlActive="bg-surface-200-700-token" 
				checked={checked} 
				{onCheckedChange}
			>
				{#snippet inactiveChild()}<Moon size={14} />{/snippet}
				{#snippet activeChild()}<Sun size={14} />{/snippet}
			</Switch>
		</div>
	</div>
</header> 