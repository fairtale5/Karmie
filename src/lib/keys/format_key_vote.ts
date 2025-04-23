import type { ULID } from './ulid_types';
import { validateUlid } from './ulid_types';

/**
 * Formats a vote document key
 * Format: USR_{userUlid}_TAG_{tagUlid}_TAR_{targetUlid}_KEY_{voteUlid}
 */
export function formatVoteKey(userUlid: ULID, tagUlid: ULID, targetUlid: ULID, voteUlid: ULID): string {
    if (!validateUlid(userUlid) || !validateUlid(tagUlid) || !validateUlid(targetUlid) || !validateUlid(voteUlid)) {
        throw new Error('Invalid ULID provided for vote key formatting');
    }
    return `USR_${userUlid}_TAG_${tagUlid}_TAR_${targetUlid}_KEY_${voteUlid}`;
} 