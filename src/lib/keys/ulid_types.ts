/**
 * Branded type for ULID strings
 * This ensures type safety by distinguishing ULID strings from regular strings
 */
export type ULID = string & { readonly __brand: unique symbol };

/**
 * Type for document key prefixes to ensure consistency
 */
export type DocumentPrefix = 'USR' | 'TAG' | 'TAR' | 'KEY';

/**
 * Helper type for parsed document key components
 */
export interface ParsedDocumentKey {
    prefix: DocumentPrefix;
    ulid: ULID;
    rest?: string;
} 