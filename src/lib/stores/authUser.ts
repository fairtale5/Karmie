import { writable } from 'svelte/store';
import { type User } from '@junobuild/core';

/**
 * Svelte store for tracking the current authenticated user via Juno.
 * This store is updated by the Juno auth worker.
 *
 * - Null if not logged in
 * - User object if logged in
 *
 * @example
 * import { authUser } from './stores/authUser';
 * $authUser // null or User
 */
export const authUser = writable<User | null>(null);

/**
 * Indicates whether the authentication state has been initialized.
 * Starts as false, becomes true after the first value from authSubscribe.
 * @example
 * import { authUserDoneInitializing } from './stores/authUser';
 * $authUserDoneInitializing // boolean
 */
export const authUserDoneInitializing = writable(false); 