import { deleteDoc } from '@junobuild/core';

/**
 * Deletes a tag document from the 'tags' collection.
 *
 * @param key - The document key of the tag to delete (required)
 * @param version - The current version of the document (required by Juno for concurrency control)
 * @returns {Promise<void>} Resolves if deletion is successful, throws on error
 *
 * @example
 * await deleteTagDoc('tag_doc_key', 1n);
 */
export async function deleteTagDoc(key: string, version: bigint): Promise<void> {
  try {
    await deleteDoc({
      collection: 'tags',
      doc: {
        key,
        data: {},
        version
      }
    });
    console.log(`[tag_delete] Successfully deleted tag document: ${key} (version: ${version})`);
  } catch (err) {
    console.error(`[tag_delete] Failed to delete tag document: ${key} (version: ${version})`, err);
    throw err;
  }
} 