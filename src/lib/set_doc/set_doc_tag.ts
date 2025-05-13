import { formatTagKey } from '../keys/format_key_tag';
import type { TagData, TagDocument } from '../types';
import type { ULID } from '../keys/ulid_types';
import { VALIDATE_TAG_DOC } from '../settings';

/**
 * Creates a full tag document object for setDoc, using the standard key format.
 *
 * @param user_key - The creator's user ULID (ULID type)
 * @param tag_key - The tag's ULID (ULID type)
 * @param tag_handle - The tag's handle (string, required)
 * @param description - The tag's description (string)
 * @param time_periods - Array of time period objects for vote decay
 * @param reputation_threshold - Minimum reputation needed for voting power
 * @param vote_reward - Reputation points given for casting votes
 * @param min_users_for_threshold - Minimum users needed before vote rewards are restricted
 * @param validationOverride - Optional boolean to override global validation toggle
 * @returns {TagDocument} The full tag document object, ready for setDoc
 *
 * This function:
 * - Gets the tag_handle, creator's user_key, and tag_key (ULID)
 * - Uses formatTagKey to generate the document key
 * - Assembles the TagData object, including all required and optional fields
 * - Returns the full TagDocument object, without owner, created_at, and updated_at, and only including provided fields
 * - Optionally runs frontend validation if enabled in settings
 */
export default function setDocTag({
  user_key,
  tag_key,
  tag_handle,
  description,
  time_periods,
  reputation_threshold,
  vote_reward,
  min_users_for_threshold,
  validationOverride
}: {
  user_key: ULID;
  tag_key: ULID;
  tag_handle: string;
  description: string;
  time_periods: Array<{ months: number; multiplier: number }>;
  reputation_threshold: number;
  vote_reward: number;
  min_users_for_threshold: number;
  validationOverride?: boolean;
}): TagDocument {
  // Step 1: Generate the document key using the standard format
  // This key will be used for efficient queries and must be unique
  const key = formatTagKey(user_key, tag_key, tag_handle);

  // Step 2: Assemble the TagData object, only including provided fields
  // This includes the original-case name, description, and all config fields
  const data: TagData = {
    user_key,
    tag_key,
    tag_handle,
    description,
    time_periods,
    reputation_threshold,
    vote_reward,
    min_users_for_threshold
  };

  // Step 3: Optionally run frontend validation if enabled
  const doValidate = validationOverride !== undefined ? validationOverride : VALIDATE_TAG_DOC;
  if (doValidate) {
    // TODO: Implement frontend validation for tag fields (tag_handle, ULIDs, etc.)
    // For now, this is a stub
  }

  // Step 4: Return the full tag document object
  const doc: TagDocument = {
    key,
    data,
    version: BigInt(0),
    created_at: BigInt(0),
    updated_at: BigInt(0)
  } as TagDocument;
  return doc;
} 