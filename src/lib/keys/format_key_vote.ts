import type { ULID } from './ulid_types';
import { validateUlid, ulidToString } from './ulid_types';

/**
 * Formats a vote document key
 * Format: usr_{userUlid}_tag_{tagUlid}_tar_{targetUlid}_key_{voteUlid}_
 *
 * @param userUlid - The voter's user ULID (ULID type)
 * @param tagUlid - The tag's ULID (ULID type)
 * @param targetUlid - The target user's ULID (ULID type)
 * @param voteUlid - The vote's ULID (ULID type)
 * @returns {string} The formatted vote document key
 */
export function formatVoteKey(userUlid: ULID, tagUlid: ULID, targetUlid: ULID, voteUlid: ULID): string {
    if (!validateUlid(userUlid)) {
        throw new Error('Invalid user ULID provided for vote key formatting');
    }
    if (!validateUlid(tagUlid)) {
        throw new Error('Invalid tag ULID provided for vote key formatting');
    }
    if (!validateUlid(targetUlid)) {
        throw new Error('Invalid target ULID provided for vote key formatting');
    }
    if (!validateUlid(voteUlid)) {
        throw new Error('Invalid vote ULID provided for vote key formatting');
    }
    // Compose the key in the required format, converting ULIDs to string
    return `usr_${ulidToString(userUlid)}_tag_${ulidToString(tagUlid)}_tar_${ulidToString(targetUlid)}_key_${ulidToString(voteUlid)}_`;
} 