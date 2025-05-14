import { setDoc } from '@junobuild/core';

/**
 * Updates an existing user document in the 'users' collection.
 *
 * @param key - The document key of the user to update (required)
 * @param version - The current version of the document (required by Juno for concurrency control)
 * @param user_handle - Updated username/handle (required)
 * @param display_name - Updated display name (optional)
 * @param description - Updated user profile description (optional)
 * @param avatar_url - Updated profile picture URL (optional)
 * @param user_key - The user's ULID (required, should not change)
 *
 * @returns {Promise<void>} Resolves when the user is updated, throws on error
 *
 * @example
 * await updateUserDoc({ key, version, user_handle: 'alice', display_name: 'Alice', ... });
 */
export async function updateUserDoc({
  key,
  version,
  user_handle,
  display_name = '',
  description = '',
  avatar_url = '',
  user_key
}: {
  key: string;
  version: bigint;
  user_handle: string;
  display_name?: string;
  description?: string;
  avatar_url?: string;
  user_key: string;
}): Promise<void> {
  // Prepare and assemblethe updated user data object with all new field values.
  const userDocData = {
    user_handle,
    display_name,
    description,
    avatar_url,
    user_key
  };

  // Create the document object for setDoc, including the key and version for concurrency control.
  const doc = {
    collection: 'users',
    doc: {
      key,
      data: userDocData,
      version
    }
  };

  // Log the document payload for debugging.
  console.log('[user_update] Sending to setDoc:', doc);

  try {
    // Attempt to update the user document; log and rethrow any errors.
    await setDoc(doc);
  } catch (err) {
    console.error('[user_update] Error updating user:', err);
    throw err;
  }
} 