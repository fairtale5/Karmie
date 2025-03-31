/**
 * Description Field Utilities
 * 
 * Document descriptions follow these rules:
 * 
 * Users Collection:
 * - owner: The document key of the user document being created/edited
 * - username: The username field from the document data
 * Format: owner=documentKey;username=username;
 * 
 * Tags Collection:
 * - owner: The document key of the user who is creating the tag
 * - name: The name field provided in the frontend
 * Format: owner=creatorUserKey;name=tagName;
 * 
 * Votes Collection:
 * - owner: The document key of the user casting the vote
 * - target: The document key of the user being voted on
 * - tag: The document key of the tag this vote belongs to
 * Format: owner=voterUserKey;target=targetUserKey;tag=tagKey;
 * 
 * Reputations Collection (backend only):
 * - owner: The document key of the user this reputation belongs to
 * - tag: The document key of the tag this reputation is for
 * Format: owner=userKey;tag=tagKey;
 */

import type { User } from '@junobuild/core';

/**
 * Sanitizes keys to remove special characters
 * Only allows alphanumeric characters and underscores
 */
function sanitizeKey(key: string): string {
    // If the key is empty, return a default value
    if (!key) {
        return "key";
    }
    
    // Filter out non-alphanumeric and non-underscore characters
    const sanitized = key.replace(/[^a-zA-Z0-9_]/g, '');
    
    // If sanitizing removed all characters, return a default value
    if (!sanitized) {
        return "key";
    }
    
    // Ensure the key doesn't start with a number
    if (/^[0-9]/.test(sanitized)) {
        return `k${sanitized}`;
    }
    
    return sanitized;
}

/**
 * Creates a description for a user document
 * Format: owner=documentKey;username=username;
 * 
 * In playground mode, we use the document key as the owner.
 * The document key is the nanoid generated when creating the document.
 */
export function createUserDescription(documentKey: string, username: string): string {
    // Sanitize keys to avoid special character issues
    const sanitizedDocKey = sanitizeKey(documentKey);
    const sanitizedUsername = sanitizeKey(username.toLowerCase());
    
    // In playground mode, always use the document key as owner
    return `owner=${sanitizedDocKey};username=${sanitizedUsername};`;
}

/**
 * Creates a description for a tag document
 * Format: owner=creatorUserKey;name=tagName;
 */
export function createTagDescription(user: User | null, documentKey: string, name: string, authorKey: string): string {
    // Sanitize keys to avoid special character issues
    const sanitizedAuthorKey = sanitizeKey(authorKey);
    const sanitizedName = sanitizeKey(name);
    
    // Note: We use authorKey (the creator's user document key) as owner, NOT the tag's documentKey
    return `owner=${sanitizedAuthorKey};name=${sanitizedName};`;
}

/**
 * Creates a description for a vote document
 * Format: owner=voterUserKey;target=targetUserKey;tag=tagKey;
 */
export function createVoteDescription(
    user: User | null,
    documentKey: string,
    authorKey: string,
    targetKey: string,
    tagKey: string
): string {
    // Sanitize keys to avoid special character issues
    const sanitizedAuthorKey = sanitizeKey(authorKey);
    const sanitizedTargetKey = sanitizeKey(targetKey);
    const sanitizedTagKey = sanitizeKey(tagKey);
    
    // Note: We use authorKey (the voter's user document key) as owner, NOT the vote's documentKey
    return `owner=${sanitizedAuthorKey};target=${sanitizedTargetKey};tag=${sanitizedTagKey};`;
}

/**
 * Creates a search pattern for matching descriptions
 * Use this for creating exact matches in listDocs queries, example:
 * listDocs({
 *   filter: {
 *     matcher: {
 *       description: createSearchPattern("username", "john")
 *     }
 *   }
 * });
 */
export function createSearchPattern(field: string, value: string): string {
    // Sanitize the value to avoid special character issues
    const sanitizedValue = sanitizeKey(value);
    
    // Return the new format pattern
    return `${field}=${sanitizedValue};`;
} 