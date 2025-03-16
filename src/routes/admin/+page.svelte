<script lang="ts">
	import { onMount } from 'svelte';
	import { nanoid } from 'nanoid';
	import { listDocs, setDoc, deleteDoc, type Doc, authSubscribe, type User, getDoc } from '@junobuild/core';
	import { goto } from '$app/navigation';

	// Form data for creating/updating users
	let newUser = {
		key: '',  // Optional - if provided, will update existing user
		handle: '',
		display_name: ''
	};

	// List of all users
	let users: Doc<{
		handle: string;
		display_name: string;
	}>[] = [];

	// Form data for creating votes
	let newVote = {
		key: '',
		author: '',  // User key of the vote author
		target: '',  // User key of the vote target
		is_positive: true,  // Default to positive vote
		tag: ''  // Tag key
	};

	// List of all votes
	let votes: Doc<{
		tag: string;
		target: string;
		is_positive: boolean;
		weight: number;
	}>[] = [];

	// Error message if something goes wrong
	let error = '';
	// Success message for feedback
	let success = '';
	// Current authenticated user
	let user: User | null = null;

	// Function to load all users from Juno collection
	async function loadUsers() {
		try {
			const result = await listDocs<{
				handle: string;
				display_name: string;
			}>({
				collection: 'users',
				filter: {
					order: {
						desc: true,
						field: 'created_at'
					}
				}
			});
			users = result.items;
		} catch (e) {
			console.error('Error loading users:', e);
			error = 'Failed to load users';
		}
	}

	// Function to create or update a user
	async function saveUser() {
		try {
			error = '';
			success = '';

			// Basic client-side validation
			if (!newUser.handle || !newUser.display_name) {
				error = 'Both handle and display name are required';
				return;
			}

			// If we're updating an existing user, we need to get their current version
			let version;
			if (newUser.key) {
				try {
					const existingDoc = await getDoc({
						collection: 'users',
						key: newUser.key
					});
					if (!existingDoc) {
						error = 'User not found';
						return;
					}
					version = existingDoc.version;
				} catch (e) {
					console.error('Error fetching existing user:', e);
					error = 'Failed to fetch existing user version';
					return;
				}
			}

			// Create or update user document
			await setDoc({
				collection: 'users',
				doc: {
					key: newUser.key || nanoid(), // Use existing key or generate new one
					data: {
						handle: newUser.handle,
						display_name: newUser.display_name
					},
					...(version && { version }) // Only include version if we're updating
				}
			});

			// Clear form and show success message
			newUser = {
				key: '',
				handle: '',
				display_name: ''
			};
			success = newUser.key ? 'User updated successfully!' : 'User created successfully!';

			// Reload the user list
			await loadUsers();
		} catch (e) {
			console.error('Error saving user:', e);
			error = e instanceof Error ? e.message : 'Failed to save user';
		}
	}

	// Function to delete a user
	async function deleteUser(key: string) {
		if (!confirm('Are you sure you want to delete this user?')) {
			return;
		}

		try {
			error = '';
			success = '';

			await deleteDoc({
				collection: 'users',
				doc: {
					key,
					data: {} // Add empty data object to satisfy type requirement
				}
			});

			success = 'User deleted successfully!';
			await loadUsers();
		} catch (e) {
			console.error('Error deleting user:', e);
			error = e instanceof Error ? e.message : 'Failed to delete user';
		}
	}

	// Function to load user data for editing
	function editUser(userDoc: Doc<{ handle: string; display_name: string }>) {
		newUser = {
			key: userDoc.key,
			handle: userDoc.data.handle,
			display_name: userDoc.data.display_name
		};
		// Scroll to form
		document.getElementById('userForm')?.scrollIntoView({ behavior: 'smooth' });
	}

	// Function to load all votes from Juno collection
	async function loadVotes() {
		try {
			const result = await listDocs<{
				tag: string;
				target: string;
				is_positive: boolean;
				weight: number;
			}>({
				collection: 'votes',
				filter: {
					order: {
						desc: true,
						field: 'created_at'
					}
				}
			});
			votes = result.items;
		} catch (e) {
			console.error('Error loading votes:', e);
			error = 'Failed to load votes';
		}
	}

	// Function to create a vote
	async function saveVote() {
		try {
			error = '';
			success = '';

			// Basic client-side validation
			if (!newVote.author || !newVote.target || !newVote.tag) {
				error = 'Author, target, and tag are required';
				return;
			}

			// Create vote document
			await setDoc({
				collection: 'votes',
				doc: {
					key: newVote.key || nanoid(),
					description: `author:${newVote.author},target:${newVote.target}`,
					data: {
						tag: newVote.tag,
						target: newVote.target,
						is_positive: newVote.is_positive,
						weight: 1  // Fixed weight for now
					}
				}
			});

			// Clear form and show success message
			newVote = {
				key: '',
				author: '',
				target: '',
				is_positive: true,
				tag: ''
			};
			success = 'Vote created successfully!';

			// Reload the vote list
			await loadVotes();
		} catch (e) {
			console.error('Error saving vote:', e);
			error = e instanceof Error ? e.message : 'Failed to save vote';
		}
	}

	// Function to delete a vote
	async function deleteVote(key: string) {
		if (!confirm('Are you sure you want to delete this vote?')) {
			return;
		}

		try {
			error = '';
			success = '';

			await deleteDoc({
				collection: 'votes',
				doc: {
					key,
					data: {}
				}
			});

			success = 'Vote deleted successfully!';
			await loadVotes();
		} catch (e) {
			console.error('Error deleting vote:', e);
			error = e instanceof Error ? e.message : 'Failed to delete vote';
		}
	}

	// Load both users and votes when the component mounts
	onMount(() => {
		// Subscribe to auth state
		const sub = authSubscribe((state) => {
			user = state;
			
			// If user is not logged in, redirect to home
			if (user === null) {
				goto('/');
			} else {
				// Load users and votes if authenticated
				loadUsers();
				loadVotes();
			}
		});

		// Cleanup subscription on component destroy
		return () => {
			sub();
		};
	});
