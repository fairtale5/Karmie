/**
 * This is a "barrel" file - a TypeScript/JavaScript pattern that provides a single entry point
 * to export multiple modules. Benefits:
 * 
 * 1. Cleaner imports - instead of:
 *    import { createUserKey } from '@lib/keys/user';
 *    import { createTagKey } from '@lib/keys/tag';
 *    We can do:
 *    import { createUserKey, createTagKey } from '@lib/keys';
 * 
 * 2. Encapsulation - consumers don't need to know about our internal file structure
 * 
 * 3. Easier refactoring - we can move files around without changing import statements
 *    in consuming code
 */

// ULID functionality
export { generateUlid, validateUlid } from './ulid';
export type { ULID, DocumentPrefix, ParsedDocumentKey } from './ulid_types';

// Document key generation
export { createUserKey } from './user';
export { createTagKey } from './tag';
export { createReputationKey } from './reputation';
export { createVoteKey } from './vote'; 