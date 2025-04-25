/**
 * This is a "barrel" file - a TypeScript/JavaScript pattern that provides a single entry point
 * to export multiple modules. Benefits:
 * 
 * 1. Cleaner imports - instead of:
 *    import { formatUserKey } from '@lib/keys/format_key_user';
 *    import { formatTagKey } from '@lib/keys/format_key_tag';
 *    We can do:
 *    import { formatUserKey, formatTagKey } from '@lib/keys';
 * 
 * 2. Encapsulation - consumers don't need to know about our internal file structure
 * 
 * 3. Easier refactoring - we can move files around without changing import statements
 *    in consuming code
 */

/**
 * Types that help us write safer code by catching mistakes early:
 * 
 * ULID type:
 * - Prevents accidentally using regular strings where ULIDs are required
 * - Shows clear error messages if we try to use an invalid ULID
 * - Makes sure we can't mix up ULIDs with other kinds of IDs
 * - Helps our editor provide better autocomplete and hints
 * 
 * DocumentPrefix type:
 * Think of this like a template checker. For example, if you try to use:
 * "USER_123" -> Error! (wrong format)
 * "usr_abc" -> Error! (not a valid ULID)
 * "usr_01ARZ3NDEKTSV4RRFFQ69G5FAV" -> ✓ Success! (correct format)
 * 
 * ParsedDocumentKey type:
 * Helps us safely break down complex keys into their parts. Like taking:
 * "usr_123_tag_456" and getting:
 * - What kind of document is it? ("usr")
 * - What's the user's ID? ("123")
 * - What's the tag's ID? ("456")
 * This prevents us from accidentally splitting keys in the wrong place
 */
export type { ULID, DocumentPrefix, ParsedDocumentKey } from './ulid_types';
export { validateUlid } from './ulid_types';

// ULID creation
export { createUlid } from './create_ulid';

// Document key formatting functions
export { formatUserKey } from './format_key_user';
export { formatTagKey } from './format_key_tag';
export { formatReputationKey } from './format_key_reputation';
export { formatVoteKey } from './format_key_vote'; 