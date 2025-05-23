<script lang="ts">
	import Header from '$lib/components/layout/Header.svelte';
	import SidebarNavLeftRail from '$lib/components/layout/SidebarNavLeftRail.svelte';
	import SidebarNavBottomBar from '$lib/components/layout/SidebarNavBottomBar.svelte';
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
	import Footer from '$lib/components/layout/Footer.svelte';
	import FooterDeadzone from '$lib/components/layout/FooterDeadzone.svelte';
	// Placeholder for theme toggle (replace with your actual logic)
	let checked = false;
	// Placeholder for authUser (replace with your actual store/logic)
	let authUser: any = null;
	let isExpanded = true;

	// On mount, check localStorage for sidebar state
	if (typeof window !== 'undefined') {
		const stored = localStorage.getItem('sidebarExpanded');
		if (stored !== null) {
			isExpanded = stored === 'true';
		}
	}

	function toggleExpanded() {
		isExpanded = !isExpanded;
		if (typeof window !== 'undefined') {
			localStorage.setItem('sidebarExpanded', isExpanded ? 'true' : 'false');
		}
	}
	$: currentPath = $page.url.pathname;
</script>

<div class="grid grid-cols-1 md:grid-cols-[auto_1fr]">
	<!-- Desktop Sidebar (left, full height) -->
	<aside class="hidden md:block bg-transparent sticky top-0 h-screen overflow-y-auto" style="width: {isExpanded ? '256px' : '72px'};">
		<SidebarNavLeftRail {isExpanded} {toggleExpanded} {currentPath} />
	</aside>
	<!-- Content Area: header, main, footer -->
	<div class="flex flex-col min-h-screen w-full">
		<div class="sticky top-0 z-10">
			<Header />
		</div>
		<main class="flex-1 overflow-y-auto p-4 md:pb-0">
			<slot />
		</main>
		<footer class="flex-shrink-0 p-4">
			<Footer />
		</footer>
		<FooterDeadzone />
	</div>
	<!-- Mobile Bottom Bar Sidebar -->
	<div class="block md:hidden">
		<SidebarNavBottomBar />
	</div>
</div>

<!--
Tasklist:
1. Header: Use real Header component (done)
2. Sidebar: Transparent, expanded/collapsed, only as tall as needed (done)
3. Main area: Transparent, body gradient visible (done)
4. Mobile bar: Transparent, uses Navigation.Bar with href for routing (done)
5. Sidebar height/scroll: done, default collapsed
-->
