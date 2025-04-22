import { generateUlid, validateUlid } from './ulid';

/**
 * Creates a vote document key
 * Format: USR_{userUlid}_TAG_{tagUlid}_TAR_{targetUlid}_KEY_{ulid}
 */
export function createVoteKey(userUlid: string, tagUlid: string, targetUlid: string): string {
    if (!validateUlid(userUlid) || !validateUlid(tagUlid) || !validateUlid(targetUlid)) {
        throw new Error('Invalid ULID provided for vote key generation');
    }
    return `USR_${userUlid}_TAG_${tagUlid}_TAR_${targetUlid}_KEY_${generateUlid()}`;
} 