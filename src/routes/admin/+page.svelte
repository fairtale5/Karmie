<script lang="ts">
	import { onMount } from 'svelte';
	import { nanoid } from 'nanoid';
	import { listDocs, setDoc, deleteDoc, type Doc, authSubscribe, type User, getDoc, signOut, getSatelliteExtendedActor } from '@junobuild/core';
	import { goto } from '$app/navigation';
	import { REPUTATION_SETTINGS } from '$lib/settings';
	import { idlFactory } from '../../declarations/satellite/satellite.factory.did.js';
	import type { _SERVICE as SatelliteActor } from '../../declarations/satellite/satellite.did';
	import { createUserDescription, createTagDescription, createVoteDescription, createSearchPattern } from '$lib/description';
	import { getUserReputationFull } from '../../declarations/satellite/satellite.api';

	// Configuration Constants
	const COLLECTIONS = {
		USERS: 'users',
		VOTES: 'votes',
		TAGS: 'tags',
		REPUTATIONS: 'reputations'
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

	// User form data
	let newUser = {
		key: '',
		username: '',
		display_name: ''
	};

	// User type definition
	interface UserData {
		username: string;
		display_name: string;
	}

	// List of all users
	let users: Doc<{
		username: string;
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
		author_key: string;
		time_periods: Array<{ months: number; multiplier: number }>;
		reputation_threshold: number;
		vote_reward: number;
		min_users_for_threshold: number;
	}>[] = [];

	// Form data for creating votes
	let newVote = {
		data: {
			author_key: '',
			target_key: '',
			tag_key: '',
			value: 1,
			weight: 1.0
		}
	};

	// List of all votes
	let votes: Doc<{
		author_key: string;
		target_key: string;
		tag_key: string;
		value: number;
		weight: number;
	}>[] = [];

	// Error message if something goes wrong
	let error = '';
	// Success message for feedback
	let success = '';
	// Current authenticated user
	let user: User | null = null;

	// Add selected tag state
	let selectedTag = '';

	// Add this type definition at the top of the script section
	type ReputationData = {
		user_key: string;
		tag_key: string;
		total_basis_reputation: number;
		total_voting_rewards_reputation: number;
		last_known_effective_reputation: number;
		last_calculation: bigint;
		vote_weight: { _value: number } | number;
		has_voting_power: boolean;
	};

	// Update the userReputations type
	let userReputations: Record<string, ReputationData> = {};

	// Add a variable to store reputation documents
	let reputationDocs: Doc<ReputationData>[] = [];

	// Function to load user reputations for selected tag
	async function loadUserReputations(tagKey: string) {
		try {
			console.log('[Admin] Loading reputations for tag:', tagKey);
			
			// Get all users first
			const usersList = await listDocs<{ username: string; display_name: string }>({
				collection: COLLECTIONS.USERS
			});
			
			// There are two approaches to fetching reputation data:
			// 1. Bulk approach (current): Get all reputations for the tag at once with listDocs
			//    - Pros: Fewer network requests, better for many users
			//    - Cons: Returns more data than needed if we only want specific users
			// 2. Individual approach: Use getUserReputationFull for each user
			//    - Pros: More precise, only gets data for users we care about
			//    - Cons: More network requests, worse for many users
			//
			// The bulk approach is more efficient for admin views where we need all user data
			
			// Get the satellite actor
			const actor = await getSatelliteExtendedActor<SatelliteActor>({
				idlFactory
			});
			
			// Get all reputations for this tag
			const reputationsList = await listDocs<ReputationData>({
				collection: COLLECTIONS.REPUTATIONS,
				filter: {
					matcher: {
						description: `tag=${tagKey};`
					}
				}
			});
			
			// Create a map of user_key to reputation data
			const reputationMap = new Map(
				reputationsList.items.map(item => [item.data.user_key, item.data])
			);
			
			// Get reputation data for each user
			userReputations = {};
			for (const user of usersList.items) {
				const reputation = reputationMap.get(user.key) || {
					user_key: user.key,
					tag_key: tagKey,
					total_basis_reputation: 0,
					total_voting_rewards_reputation: 0,
					last_known_effective_reputation: 0,
					last_calculation: BigInt(0),
					vote_weight: 0,
					has_voting_power: false
				};
				userReputations[user.key] = reputation;
			}
			
			// Example of individual approach (not used but shown for reference):
			// userReputations = {};
			// for (const user of usersList.items) {
			//     try {
			//         const result = await getUserReputationFull(user.key, tagKey);
			//         if ('Ok' in result) {
			//             userReputations[user.key] = result.Ok;
			//         } else {
			//             // User has no reputation yet, use default values
			//             userReputations[user.key] = {
			//                 user_key: user.key,
			//                 tag_key: tagKey,
			//                 total_basis_reputation: 0,
			//                 total_voting_rewards_reputation: 0,
			//                 last_known_effective_reputation: 0,
			//                 last_calculation: BigInt(0),
			//                 vote_weight: 0,
			//                 has_voting_power: false
			//             };
			//         }
			//     } catch (error) {
			//         console.error(`[Admin] Error loading reputation for user ${user.key}:`, error);
			//         // Use default values on error
			//         userReputations[user.key] = {
			//             user_key: user.key,
			//             tag_key: tagKey,
			//             total_basis_reputation: 0,
			//             total_voting_rewards_reputation: 0,
			//             last_known_effective_reputation: 0,
			//             last_calculation: BigInt(0),
			//             vote_weight: 0,
			//             has_voting_power: false
			//         };
			//     }
			// }
			
			console.log('[Admin] Loaded reputations:', userReputations);
		} catch (error) {
			console.error('[Admin] Error loading reputations:', error);
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
				loadReputations();
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
			const usersList = await listDocs<{ username: string; display_name: string }>({
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
			const votesList = await listDocs<{ author_key: string; target_key: string; tag_key: string; value: number; weight: number }>({
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
				author_key: string;
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

	// Load reputation documents
	async function loadReputations() {
		try {
			const reputationsList = await listDocs<ReputationData>({
				collection: COLLECTIONS.REPUTATIONS
			});
			console.log('Loaded reputation documents:', reputationsList.items);
			reputationDocs = reputationsList.items;
		} catch (error) {
			console.error('Error loading reputation documents:', error);
		}
	}

	// User form data type definition
	interface DocData {
		collection: string;
		doc: {
			key: string;
			data: {
				username: string;
				display_name: string;
			};
			description: string;
			version?: bigint;
		};
	}

	/**
	 * Creates or updates a user in the Juno collection
	 * @throws {Error} If saving fails
	 */
	async function saveUser() {
		try {
			// Clear any previous messages
			error = '';
			success = '';

			// Comment out frontend validation to test satellite validation
			/*
			// Basic validation
			if (!newUser.username || !newUser.display_name) {
				error = 'Please fill in all required fields';
				return;
			}

			// Validate username format
			const usernameRegex = /^[a-zA-Z0-9_-]{3,30}$/;
			if (!usernameRegex.test(newUser.username)) {
				error = 'Username must be 3-30 characters long and can only contain letters, numbers, underscores, and hyphens';
				return;
			}

			// Validate display name format
			if (newUser.display_name.trim().length === 0) {
				error = 'Display name cannot be empty';
				return;
			}
			if (newUser.display_name.length > 100) {
				error = 'Display name cannot be longer than 100 characters';
				return;
			}
			*/

			// For new documents: generate key with nanoid
			const documentKey = newUser.key || nanoid();

			// Create the document data with required fields
			const docData = {
				collection: COLLECTIONS.USERS,
				doc: {
					key: documentKey,
					data: {
						key: documentKey,
						username: newUser.username.toLowerCase(),
						display_name: newUser.display_name.trim()
					},
					description: createUserDescription(documentKey, newUser.username.toLowerCase())
				}
			};

			// Log the exact data being sent to setDoc
			console.log('[Admin] Sending to setDoc:', docData);

			// Create or update user document
			await setDoc(docData);

			// Clear form and show success message
			newUser = {
				key: '',
				username: '',
				display_name: ''
			};
			success = 'User saved successfully';

			// Reload users list
			await loadUsers();
		} catch (e) {
			console.error('[Admin] Error saving user:', e);
			
			// Enhanced error handling
			if (e instanceof Error) {
				if (e.message.includes('Username')) {
					error = 'This username is already taken. Please choose a different one.';
				} else if (e.message.includes('Invalid user data format')) {
					error = 'Invalid user data format. Please check your input and try again.';
				} else {
					error = e.message;
				}
			} else {
				error = 'Failed to save user. Please try again.';
			}
		}
	}

	/**
	 * Creates or updates a tag in the Juno collection
	 * @throws {Error} If saving fails
	 */
	async function saveTag() {
		try {
			console.log('[Admin] Saving tag:', newTag);
			
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
					console.error('[Admin] Error fetching existing tag:', e);
					error = 'Failed to fetch existing tag version';
					return;
				}
			}

			// Generate document key if not updating
			const documentKey = newTag.key || nanoid();

			// Create or update tag document
			if (!user?.key) {
				throw new Error('User must be authenticated to create or update tags');
			}

			await setDoc({
				collection: COLLECTIONS.TAGS,
				doc: {
					key: documentKey,
					data: {
						author_key: user.key,  // Admin creates tags
						name: newTag.name,
						description: newTag.description,
						time_periods: newTag.time_periods,
						reputation_threshold: newTag.reputation_threshold,
						vote_reward: newTag.vote_reward,
						min_users_for_threshold: newTag.min_users_for_threshold
					},
					description: createTagDescription(user, documentKey, newTag.name, user.key),  // Admin creates tags
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
			success = 'Tag saved successfully';
			error = '';

			// Reload tags list
			await loadTags();
		} catch (e) {
			console.error('[Admin] Error saving tag:', e);
			error = e instanceof Error ? e.message : 'Failed to save tag';
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
	function editUser(userDoc: Doc<{ username: string; display_name: string }>) {
		newUser = {
			key: userDoc.key,
			username: userDoc.data.username,
			display_name: userDoc.data.display_name
		};
		// Scroll to form
		document.getElementById('userForm')?.scrollIntoView({ behavior: 'smooth' });
	}

	/**
	 * Creates a new vote in the Juno collection
	 * @throws {Error} If saving fails
	 */
	async function saveVote() {
		try {
			// Generate a new document key
			const documentKey = nanoid();
			
			// Create the vote document with proper structure
			// WARNING: key is a root-level field in the document structure, NOT part of data
			const voteDoc = {
				collection: "votes",
				doc: {
					key: documentKey,
					description: createVoteDescription(user, documentKey, newVote.data.author_key, newVote.data.target_key, newVote.data.tag_key),
					data: {
						author_key: newVote.data.author_key,
						target_key: newVote.data.target_key,
						tag_key: newVote.data.tag_key,
						value: newVote.data.value,
						weight: newVote.data.weight
					}
				}
			};

			// Log the document being saved
			console.log('[Admin] Sending to setDoc:', voteDoc);
			
			await setDoc(voteDoc);
			
			// Reset form
			newVote.data = {
				author_key: '',
				target_key: '',
				value: 1,
				tag_key: '',
				weight: 1.0
			};
			
			// Refresh votes list
			await loadVotes();
		} catch (e) {
			console.error('[Admin] Error saving vote:', e);
			error = e instanceof Error ? e.message : 'Failed to save vote';
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

	// Update the recalculateUserReputation function
	async function recalculateUserReputation(userKey: string) {
		try {
			console.log('[Admin] Recalculating reputation for user:', userKey);
			
			// Get the satellite actor
			const actor = await getSatelliteExtendedActor<SatelliteActor>({
				idlFactory
			});
			
			// Get the current tag key from the URL
			const tagKey = window.location.pathname.split('/').pop();
			if (!tagKey) {
				throw new Error('No tag selected');
			}
			
			// Call recalculate_reputation
			const result = await actor.recalculate_reputation(userKey, tagKey);
			
			// Reload reputations to show updated values
			await loadUserReputations(tagKey);
			
			success = `Recalculated reputation for user ${userKey}`;
			error = '';
		} catch (e) {
			console.error('[Admin] Error recalculating reputation:', e);
			error = e instanceof Error ? e.message : 'Failed to recalculate reputation';
			success = '';
		}
	}

	// Helper function to format time ago
	function getTimeAgo(timestamp: number): string {
		const now = Date.now() * 1_000_000; // Convert to nanoseconds
		const diff = now - timestamp;
		
		const seconds = diff / 1_000_000_000;
		const minutes = seconds / 60;
		const hours = minutes / 60;
		const days = hours / 24;
		
		if (days > 1) return `${Math.floor(days)}d ago`;
		if (hours > 1) return `${Math.floor(hours)}h ago`;
		if (minutes > 1) return `${Math.floor(minutes)}m ago`;
		return `${Math.floor(seconds)}s ago`;
	}

	// Function to recalculate reputation
	async function recalculateReputation(userKey: string, tagKey: string) {
		try {
			const actor = await getSatelliteExtendedActor<SatelliteActor>({
				idlFactory
			});
			
			const result = await actor.recalculate_reputation(userKey, tagKey);
			if ('Ok' in result) {
				// Reload reputations after recalculation
				await loadUserReputations(tagKey);
				success = `Recalculated reputation for ${userKey}`;
			} else {
				error = `Failed to recalculate: ${result.Err}`;
			}
		} catch (e) {
			error = `Error recalculating reputation: ${e}`;
			console.error('[Admin] Recalculation error:', e);
		}
	}

	// Add a function to delete reputation documents
	async function deleteReputationDoc(key: string) {
		if (!confirm('Are you sure you want to delete this reputation document?')) {
			return;
		}

		try {
			error = '';
			success = '';

			// Get the current version of the document
			const existingDoc = await getDoc({
				collection: COLLECTIONS.REPUTATIONS,
				key
			});

			if (!existingDoc) {
				error = 'Reputation document not found';
				return;
			}

			await deleteDoc({
				collection: COLLECTIONS.REPUTATIONS,
				doc: {
					key,
					data: {},
					version: existingDoc.version
				}
			});

			success = 'Reputation document deleted successfully!';
			await loadReputations();
		} catch (e) {
			console.error('Error deleting reputation document:', e);
			error = e instanceof Error ? e.message : 'Failed to delete reputation document';
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
			<label for="tag-select" class="block text-lg mb-2">Select Tag to Filter Data:</label>
			<select
				id="tag-select"
				bind:value={selectedTag}
				class="border p-2 w-full max-w-md"
			>
				<option value="">Show all data</option>
				{#each tags as tag}
					<option value={tag.key}>
						{tag.data.name}
					</option>
				{/each}
			</select>
		</div>

		{#if selectedTag}
			<!-- Selected Tag Details -->
			{@const selectedTagDoc = tags.find(t => t.key === selectedTag)}
			{#if selectedTagDoc}
				<div class="mb-8">
					<h2 class="text-xl mb-4">Selected Tag Details</h2>
					<div class="overflow-x-auto">
						<table class="table table-zebra w-full">
							<tbody>
								<tr>
									<td class="font-bold">Document Key</td>
									<td class="font-mono">{selectedTagDoc.key}</td>
								</tr>
								<tr>
									<td class="font-bold">Description</td>
									<td class="font-mono">{selectedTagDoc.description}</td>
								</tr>
								<tr>
									<td class="font-bold">Owner</td>
									<td class="font-mono">{selectedTagDoc.owner}</td>
								</tr>
								<tr>
									<td class="font-bold">Created At</td>
									<td>{new Date(Number(selectedTagDoc.created_at) / 1_000_000).toLocaleString()}</td>
								</tr>
								<tr>
									<td class="font-bold">Updated At</td>
									<td>{new Date(Number(selectedTagDoc.updated_at) / 1_000_000).toLocaleString()}</td>
								</tr>
								<tr>
									<td class="font-bold">Version</td>
									<td>{selectedTagDoc.version}</td>
								</tr>
								<tr>
									<td class="font-bold">Author Key</td>
									<td class="font-mono">{selectedTagDoc.data.author_key}</td>
								</tr>
								<tr>
									<td class="font-bold">Name</td>
									<td>{selectedTagDoc.data.name}</td>
								</tr>
								<tr>
									<td class="font-bold">Description</td>
									<td>{selectedTagDoc.data.description}</td>
								</tr>
								<tr>
									<td class="font-bold">Time Periods</td>
									<td>
										<ul class="list-disc list-inside">
											{#each selectedTagDoc.data.time_periods as period}
												<li>{period.months} months: {period.multiplier}x</li>
											{/each}
										</ul>
									</td>
								</tr>
								<tr>
									<td class="font-bold">Reputation Threshold</td>
									<td>{selectedTagDoc.data.reputation_threshold}</td>
								</tr>
								<tr>
									<td class="font-bold">Vote Reward</td>
									<td>{selectedTagDoc.data.vote_reward}</td>
								</tr>
								<tr>
									<td class="font-bold">Min Users for Threshold</td>
									<td>{selectedTagDoc.data.min_users_for_threshold}</td>
								</tr>
							</tbody>
						</table>
					</div>
				</div>
			{/if}
		{/if}

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
					<label for="username" class="block">Username:</label>
					<input
						type="text"
						id="username"
						bind:value={newUser.username}
						class="border p-2 w-full"
						placeholder="Enter username"
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
								newUser = { key: '', username: '', display_name: '' };
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

		<!-- Users with Reputation -->
		<div class="mb-8">
			<h2 class="text-xl mb-4">Users and Their Reputation</h2>
			<div class="overflow-x-auto">
				<table class="table table-zebra w-full">
					<thead>
						<tr>
							<th>Document Info</th>
							<th>User Data</th>
							<th>Reputation Data</th>
							<th>Actions</th>
						</tr>
					</thead>
					<tbody>
						{#each users as user}
							{@const reputation = userReputations[user.key] ?? {
								total_basis_reputation: 0,
								total_voting_rewards_reputation: 0,
								last_known_effective_reputation: 0,
								vote_weight: 0,
								has_voting_power: false,
								last_calculation: BigInt(0)
							}}
							<tr>
								<td>
									<div class="space-y-1">
										<div class="font-mono text-xs">Key: {user.key}</div>
										<div class="font-mono text-xs">Description: {user.description}</div>
										<div class="font-mono text-xs">Owner: {user.owner}</div>
										<div class="text-xs">Created: {new Date(Number(user.created_at) / 1_000_000).toLocaleString()}</div>
										<div class="text-xs">Updated: {new Date(Number(user.updated_at) / 1_000_000).toLocaleString()}</div>
										<div class="text-xs">Version: {user.version}</div>
									</div>
								</td>
								<td>
									<div class="space-y-1">
										<div class="font-bold">{user.data.username}</div>
										<div class="text-sm opacity-75">{user.data.display_name}</div>
									</div>
								</td>
								<td>
									<div class="space-y-1">
										<div>Base Rep: {reputation.total_basis_reputation.toFixed(2)}</div>
										<div>Vote Rep: {reputation.total_voting_rewards_reputation.toFixed(2)}</div>
										<div>Total Rep: {reputation.last_known_effective_reputation.toFixed(2)}</div>
										<div>Weight: {(Number(reputation.vote_weight) * 100).toFixed(4)}%</div>
										<div>Status: {reputation.has_voting_power ? 'Active' : 'Inactive'}</div>
										<div class="text-xs">Last Calc: {new Date(Number(reputation.last_calculation) / 1_000_000).toLocaleString()}</div>
									</div>
								</td>
								<td>
									<div class="flex gap-2 justify-center">
										<button
											class="btn btn-xs btn-primary"
											on:click={() => recalculateReputation(user.key, selectedTag)}
											title="Recalculate reputation"
										>
											🔄
										</button>
										<button
											class="btn btn-xs btn-info"
											on:click={() => editUser(user)}
											title="Edit user"
										>
											✏️
										</button>
										<button
											class="btn btn-xs btn-error"
											on:click={() => deleteUser(user.key)}
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
		</div>

		<!-- Create Vote Form -->
		<div class="mb-8" id="voteForm">
			<h2 class="text-xl mb-4">Create New Vote</h2>
			<form on:submit|preventDefault={saveVote} class="space-y-4">
				<div>
					<label for="author" class="block">Author (User Key):</label>
					<select
						id="author"
						bind:value={newVote.data.author_key}
						class="border p-2 w-full"
					>
						<option value="">Select Author</option>
						{#each users as user}
							<option value={user.key}>
								{user.data.display_name} ({user.data.username})
							</option>
						{/each}
					</select>
				</div>

				<div>
					<label for="target" class="block">Target (User Key):</label>
					<select
						id="target"
						bind:value={newVote.data.target_key}
						class="border p-2 w-full"
					>
						<option value="">Select Target</option>
						{#each users as user}
							<option value={user.key}>
								{user.data.display_name} ({user.data.username})
							</option>
						{/each}
					</select>
				</div>

				<div>
					<label for="tag" class="block">Tag:</label>
					<select
						id="tag"
						bind:value={newVote.data.tag_key}
						class="border p-2 w-full"
						required
					>
						<option value="">Select Tag</option>
						{#each tags as tag}
							<option value={tag.key}>
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
									bind:group={newVote.data.value}
									value={1}
									class="mr-2"
								/>
								Positive (+1)
							</label>
							<label class="inline-flex items-center">
								<input
									type="radio"
									bind:group={newVote.data.value}
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

		<!-- All Votes -->
		<div class="mb-8">
			<h2 class="text-xl mb-4">{selectedTag ? 'Votes in Selected Tag' : 'All Votes'}</h2>
			<div class="overflow-x-auto">
				<table class="table table-zebra w-full">
					<thead>
						<tr>
							<th>Document Info</th>
							<th>Vote Data</th>
							<th>References</th>
							<th>Actions</th>
						</tr>
					</thead>
					<tbody>
						{#each votes.filter(v => !selectedTag || v.data.tag_key === selectedTag) as vote}
							{@const author = users.find(u => u.key === vote.data.author_key)}
							{@const target = users.find(u => u.key === vote.data.target_key)}
							{@const tag = tags.find(t => t.key === vote.data.tag_key)}
							<tr>
								<td>
									<div class="space-y-1">
										<div class="font-mono text-xs">Key: {vote.key}</div>
										<div class="font-mono text-xs">Description: {vote.description}</div>
										<div class="font-mono text-xs">Owner: {vote.owner}</div>
										<div class="text-xs">Created: {new Date(Number(vote.created_at) / 1_000_000).toLocaleString()}</div>
										<div class="text-xs">Updated: {new Date(Number(vote.updated_at) / 1_000_000).toLocaleString()}</div>
										<div class="text-xs">Version: {vote.version}</div>
									</div>
								</td>
								<td>
									<div class="space-y-1">
										<div>Value: {vote.data.value > 0 ? '✅ +1' : '❌ -1'}</div>
										<div>Weight: {vote.data.weight.toFixed(2)}</div>
									</div>
								</td>
								<td>
									<div class="space-y-1">
										<div>Author: {author ? `${author.data.display_name} (${author.data.username})` : 'Unknown'}</div>
										<div>Target: {target ? `${target.data.display_name} (${target.data.username})` : 'Unknown'}</div>
										<div>Tag: {tag ? tag.data.name : 'No Tag'}</div>
										<div class="font-mono text-xs">Author Key: {vote.data.author_key}</div>
										<div class="font-mono text-xs">Target Key: {vote.data.target_key}</div>
										<div class="font-mono text-xs">Tag Key: {vote.data.tag_key || 'None'}</div>
									</div>
								</td>
								<td>
									<div class="flex gap-2 justify-center">
										<button
											on:click={() => deleteVote(vote.key)}
											class="btn btn-xs btn-error"
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
			<div class="overflow-x-auto">
				<table class="table table-zebra w-full">
					<thead>
						<tr>
							<th>Document Info</th>
							<th>Tag Data</th>
							<th>Time Periods</th>
							<th>Settings</th>
							<th>Actions</th>
						</tr>
					</thead>
					<tbody>
						{#each tags as tag}
							<tr>
								<td>
									<div class="space-y-1">
										<div class="font-mono text-xs">Key: {tag.key}</div>
										<div class="font-mono text-xs">Description: {tag.description}</div>
										<div class="font-mono text-xs">Owner: {tag.owner}</div>
										<div class="text-xs">Created: {new Date(Number(tag.created_at) / 1_000_000).toLocaleString()}</div>
										<div class="text-xs">Updated: {new Date(Number(tag.updated_at) / 1_000_000).toLocaleString()}</div>
										<div class="text-xs">Version: {tag.version}</div>
									</div>
								</td>
								<td>
									<div class="space-y-1">
										<div class="font-bold">{tag.data.name}</div>
										<div class="text-sm opacity-75">{tag.data.description}</div>
										<div class="font-mono text-xs">Author: {tag.data.author_key}</div>
									</div>
								</td>
								<td>
									<ul class="list-disc list-inside">
										{#each tag.data.time_periods as period}
											<li>{period.months}mo: {period.multiplier}x</li>
										{/each}
									</ul>
								</td>
								<td>
									<div class="space-y-1">
										<div>Threshold: {tag.data.reputation_threshold}</div>
										<div>Reward: {tag.data.vote_reward}</div>
										<div>Min Users: {tag.data.min_users_for_threshold}</div>
									</div>
								</td>
								<td>
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

		<!-- Reputation Documents -->
		<div class="mt-8">
			<h2 class="text-xl mb-4">Reputation Documents</h2>
			<div class="overflow-x-auto">
				<table class="table table-zebra w-full">
					<thead>
						<tr>
							<th>Document Info</th>
							<th>Reputation Data</th>
							<th>Actions</th>
						</tr>
					</thead>
					<tbody>
						{#each reputationDocs as doc}
							<tr>
								<td>
									<div class="space-y-1">
										<div class="font-mono text-xs">Key: {doc.key}</div>
										<div class="font-mono text-xs">Description: {doc.description}</div>
										<div class="font-mono text-xs">Owner: {doc.owner}</div>
										<div class="text-xs">Created: {new Date(Number(doc.created_at) / 1_000_000).toLocaleString()}</div>
										<div class="text-xs">Updated: {new Date(Number(doc.updated_at) / 1_000_000).toLocaleString()}</div>
										<div class="text-xs">Version: {doc.version}</div>
									</div>
								</td>
								<td>
									<div class="space-y-1">
										<div>User Key: {doc.data.user_key}</div>
										<div>Tag Key: {doc.data.tag_key}</div>
										<div>Base Rep: {doc.data.total_basis_reputation.toFixed(2)}</div>
										<div>Vote Rep: {doc.data.total_voting_rewards_reputation.toFixed(2)}</div>
										<div>Total Rep: {doc.data.last_known_effective_reputation.toFixed(2)}</div>
										<div>Vote Weight: {(Number(doc.data.vote_weight) * 100).toFixed(4)}%</div>
										<div>Status: {doc.data.has_voting_power ? 'Active' : 'Inactive'}</div>
										<div class="text-xs">Last Calc: {new Date(Number(doc.data.last_calculation) / 1_000_000).toLocaleString()}</div>
									</div>
								</td>
								<td>
									<div class="flex gap-2 justify-center">
										<button
											on:click={() => recalculateReputation(doc.data.user_key, doc.data.tag_key)}
											class="btn btn-xs btn-primary"
											title="Recalculate reputation"
										>
											🔄
										</button>
										<button
											on:click={() => deleteReputationDoc(doc.key)}
											class="btn btn-xs btn-error"
											title="Delete document"
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
	</div>
{:else}
	<div class="container mx-auto p-4">
		<p>Please log in to access the admin interface.</p>
	</div>
{/if} 