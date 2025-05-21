import { setDoc } from '@junobuild/core';

/**
 * Updates an existing tag document in the 'tags' collection.
 *
 * @param key - The document key of the tag to update (required)
 * @param version - The current version of the document (required by Juno for concurrency control)
 * @param tag_handle - Tag name/handle (required)
 * @param description - Tag description (required, can be empty)
 * @param user_key - Author's ULID (required)
 * @param tag_key - Tag's ULID (required)
 * @param time_periods - Array of { months, multiplier } (required, can be defaulted)
 * @param reputation_threshold - Reputation threshold (required, can be defaulted)
 * @param vote_reward - Vote reward (required, can be defaulted)
 * @param min_users_for_threshold - Minimum users for threshold (required, can be defaulted)
 *
 * @returns {Promise<void>} Resolves when the tag is updated, throws on error
 *
 * @example
 * await updateTagDoc({ key, version, tag_handle: 'dev', description: '', user_key: '01...', tag_key: '02...', time_periods: [...], reputation_threshold: 10, vote_reward: 1, min_users_for_threshold: 3 });
 */
export async function updateTagDoc({
  key,
  version,
  tag_handle,
  description,
  user_key,
  tag_key,
  time_periods,
  reputation_threshold,
  vote_reward,
  min_users_for_threshold
}: {
  key: string;
  version: bigint;
  tag_handle: string;
  description: string;
  user_key: string;
  tag_key: string;
  time_periods: Array<{ months: number; multiplier: number }>;
  reputation_threshold: number;
  vote_reward: number;
  min_users_for_threshold: number;
}): Promise<void> {
  // Prepare the updated tag data object
  const tagDocData = {
    tag_handle,
    description,
    user_key,
    tag_key,
    time_periods,
    reputation_threshold,
    vote_reward,
    min_users_for_threshold
  };
  // Create the document object for setDoc, including the key and version
  const doc = {
    collection: 'tags',
    doc: {
      key,
      data: tagDocData,
      version
    }
  };
  // Log the document payload for debugging
  console.log('[tag_update] Sending to setDoc:', doc);
  try {
    await setDoc(doc);
    console.log(`[tag_update] Successfully updated tag: ${tag_handle} (key: ${key})`);
  } catch (err) {
    console.error(`[tag_update] Failed to update tag: ${tag_handle} (key: ${key})`, err);
    throw err;
  }
} 