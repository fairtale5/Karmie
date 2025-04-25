import type { ULID } from './ulid_types';
import { validateUlid } from './ulid_types';

/**
 * Formats a tag document key
 * Format: usr_{userUlid}_tag_{tagUlid}_tagName_{tagName}_
 * 
 * @param userUlid - ULID of the user who created the tag
 * @param tagUlid - ULID of the tag itself
 * @param tagName - Name of the tag (will be converted to lowercase)
 * @returns A formatted tag key string
 */
export function formatTagKey(userUlid: ULID, tagUlid: ULID, tagName: string): string {
    if (!validateUlid(userUlid) || !validateUlid(tagUlid)) {
        throw new Error('Invalid ULID provided for tag key formatting');
    }
    
    // Convert tag name to lowercase and sanitize for key usage
    const sanitizedTagName = tagName.toLowerCase();
    
    return `usr_${userUlid}_tag_${tagUlid}_tagName_${sanitizedTagName}_`;
} 