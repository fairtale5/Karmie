import { setDoc, type User } from '@junobuild/core';
import { get } from 'svelte/store';
import { authUser } from '../stores/authUser';
import { createUlid } from '../keys/create_ulid';
import { formatUserKey } from '../keys/format_key_user';

/**
 * Creates a new user document in the 'users' collection.
 *
 * @param user_handle - Unique username/handle (required)
 * @param display_name - Display name (optional)
 * @param avatar_url - Profile picture URL (optional)
 *
 * @returns {Promise<void>} Resolves when the user is created, throws on error
 *
 * @example
 * await createUserDoc({ user_handle: 'alice', display_name: 'Alice', avatar_url: 'https://...' });
 *
 * Note: This function uses the global authUser Svelte store for principal lookup.
 */
export async function createUserDoc({
  user_handle,
  display_name = '',
  avatar_url = ''
}: {
  user_handle: string;
  display_name?: string;
  avatar_url: string;
}): Promise<void> {
  // Get the current authenticated user (principal) from the Svelte store
  const user = get(authUser);
  // Then checks if the user or user.key is missing (i.e., not authenticated), and throws an error if so.
  if (!user || !user.key) {
    console.error('[user_create] No authenticated principal found.');
    throw new Error('No authenticated principal found.');
  }
  // Use the user's unique key as their principal identifier.
  const principal = user.key;

  // Generate a new unique user ID (ULID) for this user document.
  const userUlid = createUlid();

  // Build the document key using the principal, ULID, and user handle.
  const userDocKey = formatUserKey(principal, userUlid, user_handle);

  // Prepare the user data object to be stored in the database.
  const userDocData = {
    user_handle,
    display_name,
    avatar_url,
    user_key: userUlid
  };

  // Create the document object for setDoc
  const doc = {
    collection: 'users',
    doc: {
      key: userDocKey,
      data: userDocData
    }
  };

  // Log the document payload for debugging.
  console.log('[user_create] Sending to setDoc:', doc);

  try {
    // Attempt to save the user document; log and rethrow any errors.
    await setDoc(doc);
    console.log(`[user_create] Successfully created user: ${user_handle} (key: ${userDocKey})`);
  } catch (err) {
    console.error(`[user_create] Failed to create user: ${user_handle} (key: ${userDocKey})`, err);
    throw err;
  }
} 