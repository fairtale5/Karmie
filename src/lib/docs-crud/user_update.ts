import { setDoc, type Doc } from '@junobuild/core';
import type { UserData } from '$lib/types';

/**
 * Updates an existing user document in the 'users' collection.
 *
 * @param key - The document key of the user to update (required)
 * @param version - The current version of the document (required by Juno for concurrency control)
 * @param data - The updated user data (UserData interface) containing:
 *   - user_handle: Unique username/handle (required, cannot be changed)
 *   - display_name: Display name (required)
 *   - user_ulid: ULID for this user (required, should not change)
 *   - avatar_url: Avatar URL (required, can be empty string)
 *   - description: User profile description (optional)
 *   See {@link UserData} interface for complete field definitions.
 *
 * @returns {Promise<void>} Resolves when the user is updated, throws on error
 *
 * @example
 * // Internal implementation uses explicit type checking:
 * const doc_user_data: UserData = {
 *   user_handle: 'alice',
 *   display_name: 'Alice', 
 *   user_ulid: 'ABC123',
 *   avatar_url: '',
 *   description: 'Software developer'
 * };
 * const doc_user: Doc<UserData> = {
 *   key: 'user_123',
 *   data: doc_user_data,
 *   version: 1n
 * };
 * await setDoc({
 *   collection: 'users',
 *   doc: doc_user
 * });
 */
export async function updateUserDoc({
  key,
  version,
  data
}: {
  key: string;
  version: bigint;
  data: UserData;
}): Promise<void> {
  // Explicit type checking - catches errors at declaration
  const doc_user_data: UserData = data;
  const doc_user: Doc<UserData> = {
    key,
    data: doc_user_data,
    version
  };

  // Log the document payload for debugging.
  console.log('[user_update] Sending to setDoc:', { collection: 'users', doc: doc_user });

  try {
    // Attempt to update the user document; log and rethrow any errors.
    await setDoc({
      collection: 'users',
      doc: doc_user
    });
  } catch (err) {
    console.error('[user_update] Error updating user:', err);
    throw err;
  }
} 