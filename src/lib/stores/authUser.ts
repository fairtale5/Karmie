import { writable } from 'svelte/store';
import { authSubscribe, type User } from '@junobuild/core';

/**
 * Svelte store for tracking the current authenticated user via Juno.
 *
 * - Null if not logged in
 * - User object if logged in
 *
 * Uses Juno's authSubscribe: https://www.junobuild.dev/docs/build/authentication/development#subscription
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

// Subscribe to Juno auth state on module load
const unsubscribe = authSubscribe((user: User | null) => {
    authUser.set(user);
    authUserDoneInitializing.set(true);
});

// Optionally export unsubscribe for cleanup in tests
export { unsubscribe }; 