import { setDoc } from '@junobuild/core';
import { createUlid } from '../keys/create_ulid';
import { formatTagKey } from '../keys/format_key_tag';
import type { TagDocument } from '../types';

/**
 * Creates a new tag document in the 'tags' collection.
 *
 * @param doc - Tag document with optional fields
 * @returns {Promise<void>} Resolves when the tag is created, throws on error
 *
 * @example
 * await createTagDoc({ 
 *   data: {
 *     tag_handle: 'dev', 
 *     description: '', 
 *     user_key: '01...', 
 *     time_periods: [...], 
 *     reputation_threshold: 10, 
 *     vote_reward: 1, 
 *     min_users_for_threshold: 3 
 *   }
 * });
 */
export async function createTagDoc(doc: TagDocument): Promise<void> {
  if (!doc.data.tag_handle || !doc.data.owner_ulid) {
    throw new Error('Tag handle and user key are required');
  }

  // Generate a new unique tag ULID
  const tagUlid = createUlid();
  // Build the document key using the author's ULID, tag ULID, and tag handle
  const tagDocKey = formatTagKey(doc.data.owner_ulid, tagUlid, doc.data.tag_handle);

  // Set the generated fields
  doc.key = tagDocKey;
  doc.data.tag_ulid = tagUlid;

  // Log the document payload for debugging
  console.log('[tag_create] Sending to setDoc:', doc);
  try {
    await setDoc({ collection: 'tags', doc });
    console.log(`[tag_create] Successfully created tag: ${doc.data.tag_handle} (key: ${tagDocKey})`);
  } catch (err) {
    console.error(`[tag_create] Failed to create tag: ${doc.data.tag_handle} (key: ${tagDocKey})`, err);
    throw err;
  }
} 