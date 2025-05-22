import { deleteDoc } from '@junobuild/core';

/**
 * Deletes a vote document from the 'votes' collection.
 *
 * @param key - The document key of the vote to delete (required)
 * @param version - The current version of the document (required)
 * @returns {Promise<void>} Resolves if deletion is successful, throws on error
 *
 * @example
 * await deleteVoteDoc('vote_doc_key', 1n);
 */
export async function deleteVoteDoc(key: string, version: bigint): Promise<void> {
  try {
    await deleteDoc({
      collection: 'votes',
      doc: {
        key,
        data: {},
        version
      }
    });
    console.log(`[vote_delete] Successfully deleted vote document: ${key} (version: ${version})`);
  } catch (err) {
    console.error(`[vote_delete] Failed to delete vote document: ${key} (version: ${version})`, err);
    throw err;
  }
} 