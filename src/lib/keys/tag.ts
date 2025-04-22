import { generateUlid, validateUlid } from './ulid';

/**
 * Creates a tag document key
 * Format: USR_{userUlid}_TAG_{ulid}
 */
export function createTagKey(userUlid: string): string {
    if (!validateUlid(userUlid)) {
        throw new Error('Invalid ULID provided for tag key generation');
    }
    return `USR_${userUlid}_TAG_${generateUlid()}`;
} 