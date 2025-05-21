import { setDoc } from '@junobuild/core';
import { createUlid } from '../keys/create_ulid';
import { formatVoteKey } from '../keys/format_key_vote';

/**
 * Creates a new vote document in the 'votes' collection.
 *
 * @param user_key - Voter's ULID (required)
 * @param target_key - Target user's ULID (required)
 * @param tag_key - Tag's ULID (required)
 * @param value - Vote value (+1 or -1, required)
 * @param weight - Vote weight (required)
 * @returns {Promise<void>} Resolves when the vote is created, throws on error
 *
 * @example
 * await createVoteDoc({ user_key: '01...', target_key: '02...', tag_key: '03...', value: 1, weight: 1 });
 */
export async function createVoteDoc({
  user_key,
  target_key,
  tag_key,
  value,
  weight
}: {
  user_key: string;
  target_key: string;
  tag_key: string;
  value: number;
  weight: number;
}): Promise<void> {
  const voteUlid = createUlid();
  const voteDocKey = formatVoteKey(user_key, tag_key, target_key, voteUlid);
  const voteDocData = {
    user_key,
    target_key,
    tag_key,
    vote_key: voteUlid,
    value,
    weight
  };
  const doc = {
    collection: 'votes',
    doc: {
      key: voteDocKey,
      data: voteDocData
    }
  };
  console.log('[vote_create] Sending to setDoc:', doc);
  try {
    await setDoc(doc);
    console.log(`[vote_create] Successfully created vote: ${voteDocKey}`);
  } catch (err) {
    console.error(`[vote_create] Failed to create vote: ${voteDocKey}`, err);
    throw err;
  }
} 