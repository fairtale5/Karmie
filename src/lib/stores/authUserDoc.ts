import { writable } from 'svelte/store';
import type { UserDocument } from '$lib/types';

/**
 * Svelte store for the authenticated user's full Juno user document.
 * - Null if not loaded or not logged in
 * - UserDocument if loaded
 *
 * @example
 * import { authUserDoc } from '$lib/stores/authUserDoc';
 * $authUserDoc // null or UserDocument
 */
export const authUserDoc = writable<UserDocument | null>(null); 