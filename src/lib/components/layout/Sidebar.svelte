<script lang="ts">
	import { Navigation } from '@skeletonlabs/skeleton-svelte';
	import {
		Home,
		LayoutDashboard,
		Orbit,
		SquarePen,
		UserRoundSearch,
		User,
		ShieldMinus,
		Github,
		Menu,
		MessageCircleQuestion
	} from 'lucide-svelte';

	/**
	 * Sidebar component for desktop navigation.
	 * @param isExpanded - Whether the sidebar is expanded
	 * @param toggleExpanded - Function to toggle expansion
	 * @param currentPath - The current route path
	 */
	export let isExpanded: boolean;
	export let toggleExpanded: () => void;
	export let currentPath: string;
</script>

<aside class="hidden md:block h-screen bg-transparent">
	<Navigation.Rail 
		expanded={isExpanded} 
		value={currentPath} 
		width="w-18"
		widthExpanded="w-64"
		headerBase={isExpanded ? 'pt-1' : 'pt-1'} 
		padding={isExpanded ? 'p-1 pl-2 pr-2 ' : 'p-1 pl-3 pr-3 '} 
		tilesGap="gap-2"
		tilesItems="items-center"
		tilesClasses="h-[120px]"
	>
		{#snippet header()}
			<Navigation.Tile id="menu" labelExpanded="Menu" onclick={toggleExpanded} title="Toggle Menu Width"><Menu /></Navigation.Tile>
		{/snippet}
		{#snippet tiles()}
			<Navigation.Tile id="/" href="/" labelExpanded="Home" label="" selected={currentPath === '/'} labelClasses={currentPath === '/' ? 'text-primary-600-300' : ''}>
				<Home class={currentPath === '/' ? 'text-primary-600-300' : ''} />
			</Navigation.Tile>
			<Navigation.Tile id="/dashboard" href="/dashboard" labelExpanded="Dashboard" label="" selected={currentPath === '/dashboard'} labelClasses={currentPath === '/dashboard' ? 'text-primary-600-300' : ''}>
				<LayoutDashboard class={currentPath === '/dashboard' ? 'text-primary-600-300' : ''} />
			</Navigation.Tile>
			<Navigation.Tile id="/tags-hub" href="/tags-hub" labelExpanded="Tags Hub" label="" selected={currentPath === '/tags-hub'} labelClasses={currentPath === '/tags-hub' ? 'text-primary-600-300' : ''}>
				<Orbit class={currentPath === '/tags-hub' ? 'text-primary-600-300' : ''} />
			</Navigation.Tile>
			<Navigation.Tile id="/tag/new" href="/tag/new" labelExpanded="Create Tag" label="" selected={currentPath === '/tag/new'} labelClasses={currentPath === '/tag/new' ? 'text-primary-600-300' : ''}>
				<SquarePen class={currentPath === '/tag/new' ? 'text-primary-600-300' : ''} />
			</Navigation.Tile>
			<Navigation.Tile id="/user/me" href="/user/me" labelExpanded="Profile" label="" selected={currentPath === '/user/me'} labelClasses={currentPath === '/user/me' ? 'text-primary-600-300' : ''}>
				<User class={currentPath === '/user/me' ? 'text-primary-600-300' : ''} />
			</Navigation.Tile>
			<Navigation.Tile id="/user" href="/user" labelExpanded="Users" label="" selected={currentPath === '/user'} labelClasses={currentPath === '/user' ? 'text-primary-600-300' : ''}>
				<UserRoundSearch class={currentPath === '/user' ? 'text-primary-600-300' : ''} />
			</Navigation.Tile>
			<!-- Separator -->
			<div
				class="my-2 h-px mx-auto bg-surface-300 opacity-50"
				class:w-[92%]={isExpanded}
				class:w-[79%]={!isExpanded}
			></div>
			<Navigation.Tile id="/onboarding" href="/onboarding" labelExpanded="Onboarding" label="" selected={currentPath === '/onboarding'} labelClasses={currentPath === '/onboarding' ? 'text-primary-600-300' : ''}>
				<MessageCircleQuestion class={currentPath === '/onboarding' ? 'text-primary-600-300' : ''} />
			</Navigation.Tile>
			<Navigation.Tile id="/admin" href="/admin" labelExpanded="Admin" label="" selected={currentPath === '/admin'} labelClasses={currentPath === '/admin' ? 'text-primary-600-300' : ''}>
				<ShieldMinus class={currentPath === '/admin' ? 'text-primary-600-300' : ''} />
			</Navigation.Tile>
		{/snippet}
		{#snippet footer()}
			<Navigation.Tile id="github" labelExpanded="GitHub" label="" href="https://github.com/your-repo" target="_blank"><Github /></Navigation.Tile>
		{/snippet}
	</Navigation.Rail>
</aside> 