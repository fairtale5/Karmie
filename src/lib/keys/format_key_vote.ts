import type { ULID } from './ulid_types';
import { validateUlid } from './ulid_types';

/**
 * Formats a vote document key
 * Format: usr_{userUlid}_tag_{tagUlid}_tar_{targetUlid}_key_{voteUlid}
 */
export function formatVoteKey(userUlid: ULID, tagUlid: ULID, targetUlid: ULID, voteUlid: ULID): string {
    if (!validateUlid(userUlid) || !validateUlid(tagUlid) || !validateUlid(targetUlid) || !validateUlid(voteUlid)) {
        throw new Error('Invalid ULID provided for vote key formatting');
    }
    return `usr_${userUlid}_tag_${tagUlid}_tar_${targetUlid}_key_${voteUlid}`;
} 