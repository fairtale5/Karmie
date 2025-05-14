import { setDoc } from '@junobuild/core';

/**
 * Updates an existing vote document in the 'votes' collection.
 *
 * @param key - The document key of the vote to update (required)
 * @param version - The current version of the document (required)
 * @param user_key - Voter's ULID (required)
 * @param target_key - Target user's ULID (required)
 * @param tag_key - Tag's ULID (required)
 * @param vote_key - Vote's ULID (required)
 * @param value - Vote value (+1 or -1, required)
 * @param weight - Vote weight (required)
 * @returns {Promise<void>} Resolves when the vote is updated, throws on error
 *
 * @example
 * await updateVoteDoc({ key, version, user_key: '01...', target_key: '02...', tag_key: '03...', vote_key: '04...', value: 1, weight: 1 });
 */
export async function updateVoteDoc({
  key,
  version,
  user_key,
  target_key,
  tag_key,
  vote_key,
  value,
  weight
}: {
  key: string;
  version: bigint;
  user_key: string;
  target_key: string;
  tag_key: string;
  vote_key: string;
  value: number;
  weight: number;
}): Promise<void> {
  const voteDocData = {
    user_key,
    target_key,
    tag_key,
    vote_key,
    value,
    weight
  };
  const doc = {
    collection: 'votes',
    doc: {
      key,
      data: voteDocData,
      version
    }
  };
  console.log('[vote_update] Sending to setDoc:', doc);
  try {
    await setDoc(doc);
    console.log(`[vote_update] Successfully updated vote: ${key}`);
  } catch (err) {
    console.error(`[vote_update] Failed to update vote: ${key}`, err);
    throw err;
  }
} 