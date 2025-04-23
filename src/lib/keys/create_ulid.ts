/**
 * ULID (Universally Unique Lexicographically Sortable Identifier) creation utility
 * Using the ulid library: https://github.com/ulid/javascript
 * ULID Spec: https://github.com/ulid/spec
 * 
 * Benefits:
 * - 26 characters, case insensitive
 * - Lexicographically sortable
 * - Monotonically increasing
 * - URL safe (Crockford's Base32)
 * - 128-bit compatibility with UUID
 */

import { ulid } from 'ulid';
import type { ULID } from './ulid_types';

/**
 * Generates a ULID in uppercase format
 * @returns A 26-character ULID string in uppercase
 */
export function createUlid(): ULID {
    return ulid() as ULID;
} 