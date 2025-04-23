import type { ULID } from './ulid_types';
import { validateUlid } from './ulid_types';

/**
 * Formats a tag document key
 * Format: USR_{userUlid}_TAG_{tagUlid}
 */
export function formatTagKey(userUlid: ULID, tagUlid: ULID): string {
    if (!validateUlid(userUlid) || !validateUlid(tagUlid)) {
        throw new Error('Invalid ULID provided for tag key formatting');
    }
    return `USR_${userUlid}_TAG_${tagUlid}`;
} 