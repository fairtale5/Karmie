import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';
import { get } from 'svelte/store';
import { authUserDoc } from '$lib/stores/authUserDoc';
import { listDocs } from '@junobuild/core';
import type { TagDocument } from '$lib/types';
import { queryDocsByKey } from '$lib/docs-crud/query_by_key';

// Disable prerendering for this dynamic route
export const prerender = false;

// Preview data constants
const PREVIEW_TAG_KEY = '___PREVIEW_DATA___';
const previewTagData: TagDocument = {
  key: PREVIEW_TAG_KEY,
  data: {
    tag_handle: 'preview-mode',
    description: 'Currently displaying sample data. Select or create a real tag to see live information and interact with the platform.',
    reputation_threshold: 10,
    vote_reward: 0.1,
    min_users_for_threshold: 5,
    time_periods: [
      { months: 1, multiplier: 0.95 },
      { months: 3, multiplier: 0.90 },
      { months: 6, multiplier: 0.80 }
    ]
  }
};

// Preview data generators
function generateInitialUserActivityPreview(): any[] {
  const activities = [];
  const peerNames = ['alpha', 'bravo', 'charlie', 'delta', 'echo', 'foxtrot', 'golf', 'hotel', 'india', 'juliet'];
  for (let i = 0; i < 10; i++) {
    activities.push({
      id: `cast-preview-${i}`, 
      type: 'cast', 
      peerName: peerNames[i % peerNames.length],
      value: i < 5 ? 1 : -1, 
      date: new Date(Date.now() - Math.random() * 1000000000).toISOString()
    });
    activities.push({
      id: `received-preview-${i}`, 
      type: 'received', 
      peerName: peerNames[(i + 2) % peerNames.length],
      value: i < 2 ? 1 : -1, 
      date: new Date(Date.now() - Math.random() * 1000000000).toISOString()
    });
  }
  return activities.sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());
}

const previewData = {
  userReputation: { score: 123, rank: 5, badges: ['Active', 'Top Voter'] },
  topUsers: [
    { username: 'alice', score: 200, bar: 1 },
    { username: 'bob', score: 180, bar: 0.9 },
    { username: 'carol', score: 150, bar: 0.75 }
  ],
  recentVotes: [
    { author: 'alice', target: 'bob', value: 1, date: new Date().toISOString() },
    { author: 'carol', target: 'alice', value: -1, date: new Date(Date.now() - 86400000).toISOString() }
  ],
  userRecentActivity: generateInitialUserActivityPreview(),
  stats: {
    totalUsers: 1234,
    verifiedUsers: 567,
    activeUsers: 89
  }
};

export const load: PageLoad = async ({ params }) => {
  const tagHandle = params.tagHandle;
  
  // Case 1: Preview mode - return dummy data immediately
  if (tagHandle === 'preview-mode') {
    return {
      tagHandle,
      tag: previewTagData,
      ...previewData,
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
            userReputation: currentUserDoc ? { score: 123, rank: 5, badges: ['Active', 'Top Voter'] } : null,
            topUsers: [
              { username: 'alice', score: 200, bar: 1 },
              { username: 'bob', score: 180, bar: 0.9 },
              { username: 'carol', score: 150, bar: 0.75 }
            ],
            recentVotes: [
              { author: 'alice', target: 'bob', value: 1, date: new Date().toISOString() },
              { author: 'carol', target: 'alice', value: -1, date: new Date(Date.now() - 86400000).toISOString() }
            ],
            userRecentActivity: currentUserDoc ? generateInitialUserActivityPreview() : [],
            stats: {
              totalUsers: 1234,
              verifiedUsers: 567,
              activeUsers: 89
            },
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