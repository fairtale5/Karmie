<script lang="ts">
	import { onMount } from 'svelte';
	import {
		listDocs,
		setDoc,
		deleteDoc,
		type Doc,
		authSubscribe,
		type User,
		getDoc,
		signOut,
		getSatelliteExtendedActor
	} from '@junobuild/core';
	import { goto } from '$app/navigation';
	import { REPUTATION_SETTINGS } from '$lib/settings';
	import { idlFactory } from '../../declarations/satellite/satellite.factory.did.js';
	import type { _SERVICE as SatelliteActor } from '../../declarations/satellite/satellite.did';
	import { getUserReputationFull } from '../../declarations/satellite/satellite.api';
	import {
		formatUserKey,
		formatTagKey,
		formatVoteKey,
		formatReputationKey,
		createUlid,
		validateUlid,
		type ULID
	} from '$lib/keys/keys_index';
	import type { Principal } from '@dfinity/principal';
	import type { UserData, TagData, VoteData, ReputationData } from '$lib/types';
	import { initJuno } from '$lib/juno';
	import NotLoggedInAlert from '$lib/components/NotLoggedInAlert.svelte';
	import { authUser, authUserDoneInitializing } from '$lib/stores/authUser';

	// Configuration Constants
	const DEFAULT_VOTE_WEIGHT = 1;
	const DEFAULT_TAG_MULTIPLIERS = [
		{ months: 1, multiplier: 1.5 }, // Period 1: First month
		{ months: 2, multiplier: 1.2 }, // Period 2: Months 2-3
		{ months: 3, multiplier: 1.1 }, // Period 3: Months 4-6
		{ months: 6, multiplier: 1.0 }, // Period 4: Months 7-12
		{ months: 12, multiplier: 0.95 }, // Period 5: Months 13-24
		{ months: 12, multiplier: 0.75 }, // Period 6: Months 25-36
		{ months: 12, multiplier: 0.55 }, // Period 7: Months 37-48
		{ months: 999, multiplier: 0.25 } // Period 8: Months 49+ (treated as infinity)
	];

	// User form data
	let userBeingEdited: Doc<UserData> = {
		key: '',
		description: '',
		owner: '',
		created_at: BigInt(0),
		updated_at: BigInt(0),
		version: BigInt(0),
		data: {
			user_handle: '',
			display_name: '',
			user_key: '' 
		}
	};

	// List of all users
	let users: Doc<any>[] = [];

	// Form data for creating/updating tags
	let tagBeingEdited: Doc<TagData> = {
		key: '',
		description: '',
		owner: '',
		created_at: BigInt(0),
		updated_at: BigInt(0),
		version: BigInt(0),
		data: {
			user_key: '', // Keep as string for now
			tag_key: '', // Keep as string for now
			tag_handle: '',
			description: '',
			time_periods: [...REPUTATION_SETTINGS.DEFAULT_TIME_PERIODS],
			reputation_threshold: REPUTATION_SETTINGS.DEFAULT_TAG.REPUTATION_THRESHOLD,
			vote_reward: REPUTATION_SETTINGS.DEFAULT_TAG.VOTE_REWARD,
			min_users_for_threshold: REPUTATION_SETTINGS.DEFAULT_TAG.MIN_USERS_FOR_THRESHOLD
		}
	};

	// New selectedAuthorKey field for tag creation (mirror's the user document pattern)
	let selectedAuthorKey = '';

	// List of all tags
	let tags: Doc<{
		tag_handle: string;
		description: string;
		user_key: string;
		tag_key: string;
		time_periods: Array<{ months: number; multiplier: number }>;
		reputation_threshold: number;
		vote_reward: number;
		min_users_for_threshold: number;
	}>[] = [];

	// Initialize newVote with proper structure
	let newVote = {
		data: {
			user_key: '',
			target_key: '',
			tag_key: '',
			value: 1,
			weight: 1
		}
	};

	// List of all votes
	let votes: Doc<{
		user_key: string;
		target_key: string;
		tag_key: string;
		value: number;
		weight: number;
	}>[] = [];

	// Error message if something goes wrong
	let errorGlobal = ''; // delete this as soon as possible
	// Success message for feedback
	let successGlobal = ''; // delete this as soon as possible
	// Current authenticated user
	let user: User | null = null;

	// Add selected tag state
	let selectedTag = '';

	// Update the userReputations type to use string keys
	let userReputations: Record<string, ReputationData> = {};

	// Add a variable to store reputation documents
	let reputationDocs: Doc<ReputationData>[] = [];

	// Explicitly type the version variable
	let version: bigint | undefined;

	// Function to load user reputations for selected tag
	async function loadUserReputations(tagKey: string) {
		try {
			console.log('[Admin] Loading reputations for tag:', tagKey);
			
			// Get all users first
			const usersList = await listDocs<UserData>({
				collection: 'users'
			});
			
			// Get the selected tag document to access its data
			const selectedTagDoc = tags.find(tag => tag.key === tagKey);
			if (!selectedTagDoc || !selectedTagDoc.data.tag_key) {
				console.error('[Admin] Tag document not found or missing tag_key:', tagKey);
				return;
			}
			
			// Use key-based filtering with proper pattern using the tag's ULID directly
			// Reputation keys format: usr_{userUlid}_tag_{tagUlid}
			const reputationsList = await listDocs<ReputationData>({
				collection: 'reputations',
				filter: {
					matcher: {
						key: `tag_${selectedTagDoc.data.tag_key}`
					}
				}
			});
			
			// Create a map of user_key to reputation data
			const reputationMap = new Map<string, ReputationData>();
			for (const item of reputationsList.items) {
				if (item.data.user_key) {
					reputationMap.set(item.data.user_key, item.data);
				}
			}
			
			// Get reputation data for each user
			userReputations = {};
			for (const user of usersList.items) {
				if (!user.data.user_key) continue;
				
				const reputation = reputationMap.get(user.data.user_key) || {
					user_key: user.data.user_key,
					tag_key: selectedTagDoc.data.tag_key,
					reputation_basis: 0,
					reputation_rewards: 0,
					reputation_total_effective: 0,
					vote_weight: 0,
					has_voting_power: false,
					last_calculation: BigInt(0)
				};
				userReputations[user.data.user_key] = reputation;
			}
			
			console.log('[Admin] Loaded reputations:', userReputations);
		} catch (error) {
			console.error('[Admin] Error loading reputations:', error);
		}
	}

	// Update onMount to load reputations when tag is selected
	onMount(async () => {
		await initJuno();
		authSubscribe((state) => {
			user = state;
			if (user !== null) {
				// Load data if authenticated
				loadUsers();
				loadVotes(); // Initial load of all votes
				loadTags(); // Initial load of all tags
				loadReputations(); // Initial load of all reputations
			}
			// If not authenticated, do not redirect; allow browsing
		});
	});

	// Watch for tag selection changes
	$: if (selectedTag) {
		// When a tag is selected, load reputations for that tag
		loadUserReputations(selectedTag);
		// Also update votes filtered by this tag
		loadVotes();
		// Load reputations filtered by this tag
		loadReputations(undefined, selectedTag);
	}

	// Load users
	async function loadUsers() {
		try {
			const usersList = await listDocs<{
				user_handle: string;
				display_name: string;
				user_key: string;
			}>({
				collection: 'users'
			});
			users = usersList.items;
			console.log('Loaded users:', users);
		} catch (error) {
			console.error('Error loading users:', error);
		}
	}

	// Load votes
	async function loadVotes() {
		try {
			// If a tag is selected, filter votes by tag using key-based filtering
			// Vote key format: usr_{ulid}_tag_{tagUlid}_tar_{ulid}_key_{ulid}_
			let filter = {};
			
			if (selectedTag) {
				// Get the selected tag document to access its data
				const selectedTagDoc = tags.find(tag => tag.key === selectedTag);
				if (selectedTagDoc && selectedTagDoc.data.tag_key) {
					// Filter votes by tag ULID directly from the data field
					filter = {
						matcher: {
							key: `tag_${selectedTagDoc.data.tag_key}`
						}
					};
				}
			}
			
			const votesList = await listDocs<{
				user_key: string;
				target_key: string;
				tag_key: string;
				value: number;
				weight: number;
			}>({
				collection: 'votes',
				...(Object.keys(filter).length > 0 && { filter })
			});
			
			console.log('[Admin] Loaded votes:', votesList.items);
			votes = votesList.items;
		} catch (error) {
			console.error('[Admin] Error loading votes:', error);
		}
	}

	// Load tags
	async function loadTags(userKey?: string) {
		try {
			// If a user key is provided, filter tags by user using key-based filtering
			// Tag key format: usr_{userUlid}_tag_{tagUlid}_hdl_{tagHandle}_
			let filter = {};
			
			if (userKey) {
				// Get the user document to access its data
				const userDoc = users.find(user => user.key === userKey);
				if (userDoc && userDoc.data.user_key) {
					// Filter tags by user ULID directly from the data field
					filter = {
						matcher: {
							key: `usr_${userDoc.data.user_key}`
						}
					};
				}
			}
			
			const tagsList = await listDocs<{ 
				tag_handle: string; 
				description: string; 
				user_key: string;
				tag_key: string;
				time_periods: { months: number; multiplier: number }[];
				reputation_threshold: number;
				vote_reward: number;
				min_users_for_threshold: number;
			}>({
				collection: 'tags',
				...(Object.keys(filter).length > 0 && { filter })
			});
			
			console.log('[Admin] Loaded tags:', tagsList.items);
			tags = tagsList.items;
		} catch (error) {
			console.error('[Admin] Error loading tags:', error);
		}
	}

	// Load reputation documents
	async function loadReputations(userKey?: string, tagKey?: string) {
		try {
			// If a user key or tag key is provided, filter reputations
			// Reputation key format: usr_{userUlid}_tag_{tagUlid}
			let filter = {};
			
			if (userKey) {
				// Get the user document to access its data
				const userDoc = users.find(user => user.key === userKey);
				if (userDoc && userDoc.data.user_key) {
					// Filter reputations by user ULID directly from the data field
					filter = {
						matcher: {
							key: `usr_${userDoc.data.user_key}`
						}
					};
				}
			} else if (tagKey) {
				// Get the tag document to access its data
				const tagDoc = tags.find(tag => tag.key === tagKey);
				if (tagDoc && tagDoc.data.tag_key) {
					// Filter reputations by tag ULID directly from the data field
					filter = {
						matcher: {
							key: `tag_${tagDoc.data.tag_key}`
						}
					};
				}
			}
			
			const reputationsList = await listDocs<ReputationData>({
				collection: 'reputations',
				...(Object.keys(filter).length > 0 && { filter })
			});
			
			console.log('[Admin] Loaded reputation documents:', reputationsList.items);
			reputationDocs = reputationsList.items;
		} catch (error) {
			console.error('[Admin] Error loading reputation documents:', error);
		}
	}

	// User form data type definition
	interface DocData {
		collection: string;
		doc: {
			key: string;
			data: {
				user_handle: string;
				display_name: string;
			};
			description: string;
			version?: bigint;
		};
	}

	// Function to load user data for editing
	function editUser(userDocSelected: Doc<UserData>) {
		userBeingEdited = userDocSelected;
		// Scroll to form
		document.getElementById('userForm')?.scrollIntoView({ behavior: 'smooth' });
	}

	/**
	 * Creates or updates a user in the Juno collection
	 * @throws {Error} If saving fails
	 */
	async function saveUser() {
		try {
			// For new documents: generate key with ULID
			const userDocUserHandle = userBeingEdited.data.user_handle!.toString().trim();
			const userDocKeyResult = !userBeingEdited.key ? createUlid() : null;
			const userDocumentKey =
				userBeingEdited.key || formatUserKey(user?.key ?? '', userDocKeyResult!, userDocUserHandle);
			const userDocVersion = userBeingEdited.version;

			// Create the document data with required fields
			const docData = {
				collection: 'users',
				doc: {
					key: userDocumentKey,
					data: {
						user_handle: userBeingEdited.data.user_handle!.toString().trim(),
						display_name: userBeingEdited.data.display_name!.toString().trim(),
						user_key: userDocKeyResult || userBeingEdited.data.user_key
					},
					...(userDocVersion && { version: userDocVersion })
				}
			};

			// Log the exact data being sent to setDoc
			console.log('[Admin] Sending to setDoc:', docData);

			// Create or update user document
			await setDoc(docData);

			// Clear form and show success message
			userBeingEdited = {
				key: '',
				description: '',
				owner: '',
				created_at: BigInt(0),
				updated_at: BigInt(0),
				version: BigInt(0),
				data: {
					user_handle: '',
					display_name: '',
					user_key: '' // Keep as string for now
				}
			};
			successGlobal = 'User saved successfully';

			// Reload users list
			await loadUsers();
		} catch (e) {
			console.error('[Admin] Error saving user:', e);
			
			// Enhanced error handling
			if (e instanceof Error) {
				if (e.message.includes('user_handle')) {
					errorGlobal = 'This user-handle is already taken. Please choose a different one.';
				} else if (e.message.includes('Invalid user data format')) {
					errorGlobal = 'Invalid user data format. Please check your input and try again.';
				} else {
					errorGlobal = e.message;
				}
			} else {
				errorGlobal = 'Failed to save user. Please try again.';
			}
		}
	}

	/**
	 * Loads tag data into the form for editing
	 * @param tagDoc - The tag document to edit
	 */
	function editTag(
		tagDoc: Doc<{
			tag_handle: string;
			description: string;
			user_key: string;
			tag_key: string;
			author_key?: string;
			time_periods: Array<{ months: number; multiplier: number }>;
			reputation_threshold: number;
			vote_reward: number;
			min_users_for_threshold: number;
		}>
	) {
		// Clone tag document into tagBeingEdited
		tagBeingEdited = {
			key: tagDoc.key,
			description: tagDoc.description,
			owner: tagDoc.owner,
			created_at: tagDoc.created_at,
			updated_at: tagDoc.updated_at,
			version: tagDoc.version,
			data: {
				tag_handle: tagDoc.data.tag_handle,
				description: tagDoc.data.description,
				user_key: tagDoc.data.user_key, // Copy the user_key field if it exists
				tag_key: tagDoc.data.tag_key, // Copy the tag_key field if it exists
				time_periods: [...tagDoc.data.time_periods],
				reputation_threshold: tagDoc.data.reputation_threshold,
				vote_reward: tagDoc.data.vote_reward,
				min_users_for_threshold: tagDoc.data.min_users_for_threshold
			}
		};

		// Scroll to form
		document.getElementById('tagForm')?.scrollIntoView({ behavior: 'smooth' });
	}

	/**
	 * Creates or updates a tag in the Juno collection
	 * @throws {Error} If saving fails
	 */
	async function saveTag() {
		try {
			console.log('[Admin] Saving tag:', tagBeingEdited);
			
			// Validate inputs
			if (!tagBeingEdited.data.tag_handle || !tagBeingEdited.data.description || !selectedAuthorKey) {
				errorGlobal = 'Please fill in all required fields, including selecting an author';
				return;
			}

			// Find the selected user to get their user_key
			const selectedUser = users.find(u => u.key === selectedAuthorKey);
			if (!selectedUser || !selectedUser.data.user_key) {
				errorGlobal = 'Selected user not found or missing user key';
				return;
			}

			// For new documents: generate tag ULID and format key
			let tagDocKey: string;
			let tagDocUlid: string | null = null;
			let tagDocVersion = tagBeingEdited.version;

			if (tagBeingEdited.key) {
				// If tagBeingEdited.key is not null, we know we're updating an existing tag
				
				// For updates, use existing tag_key
				tagDocUlid = tagBeingEdited.data.tag_key || null;
			} else {
				// Creating new tag - generate new ULID for the tag
				tagDocUlid = createUlid();
				const selectedUser = users.find(u => u.key === selectedAuthorKey);
				if (!selectedUser?.data?.user_key) {
					throw new Error('Selected user not found or missing user_key');
				}
				if (!tagDocUlid) {
					throw new Error('Tag ULID key is missing');
				}
				tagDocKey = formatTagKey(
					selectedUser.data.user_key,
					tagDocUlid,
					tagBeingEdited.data.tag_handle
				);
			}

			// Create the document data with required fields
			const docData = {
				collection: 'tags',
				doc: {
					key: tagDocKey!,
					data: {
						name: tagBeingEdited.data.tag_handle,
						description: tagBeingEdited.data.description,
						user_key: selectedUser.data.user_key,
						tag_key: tagDocUlid || tagBeingEdited.data.tag_key,
						time_periods: tagBeingEdited.data.time_periods,
						reputation_threshold: tagBeingEdited.data.reputation_threshold,
						vote_reward: tagBeingEdited.data.vote_reward,
						min_users_for_threshold: tagBeingEdited.data.min_users_for_threshold
					},
					...(tagDocVersion && tagDocVersion > BigInt(0) && { version: tagDocVersion })
				}
			};

			// Log the exact data being sent to setDoc
			console.log('[Admin] Sending to setDoc:', docData);

			// Create or update tag document
			await setDoc(docData);

			// Clear form and show success message
			tagBeingEdited = {
				key: '',
				description: '',
				owner: '',
				created_at: BigInt(0),
				updated_at: BigInt(0),
				version: BigInt(0),
				data: {
					user_key: '',
					tag_key: '',
					tag_handle: '',
					description: '',
					time_periods: [...REPUTATION_SETTINGS.DEFAULT_TIME_PERIODS],
					reputation_threshold: REPUTATION_SETTINGS.DEFAULT_TAG.REPUTATION_THRESHOLD,
					vote_reward: REPUTATION_SETTINGS.DEFAULT_TAG.VOTE_REWARD,
					min_users_for_threshold: REPUTATION_SETTINGS.DEFAULT_TAG.MIN_USERS_FOR_THRESHOLD
				}
			};
			selectedAuthorKey = '';
			successGlobal = 'Tag saved successfully';
			errorGlobal = '';

			// Reload tags list
			await loadTags();
		} catch (e) {
			console.error('[Admin] Error saving tag:', e);
			
			// Enhanced error handling
			if (e instanceof Error) {
				if (e.message.includes('Tag name')) {
					errorGlobal = 'This tag name is already taken. Please choose a different one.';
				} else if (e.message.includes('Invalid ULID')) {
					errorGlobal = 'Invalid ULID format detected. Please check user selection.';
				} else {
					errorGlobal = e.message;
				}
			} else {
				errorGlobal = 'Failed to save tag. Please try again.';
			}
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
			errorGlobal = '';
			successGlobal = '';

			// Get the current version of the tag
			const existingDoc = await getDoc({
				collection: 'tags',
				key
			});

			if (!existingDoc) {
				errorGlobal = 'Tag not found';
				return;
			}

			await deleteDoc({
				collection: 'tags',
				doc: {
					key,
					data: {},
					version: existingDoc.version
				}
			});

			successGlobal = 'Tag deleted successfully!';
			await loadTags();
		} catch (e) {
			console.error('Error deleting tag:', e);
			errorGlobal = e instanceof Error ? e.message : 'Failed to delete tag';
		}
	}

	// Function to delete a user
	async function deleteUser(key: string) {
		if (!confirm('Are you sure you want to delete this user?')) {
			return;
		}

		try {
			errorGlobal = '';
			successGlobal = '';

			// Get the current version of the user
			const existingDoc = await getDoc({
				collection: 'users',
				key
			});

			if (!existingDoc) {
				errorGlobal = 'User not found';
				return;
			}

			await deleteDoc({
				collection: 'users',
				doc: {
					key,
					data: {},
					version: existingDoc.version
				}
			});

			successGlobal = 'User deleted successfully!';
			await loadUsers();
		} catch (e) {
			console.error('Error deleting user:', e);
			errorGlobal = e instanceof Error ? e.message : 'Failed to delete user';
		}
	}

	/**
	 * Creates a new vote in the Juno collection
	 * @throws {Error} If saving fails
	 */
	async function saveVote() {
		try {
			console.log('[Admin] Saving vote:', newVote);
			
			// Validate inputs
			if (!newVote.data.user_key || !newVote.data.target_key || !newVote.data.tag_key) {
				errorGlobal = 'Please select author, target user, and tag';
				return;
			}

			// Find the user and target documents to get their user_key values
			const authorDoc = users.find(u => u.key === newVote.data.user_key);
			const targetDoc = users.find(u => u.key === newVote.data.target_key);
			const tagDoc = tags.find(t => t.key === newVote.data.tag_key);

			if (!authorDoc?.data.user_key || !targetDoc?.data.user_key || !tagDoc?.data.tag_key) {
				errorGlobal = 'Could not find required keys for author, target, or tag';
				return;
			}

			// Generate a new ULID for the vote
			const voteUlid = createUlid();

			// Format the vote key using all required ULIDs
			const voteKey = formatVoteKey(
				authorDoc.data.user_key,
				tagDoc.data.tag_key,
				targetDoc.data.user_key,
				voteUlid
			);

			// Create vote document with proper structure
			const docData = {
				collection: 'votes',
				doc: {
					key: voteKey,
					data: {
						user_key: authorDoc.data.user_key,
						target_key: targetDoc.data.user_key,
						tag_key: tagDoc.data.tag_key,
						vote_key: voteUlid,
						value: newVote.data.value,
						weight: 1 // Fixed weight for now
					}
				}
			};

			console.log('[Admin] Creating vote document:', docData);
			await setDoc(docData);

			// Clear form and show success message
			newVote = {
				data: {
					user_key: '',
					target_key: '',
					tag_key: '',
					value: 1,
					weight: 1
				}
			};
			successGlobal = 'Vote created successfully!';

			// Reload the vote list
			await loadVotes();
		} catch (e) {
			console.error('[Admin] Error saving vote:', e);
			errorGlobal = e instanceof Error ? e.message : 'Failed to save vote';
		}
	}

	// Function to delete a vote
	async function deleteVote(voteKey: string) {
		if (!confirm('Are you sure you want to delete this vote?')) {
			return;
		}

		try {
			errorGlobal = '';
			successGlobal = '';

			// Get the current version of the vote
			const existingDoc = await getDoc({
				collection: 'votes',
				key: voteKey
			});

			if (!existingDoc) {
				errorGlobal = 'Vote not found';
				return;
			}

			await deleteDoc({
				collection: 'votes',
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

			successGlobal = 'Vote deleted successfully';
		} catch (err) {
			console.error('Error deleting vote:', err);
			errorGlobal = 'Failed to delete vote';
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
			
			successGlobal = `Recalculated reputation for user ${userKey}`;
			errorGlobal = '';
		} catch (e) {
			console.error('[Admin] Error recalculating reputation:', e);
			errorGlobal = e instanceof Error ? e.message : 'Failed to recalculate reputation';
			successGlobal = '';
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
				successGlobal = `Recalculated reputation for ${userKey}`;
			} else {
				errorGlobal = `Failed to recalculate: ${result.Err}`;
			}
		} catch (e) {
			errorGlobal = `Error recalculating reputation: ${e}`;
			console.error('[Admin] Recalculation error:', e);
		}
	}

	// Add a function to delete reputation documents
	async function deleteReputationDoc(key: string) {
		if (!confirm('Are you sure you want to delete this reputation document?')) {
			return;
		}

		try {
			errorGlobal = '';
			successGlobal = '';

			// Get the current version of the document
			const existingDoc = await getDoc({
				collection: 'reputations',
				key
			});

			if (!existingDoc) {
				errorGlobal = 'Reputation document not found';
				return;
			}

			await deleteDoc({
				collection: 'reputations',
				doc: {
					key,
					data: {},
					version: existingDoc.version
				}
			});

			successGlobal = 'Reputation document deleted successfully!';
			await loadReputations();
		} catch (e) {
			console.error('Error deleting reputation document:', e);
			errorGlobal = e instanceof Error ? e.message : 'Failed to delete reputation document';
		}
	}
</script>

<!-- Show warning if not logged in -->
<NotLoggedInAlert />

{#if !user && !$authUserDoneInitializing}
	<!-- Loading placeholder for admin page -->
	<div class="container mx-auto p-4 animate-pulse">
		<div class="h-8 bg-surface-300-700 rounded w-1/2 mb-4"></div>
		<div class="h-4 bg-surface-200-800 rounded w-3/4 mb-2"></div>
		<div class="h-4 bg-surface-200-800 rounded w-2/3 mb-2"></div>
		<div class="h-10 bg-surface-200-800 rounded w-full mb-4"></div>
		<div class="h-10 bg-surface-200-800 rounded w-full"></div>
	</div>
{:else}
	<div class="container mx-auto p-4">
		<!-- Admin Tools Section -->
		<div class="bg-base-200 mb-8 rounded-lg p-4">
			<h2 class="mb-4 text-2xl font-bold">Admin Tools</h2>
			{#if successGlobal}
				<div class="alert alert-success mt-4">
					<span>{successGlobal}</span>
				</div>
			{/if}
			{#if errorGlobal}
				<div class="alert alert-error mt-4">
					<span>{errorGlobal}</span>
				</div>
			{/if}
		</div>

		<div class="mb-8 flex items-center justify-between">
			<h1 class="text-2xl">Admin Dashboard</h1>
			<button
				on:click={() => signOut()}
				class="rounded bg-red-500 px-4 py-2 text-white hover:bg-red-600"
			>
				Log Out
			</button>
		</div>

		<!-- Tag Selector -->
		<div class="mb-8">
			<label for="tag-select" class="mb-2 block text-lg">Select Tag to Filter Data:</label>
			<select id="tag-select" bind:value={selectedTag} class="w-full max-w-md border p-2">
				<option value="">Show all data</option>
				{#each tags as tag}
					<option value={tag.key}>
						{tag.data.tag_handle}
					</option>
				{/each}
			</select>
		</div>

		{#if selectedTag}
			<!-- Selected Tag Details -->
			{@const selectedTagDoc = tags.find((t) => t.key === selectedTag)}
			{#if selectedTagDoc}
				<div class="mb-8">
					<h2 class="mb-4 text-xl">Selected Tag Details</h2>
					<div class="overflow-x-auto">
						<table class="table-zebra table w-full">
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
									<td>{new Date(Number(selectedTagDoc.created_at) / 1_000_000).toLocaleString()}</td
									>
								</tr>
								<tr>
									<td class="font-bold">Updated At</td>
									<td>{new Date(Number(selectedTagDoc.updated_at) / 1_000_000).toLocaleString()}</td
									>
								</tr>
								<tr>
									<td class="font-bold">Version</td>
									<td>{selectedTagDoc.version}</td>
								</tr>
								<tr>
									<td class="font-bold">Name</td>
									<td>{selectedTagDoc.data.tag_handle}</td>
								</tr>
								<tr>
									<td class="font-bold">Description</td>
									<td>{selectedTagDoc.data.description}</td>
								</tr>
								<tr>
									<td class="font-bold">Time Periods</td>
									<td>
										<ul class="list-inside list-disc">
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
			<h2 class="mb-4 text-xl">{userBeingEdited.key ? 'Update User' : 'Create New User'}</h2>
			<form on:submit|preventDefault={saveUser} class="space-y-4">
				{#if userBeingEdited.key}
					<div>
						<label for="key" class="block">User Key:</label>
						<input
							type="text"
							id="key"
							bind:value={userBeingEdited.key}
							class="w-full border bg-gray-100 p-2"
							readonly
						/>
					</div>
				{/if}

				<div>
					<label for="user_handle" class="block">Username Handle:</label>
					<input
						type="text"
						id="user_handle"
						bind:value={userBeingEdited.data.user_handle}
						class="w-full border p-2"
						placeholder="Enter username"
					/>
				</div>

				<div>
					<label for="display_name" class="block">Display Name:</label>
					<input
						type="text"
						id="display_name"
						bind:value={userBeingEdited.data.display_name}
						class="w-full border p-2"
						placeholder="e.g., John Doe"
					/>
				</div>

				<div class="flex gap-4">
					<button type="submit" class="rounded bg-blue-500 px-4 py-2 text-white">
						{userBeingEdited.key ? 'Update User' : 'Create User'}
					</button>
					{#if userBeingEdited.key}
						<button
							type="button"
							class="rounded bg-gray-500 px-4 py-2 text-white"
							on:click={() => {
								userBeingEdited = {
									key: '',
									description: '',
									owner: '',
									created_at: BigInt(0),
									updated_at: BigInt(0),
									version: BigInt(0),
									data: {
										user_handle: '',
										display_name: '',
										user_key: '' // Keep as string for now
									}
								};
							}}
						>
							Cancel Edit
						</button>
					{/if}
				</div>
			</form>

			{#if errorGlobal}
				<div class="mt-2 text-red-500">{errorGlobal}</div>
			{/if}
			{#if successGlobal}
				<div class="mt-2 text-green-500">{successGlobal}</div>
			{/if}
		</div>

		<!-- Users with Reputation -->
		<div class="mb-8">
			<h2 class="mb-4 text-xl">Users and Their Reputation</h2>
			<div class="overflow-x-auto">
				<table class="table-zebra table w-full">
					<thead>
						<tr>
							<th>Document Info</th>
							<th>User Data</th>
							{#if selectedTag}
								<th>Reputation Data</th>
							{/if}
							<th>Actions</th>
						</tr>
					</thead>
					<tbody>
						{#each users as userSelected}
							{@const reputation = userReputations[userSelected.data.user_key] ?? {
								reputation_basis: 0,
								reputation_rewards: 0,
								reputation_total_effective: 0,
								vote_weight: 0,
								has_voting_power: false,
								last_calculation: BigInt(0)
							}}
							<tr>
								<td>
									<div class="space-y-1">
										<div class="font-mono text-xs">Key: {userSelected.key}</div>
										<div class="font-mono text-xs">Description: {userSelected.description}</div>
										<div class="font-mono text-xs">Owner: {userSelected.owner}</div>
										<div class="text-xs">
											Created: {new Date(Number(userSelected.created_at) / 1_000_000).toLocaleString()}
										</div>
										<div class="text-xs">
											Updated: {new Date(Number(userSelected.updated_at) / 1_000_000).toLocaleString()}
										</div>
										<div class="text-xs">Version: {userSelected.version}</div>
									</div>
								</td>
								<td>
									<div class="space-y-1">
										<div class="font-bold">{userSelected.data.user_handle}</div>
										<div class="text-sm opacity-75">{userSelected.data.display_name}</div>
									</div>
								</td>
								{#if selectedTag}
									<td>
										<div class="space-y-1">
											<div>Base Rep: {reputation.reputation_basis.toFixed(2)}</div>
											<div>Vote Rep: {reputation.reputation_rewards.toFixed(2)}</div>
											<div>Total Rep: {reputation.reputation_total_effective.toFixed(2)}</div>
											<div>Weight: {(Number(reputation.vote_weight) * 100).toFixed(4)}%</div>
											<div>Status: {reputation.has_voting_power ? 'Active' : 'Inactive'}</div>
											<div class="text-xs">
												Last Calc: {new Date(Number(reputation.last_calculation) / 1_000_000).toLocaleString()}
											</div>
										</div>
									</td>
								{/if}
								<td>
									<div class="flex justify-center gap-2">
										<button
											class="btn btn-xs btn-primary"
											on:click={() => recalculateReputation(userSelected.data.user_key, selectedTag)}
											title="Recalculate reputation"
										>
											🔄
										</button>
										<button
											class="btn btn-xs btn-info"
											on:click={() => editUser(userSelected)}
											title="Edit user"
										>
											✏️
										</button>
										<button
											class="btn btn-xs btn-error"
											on:click={() => deleteUser(userSelected.key)}
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
			<h2 class="mb-4 text-xl">Create New Vote</h2>
			<form on:submit|preventDefault={saveVote} class="space-y-4">
				<div>
					<label for="author" class="block">Author (User Key):</label>
					<select id="author" bind:value={newVote.data.user_key} class="w-full border p-2">
						<option value="">Select Author</option>
						{#each users as user}
							<option value={user.key}>
								{user.data.display_name} ({user.data.user_handle}) - {user.data.user_key}
							</option>
						{/each}
					</select>
				</div>

				<div>
					<label for="target" class="block">Target (User Key):</label>
					<select id="target" bind:value={newVote.data.target_key} class="w-full border p-2">
						<option value="">Select Target</option>
						{#each users as user}
							<option value={user.key}>
								{user.data.display_name} ({user.data.user_handle}) - {user.data.user_key}
							</option>
						{/each}
					</select>
				</div>

				<div>
					<label for="tag" class="block">Tag:</label>
					<select id="tag" bind:value={newVote.data.tag_key} class="w-full border p-2" required>
						<option value="">Select Tag</option>
						{#each tags as tag}
							<option value={tag.key}>
								{tag.data.tag_handle} - {tag.data.tag_key}
							</option>
						{/each}
					</select>
				</div>

				<div>
					<fieldset>
						<legend class="block mb-2">Vote Type:</legend>
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
					<button type="submit" class="rounded bg-blue-500 px-4 py-2 text-white">
						Create Vote
					</button>
				</div>
			</form>
		</div>

		<!-- All Votes -->
		<div class="mb-8">
			<h2 class="mb-4 text-xl">{selectedTag ? 'Votes in Selected Tag' : 'All Votes'}</h2>
			<div class="overflow-x-auto">
				<table class="table-zebra table w-full">
					<thead>
						<tr>
							<th>Document Info</th>
							<th>Vote Data</th>
							<th>References</th>
							<th>Actions</th>
						</tr>
					</thead>
					<tbody>
						{#each votes as vote}
							{@const author = users.find((u) => u.data.user_key === vote.data.user_key)}
							{@const target = users.find((u) => u.data.user_key === vote.data.target_key)}
							{@const tag = tags.find((t) => t.data.tag_key === vote.data.tag_key)}
							<tr>
								<td>
									<div class="space-y-1">
										<div class="font-mono text-xs">Key: {vote.key}</div>
										<div class="font-mono text-xs">Owner: {vote.owner}</div>
										<div class="text-xs">
											Created: {new Date(Number(vote.created_at) / 1_000_000).toLocaleString()}
										</div>
										<div class="text-xs">
											Updated: {new Date(Number(vote.updated_at) / 1_000_000).toLocaleString()}
										</div>
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
										<div>
											Author: {author
												? `${author.data.display_name} (${author.data.user_handle})`
												: 'Unknown'}
										</div>
										<div>
											Target: {target
												? `${target.data.display_name} (${target.data.user_handle})`
												: 'Unknown'}
										</div>
										<div>Tag: {tag ? tag.data.tag_handle : 'No Tag'}</div>
										<div class="font-mono text-xs">Author Key: {vote.data.user_key}</div>
										<div class="font-mono text-xs">Target Key: {vote.data.target_key}</div>
										<div class="font-mono text-xs">Tag Key: {vote.data.tag_key || 'None'}</div>
									</div>
								</td>
								<td>
									<div class="flex justify-center gap-2">
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
			<h2 class="mb-4 text-xl">{tagBeingEdited.key ? 'Update Tag' : 'Create New Tag'}</h2>
			<form on:submit|preventDefault={saveTag} class="space-y-4">
				{#if tagBeingEdited.key}
					<div>
						<label for="tagKey" class="block">Tag Key:</label>
						<input
							type="text"
							id="tagKey"
							bind:value={tagBeingEdited.key}
							class="w-full border bg-gray-100 p-2"
							readonly
						/>
					</div>
				{/if}

				<!-- Add author selection dropdown -->
				<div>
					<label for="tagAuthor" class="block text-sm font-medium text-gray-700">
						Tag Author
					</label>
					<select
						id="tagAuthor"
						bind:value={selectedAuthorKey}
						class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
						required
					>
						<option value="">Select Author</option>
						{#each users as user}
							<option value={user.key}>
								{user.data.display_name} ({user.data.user_handle})
							</option>
						{/each}
					</select>
					<p class="mt-1 text-sm text-gray-500">Select the user who will be the author of this tag</p>
				</div>

				<div>
					<label for="tagName" class="block">Tag Name:</label>
					<input
						type="text"
						id="tagName"
						bind:value={tagBeingEdited.data.tag_handle}
						class="w-full border p-2"
						placeholder="e.g., Technical Skills"
					/>
				</div>

				<div>
					<label for="tagDescription" class="block">Description:</label>
					<textarea
						id="tagDescription"
						bind:value={tagBeingEdited.data.description}
						class="w-full border p-2"
						placeholder="Describe what this tag represents"
						rows="3"
					></textarea>
				</div>

				<div>
					<label for="time-periods" class="mb-2 block">Time Period Multipliers:</label>
					<div id="time-periods" class="space-y-2">
						<table class="w-full border-collapse">
							<thead>
								<tr>
									<th class="w-1/6 border p-2 text-left">Period</th>
									<th class="w-2/6 border p-2 text-left">Months</th>
									<th class="w-2/6 border p-2 text-left">Multiplier</th>
									<th class="w-1/6 border p-2 text-left">Actions</th>
								</tr>
							</thead>
							<tbody>
								{#each tagBeingEdited.data.time_periods as period, i}
									<tr>
										<td class="border p-2">Period {i + 1}</td>
										<td class="border p-2">
											<input
												type="number"
												id="months-{i}"
												bind:value={period.months}
												class="w-full rounded-md border-gray-300 px-2 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
												min="1"
												max={i === tagBeingEdited.data.time_periods.length - 1 ? 999 : 12}
											/>
										</td>
										<td class="border p-2">
											<input
												type="number"
												id="multiplier-{i}"
												bind:value={period.multiplier}
												class="w-full rounded-md border-gray-300 px-2 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
												min="0"
												max="2"
												step="0.05"
											/>
										</td>
										<td class="border p-2 text-center">
											{#if i === tagBeingEdited.data.time_periods.length - 1}
												<button
													type="button"
													on:click={() => {
														tagBeingEdited.data.time_periods = [
															...tagBeingEdited.data.time_periods,
															{ months: 12, multiplier: 1.0 }
														];
													}}
													class="inline-flex items-center rounded border border-transparent bg-blue-100 px-3 py-1 text-sm text-blue-700 hover:bg-blue-200 focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:outline-none"
												>
													Add Period
												</button>
											{:else}
												<button
													type="button"
													on:click={() => {
														tagBeingEdited.data.time_periods = tagBeingEdited.data.time_periods.filter(
															(_, index) => index !== i
														);
													}}
													class="inline-flex items-center rounded border border-transparent bg-red-100 px-3 py-1 text-sm text-red-700 hover:bg-red-200 focus:ring-2 focus:ring-red-500 focus:ring-offset-2 focus:outline-none"
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
					<label for="reputation_threshold" class="block text-sm font-medium text-gray-700"
						>Reputation Threshold</label
					>
					<input
						type="number"
						id="reputation_threshold"
						bind:value={tagBeingEdited.data.reputation_threshold}
						step="1"
						min="0"
						class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
					/>
				</div>
				<div class="mb-4">
					<label for="vote_reward" class="block text-sm font-medium text-gray-700"
						>Vote Reward</label
					>
					<input
						type="number"
						id="vote_reward"
						bind:value={tagBeingEdited.data.vote_reward}
						step="0.1"
						min="0"
						class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
					/>
				</div>
				<div class="mb-4">
					<label for="min_users_for_threshold" class="block text-sm font-medium text-gray-700"
						>Minimum Users for Threshold</label
					>
					<input
						type="number"
						id="min_users_for_threshold"
						bind:value={tagBeingEdited.data.min_users_for_threshold}
						step="1"
						min="1"
						class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
					/>
					<p class="mt-1 text-sm text-gray-500">
						Number of users that need to reach threshold before vote rewards are restricted
					</p>
				</div>

				<div class="flex gap-4">
					<button type="submit" class="rounded bg-blue-500 px-4 py-2 text-white">
						{tagBeingEdited.key ? 'Update Tag' : 'Create Tag'}
					</button>
					{#if tagBeingEdited.key}
						<button
							type="button"
							class="rounded bg-gray-500 px-4 py-2 text-white"
							on:click={() => {
								tagBeingEdited = {
									key: '',
									description: '',
									owner: '',
									created_at: BigInt(0),
									updated_at: BigInt(0),
									version: BigInt(0),
									data: {
										user_key: '',
										tag_key: '',
										tag_handle: '',
										description: '',
										time_periods: [...REPUTATION_SETTINGS.DEFAULT_TIME_PERIODS],
										reputation_threshold: REPUTATION_SETTINGS.DEFAULT_TAG.REPUTATION_THRESHOLD,
										vote_reward: REPUTATION_SETTINGS.DEFAULT_TAG.VOTE_REWARD,
										min_users_for_threshold: REPUTATION_SETTINGS.DEFAULT_TAG.MIN_USERS_FOR_THRESHOLD
									}
								};
								selectedAuthorKey = '';
							}}
						>
							Cancel Edit
						</button>
					{/if}
				</div>
			</form>

			{#if errorGlobal}
				<div class="mt-2 text-red-500">{errorGlobal}</div>
			{/if}
			{#if successGlobal}
				<div class="mt-2 text-green-500">{successGlobal}</div>
			{/if}
		</div>

		<!-- Tag List -->
		<div>
			<h2 class="mb-4 text-xl">Existing Tags</h2>
			<div class="overflow-x-auto">
				<table class="table-zebra table w-full">
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
										<div class="text-xs">
											Created: {new Date(Number(tag.created_at) / 1_000_000).toLocaleString()}
										</div>
										<div class="text-xs">
											Updated: {new Date(Number(tag.updated_at) / 1_000_000).toLocaleString()}
										</div>
										<div class="text-xs">Version: {tag.version}</div>
									</div>
								</td>
								<td>
									<div class="space-y-1">
										<div class="font-bold">{tag.data.tag_handle}</div>
										<div class="text-sm opacity-75">{tag.data.description}</div>
										<div class="font-mono text-xs">User ULID: {tag.data.user_key}
										</div>
										{#if tag.data.tag_key}
											<div class="font-mono text-xs">Tag ULID: {tag.data.tag_key}</div>
										{/if}
									</div>
								</td>
								<td>
									<ul class="list-inside list-disc">
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
									<div class="flex justify-center gap-2">
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
			<h2 class="mb-4 text-xl">Reputation Documents</h2>
			<div class="overflow-x-auto">
				<table class="table-zebra table w-full">
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
										<div class="font-mono text-xs">Owner: {doc.owner}</div>
										<div class="text-xs">
											Created: {new Date(Number(doc.created_at) / 1_000_000).toLocaleString()}
										</div>
										<div class="text-xs">
											Updated: {new Date(Number(doc.updated_at) / 1_000_000).toLocaleString()}
										</div>
										<div class="text-xs">Version: {doc.version}</div>
									</div>
								</td>
								<td>
									<div class="space-y-1">
										<div>User Key: {doc.data.user_key || 'N/A'}</div>
										<div>Tag Key: {doc.data.tag_key || 'N/A'}</div>
										<div>Base Rep: {(doc.data.reputation_basis || 0).toFixed(2)}</div>
										<div>Vote Rep: {(doc.data.reputation_rewards || 0).toFixed(2)}</div>
										<div>Total Rep: {(doc.data.reputation_total_effective || 0).toFixed(2)}</div>
										<div>Vote Weight: {((Number(doc.data.vote_weight) || 0) * 100).toFixed(4)}%</div>
										<div>Status: {doc.data.has_voting_power ? 'Active' : 'Inactive'}</div>
										<div class="text-xs">
											Last Calc: {new Date(Number(doc.data.last_calculation || 0) / 1_000_000).toLocaleString()}
										</div>
									</div>
								</td>
								<td>
									<div class="flex justify-center gap-2">
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
{/if} 
