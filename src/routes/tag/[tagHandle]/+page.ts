import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';
import { get } from 'svelte/store';
import { authUserDoc } from '$lib/stores/authUserDoc';
import { listDocs } from '@junobuild/core';
import type { TagDocument } from '$lib/types';
import { queryDocsByKey } from '$lib/docs-crud/query_by_key';
import { dummyData } from '$lib/data/dummyProfileData';

// Disable prerendering for this dynamic route
export const prerender = false;

export const load: PageLoad = async ({ params }) => {
  const tagHandle = params.tagHandle;
  
  // Case 1: Preview mode - return dummy data immediately
  if (tagHandle === 'preview-mode') {
    return {
      tagHandle,
      tag: dummyData.tag.previewTag,
      userRecentActivity: [],
      stats: dummyData.tag.stats,
      isPreview: true
    };
  }

  // Case 2: Try to fetch real data, but don't fail if Juno isn't initialized yet
  try {
    // For non-preview tags, we'll always return the fetch function
    // because we can't guarantee Juno is initialized at this point
    return {
      tagHandle,
      fetchTagData: async () => {
        try {
          // Normalize handle to lowercase to match database storage format
          const normalizedHandle = tagHandle.toLowerCase();
          
          // Query using the normalized handle pattern in the key (tags are stored as: usr_{userUlid}_tag_{tagUlid}_hdl_{tagHandle}_)
          const results = await queryDocsByKey<TagDocument>('tags', `hdl_${normalizedHandle}_`);
          
          if (!results.items.length) {
            throw error(404, 'Tag not found');
          }
          
          const tag = results.items[0];

          // Get current user from store at fetch time
          const currentUserDoc = get(authUserDoc);

          // Fetch tag-specific data
          const tagData = {
            tag,
            userRecentActivity: currentUserDoc ? [] : [],
            isPreview: false
          };

          return tagData;
        } catch (e: unknown) {
          console.error('Error fetching tag:', e);
          if (e && typeof e === 'object' && 'status' in e && e.status === 404) {
            throw e; // Re-throw 404 errors
          }
          throw error(500, 'Failed to fetch tag data');
        }
      }
    };
  } catch (e) {
    // If anything fails during load, still return the fetch function
    // The component will handle the initialization
    return {
      tagHandle,
      fetchTagData: async () => {
        throw error(500, 'Failed to initialize tag data');
      }
    };
  }
}; 