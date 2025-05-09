<script lang="ts">
	import { Switch } from '@skeletonlabs/skeleton-svelte';
	import { Sun, Moon } from 'lucide-svelte';
	import { onMount } from 'svelte';
	import { page } from '$app/stores';

	let checked = false;
	$: currentPath = $page.url.pathname;

	onMount(() => {
		// Handle theme dark vs light mode:
		// 1. Check if user has a stored preference ("last used on last visit")
		const stored = localStorage.getItem('mode');
		if (stored) {
			// 1.1 Use stored preference
			checked = stored === 'light';
			document.documentElement.setAttribute('data-mode', stored);
		} else {
			// 1.2 Use system preference
			const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
			checked = !prefersDark;
			document.documentElement.setAttribute('data-mode', prefersDark ? 'dark' : 'light');
		}
	});

	function onCheckedChange(event: { checked: boolean }) {
		const mode = event.checked ? 'light' : 'dark';
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

<header class="bg-[var(--color-surface-50-950)] border-b border-[color-mix(in oklab,var(--color-surface-500)20%,transparent)]">
	<div class="container mx-auto p-3 flex justify-between items-center">
		<div class="flex items-center gap-4">
			<a href="/" class="text-2xl font-bold text-[var(--color-primary-500)]">Reputator</a>
			<nav class="flex items-center gap-2">
				<a href="/" class="btn hover:preset-tonal" class:text-primary-700-300={currentPath === '/'}>Home</a>
				<a href="/reputations" class="btn hover:preset-tonal" class:text-primary-700-300={currentPath === '/reputations'}>Reputations</a>
				<a href="/admin" class="btn hover:preset-tonal" class:text-primary-700-300={currentPath === '/admin'}>Admin</a>
				<a href="/profile" class="btn hover:preset-tonal" class:text-primary-700-300={currentPath === '/profile'}>Profile</a>
			</nav>
		</div>
		<div class="flex items-center gap-4">
			<Switch 
				name="mode" 
				controlActive="bg-[var(--color-surface-200-800)]" 
				checked={checked} 
				{onCheckedChange}
			>
				{#snippet inactiveChild()}<Moon size={14} />{/snippet}
				{#snippet activeChild()}<Sun size={14} />{/snippet}
			</Switch>
			<a href="/login" class="btn preset-filled-primary-500">Login</a>
		</div>
	</div>
</header>
