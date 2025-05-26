import { setDoc } from '@junobuild/core';
import { createUlid } from '../keys/create_ulid';
import { formatVoteKey } from '../keys/format_key_vote';
import type { VoteDocument } from '../types';

/**
 * Creates a new vote document in the 'votes' collection.
 *
 * @param doc - Vote document with optional fields
 * @returns {Promise<void>} Resolves when the vote is created, throws on error
 *
 * @example
 * await createVoteDoc({ 
 *   data: {
 *     user_key: '01...', 
 *     target_key: '02...', 
 *     tag_key: '03...', 
 *     value: 1, 
 *     weight: 1 
 *   }
 * });
 */
export async function createVoteDoc(doc: VoteDocument): Promise<void> {
  if (!doc.data.owner_ulid || !doc.data.target_ulid || !doc.data.tag_ulid || doc.data.value === undefined) {
    throw new Error('User key, target key, tag key, and vote value are required');
  }

  // Generate a new unique vote ULID
  const voteUlid = createUlid();
  // Build the document key using the voter's ULID, tag ULID, target ULID, and vote ULID
  const voteDocKey = formatVoteKey(doc.data.owner_ulid, doc.data.tag_ulid, doc.data.target_ulid, voteUlid);

  // Set the generated fields
  doc.key = voteDocKey;
  doc.data.vote_ulid = voteUlid;

  // Log the document payload for debugging
  console.log('[vote_create] Sending to setDoc:', doc);
  try {
    await setDoc({ collection: 'votes', doc });
    console.log(`[vote_create] Successfully created vote: ${voteDocKey}`);
  } catch (err) {
    console.error(`[vote_create] Failed to create vote: ${voteDocKey}`, err);
    throw err;
  }
} 