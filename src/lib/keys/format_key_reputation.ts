import type { ULID } from './ulid_types';
import { validateUlid } from './ulid_types';

/**
 * Formats a reputation document key
 * Format: usr_{userUlid}_tag_{tagUlid}
 */
export function formatReputationKey(userUlid: ULID, tagUlid: ULID): string {
    if (!validateUlid(userUlid) || !validateUlid(tagUlid)) {
        throw new Error('Invalid ULID provided for reputation key formatting');
    }
    return `usr_${userUlid}_tag_${tagUlid}`;
} 