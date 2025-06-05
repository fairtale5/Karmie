import type { UserDocument } from '$lib/types';

interface Review {
  type: 'vote' | 'received';
  target: string;
  value: number;
  tag: string;
  date: string;
  message: string;
}

export const dummyProfileData = {
  user: {
    key: 'user_123',
    data: {
      user_handle: 'demo_user',
      display_name: 'Demo User',
      user_ulid: 'user_123',
      avatar_url: 'https://i.pravatar.cc/100?img=3'
    }
  } as UserDocument,

  trustedCommunities: [
    { tag: 'ICP', score: 850, rank: 5, isTrusted: true, progress: 100 },
    { tag: 'Rust', score: 720, rank: 12, isTrusted: true, progress: 100 }
  ],

  reputationStats: {
    trustedIn: 2,
    totalCommunities: 4,
    activeIn: 4
  },

  activeReputations: [
    { tag: 'ICP', score: 850, rank: 5, isTrusted: true, progress: 100 },
    { tag: 'Rust', score: 720, rank: 12, isTrusted: true, progress: 100 },
    { tag: 'Svelte', score: 450, rank: 25, isTrusted: false, progress: 75 },
    { tag: 'TypeScript', score: 380, rank: 30, isTrusted: false, progress: 60 }
  ],

  communityStats: {
    totalVotesGiven: 156,
    totalVotesReceived: 89,
    trustedCommunities: 2,
    activeCommunities: 4,
    averageScore: 600
  },

  recentReviews: [
    { type: 'vote' as const, target: 'alice', value: 1, tag: 'ICP', date: '2h ago', message: 'Great contribution to the community!' },
    { type: 'vote' as const, target: 'bob', value: -1, tag: 'Rust', date: '5h ago', message: 'Incorrect information provided' },
    { type: 'received' as const, target: 'carol', value: 1, tag: 'Svelte', date: '1d ago', message: 'Helpful explanation' }
  ] as Review[]
}; 