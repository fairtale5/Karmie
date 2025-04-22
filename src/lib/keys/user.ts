import { generateUlid } from './ulid';

/**
 * Creates a user document key
 * Format: USR_{ulid}
 */
export function createUserKey(): string {
    return `USR_${generateUlid()}`;
} 