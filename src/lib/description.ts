/**
 * Description Field Utilities
 * 
 * Document descriptions follow these rules:
 * 
 * Users Collection:
 * - owner: The document key of the user document being created/edited
 * - username: The username field from the document data
 * Format: [owner:documentKey],[username:username]
 * 
 * Tags Collection:
 * - owner: The document key of the user who is creating the tag
 * - name: The name field provided in the frontend
 * Format: [owner:creatorUserKey],[name:tagName]
 * 
 * Votes Collection:
 * - owner: The document key of the user casting the vote
 * - target: The document key of the user being voted on
 * - tag: The document key of the tag this vote belongs to
 * Format: [owner:voterUserKey],[target:targetUserKey],[tag:tagKey]
 * 
 * Reputations Collection (backend only):
 * - owner: The document key of the user this reputation belongs to
 * - tag: The document key of the tag this reputation is for
 * Format: [owner:userKey],[tag:tagKey]
 */

import type { User } from '@junobuild/core';

/**
 * Creates a description for a user document
 * Format: [owner:documentKey],[username:data.username]
 */
export function createUserDescription(user: User | null, documentKey: string, username: string): string {
    return `[owner:${documentKey}],[username:${username}]`;
}

/**
 * Creates a description for a tag document
 * Format: [owner:creatorUserKey],[name:tagName]
 */
export function createTagDescription(user: User | null, documentKey: string, name: string, authorKey: string): string {
    // Note: We use authorKey (the creator's user document key) as owner, NOT the tag's documentKey
    return `[owner:${authorKey}],[name:${name}]`;
}

/**
 * Creates a description for a vote document
 * Format: [owner:voterUserKey],[target:targetUserKey],[tag:tagKey]
 */
export function createVoteDescription(
    user: User | null,
    documentKey: string,
    authorKey: string,
    targetKey: string,
    tagKey: string
): string {
    // Note: We use authorKey (the voter's user document key) as owner, NOT the vote's documentKey
    return `[owner:${authorKey}],[target:${targetKey}],[tag:${tagKey}]`;
}

/**
 * Creates a search pattern for matching descriptions
 * NOTE: Currently unused - review if this is still needed.
 * Used for creating partial matches in listDocs queries, example:
 * listDocs({
 *   filter: {
 *     matcher: {
 *       description: createSearchPattern("username", "john")
 *     }
 *   }
 * });
 */
export function createSearchPattern(field: string, value: string): string {
    return `[${field}:${value}]`;
} 