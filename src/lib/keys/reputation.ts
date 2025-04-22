import { generateUlid, validateUlid } from './ulid';

/**
 * Creates a reputation document key
 * Format: USR_{userUlid}_TAG_{tagUlid}
 */
export function createReputationKey(userUlid: string, tagUlid: string): string {
    if (!validateUlid(userUlid) || !validateUlid(tagUlid)) {
        throw new Error('Invalid ULID provided for reputation key generation');
    }
    return `USR_${userUlid}_TAG_${tagUlid}`;
} 