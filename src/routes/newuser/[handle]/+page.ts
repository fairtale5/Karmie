import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';
import { queryDocsByKey } from '$lib/docs-crud/query_by_key';
import { get } from 'svelte/store';
import { authUserDoc } from '$lib/stores/authUserDoc';
import { dummyProfileData } from '$lib/data/dummyProfileData';
import type { UserDocument } from '$lib/types';

export const load: PageLoad = async ({ params }) => {
  const handle = params.handle;
  const currentUserDoc = get(authUserDoc);

  // Case 1: Demo user - return dummy data
  if (handle === 'demo_user') {
    return {
      user: dummyProfileData.user,
      stats: dummyProfileData.communityStats,
      trustedCommunities: dummyProfileData.trustedCommunities,
      reputationStats: dummyProfileData.reputationStats,
      activeReputations: dummyProfileData.activeReputations,
      recentActivity: dummyProfileData.recentActivity
    };
  }

  // Case 2: Logged in user viewing their own profile
  if (currentUserDoc && handle === currentUserDoc.data.user_handle) {
    return {
      user: currentUserDoc,
      stats: dummyProfileData.communityStats,
      trustedCommunities: dummyProfileData.trustedCommunities,
      reputationStats: dummyProfileData.reputationStats,
      activeReputations: dummyProfileData.activeReputations,
      recentActivity: dummyProfileData.recentActivity
    };
  }

  // Case 3: Return a function to fetch user data after Juno is initialized
  return {
    handle,
    fetchUserData: async () => {
      try {
        // Query using the hdl_ prefix pattern as per database schema
        const results = await queryDocsByKey<UserDocument>('users', `hdl_${handle}_`);
        
        if (!results.items.length) {
          throw error(404, 'User not found');
        }

        const userDoc = results.items[0];
        
        return {
          user: userDoc,
          stats: dummyProfileData.communityStats,
          trustedCommunities: dummyProfileData.trustedCommunities,
          reputationStats: dummyProfileData.reputationStats,
          activeReputations: dummyProfileData.activeReputations,
          recentActivity: dummyProfileData.recentActivity
        };
      } catch (e: unknown) {
        console.error('Error fetching user:', e);
        if (e && typeof e === 'object' && 'status' in e && e.status === 404) {
          throw e; // Re-throw 404 errors
        }
        throw error(500, 'Failed to fetch user data');
      }
    }
  };
}; 