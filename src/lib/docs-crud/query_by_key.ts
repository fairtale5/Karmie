import { listDocs } from '@junobuild/core';

/**
 * Query documents by key pattern (filtering happens in the backend).
 *
 * @template T - The document data type
 * @param collection - The collection name (e.g., 'users', 'tags', 'votes', 'reputations')
 * @param keyPattern - The key pattern to match (prefix, infix, or exact)
 * @returns {Promise<import('@junobuild/core').ListResults<T>>} - The matching documents
 *
 * @example
 * const results = await queryDocsByKey<UserData>('users', 'usr_01ARZ3NDEKTSV4RRFFQ69G5FAV');
 * 
 * Returns an array of documents in the format:
 * [
 *   {
 *     key: 'usr_01ARZ3NDEKTSV4RRFFQ69G5FAV',
 *     owner: '{principal_string}',    // <--- This is the owner of the document.
 *     createdAt: '{timestamp}',       // <--- This is the creation timestamp.
 *     updatedAt: '{timestamp}',       // <--- This is the last update timestamp.
 *     version: '{version_number}',    // <--- This is the version number of the document.
 *     description: '{description_string}', // <--- This is the description of the document.
 *     data: {          // <--- This is the data object containing all the custom fields.
 *       name: 'John Doe',
 *       email: 'john.doe@example.com'
 *     }
 *   },
 *   {
 *     ...
 *     }
 *   }
 * ]
 * 
 * keyPattern examples:
 *  - 'usr_' - prefix
 *  - 'usr_01ARZ3NDEKTSV4RRFFQ69G5FAV' - exact
 *  - 'usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAV' - infix and exact
 */
export async function queryDocsByKey<T>(collection: string, keyPattern: string) {
  return listDocs<T>({
    collection,
    filter: { matcher: { key: keyPattern } }
  });
} 