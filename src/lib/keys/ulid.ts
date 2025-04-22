import * as ULID_LIB from 'ulid';
import type { ULID } from './ulid_types';

/**
 * Generates a ULID in uppercase format
 * @returns A 26-character ULID string in uppercase
 */
export function generateUlid(): ULID {
    return ULID_LIB.ulid() as ULID;
}

/**
 * Validates a ULID string using the library's native validation
 * @param str - The string to validate
 * @returns true if the string is a valid ULID, false otherwise
 */
export function validateUlid(str: string): str is ULID {
    return ULID_LIB.isValid(str);
} 