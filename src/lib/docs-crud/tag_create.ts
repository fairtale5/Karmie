import { setDoc } from '@junobuild/core';
import { createUlid } from '../keys/create_ulid';
import { formatTagKey } from '../keys/format_key_tag';

/**
 * Creates a new tag document in the 'tags' collection.
 *
 * @param tag_handle - Tag name/handle (required)
 * @param description - Tag description (required, can be empty)
 * @param user_key - Author's ULID (required)
 * @param time_periods - Array of { months, multiplier } (required, can be defaulted)
 * @param reputation_threshold - Reputation threshold (required, can be defaulted)
 * @param vote_reward - Vote reward (required, can be defaulted)
 * @param min_users_for_threshold - Minimum users for threshold (required, can be defaulted)
 *
 * @returns {Promise<void>} Resolves when the tag is created, throws on error
 *
 * @example
 * await createTagDoc({ tag_handle: 'dev', description: '', user_key: '01...', time_periods: [...], reputation_threshold: 10, vote_reward: 1, min_users_for_threshold: 3 });
 */
export async function createTagDoc({
  tag_handle,
  description,
  user_key,
  time_periods,
  reputation_threshold,
  vote_reward,
  min_users_for_threshold
}: {
  tag_handle: string;
  description: string;
  user_key: string;
  time_periods: Array<{ months: number; multiplier: number }>;
  reputation_threshold: number;
  vote_reward: number;
  min_users_for_threshold: number;
}): Promise<void> {
  // Generate a new unique tag ULID
  const tagUlid = createUlid();
  // Build the document key using the author's ULID, tag ULID, and tag handle
  const tagDocKey = formatTagKey(user_key, tagUlid, tag_handle);
  // Prepare the tag data object
  const tagDocData = {
    tag_handle,
    description,
    user_key,
    tag_key: tagUlid,
    time_periods,
    reputation_threshold,
    vote_reward,
    min_users_for_threshold
  };
  // Create the document object for setDoc
  const doc = {
    collection: 'tags',
    doc: {
      key: tagDocKey,
      data: tagDocData
    }
  };
  // Log the document payload for debugging
  console.log('[tag_create] Sending to setDoc:', doc);
  try {
    await setDoc(doc);
    console.log(`[tag_create] Successfully created tag: ${tag_handle} (key: ${tagDocKey})`);
  } catch (err) {
    console.error(`[tag_create] Failed to create tag: ${tag_handle} (key: ${tagDocKey})`, err);
    throw err;
  }
} 