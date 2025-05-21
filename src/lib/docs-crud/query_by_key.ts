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
 */
export async function queryDocsByKey<T>(collection: string, keyPattern: string) {
  return listDocs<T>({
    collection,
    filter: { matcher: { key: keyPattern } }
  });
} 