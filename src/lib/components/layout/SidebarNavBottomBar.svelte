<script lang="ts">
import { Navigation } from '@skeletonlabs/skeleton-svelte';
import {
	Home,
	LayoutDashboard,
	Orbit,
	UserRoundSearch,
	User
} from 'lucide-svelte';
import { authUserDoc } from '$lib/stores/authUserDoc';

$: profilePath = $authUserDoc ? `/u/${$authUserDoc.data.user_handle}` : '/u/me';
</script>

<aside class="fixed right-0 bottom-0 left-0 z-50 w-full bg-transparent" style="height:80px;">
	<Navigation.Bar>
		<Navigation.Tile label="Home" href="/"><Home class="h-6 w-6" /></Navigation.Tile>
		<Navigation.Tile label="Dashboard" href="/dashboard"><LayoutDashboard class="h-6 w-6" /></Navigation.Tile>
		<Navigation.Tile label="Tags" href="/tags"><Orbit class="h-6 w-6" /></Navigation.Tile>
		<Navigation.Tile label="Profile" href={profilePath}><User class="h-6 w-6" /></Navigation.Tile>
		<Navigation.Tile label="Users" href="/newuser/demo_user"><UserRoundSearch class="h-6 w-6" /></Navigation.Tile>
	</Navigation.Bar>
</aside> 