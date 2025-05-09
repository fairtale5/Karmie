<script lang="ts">
	import { onMount } from 'svelte';
	import { authSubscribe, signIn, signOut, type User } from '@junobuild/core';
	import { goto } from '$app/navigation';
	
	let initialized = false;
	let user: User | null = null;
	let error: string | null = null;
	let unsubscribe: (() => void) | undefined;
	
	onMount(() => {
		// Set up auth subscription only (Juno is now initialized in layout)
				unsubscribe = authSubscribe((state) => {
					user = state;
					
					// Redirect to admin when logged in
					if (user !== null) {
						goto('/admin');
					}
			});

		// Cleanup function
		return (): void => {
			if (unsubscribe) {
				unsubscribe();
			}
		};
	});

	async function login() {
		try {
			error = '';
			await signIn();
		} catch (e) {
			console.error('Login failed:', e);
			error = e instanceof Error ? e.message : 'Login failed';
		}
	}

	async function logout() {
		try {
			error = '';
			await signOut();
		} catch (e) {
			console.error('Logout failed:', e);
			error = e instanceof Error ? e.message : 'Logout failed';
		}
	}
</script>

<div class="container mx-auto p-4">
	{#if error}
		<div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
			<strong class="font-bold">Error:</strong>
			<span class="block sm:inline">{error}</span>
		</div>
	{:else if !initialized}
		<div class="text-center">
			<p>Initializing Juno...</p>
		</div>
	{:else}
		<h1 class="text-4xl font-bold mb-4">Welcome to Reputator</h1>
		<p class="text-lg mb-8">Your decentralized reputation management system</p>

		<div class="space-y-4">
			{#if !user}
				<button
					on:click={login}
					class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
				>
					Login with Internet Identity
				</button>
				{#if error}
					<div class="text-red-500">{error}</div>
				{/if}
			{:else}
				<div class="space-y-2">
					<p>You are logged in. Redirecting to admin interface...</p>
					<button
						on:click={logout}
						class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded"
					>
						Logout
					</button>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	/* Add any component-specific styles here */
</style>
