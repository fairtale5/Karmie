import type { UserDocument, VoteDocument } from '$lib/types';

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
  ] as Review[],

  recentVotes: [
    {
      key: 'vote_demo_1',
      data: {
        owner_ulid: 'user_123', // demo user casting vote
        target_ulid: 'user_alice',
        tag_ulid: 'tag_icp',
        vote_ulid: 'vote_demo_1',
        value: 1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 7200000) * BigInt(1_000_000), // 2h ago in nanoseconds
      updated_at: BigInt(Date.now() - 7200000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_2',
      data: {
        owner_ulid: 'user_bob',
        target_ulid: 'user_123', // demo user receiving vote
        tag_ulid: 'tag_rust',
        vote_ulid: 'vote_demo_2',
        value: 1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 18000000) * BigInt(1_000_000), // 5h ago in nanoseconds
      updated_at: BigInt(Date.now() - 18000000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_3',
      data: {
        owner_ulid: 'user_123', // demo user casting vote
        target_ulid: 'user_carol',
        tag_ulid: 'tag_svelte',
        vote_ulid: 'vote_demo_3',
        value: -1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 86400000) * BigInt(1_000_000), // 1d ago in nanoseconds
      updated_at: BigInt(Date.now() - 86400000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_4',
      data: {
        owner_ulid: 'user_delta',
        target_ulid: 'user_123', // demo user receiving vote
        tag_ulid: 'tag_typescript',
        vote_ulid: 'vote_demo_4',
        value: 1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 172800000) * BigInt(1_000_000), // 2d ago in nanoseconds
      updated_at: BigInt(Date.now() - 172800000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_5',
      data: {
        owner_ulid: 'user_123', // demo user casting vote
        target_ulid: 'user_echo',
        tag_ulid: 'tag_icp',
        vote_ulid: 'vote_demo_5',
        value: 1,
        weight: 1.0
      },
             created_at: BigInt(Date.now() - 259200000) * BigInt(1_000_000), // 3d ago in nanoseconds
       updated_at: BigInt(Date.now() - 259200000) * BigInt(1_000_000)
     }
   ] as VoteDocument[],

  // Dummy user data for users referenced in votes
  dummyUsers: [
    {
      key: 'user_alice',
      data: {
        user_handle: 'alice',
        display_name: 'Alice Johnson',
        user_ulid: 'user_alice',
        avatar_url: 'https://i.pravatar.cc/100?img=1'
      },
      created_at: BigInt(Date.now() - 86400000 * 30) * BigInt(1_000_000), // 30d ago
      updated_at: BigInt(Date.now() - 86400000 * 30) * BigInt(1_000_000)
    },
    {
      key: 'user_bob',
      data: {
        user_handle: 'bob',
        display_name: 'Bob Smith',
        user_ulid: 'user_bob',
        avatar_url: 'https://i.pravatar.cc/100?img=2'
      },
      created_at: BigInt(Date.now() - 86400000 * 25) * BigInt(1_000_000), // 25d ago
      updated_at: BigInt(Date.now() - 86400000 * 25) * BigInt(1_000_000)
    },
    {
      key: 'user_carol',
      data: {
        user_handle: 'carol',
        display_name: 'Carol Williams',
        user_ulid: 'user_carol',
        avatar_url: 'https://i.pravatar.cc/100?img=4'
      },
      created_at: BigInt(Date.now() - 86400000 * 20) * BigInt(1_000_000), // 20d ago
      updated_at: BigInt(Date.now() - 86400000 * 20) * BigInt(1_000_000)
    },
    {
      key: 'user_delta',
      data: {
        user_handle: 'delta',
        display_name: 'Delta Brown',
        user_ulid: 'user_delta',
        avatar_url: 'https://i.pravatar.cc/100?img=5'
      },
      created_at: BigInt(Date.now() - 86400000 * 15) * BigInt(1_000_000), // 15d ago
      updated_at: BigInt(Date.now() - 86400000 * 15) * BigInt(1_000_000)
    },
    {
      key: 'user_echo',
      data: {
        user_handle: 'echo',
        display_name: 'Echo Davis',
        user_ulid: 'user_echo',
        avatar_url: 'https://i.pravatar.cc/100?img=6'
      },
      created_at: BigInt(Date.now() - 86400000 * 10) * BigInt(1_000_000), // 10d ago
      updated_at: BigInt(Date.now() - 86400000 * 10) * BigInt(1_000_000)
    }
  ] as UserDocument[]
}; 