import type { ULID } from './ulid_types';
import { validateUlid } from './ulid_types';

/**
 * Formats a user document key
 * Format: usr_{userUlid}_hdl_{username}_
 */
export function formatUserKey(userUlid: ULID, username: string): string {
    if (!validateUlid(userUlid)) {
        throw new Error('Invalid ULID provided for user key formatting');
    }
    username = username.toLowerCase();
    return `usr_${userUlid}_hdl_${username}_`;
} 