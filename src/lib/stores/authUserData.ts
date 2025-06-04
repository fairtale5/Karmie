import { derived } from 'svelte/store';
import { authUserDoc } from '$lib/stores/authUserDoc';

/**
 * Derived store that provides the correct profile link based on authentication state.
 * Returns `/u/demo_user` for logged-out users and `/u/[handle]` for logged-in users.
 * 
 * @example
 * import { profileLink } from '$lib/stores/authUserData';
 * <Navigation.Tile href={$profileLink}>Profile</Navigation.Tile>
 */
export const profileLink = derived(authUserDoc, ($doc) =>
  $doc ? `/u/${$doc.data.user_handle}` : '/u/demo_user'
); 