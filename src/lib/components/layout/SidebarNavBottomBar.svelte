<script lang="ts">
import { Navigation } from '@skeletonlabs/skeleton-svelte';
import {
	Home,
	LayoutDashboard,
	Orbit,
	User,
	Plus
} from 'lucide-svelte';
import { profileLink } from '$lib/stores/authUserData';
</script>

<aside class="fixed right-0 bottom-0 left-0 z-50 w-full bg-transparent" style="height:80px;">
	<Navigation.Bar>
		<Navigation.Tile label="Home" href="/"><Home class="h-6 w-6" /></Navigation.Tile>
		<Navigation.Tile label="Dashboard" href="/dashboard"><LayoutDashboard class="h-6 w-6" /></Navigation.Tile>
		<Navigation.Tile label="Tags" href="/tag"><Orbit class="h-6 w-6" /></Navigation.Tile>
		<Navigation.Tile label="New Tag" href="/new/tag"><Plus class="h-6 w-6" /></Navigation.Tile>
		<Navigation.Tile label="Profile" href={$profileLink}><User class="h-6 w-6" /></Navigation.Tile>
	</Navigation.Bar>
</aside> 