<script lang="ts">
	import { onMount } from 'svelte';
	import { nanoid } from 'nanoid';
	import { listDocs, setDoc, deleteDoc, type Doc, authSubscribe, type User, getDoc, signOut, getSatelliteExtendedActor } from '@junobuild/core';
	import { goto } from '$app/navigation';
	import { REPUTATION_SETTINGS } from '$lib/settings';
	import { idlFactory } from '../../declarations/satellite/satellite.factory.did.js';
	import type { _SERVICE as SatelliteActor } from '../../declarations/satellite/satellite.did';

	// Configuration Constants
	const COLLECTIONS = {
		USERS: 'users',
		VOTES: 'votes',
		TAGS: 'tags'
	} as const;

	const DEFAULT_VOTE_WEIGHT = 1;
	const DEFAULT_TAG_MULTIPLIERS = [
		{ months: 1, multiplier: 1.5 },    // Period 1: First month
		{ months: 2, multiplier: 1.2 },    // Period 2: Months 2-3
		{ months: 3, multiplier: 1.1 },    // Period 3: Months 4-6
		{ months: 6, multiplier: 1.0 },    // Period 4: Months 7-12
		{ months: 12, multiplier: 0.95 },  // Period 5: Months 13-24
		{ months: 12, multiplier: 0.75 },  // Period 6: Months 25-36
		{ months: 12, multiplier: 0.55 },  // Period 7: Months 37-48
		{ months: 999, multiplier: 0.25 }  // Period 8: Months 49+ (treated as infinity)
	];

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

	// Form data for creating/updating tags
	let newTag = {
		key: '',  // Optional - if provided, will update existing tag
		name: '',
		description: '',
		time_periods: [...REPUTATION_SETTINGS.DEFAULT_TIME_PERIODS],
		reputation_threshold: REPUTATION_SETTINGS.DEFAULT_TAG.REPUTATION_THRESHOLD,
		vote_reward: REPUTATION_SETTINGS.DEFAULT_TAG.VOTE_REWARD,
		min_users_for_threshold: REPUTATION_SETTINGS.DEFAULT_TAG.MIN_USERS_FOR_THRESHOLD
	};

	// List of all tags
	let tags: Doc<{
		name: string;
		description: string;
		time_periods: Array<{ months: number; multiplier: number }>;
		reputation_threshold: number;
		vote_reward: number;
		min_users_for_threshold: number;
	}>[] = [];

	// Form data for creating votes
	let newVote = {
		key: '',
		author_key: '',  // User key of the vote author
		target_key: '',  // User key of the vote target
		value: 1,  // Vote value (+1 for upvote, -1 for downvote)
		tag_key: ''  // Tag key
	};

	// List of all votes
	let votes: Doc<{
		author_key: string;
		target_key: string;
		tag_key: string;
		value: number;
	}>[] = [];

	// Error message if something goes wrong
	let error = '';
	// Success message for feedback
	let success = '';
	// Current authenticated user
	let user: User | null = null;

	// Add selected tag state
	let selectedTag = '';

	// Add reputation map
	let userReputations: Record<string, number> = {};

	// Function to load user reputations for selected tag
	async function loadUserReputations(tagKey: string) {
		try {
			const matcher = {
				description: `tag:${tagKey}`  // This matches the pattern used in the Rust code
			};
			const params = {
				matcher
			};
			const reputations = await listDocs<{
				user_key: string;
				total_reputation: number;
				reputation_from_votes: number;
				reputation_from_voting: number;
			}>({
				collection: 'reputations',
				...params
			});
			
			userReputations = {};
			for (const doc of reputations.items) {
				userReputations[doc.data.user_key] = doc.data.total_reputation;
			}
			console.log('Loaded reputations:', userReputations); // Add logging to debug
		} catch (error) {
			console.error('Error loading reputations:', error);
		}
	}

	// Update onMount to load reputations when tag is selected
	onMount(() => {
		// Subscribe to auth state
		const sub = authSubscribe((state) => {
			user = state;
			
			// If user is not logged in, redirect to home
			if (user === null) {
				goto('/');
			} else {
				// Load data if authenticated
				loadUsers();
				loadVotes();
				loadTags();
			}
		});

		// Cleanup subscription on component destroy
		return () => {
			sub();
		};
	});

	// Watch for tag selection changes
	$: if (selectedTag) {
		loadUserReputations(selectedTag);
	}

	// Load users
	async function loadUsers() {
		try {
			const usersList = await listDocs<{ handle: string; display_name: string }>({
				collection: COLLECTIONS.USERS
			});
			users = usersList.items;
		} catch (error) {
			console.error('Error loading users:', error);
		}
	}

	// Load votes
	async function loadVotes() {
		try {
			const votesList = await listDocs<{ author_key: string; target_key: string; tag_key: string; value: number }>({
				collection: COLLECTIONS.VOTES
			});
			console.log('Loaded votes:', votesList.items); // Debug log
			votes = votesList.items;
		} catch (error) {
			console.error('Error loading votes:', error);
		}
	}

	// Load tags
	async function loadTags() {
		try {
			const tagsList = await listDocs<{ 
				name: string; 
				description: string; 
				time_periods: { months: number; multiplier: number; }[];
				reputation_threshold: number;
				vote_reward: number;
				min_users_for_threshold: number;
			}>({
				collection: COLLECTIONS.TAGS
			});
			tags = tagsList.items;
		} catch (error) {
			console.error('Error loading tags:', error);
		}
	}

	/**
	 * Creates or updates a tag in the Juno collection
	 * @throws {Error} If saving fails
	 */
	async function saveTag() {
		try {
			// Validate inputs
			if (!newTag.name || !newTag.description) {
				error = 'Please fill in all required fields';
				return;
			}

			// If we're updating an existing tag, we need to get its current version
			let version;
			if (newTag.key) {
				try {
					const existingDoc = await getDoc({
						collection: COLLECTIONS.TAGS,
						key: newTag.key
					});
					if (!existingDoc) {
						error = 'Tag not found';
						return;
					}
					version = existingDoc.version;
				} catch (e) {
					console.error('Error fetching existing tag:', e);
					error = 'Failed to fetch existing tag version';
					return;
				}
			}

			// Create or update tag document
			await setDoc({
				collection: COLLECTIONS.TAGS,
				doc: {
					key: newTag.key || nanoid(),
					data: {
						name: newTag.name,
						description: newTag.description,
						time_periods: newTag.time_periods,
						reputation_threshold: newTag.reputation_threshold,
						vote_reward: newTag.vote_reward,
						min_users_for_threshold: newTag.min_users_for_threshold
					},
					...(version && { version })
				}
			});

			// Clear form and show success message
			newTag = {
				key: '',
				name: '',
				description: '',
				time_periods: [...REPUTATION_SETTINGS.DEFAULT_TIME_PERIODS],
				reputation_threshold: REPUTATION_SETTINGS.DEFAULT_TAG.REPUTATION_THRESHOLD,
				vote_reward: REPUTATION_SETTINGS.DEFAULT_TAG.VOTE_REWARD,
				min_users_for_threshold: REPUTATION_SETTINGS.DEFAULT_TAG.MIN_USERS_FOR_THRESHOLD
			};
			success = newTag.key ? 'Tag updated successfully!' : 'Tag created successfully!';

			// Reload the tag list
			await loadTags();
			success = 'Tag saved successfully';
		} catch (err) {
			console.error('Error saving tag:', err);
			error = 'Failed to save tag';
		}
	}

	/**
	 * Deletes a tag from the Juno collection
	 * @param key - The key of the tag to delete
	 * @throws {Error} If deletion fails
	 */
	async function deleteTag(key: string) {
		if (!confirm('Are you sure you want to delete this tag?')) {
			return;
		}

		try {
			error = '';
			success = '';

			// Get the current version of the tag
			const existingDoc = await getDoc({
				collection: COLLECTIONS.TAGS,
				key
			});

			if (!existingDoc) {
				error = 'Tag not found';
				return;
			}

			await deleteDoc({
				collection: COLLECTIONS.TAGS,
				doc: {
					key,
					data: {},
					version: existingDoc.version
				}
			});

			success = 'Tag deleted successfully!';
			await loadTags();
		} catch (e) {
			console.error('Error deleting tag:', e);
			error = e instanceof Error ? e.message : 'Failed to delete tag';
		}
	}

	/**
	 * Loads tag data into the form for editing
	 * @param tagDoc - The tag document to edit
	 */
	function editTag(tagDoc: Doc<{ 
		name: string; 
		description: string; 
		time_periods: Array<{ months: number; multiplier: number }>;
		reputation_threshold: number;
		vote_reward: number;
		min_users_for_threshold: number;
	}>) {
		newTag = {
			key: tagDoc.key,
			name: tagDoc.data.name,
			description: tagDoc.data.description,
			time_periods: [...tagDoc.data.time_periods],
			reputation_threshold: tagDoc.data.reputation_threshold,
			vote_reward: tagDoc.data.vote_reward,
			min_users_for_threshold: tagDoc.data.min_users_for_threshold
		};
		// Scroll to form
		document.getElementById('tagForm')?.scrollIntoView({ behavior: 'smooth' });
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
						collection: COLLECTIONS.USERS,
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
				collection: COLLECTIONS.USERS,
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

			// Get the current version of the user
			const existingDoc = await getDoc({
				collection: COLLECTIONS.USERS,
				key
			});

			if (!existingDoc) {
				error = 'User not found';
				return;
			}

			await deleteDoc({
				collection: COLLECTIONS.USERS,
				doc: {
					key,
					data: {},
					version: existingDoc.version
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

	// Function to save a vote
	async function saveVote() {
		try {
			// Validate inputs
			if (!newVote.author_key || !newVote.target_key || !newVote.tag_key) {
				error = 'Please fill in all fields';
				return;
			}

			console.log('Creating vote:', newVote); // Debug log

			// Create vote document
			const voteDoc = {
				key: newVote.key || nanoid(),
				data: {
					author_key: newVote.author_key,
					target_key: newVote.target_key,
					tag_key: newVote.tag_key,
					value: newVote.value
				},
				description: `author:${newVote.author_key},target:${newVote.target_key},tag:${newVote.tag_key}`
			};

			console.log('Vote document to save:', voteDoc); // Debug log

			// Save vote
			await setDoc({
				collection: COLLECTIONS.VOTES,
				doc: voteDoc
			});

			console.log('Vote saved successfully'); // Debug log

			// Clear form
			newVote = {
				key: '',
				author_key: '',
				target_key: '',
				value: 1,
				tag_key: ''
			};

			// Reload votes
			await loadVotes();
			
			// Reload reputations if a tag is selected
			if (selectedTag) {
				await loadUserReputations(selectedTag);
			}

			success = 'Vote saved successfully';
		} catch (err) {
			console.error('Error saving vote:', err);
			error = 'Failed to save vote';
		}
	}

	// Function to delete a vote
	async function deleteVote(voteKey: string) {
		if (!confirm('Are you sure you want to delete this vote?')) {
			return;
		}

		try {
			error = '';
			success = '';

			// Get the current version of the vote
			const existingDoc = await getDoc({
				collection: COLLECTIONS.VOTES,
				key: voteKey
			});

			if (!existingDoc) {
				error = 'Vote not found';
				return;
			}

			await deleteDoc({
				collection: COLLECTIONS.VOTES,
				doc: {
					key: voteKey,
					data: existingDoc.data,
					version: existingDoc.version
				}
			});

			// Reload votes
			await loadVotes();
			
			// Reload reputations if a tag is selected
			if (selectedTag) {
				await loadUserReputations(selectedTag);
			}

			success = 'Vote deleted successfully';
		} catch (err) {
			console.error('Error deleting vote:', err);
			error = 'Failed to delete vote';
		}
	}

	// Add this to the script section
	async function recalculateUserReputation(userKey: string, tagKey: string) {
		try {
			error = '';
			success = '';
			
			// Get the satellite actor
			const actor = await getSatelliteExtendedActor<SatelliteActor>({
				idlFactory
			});
			
			// Call our custom endpoint
			const result = await actor.recalculate_reputation(userKey, tagKey);
			
			// Refresh the users list to show updated reputation
			await loadUsers();
			
			success = `Reputation recalculated successfully. New score: ${result}`;
		} catch (err) {
			error = `Failed to recalculate reputation: ${err}`;
		}
	}
</script>

{#if user}
	<div class="container mx-auto p-4">
		<div class="flex justify-between items-center mb-8">
			<h1 class="text-2xl">Admin Dashboard</h1>
			<button
				on:click={() => signOut()}
				class="bg-red-500 text-white px-4 py-2 rounded hover:bg-red-600"
			>
				Log Out
			</button>
		</div>

		<!-- Tag Selector -->
		<div class="mb-8">
			<label for="tag-select" class="block text-lg mb-2">Select Reputation Tag:</label>
			<select
				id="tag-select"
				bind:value={selectedTag}
				class="border p-2 w-full max-w-md"
			>
				<option value="">Select a tag to view reputations</option>
				{#each tags as tag}
					<option value={tag.key}>
						{tag.data.name}
					</option>
				{/each}
			</select>
		</div>

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

		<!-- User List -->
		<div class="mb-8">
			<h2 class="text-xl mb-4">Existing Users</h2>
			<table class="w-full border-collapse border">
				<thead>
					<tr>
						<th class="border p-2 w-48">Key</th>
						<th class="border p-2">Handle</th>
						<th class="border p-2">Display Name</th>
						{#if selectedTag}
							<th class="border p-2">Reputation</th>
						{/if}
						<th class="border p-2">Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each users as user}
						<tr>
							<td class="border p-2 font-mono text-sm bg-gray-50">{user.key}</td>
							<td class="border p-2">{user.data.handle}</td>
							<td class="border p-2">{user.data.display_name}</td>
							{#if selectedTag}
								<td class="border p-2">
									{userReputations[user.key]?.toFixed(2) ?? '0.00'}
								</td>
							{/if}
							<td class="border p-2">
								<div class="flex gap-2">
									{#if selectedTag}
										<button
											class="btn btn-sm btn-outline-primary"
											on:click={() => recalculateUserReputation(user.key, selectedTag)}
										>
											Recalculate
										</button>
									{/if}
									<button
										class="btn btn-sm btn-outline-primary"
										on:click={() => editUser(user)}
									>
										Edit
									</button>
									<button
										class="btn btn-sm btn-outline-error"
										on:click={() => deleteUser(user.key)}
									>
										Delete
									</button>
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>

		<!-- Create Vote Form -->
		<div class="mb-8" id="voteForm">
			<h2 class="text-xl mb-4">Create New Vote</h2>
			<form on:submit|preventDefault={saveVote} class="space-y-4">
				<div>
					<label for="author" class="block">Author (User Key):</label>
					<select
						id="author"
						bind:value={newVote.author_key}
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
						bind:value={newVote.target_key}
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
					<select
						id="tag"
						bind:value={newVote.tag_key}
						class="border p-2 w-full"
					>
						<option value="">Select Tag</option>
						{#each tags as tag}
							<option value={tag.key} selected={tag.key === selectedTag}>
								{tag.data.name}
							</option>
						{/each}
					</select>
				</div>

				<div>
					<fieldset>
						<legend class="block mb-2">Vote Value:</legend>
						<div class="flex gap-4">
							<label class="inline-flex items-center">
								<input
									type="radio"
									bind:group={newVote.value}
									value={1}
									class="mr-2"
								/>
								Positive (+1)
							</label>
							<label class="inline-flex items-center">
								<input
									type="radio"
									bind:group={newVote.value}
									value={-1}
									class="mr-2"
								/>
								Negative (-1)
							</label>
						</div>
					</fieldset>
				</div>

				<div>
					<button type="submit" class="bg-blue-500 text-white px-4 py-2 rounded">
						Create Vote
					</button>
				</div>
			</form>
		</div>

		<!-- Vote List -->
		<div class="mb-8">
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
							<td class="border p-2">{vote.data.target_key}</td>
							<td class="border p-2">{vote.data.tag_key}</td>
							<td class="border p-2">{vote.data.value > 0 ? '✅ +1' : '❌ -1'}</td>
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

		<!-- Create/Update Tag Form -->
		<div class="mb-8" id="tagForm">
			<h2 class="text-xl mb-4">Create New Tag</h2>
			<form on:submit|preventDefault={saveTag} class="space-y-4">
				{#if newTag.key}
					<div>
						<label for="tagKey" class="block">Tag Key:</label>
						<input
							type="text"
							id="tagKey"
							bind:value={newTag.key}
							class="border p-2 w-full bg-gray-100"
							readonly
						/>
					</div>
				{/if}

				<div>
					<label for="tagName" class="block">Tag Name:</label>
					<input
						type="text"
						id="tagName"
						bind:value={newTag.name}
						class="border p-2 w-full"
						placeholder="e.g., Technical Skills"
					/>
				</div>

				<div>
					<label for="tagDescription" class="block">Description:</label>
					<textarea
						id="tagDescription"
						bind:value={newTag.description}
						class="border p-2 w-full"
						placeholder="Describe what this tag represents"
						rows="3"
					></textarea>
				</div>

				<div>
					<label for="time-periods" class="block mb-2">Time Period Multipliers:</label>
					<div id="time-periods" class="space-y-2">
						<table class="w-full border-collapse">
							<thead>
								<tr>
									<th class="border p-2 text-left w-1/6">Period</th>
									<th class="border p-2 text-left w-2/6">Months</th>
									<th class="border p-2 text-left w-2/6">Multiplier</th>
									<th class="border p-2 text-left w-1/6">Actions</th>
								</tr>
							</thead>
							<tbody>
								{#each newTag.time_periods as period, i}
									<tr>
										<td class="border p-2">Period {i + 1}</td>
										<td class="border p-2">
											<input
												type="number"
												id="months-{i}"
												bind:value={period.months}
												class="w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm px-2"
												min="1"
												max={i === newTag.time_periods.length - 1 ? 999 : 12}
											/>
										</td>
										<td class="border p-2">
											<input
												type="number"
												id="multiplier-{i}"
												bind:value={period.multiplier}
												class="w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm px-2"
												min="0"
												max="2"
												step="0.05"
											/>
										</td>
										<td class="border p-2 text-center">
											{#if i === newTag.time_periods.length - 1}
												<button
													type="button"
													on:click={() => {
														newTag.time_periods = [...newTag.time_periods, { months: 12, multiplier: 1.0 }];
													}}
													class="inline-flex items-center px-3 py-1 border border-transparent text-sm text-blue-700 bg-blue-100 hover:bg-blue-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 rounded"
												>
													Add Period
												</button>
											{:else}
												<button
													type="button"
													on:click={() => {
														newTag.time_periods = newTag.time_periods.filter((_, index) => index !== i);
													}}
													class="inline-flex items-center px-3 py-1 border border-transparent text-sm text-red-700 bg-red-100 hover:bg-red-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 rounded"
												>
													Remove
												</button>
											{/if}
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				</div>

				<div class="mb-4">
					<label for="reputation_threshold" class="block text-sm font-medium text-gray-700">Reputation Threshold</label>
					<input
						type="number"
						id="reputation_threshold"
						bind:value={newTag.reputation_threshold}
						step="1"
						min="0"
						class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
					/>
				</div>
				<div class="mb-4">
					<label for="vote_reward" class="block text-sm font-medium text-gray-700">Vote Reward</label>
					<input
						type="number"
						id="vote_reward"
						bind:value={newTag.vote_reward}
						step="0.1"
						min="0"
						class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
					/>
				</div>
				<div class="mb-4">
					<label for="min_users_for_threshold" class="block text-sm font-medium text-gray-700">Minimum Users for Threshold</label>
					<input
						type="number"
						id="min_users_for_threshold"
						bind:value={newTag.min_users_for_threshold}
						step="1"
						min="1"
						class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
					/>
					<p class="mt-1 text-sm text-gray-500">Number of users that need to reach threshold before vote rewards are restricted</p>
				</div>

				<div class="flex gap-4">
					<button type="submit" class="bg-blue-500 text-white px-4 py-2 rounded">
						{newTag.key ? 'Update Tag' : 'Create Tag'}
					</button>
					{#if newTag.key}
						<button
							type="button"
							class="bg-gray-500 text-white px-4 py-2 rounded"
							on:click={() => {
								newTag = {
									key: '',
									name: '',
									description: '',
									time_periods: [...REPUTATION_SETTINGS.DEFAULT_TIME_PERIODS],
									reputation_threshold: REPUTATION_SETTINGS.DEFAULT_TAG.REPUTATION_THRESHOLD,
									vote_reward: REPUTATION_SETTINGS.DEFAULT_TAG.VOTE_REWARD,
									min_users_for_threshold: REPUTATION_SETTINGS.DEFAULT_TAG.MIN_USERS_FOR_THRESHOLD
								};
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

		<!-- Tag List -->
		<div>
			<h2 class="text-xl mb-4">Existing Tags</h2>
			<table class="w-full border-collapse border">
				<thead>
					<tr>
						<th class="border p-2 w-48">Key</th>
						<th class="border p-2">Name</th>
						<th class="border p-2">Description</th>
						<th class="border p-2">Time Periods</th>
						<th class="border p-2">Threshold</th>
						<th class="border p-2">Reward</th>
						<th class="border p-2">Min Users</th>
						<th class="border p-2">Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each tags as tag}
						<tr>
							<td class="border p-2 font-mono text-sm bg-gray-50">{tag.key}</td>
							<td class="border p-2">{tag.data.name}</td>
							<td class="border p-2">{tag.data.description}</td>
							<td class="border p-2">
								<ul class="list-disc list-inside">
									{#each tag.data.time_periods as period}
										<li>{period.months}mo: {period.multiplier}x</li>
									{/each}
								</ul>
							</td>
							<td class="border p-2">{tag.data.reputation_threshold}</td>
							<td class="border p-2">{tag.data.vote_reward}</td>
							<td class="border p-2">{tag.data.min_users_for_threshold}</td>
							<td class="border p-2">
								<div class="flex gap-2 justify-center">
									<button
										on:click={() => editTag(tag)}
										class="text-blue-500 hover:text-blue-700"
										title="Edit tag"
									>
										✏️
									</button>
									<button
										on:click={() => deleteTag(tag.key)}
										class="text-red-500 hover:text-red-700"
										title="Delete tag"
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