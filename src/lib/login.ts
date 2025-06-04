import { signIn, signOut } from '@junobuild/core';
import { toaster } from '$lib/skeletonui/toaster-skeleton';
import { loginInProgress } from '$lib/stores/authUser';

/**
 * Global authentication utility
 * Handles login, logout, and notifications
 * 
 * All redirect logic is handled by +layout.svelte
 */

/**
 * Handles user login with toast notifications
 * Sets loginInProgress flag so layout knows this is an active login
 */
export async function handleLogin(currentPath: string) {
    try {
        console.log('Login: Starting login process, opening Internet Identity popup...');
        loginInProgress.set(true);

        await toaster.promise(
            (async () => {
                await signIn();
                console.log('Login: Internet Identity login completed successfully');
            })(),
            {
                loading: { title: 'Logging in...' },
                success: { title: 'Login successful!' },
                error: { title: 'Login failed', description: 'Please try again.' }
            }
        );
    } catch (e) {
        console.error('Login failed:', e);
        loginInProgress.set(false);
    }
}

/**
 * Handles user logout with toast notifications
 */
export async function handleLogout() {
    await toaster.promise(
        signOut(),
        {
            loading: { title: 'Logging out...' },
            success: { title: 'Logged out' },
            error: { title: 'Logout failed', description: 'Please try again.' }
        }
    );
} 