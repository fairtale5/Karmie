import { isValid } from 'ulid';

/**
 * Formats a vote document key
 * Format: usr_{userUlid}_tag_{tagUlid}_tar_{targetUlid}_key_{voteUlid}_
 *
 * @param userUlid - The voter's user ULID (string)
 * @param tagUlid - The tag's ULID (string)
 * @param targetUlid - The target user's ULID (string)
 * @param voteUlid - The vote's ULID (string)
 * @returns {string} The formatted vote document key
 */
export function formatVoteKey(userUlid: string, tagUlid: string, targetUlid: string, voteUlid: string): string {
    if (!isValid(userUlid)) {
        throw new Error('Invalid user ULID provided for vote key formatting');
    }
    if (!isValid(tagUlid)) {
        throw new Error('Invalid tag ULID provided for vote key formatting');
    }
    if (!isValid(targetUlid)) {
        throw new Error('Invalid target ULID provided for vote key formatting');
    }
    if (!isValid(voteUlid)) {
        throw new Error('Invalid vote ULID provided for vote key formatting');
    }
    // Compose the key in the required format
    return `usr_${userUlid}_tag_${tagUlid}_tar_${targetUlid}_key_${voteUlid}_`;
} 