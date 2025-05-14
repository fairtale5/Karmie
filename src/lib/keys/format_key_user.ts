import { isValid } from 'ulid';

/**
 * Formats a user document key
 * Format: _prn_{principal}_usr_{userUlid}_hdl_{username}_
 *
 * @param principal - The user's principal (string)
 * @param userUlid - The user's ULID (string, must be uppercase, 26 chars)
 * @param username - The user's handle (string)
 * @returns {string} The formatted user document key
 */
export function formatUserKey(principal: string, userUlid: string, username: string): string {
    if (!isValid(userUlid)) {
        throw new Error('Invalid ULID provided for user key formatting');
    }
    // Normalize username for key (lowercase, no spaces)
    const normalizedUsername = username.toLowerCase();
    // Compose the key in the new format
    return `_prn_${principal}_usr_${userUlid}_hdl_${normalizedUsername}_`;
} 