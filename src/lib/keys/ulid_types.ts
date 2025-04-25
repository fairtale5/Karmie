import { isValid } from 'ulid';

/**
 * Branded type for ULID strings
 * This ensures type safety by distinguishing ULID strings from regular strings at compile time
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

/**
 * Validates a ULID string using the library's native validation
 * @param str - The string to validate
 * @returns true if the string is a valid ULID, false otherwise
 */
export function validateUlid(str: string): str is ULID {
    return isValid(str);
} 