</script>

{#if user}
	<div class="container mx-auto p-4">
		<h1 class="text-2xl mb-4">Admin Dashboard</h1>

		<!-- Create/Update User Form -->
		<div class="mb-8" id="userForm">
			<h2 class="text-xl mb-4">{newUser.key ? 'Update User' : 'Create New User'}</h2>
			<form on:submit|preventDefault={saveUser} class="space-y-4">
				{#if newUser.key}
					<div>
						<label for="key" class="block">User Key:</label>
						<input
							type="text"
							id="key"
							bind:value={newUser.key}
							class="border p-2 w-full bg-gray-100"
							readonly
						/>
					</div>
				{/if}

				<div>
					<label for="handle" class="block">Handle (username):</label>
					<input
						type="text"
						id="handle"
						bind:value={newUser.handle}
						class="border p-2 w-full"
						placeholder="e.g., john_doe"
					/>
				</div>

				<div>
					<label for="display_name" class="block">Display Name:</label>
					<input
						type="text"
						id="display_name"
						bind:value={newUser.display_name}
						class="border p-2 w-full"
						placeholder="e.g., John Doe"
					/>
				</div>

				<div class="flex gap-4">
					<button type="submit" class="bg-blue-500 text-white px-4 py-2 rounded">
						{newUser.key ? 'Update User' : 'Create User'}
					</button>
					{#if newUser.key}
						<button
							type="button"
							class="bg-gray-500 text-white px-4 py-2 rounded"
							on:click={() => {
								newUser = { key: '', handle: '', display_name: '' };
							}}
						>
							Cancel Edit
						</button>
					{/if}
				</div>
			</form>

			{#if error}
				<div class="text-red-500 mt-2">{error}</div>
			{/if}
			{#if success}
				<div class="text-green-500 mt-2">{success}</div>
			{/if}
		</div>

		<!-- Create Vote Form -->
		<div class="mb-8" id="voteForm">
			<h2 class="text-xl mb-4">Create New Vote</h2>
			<form on:submit|preventDefault={saveVote} class="space-y-4">
				<div>
					<label for="author" class="block">Author (User Key):</label>
					<select
						id="author"
						bind:value={newVote.author}
						class="border p-2 w-full"
					>
						<option value="">Select Author</option>
						{#each users as user}
							<option value={user.key}>
								{user.data.display_name} ({user.data.handle})
							</option>
						{/each}
					</select>
				</div>

				<div>
					<label for="target" class="block">Target (User Key):</label>
					<select
						id="target"
						bind:value={newVote.target}
						class="border p-2 w-full"
					>
						<option value="">Select Target</option>
						{#each users as user}
							<option value={user.key}>
								{user.data.display_name} ({user.data.handle})
							</option>
						{/each}
					</select>
				</div>

				<div>
					<label for="tag" class="block">Tag:</label>
					<input
						type="text"
						id="tag"
						bind:value={newVote.tag}
						class="border p-2 w-full"
						placeholder="e.g., tag_key"
					/>
				</div>

				<div>
					<label class="block">Vote Type:</label>
					<div class="flex gap-4">
						<label class="inline-flex items-center">
							<input
								type="radio"
								bind:group={newVote.is_positive}
								value={true}
								class="mr-2"
							/>
							Positive (+1)
						</label>
						<label class="inline-flex items-center">
							<input
								type="radio"
								bind:group={newVote.is_positive}
								value={false}
								class="mr-2"
							/>
							Negative (-1)
						</label>
					</div>
				</div>

				<div>
					<button type="submit" class="bg-blue-500 text-white px-4 py-2 rounded">
						Create Vote
					</button>
				</div>
			</form>
		</div>

		<!-- User List -->
		<div>
			<h2 class="text-xl mb-4">Existing Users</h2>
			<table class="w-full border-collapse border">
				<thead>
					<tr>
						<th class="border p-2 w-48">Key</th>
						<th class="border p-2">Handle</th>
						<th class="border p-2">Display Name</th>
						<th class="border p-2">Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each users as user}
						<tr>
							<td class="border p-2 font-mono text-sm bg-gray-50">{user.key}</td>
							<td class="border p-2">{user.data.handle}</td>
							<td class="border p-2">{user.data.display_name}</td>
							<td class="border p-2">
								<div class="flex gap-2 justify-center">
									<button
										on:click={() => editUser(user)}
										class="text-blue-500 hover:text-blue-700"
										title="Edit user"
									>
										✏️
									</button>
									<button
										on:click={() => deleteUser(user.key)}
										class="text-red-500 hover:text-red-700"
										title="Delete user"
									>
										❌
									</button>
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>

		<!-- Vote List -->
		<div>
			<h2 class="text-xl mb-4">Existing Votes</h2>
			<table class="w-full border-collapse border">
				<thead>
					<tr>
						<th class="border p-2">Key</th>
						<th class="border p-2">Author</th>
						<th class="border p-2">Target</th>
						<th class="border p-2">Tag</th>
						<th class="border p-2">Type</th>
						<th class="border p-2">Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each votes as vote}
						<tr>
							<td class="border p-2 font-mono text-sm bg-gray-50">{vote.key}</td>
							<td class="border p-2">{vote.description?.split(',')[0].split(':')[1] || 'Unknown'}</td>
							<td class="border p-2">{vote.data.target}</td>
							<td class="border p-2">{vote.data.tag}</td>
							<td class="border p-2">{vote.data.is_positive ? '✅ +1' : '❌ -1'}</td>
							<td class="border p-2">
								<div class="flex gap-2 justify-center">
									<button
										on:click={() => deleteVote(vote.key)}
										class="text-red-500 hover:text-red-700"
										title="Delete vote"
									>
										❌
									</button>
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</div>
{:else}
	<div class="container mx-auto p-4">
		<p>Please log in to access the admin interface.</p>
	</div>
{/if} 