import type { ULID } from './ulid_types';
import { validateUlid, ulidToString } from './ulid_types';

/**
 * Formats a tag document key
 * Format: usr_{userUlid}_tag_{tagUlid}_hdl_{tagHandle}_
 *
 * @param userUlid - The creator's user ULID (ULID type)
 * @param tagUlid - The tag's ULID (ULID type)
 * @param tagHandle - The tag's handle (string, original case)
 * @returns {string} The formatted tag document key
 */
export function formatTagKey(userUlid: ULID, tagUlid: ULID, tagHandle: string): string {
    if (!validateUlid(userUlid)) {
        throw new Error('Invalid user ULID provided for tag key formatting');
    }
    if (!validateUlid(tagUlid)) {
        throw new Error('Invalid tag ULID provided for tag key formatting');
    }
    // Normalize tag handle for key (lowercase, no spaces)
    const normalizedTagHandle = tagHandle.toLowerCase();
    // Compose the key in the required format, converting ULIDs to string
    return `usr_${ulidToString(userUlid)}_tag_${ulidToString(tagUlid)}_hdl_${normalizedTagHandle}_`;
} 