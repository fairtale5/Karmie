<script lang="ts">
	import Header from '$lib/components/Header.svelte';
	import {
		Home,
		LayoutDashboard,
		Orbit,
		SquarePen,
		UserRoundSearch,
		User,
		ShieldMinus,
		Github,
		Bell,
		Menu
	} from 'lucide-svelte';
	import { Navigation } from '@skeletonlabs/skeleton-svelte';
	import { Avatar } from '@skeletonlabs/skeleton-svelte';
	import { page } from '$app/stores';
	// Placeholder for theme toggle (replace with your actual logic)
	let checked = false;
	// Placeholder for authUser (replace with your actual store/logic)
	let authUser: any = null;
	export let title = 'Page Title';
	let isExpanded = false;
	function toggleExpanded() {
		isExpanded = !isExpanded;
	}
	$: currentPath = $page.url.pathname;
</script>

<div class="grid h-screen grid-cols-1 md:grid-cols-[auto_1fr]">
	<!-- Desktop Sidebar (left, full height) -->
	<aside class="hidden md:block h-screen bg-transparent">
		<Navigation.Rail expanded={isExpanded} value={currentPath}>
			{#snippet header()}
				<Navigation.Tile id="menu" labelExpanded="Menu" onclick={toggleExpanded} title="Toggle Menu Width"><Menu /></Navigation.Tile>
			{/snippet}
			{#snippet tiles()}
				<Navigation.Tile id="/" href="/" labelExpanded="Home" label="" selected={currentPath === '/'} labelClasses={currentPath === '/' ? 'text-primary-600-300' : ''}>
					<Home class={currentPath === '/' ? 'text-primary-600-300' : ''} />
				</Navigation.Tile>
				<Navigation.Tile id="/tags-hub" href="/tags-hub" labelExpanded="Tags Hub" label="" selected={currentPath === '/tags-hub'} labelClasses={currentPath === '/tags-hub' ? 'text-primary-600-300' : ''}>
					<Orbit class={currentPath === '/tags-hub' ? 'text-primary-600-300' : ''} />
				</Navigation.Tile>
				<Navigation.Tile id="/admin" href="/admin" labelExpanded="Admin" label="" selected={currentPath === '/admin'} labelClasses={currentPath === '/admin' ? 'text-primary-600-300' : ''}>
					<ShieldMinus class={currentPath === '/admin' ? 'text-primary-600-300' : ''} />
				</Navigation.Tile>
				<Navigation.Tile id="/onboarding" href="/onboarding" labelExpanded="Onboarding" label="" selected={currentPath === '/onboarding'} labelClasses={currentPath === '/onboarding' ? 'text-primary-600-300' : ''}>
					<UserRoundSearch class={currentPath === '/onboarding' ? 'text-primary-600-300' : ''} />
				</Navigation.Tile>
				<Navigation.Tile id="/profile" href="/profile" labelExpanded="Profile" label="" selected={currentPath === '/profile'} labelClasses={currentPath === '/profile' ? 'text-primary-600-300' : ''}>
					<User class={currentPath === '/profile' ? 'text-primary-600-300' : ''} />
				</Navigation.Tile>
				<!-- Future/placeholder menu items -->
				<Navigation.Tile id="/dashboard" href="/dashboard" labelExpanded="Dashboard" label="" selected={currentPath === '/dashboard'} labelClasses={currentPath === '/dashboard' ? 'text-primary-600-300' : ''}>
					<LayoutDashboard class={currentPath === '/dashboard' ? 'text-primary-600-300' : ''} />
				</Navigation.Tile>
				<Navigation.Tile id="/tag/new" href="/tag/new" labelExpanded="Create Tag" label="" selected={currentPath === '/tag/new'} labelClasses={currentPath === '/tag/new' ? 'text-primary-600-300' : ''}>
					<SquarePen class={currentPath === '/tag/new' ? 'text-primary-600-300' : ''} />
				</Navigation.Tile>
				<Navigation.Tile id="/user" href="/user" labelExpanded="Users" label="" selected={currentPath === '/user'} labelClasses={currentPath === '/user' ? 'text-primary-600-300' : ''}>
					<UserRoundSearch class={currentPath === '/user' ? 'text-primary-600-300' : ''} />
				</Navigation.Tile>
				<Navigation.Tile id="/user/me" href="/user/me" labelExpanded="Profile (Me)" label="" selected={currentPath === '/user/me'} labelClasses={currentPath === '/user/me' ? 'text-primary-600-300' : ''}>
					<User class={currentPath === '/user/me' ? 'text-primary-600-300' : ''} />
				</Navigation.Tile>
			{/snippet}
			{#snippet footer()}
				<Navigation.Tile id="github" labelExpanded="GitHub" label="" href="https://github.com/your-repo" target="_blank"><Github /></Navigation.Tile>
			{/snippet}
		</Navigation.Rail>
	</aside>
	<!-- Content Area: header, main, footer -->
	<div class="flex flex-col h-screen w-full">
		<div class="sticky top-0 z-10">
			<Header />
		</div>
		<main class="flex-1 overflow-y-auto h-screen p-4 pb-[64px] md:pb-0">
			<slot />
		</main>
		<footer class="flex-shrink-0 bg-surface-800 p-4">
			<slot name="footer">(footer)</slot>
		</footer>
	</div>
	<!-- Mobile Bottom Bar Sidebar -->
	<aside class="fixed right-0 bottom-0 left-0 z-50 block w-full bg-transparent p-4 md:hidden" style="height:56px;">
		<Navigation.Bar>
			<Navigation.Tile label="Home" href="/"><Home class="h-6 w-6" /></Navigation.Tile>
			<Navigation.Tile label="Dashboard" href="/dashboard"><LayoutDashboard class="h-6 w-6" /></Navigation.Tile>
			<Navigation.Tile label="Tags" href="/tag"><Orbit class="h-6 w-6" /></Navigation.Tile>
			<Navigation.Tile label="Create Tag" href="/tag/new"><SquarePen class="h-6 w-6" /></Navigation.Tile>
			<Navigation.Tile label="Users" href="/user"><UserRoundSearch class="h-6 w-6" /></Navigation.Tile>
			<Navigation.Tile label="Profile" href="/user/me"><User class="h-6 w-6" /></Navigation.Tile>
			<Navigation.Tile label="Admin" href="/admin"><ShieldMinus class="h-6 w-6" /></Navigation.Tile>
			<Navigation.Tile label="GitHub" href="https://github.com/your-repo" target="_blank"><Github class="h-6 w-6" /></Navigation.Tile>
		</Navigation.Bar>
	</aside>
</div>

<!--
Tasklist:
1. Header: Use real Header component (done)
2. Sidebar: Transparent, expanded/collapsed, only as tall as needed (done)
3. Main area: Transparent, body gradient visible (done)
4. Mobile bar: Transparent, uses Navigation.Bar with href for routing (done)
5. Sidebar height/scroll: done, default collapsed
-->
