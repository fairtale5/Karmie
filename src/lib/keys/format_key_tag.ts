import type { ULID } from './ulid_types';
import { validateUlid, ulidToString } from './ulid_types';

/**
 * Formats a tag document key
 * Format: usr_{userUlid}_tag_{tagUlid}_hdl_{tagHandle}_
 *
 * @param userUlid - The creator's user ULID (string type)
 * @param tagUlid - The tag's ULID (string type)
 * @param tagHandle - The tag's handle (string, original case)
 * @returns {string} The formatted tag document key
 */
export function formatTagKey(userUlid: string, tagUlid: string, tagHandle: string): string {
    if (!validateUlid(userUlid)) {
        throw new Error('Invalid user ULID provided for tag key formatting');
    }
    if (!validateUlid(tagUlid)) {
        throw new Error('Invalid tag ULID provided for tag key formatting');
    }
    // Normalize tag handle for key (lowercase, no spaces)
    const normalizedTagHandle = tagHandle.toLowerCase();
    // Compose the key in the required format, converting ULIDs to string
    return `usr_${userUlid}_tag_${tagUlid}_hdl_${normalizedTagHandle}_`;
} 