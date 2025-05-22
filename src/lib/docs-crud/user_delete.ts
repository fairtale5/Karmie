import { deleteDoc } from '@junobuild/core';

/**
 * Deletes a user document from the 'users' collection.
 *
 * @param key - The document key of the user to delete
 * @param version - The current version of the document (required by Juno for concurrency control)
 * @returns {Promise<void>} Resolves if deletion is successful, throws on error
 *
 * @example
 * await deleteUserDoc('user_doc_key', 1n);
 */
export async function deleteUserDoc(key: string, version: bigint): Promise<void> {
  try {
    await deleteDoc({
      collection: 'users',
      doc: {
        key,
        data: {},
        version
      }
    });
    console.log(`[user_delete] Successfully deleted user document: ${key} (version: ${version})`);
  } catch (err) {
    console.error(`[user_delete] Failed to delete user document: ${key} (version: ${version})`, err);
    throw err;
  }
} 