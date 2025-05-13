// NOTE: Do not add ULID generation here. Use src/lib/keys/create_ulid.ts for all ULID creation.
import { isValid, ulid as ulidGen } from 'ulid';

/**
 * Branded type for ULID (Crockford Base32, 26 chars, uppercase)
 * Matches the ulid crate: https://docs.rs/ulid/latest/ulid/struct.Ulid.html
 */
export type ULID = string & { readonly __ulid: unique symbol };

/**
 * Converts a string to a ULID type, with validation.
 * Throws if the string is not a valid ULID.
 * @param str - The string to convert
 * @returns {ULID} The branded ULID type
 */
export function stringToUlid(str: string): ULID {
    if (!isValid(str)) throw new Error('Invalid ULID string');
    return str.toUpperCase() as ULID;
}

/**
 * Converts a ULID type to a string (for key formatting, etc.)
 * @param ulid - The ULID to convert
 * @returns {string} The string representation
 */
export function ulidToString(ulid: ULID): string {
    return ulid as string;
}

/**
 * Validates a ULID string or branded ULID
 * @param str - The string or ULID to validate
 * @returns true if valid, false otherwise
 */
export function validateUlid(str: string | ULID): str is ULID {
    return isValid(str);
}

