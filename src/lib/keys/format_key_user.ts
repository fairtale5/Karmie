import type { ULID } from './ulid_types';
import { validateUlid } from './ulid_types';

/**
 * Formats a user document key
 * Format: USR_{userUlid}_USRNAME_{username}_
 */
export function formatUserKey(userUlid: ULID, username: string): string {
    if (!validateUlid(userUlid)) {
        throw new Error('Invalid ULID provided for user key formatting');
    }
    username = username.toLowerCase();
    return `USR_${userUlid}_USRNAME_${username}_`;
} 