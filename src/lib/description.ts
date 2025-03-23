/**
 * Description Field Utilities
 * 
 * This module provides utilities for creating and parsing document descriptions
 * following the format defined in docs/core/architecture/database.md.
 * 
 * Format: [field1:{value1}][field2:{value2}]
 * Example: [owner:user_123][username:john_doe]
 */

import { IS_PLAYGROUND } from './settings';
import type { User } from '@junobuild/core';

/**
 * Creates a description field builder for consistent formatting
 */
class DescriptionBuilder {
    private fields: Array<[string, string]> = [];

    /**
     * Adds a field to the description
     * @param name Field name
     * @param value Field value
     */
    addField(name: string, value: string): this {
        this.fields.push([name, value]);
        return this;
    }

    /**
     * Adds the owner field based on playground mode
     * @param user Current user
     * @param documentKey Document key (used in playground mode)
     */
    addOwner(user: User | null, documentKey: string): this {
        // In playground mode, use document key as owner
        // In production, use user's Principal ID
        const ownerValue = IS_PLAYGROUND ? documentKey : (user?.key ?? 'anonymous');
        return this.addField('owner', ownerValue);
    }

    /**
     * Builds the final description string
     * @returns Formatted description string
     */
    build(): string {
        return this.fields
            .map(([name, value]) => `[${name}:${value}]`)
            .join(',');
    }
}

/**
 * Creates a description for a user document
 * Format: [owner:{id}][username:{name}]
 */
export function createUserDescription(user: User | null, documentKey: string, handle: string): string {
    console.log('[Description] Creating user description:', { documentKey, handle, mode: IS_PLAYGROUND ? 'playground' : 'production' });
    
    return new DescriptionBuilder()
        .addOwner(user, documentKey)
        .addField('username', handle)
        .build();
}

/**
 * Creates a description for a tag document
 * Format: [owner:{id}][name:{name}]
 */
export function createTagDescription(user: User | null, documentKey: string, name: string): string {
    console.log('[Description] Creating tag description:', { documentKey, name, mode: IS_PLAYGROUND ? 'playground' : 'production' });
    
    return new DescriptionBuilder()
        .addOwner(user, documentKey)
        .addField('name', name)
        .build();
}

/**
 * Creates a description for a vote document
 * Format: [owner:{id}][target:{key}][tag:{key}]
 */
export function createVoteDescription(
    user: User | null,
    documentKey: string,
    targetKey: string,
    tagKey: string
): string {
    console.log('[Description] Creating vote description:', { 
        documentKey, 
        targetKey, 
        tagKey, 
        mode: IS_PLAYGROUND ? 'playground' : 'production' 
    });
    
    return new DescriptionBuilder()
        .addOwner(user, documentKey)
        .addField('target', targetKey)
        .addField('tag', tagKey)
        .build();
}

/**
 * Creates a search pattern for matching descriptions
 * Adds proper brackets for the new format
 */
export function createSearchPattern(field: string, value: string): string {
    console.log('[Description] Creating search pattern:', { field, value });
    return `[${field}:${value}]`;
} 