<script lang="ts">
	import { Navigation } from '@skeletonlabs/skeleton-svelte';
	import {
		Home,
		LayoutDashboard,
		Orbit,
		SquarePen,
		User,
		ShieldMinus,
		Github,
		Menu,
		MessageCircleQuestion
	} from 'lucide-svelte';
	import { GITHUB_URL } from '$lib/settings';
	import { profileLink } from '$lib/stores/authUserData';

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
			<Navigation.Tile id="/tag" href="/tag" labelExpanded="Tags" label="" selected={currentPath === '/tag'} labelClasses={currentPath === '/tag' ? 'text-primary-600-300' : ''}>
				<Orbit class={currentPath === '/tag' ? 'text-primary-600-300' : ''} />
			</Navigation.Tile>
			<Navigation.Tile id="/new/tag" href="/new/tag" labelExpanded="Create Tag" label="" selected={currentPath === '/new/tag'} labelClasses={currentPath === '/new/tag' ? 'text-primary-600-300' : ''}>
				<SquarePen class={currentPath === '/new/tag' ? 'text-primary-600-300' : ''} />
			</Navigation.Tile>
			<Navigation.Tile id={$profileLink} href={$profileLink} labelExpanded="Profile" label="" selected={currentPath === $profileLink} labelClasses={currentPath === $profileLink ? 'text-primary-600-300' : ''}>
				<User class={currentPath === $profileLink ? 'text-primary-600-300' : ''} />
			</Navigation.Tile>
			<!-- Hidden: Separator, New User and Admin pages are still accessible via direct URLs but not shown in navigation
			<div
				class="my-2 h-px mx-auto bg-surface-300 opacity-50"
				class:w-[92%]={isExpanded}
				class:w-[79%]={!isExpanded}
			></div>
			<Navigation.Tile id="/new/user" href="/new/user" labelExpanded="New User" label="" selected={currentPath === '/new/user'} labelClasses={currentPath === '/new/user' ? 'text-primary-600-300' : ''}>
				<MessageCircleQuestion class={currentPath === '/new/user' ? 'text-primary-600-300' : ''} />
			</Navigation.Tile>
			<Navigation.Tile id="/admin" href="/admin" labelExpanded="Admin" label="" selected={currentPath === '/admin'} labelClasses={currentPath === '/admin' ? 'text-primary-600-300' : ''}>
				<ShieldMinus class={currentPath === '/admin' ? 'text-primary-600-300' : ''} />
			</Navigation.Tile>
			-->
		{/snippet}
		{#snippet footer()}
			<Navigation.Tile id="github" labelExpanded="GitHub" label="" href={GITHUB_URL} target="_blank"><Github /></Navigation.Tile>
		{/snippet}
	</Navigation.Rail>
</aside>
