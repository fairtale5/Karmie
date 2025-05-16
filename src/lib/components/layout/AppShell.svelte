<script lang="ts">
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
	// Placeholder for theme toggle (replace with your actual logic)
	let checked = false;
	// Placeholder for authUser (replace with your actual store/logic)
	let authUser: any = null;
	export let title = 'Page Title';
</script>

<div class="grid h-screen grid-cols-1 md:grid-cols-[auto_1fr]">
	<!-- Desktop Sidebar (left, full height) -->
	<aside class="hidden bg-yellow-500 p-4 md:row-span-3 md:block">
		<Navigation.Rail>
			{#snippet header()}
				<Navigation.Tile href="#" title="Menu"><Menu /></Navigation.Tile>
			{/snippet}
			{#snippet tiles()}
				<Navigation.Tile label="Home" href="/"><Home /></Navigation.Tile>
				<Navigation.Tile label="Dashboard" href="/dashboard"><LayoutDashboard /></Navigation.Tile>
				<Navigation.Tile label="Tags" href="/tag"><Orbit /></Navigation.Tile>
				<Navigation.Tile label="Create Tag" href="/tag/new"><SquarePen /></Navigation.Tile>
				<Navigation.Tile label="Users" href="/user"><UserRoundSearch /></Navigation.Tile>
				<Navigation.Tile label="Profile" href="/user/me"><User /></Navigation.Tile>
				<Navigation.Tile label="Admin" href="/admin"><ShieldMinus /></Navigation.Tile>
			{/snippet}
			{#snippet footer()}
				<Navigation.Tile label="GitHub" href="https://github.com/your-repo" target="_blank"
					><Github /></Navigation.Tile
				>
			{/snippet}
		</Navigation.Rail>
	</aside>
	<!-- Content Area: header, main, footer -->
	<div class="sticky top-0 z-10 flex h-full w-full flex-col">
		<header class="flex flex-shrink-0 items-center bg-red-500 p-4">
			<!-- Title: takes all available space -->
			<div class="flex-1 truncate text-xl font-bold">{title}</div>
			<!-- Actions: tight row, right-aligned -->
			<div class="flex items-center gap-2">
				<!-- Notification Bell -->
				<button class="btn btn-ghost rounded-full p-2" aria-label="Notifications">
					<Bell class="h-6 w-6" />
				</button>
				<!-- Theme Toggle (replace with your Switch logic) -->
				<button
					class="btn btn-ghost rounded-full p-2"
					aria-label="Toggle theme"
					on:click={() => (checked = !checked)}
				>
					{#if checked}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="h-6 w-6"
							fill="none"
							viewBox="0 0 24 24"
							stroke="currentColor"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M12 3v1m0 16v1m8.66-13.66l-.71.71M4.05 19.07l-.71.71M21 12h-1M4 12H3m16.66 6.66l-.71-.71M4.05 4.93l-.71-.71M16 12a4 4 0 11-8 0 4 4 0 018 0z"
							/></svg
						>
					{:else}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="h-6 w-6"
							fill="none"
							viewBox="0 0 24 24"
							stroke="currentColor"
							><path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M21 12.79A9 9 0 1111.21 3a7 7 0 009.79 9.79z"
							/></svg
						>
					{/if}
				</button>
				<!-- Login or Avatar -->
				{#if authUser}
					<button
						class="border-primary-500 flex h-10 w-10 items-center justify-center rounded-full border-2 p-0"
						aria-label="User profile"
					>
						<Avatar
							src={authUser.avatarUrl ?? undefined}
							name={authUser.name ?? 'User'}
							size="sm"
						/>
					</button>
				{:else}
					<button
						class="btn btn-ghost flex h-10 items-center justify-center rounded-full px-4 py-2"
						aria-label="Login"
					>
						Login
					</button>
				{/if}
			</div>
		</header>
		<main class="min-h-0 flex-1 space-y-4 overflow-auto bg-green-500 p-4 pb-[64px] md:pb-0">
			<slot />
		</main>
		<footer class="flex-shrink-0 bg-blue-500 p-4">
			<slot name="footer">(footer)</slot>
		</footer>
	</div>
	<!-- Mobile Bottom Bar Sidebar -->
	<aside
		class="fixed right-0 bottom-0 left-0 z-50 block w-full bg-yellow-500 p-4 md:hidden"
		style="height:56px;"
	>
		<nav class="flex justify-between">
			<a href="/" aria-label="Home"><Home class="h-6 w-6" /></a>
			<a href="/dashboard" aria-label="Dashboard"><LayoutDashboard class="h-6 w-6" /></a>
			<a href="/tag" aria-label="Tags"><Orbit class="h-6 w-6" /></a>
			<a href="/tag/new" aria-label="Create Tag"><SquarePen class="h-6 w-6" /></a>
			<a href="/user" aria-label="Users"><UserRoundSearch class="h-6 w-6" /></a>
			<a href="/user/me" aria-label="Profile"><User class="h-6 w-6" /></a>
			<a href="/admin" aria-label="Admin"><ShieldMinus class="h-6 w-6" /></a>
			<a href="https://github.com/your-repo" target="_blank" aria-label="GitHub"
				><Github class="h-6 w-6" /></a
			>
		</nav>
	</aside>
</div>
