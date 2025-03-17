<script lang="ts">
	import { onMount } from 'svelte';
<<<<<<< HEAD
	import { authSubscribe, signIn, signOut, type User } from '@junobuild/core';
	import { goto } from '$app/navigation';

	let user: User | null = null;
	let error = '';

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

	onMount(() => {
		const sub = authSubscribe((state) => {
			user = state;
			
			// Redirect to admin when logged in
			if (user !== null) {
				goto('/admin');
			}
		});

		return () => {
			sub();
		};
=======
	import { initJuno } from '$lib/juno';
	
	let initialized = false;
	let error: string | null = null;
	
	onMount(async () => {
		try {
			await initJuno();
			initialized = true;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to initialize Juno';
		}
>>>>>>> 8aff74eb39f3f8fe5ac52a9271bf2d11291fa864
	});
</script>

<div class="container mx-auto p-4">
<<<<<<< HEAD
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
=======
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
		<p class="text-lg">Your decentralized reputation management system</p>
	{/if}
>>>>>>> 8aff74eb39f3f8fe5ac52a9271bf2d11291fa864
</div>

<style>
	/* Add any component-specific styles here */
</style>
