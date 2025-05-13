import type { ULID } from './ulid_types';
import { validateUlid, ulidToString } from './ulid_types';

/**
 * Formats a user document key
 * Format: _prn_{principal}_usr_{userUlid}_hdl_{username}_
 *
 * @param principal - The user's Internet Identity principal (string)
 * @param userUlid - The user's ULID (ULID type, must be uppercase, 26 chars)
 * @param username - The user's username (string, original case)
 * @returns {string} The formatted user document key
 */
export function formatUserKey(principal: string, userUlid: ULID, username: string): string {
    if (!validateUlid(userUlid)) {
        throw new Error('Invalid ULID provided for user key formatting');
    }
    // Normalize username for key (lowercase, no spaces)
    const normalizedUsername = username.toLowerCase();
    // Compose the key in the new format, converting ULID to string
    return `_prn_${principal}_usr_${ulidToString(userUlid)}_hdl_${normalizedUsername}_`;
} 