import { isValid } from 'ulid';

/**
 * Temporarily using string instead of branded type for ULID
 * TODO: Restore branded type when proper ULID serialization is implemented
 */
export type ULID = string;

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