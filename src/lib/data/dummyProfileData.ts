import type { UserDocument, VoteDocument, TagDocument } from '$lib/types';

interface Review {
  type: 'vote' | 'received';
  target: string;
  value: number;
  tag: string;
  date: string;
  message: string;
}

// Consolidated dummy data for all preview modes
export const dummyData = {
  // Profile-specific data
  profile: {
  user: {
    key: 'demo_user',
    data: {
      user_handle: 'demo_user',
      display_name: 'Demo User',
      user_ulid: 'demo_user',
      avatar_url: 'https://i.pravatar.cc/100?img=3'
    }
  } as UserDocument,

  trustedCommunities: [
    { tag: 'ICP', score: 850, rank: 5, isTrusted: true, progress: 100 },
    { tag: 'Rust', score: 720, rank: 12, isTrusted: true, progress: 100 }
  ],

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

  reputationStats: {
    overallReputation: 750,
    rank: 15,
    totalContributions: 50,
    positiveFeedback: 45,
    negativeFeedback: 5
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
        owner_ulid: 'demo_user', // demo user casting vote
        target_ulid: 'user_alice',
        tag_ulid: '___PREVIEW_DATA___',
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
        target_ulid: 'demo_user', // demo user receiving vote
        tag_ulid: '___PREVIEW_DATA___',
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
        owner_ulid: 'demo_user', // demo user casting vote
        target_ulid: 'user_carol',
        tag_ulid: '___PREVIEW_DATA___',
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
        target_ulid: 'demo_user', // demo user receiving vote
        tag_ulid: '___PREVIEW_DATA___',
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
        owner_ulid: 'demo_user', // demo user casting vote
        target_ulid: 'user_echo',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_5',
        value: 1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 259200000) * BigInt(1_000_000), // 3d ago in nanoseconds
      updated_at: BigInt(Date.now() - 259200000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_6',
      data: {
        owner_ulid: 'user_frank',
        target_ulid: 'demo_user',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_6',
        value: 1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 345600000) * BigInt(1_000_000), // 4d ago
      updated_at: BigInt(Date.now() - 345600000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_7',
      data: {
        owner_ulid: 'demo_user',
        target_ulid: 'user_grace',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_7',
        value: 1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 432000000) * BigInt(1_000_000), // 5d ago
      updated_at: BigInt(Date.now() - 432000000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_8',
      data: {
        owner_ulid: 'user_henry',
        target_ulid: 'demo_user',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_8',
        value: -1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 518400000) * BigInt(1_000_000), // 6d ago
      updated_at: BigInt(Date.now() - 518400000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_9',
      data: {
        owner_ulid: 'demo_user',
        target_ulid: 'user_iris',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_9',
        value: 1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 604800000) * BigInt(1_000_000), // 7d ago
      updated_at: BigInt(Date.now() - 604800000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_10',
      data: {
        owner_ulid: 'user_jack',
        target_ulid: 'demo_user',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_10',
        value: 1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 691200000) * BigInt(1_000_000), // 8d ago
      updated_at: BigInt(Date.now() - 691200000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_11',
      data: {
        owner_ulid: 'demo_user',
        target_ulid: 'user_kelly',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_11',
        value: 1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 777600000) * BigInt(1_000_000), // 9d ago
      updated_at: BigInt(Date.now() - 777600000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_12',
      data: {
        owner_ulid: 'user_liam',
        target_ulid: 'demo_user',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_12',
        value: 1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 864000000) * BigInt(1_000_000), // 10d ago
      updated_at: BigInt(Date.now() - 864000000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_13',
      data: {
        owner_ulid: 'demo_user',
        target_ulid: 'user_mia',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_13',
        value: -1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 950400000) * BigInt(1_000_000), // 11d ago
      updated_at: BigInt(Date.now() - 950400000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_14',
      data: {
        owner_ulid: 'user_noah',
        target_ulid: 'demo_user',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_14',
        value: 1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 1036800000) * BigInt(1_000_000), // 12d ago
      updated_at: BigInt(Date.now() - 1036800000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_15',
      data: {
        owner_ulid: 'demo_user',
        target_ulid: 'user_olivia',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_15',
        value: 1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 1123200000) * BigInt(1_000_000), // 13d ago
      updated_at: BigInt(Date.now() - 1123200000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_16',
      data: {
        owner_ulid: 'user_paul',
        target_ulid: 'demo_user',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_16',
        value: 1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 1209600000) * BigInt(1_000_000), // 14d ago
      updated_at: BigInt(Date.now() - 1209600000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_17',
      data: {
        owner_ulid: 'demo_user',
        target_ulid: 'user_quinn',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_17',
        value: 1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 1296000000) * BigInt(1_000_000), // 15d ago
      updated_at: BigInt(Date.now() - 1296000000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_18',
      data: {
        owner_ulid: 'user_ruby',
        target_ulid: 'demo_user',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_18',
        value: -1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 1382400000) * BigInt(1_000_000), // 16d ago
      updated_at: BigInt(Date.now() - 1382400000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_19',
      data: {
        owner_ulid: 'demo_user',
        target_ulid: 'user_sam',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_19',
        value: 1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 1468800000) * BigInt(1_000_000), // 17d ago
      updated_at: BigInt(Date.now() - 1468800000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_20',
      data: {
        owner_ulid: 'user_tara',
        target_ulid: 'demo_user',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_20',
        value: 1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 1555200000) * BigInt(1_000_000), // 18d ago
      updated_at: BigInt(Date.now() - 1555200000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_21',
      data: {
        owner_ulid: 'demo_user',
        target_ulid: 'user_uma',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_21',
        value: 1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 1641600000) * BigInt(1_000_000), // 19d ago
      updated_at: BigInt(Date.now() - 1641600000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_22',
      data: {
        owner_ulid: 'user_victor',
        target_ulid: 'demo_user',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_22',
        value: 1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 1728000000) * BigInt(1_000_000), // 20d ago
      updated_at: BigInt(Date.now() - 1728000000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_23',
      data: {
        owner_ulid: 'demo_user',
        target_ulid: 'user_wendy',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_23',
        value: -1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 1814400000) * BigInt(1_000_000), // 21d ago
      updated_at: BigInt(Date.now() - 1814400000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_24',
      data: {
        owner_ulid: 'user_xander',
        target_ulid: 'demo_user',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_24',
        value: 1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 1900800000) * BigInt(1_000_000), // 22d ago
      updated_at: BigInt(Date.now() - 1900800000) * BigInt(1_000_000)
    },
    {
      key: 'vote_demo_25',
      data: {
        owner_ulid: 'demo_user',
        target_ulid: 'user_yara',
        tag_ulid: '___PREVIEW_DATA___',
        vote_ulid: 'vote_demo_25',
        value: 1,
        weight: 1.0
      },
      created_at: BigInt(Date.now() - 1987200000) * BigInt(1_000_000), // 23d ago
      updated_at: BigInt(Date.now() - 1987200000) * BigInt(1_000_000)
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
    },
    {
      key: 'user_frank',
      data: {
        user_handle: 'frank',
        display_name: 'Frank Miller',
        user_ulid: 'user_frank',
        avatar_url: 'https://i.pravatar.cc/100?img=7'
      },
      created_at: BigInt(Date.now() - 86400000 * 35) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 35) * BigInt(1_000_000)
    },
    {
      key: 'user_grace',
      data: {
        user_handle: 'grace',
        display_name: 'Grace Wilson',
        user_ulid: 'user_grace',
        avatar_url: 'https://i.pravatar.cc/100?img=8'
      },
      created_at: BigInt(Date.now() - 86400000 * 40) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 40) * BigInt(1_000_000)
    },
    {
      key: 'user_henry',
      data: {
        user_handle: 'henry',
        display_name: 'Henry Moore',
        user_ulid: 'user_henry',
        avatar_url: 'https://i.pravatar.cc/100?img=9'
      },
      created_at: BigInt(Date.now() - 86400000 * 45) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 45) * BigInt(1_000_000)
    },
    {
      key: 'user_iris',
      data: {
        user_handle: 'iris',
        display_name: 'Iris Taylor',
        user_ulid: 'user_iris',
        avatar_url: 'https://i.pravatar.cc/100?img=10'
      },
      created_at: BigInt(Date.now() - 86400000 * 50) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 50) * BigInt(1_000_000)
    },
    {
      key: 'user_jack',
      data: {
        user_handle: 'jack',
        display_name: 'Jack Anderson',
        user_ulid: 'user_jack',
        avatar_url: 'https://i.pravatar.cc/100?img=11'
      },
      created_at: BigInt(Date.now() - 86400000 * 55) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 55) * BigInt(1_000_000)
    },
    {
      key: 'user_kelly',
      data: {
        user_handle: 'kelly',
        display_name: 'Kelly Thomas',
        user_ulid: 'user_kelly',
        avatar_url: 'https://i.pravatar.cc/100?img=12'
      },
      created_at: BigInt(Date.now() - 86400000 * 60) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 60) * BigInt(1_000_000)
    },
    {
      key: 'user_liam',
      data: {
        user_handle: 'liam',
        display_name: 'Liam Jackson',
        user_ulid: 'user_liam',
        avatar_url: 'https://i.pravatar.cc/100?img=13'
      },
      created_at: BigInt(Date.now() - 86400000 * 65) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 65) * BigInt(1_000_000)
    },
    {
      key: 'user_mia',
      data: {
        user_handle: 'mia',
        display_name: 'Mia White',
        user_ulid: 'user_mia',
        avatar_url: 'https://i.pravatar.cc/100?img=14'
      },
      created_at: BigInt(Date.now() - 86400000 * 70) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 70) * BigInt(1_000_000)
    },
    {
      key: 'user_noah',
      data: {
        user_handle: 'noah',
        display_name: 'Noah Harris',
        user_ulid: 'user_noah',
        avatar_url: 'https://i.pravatar.cc/100?img=15'
      },
      created_at: BigInt(Date.now() - 86400000 * 75) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 75) * BigInt(1_000_000)
    },
    {
      key: 'user_olivia',
      data: {
        user_handle: 'olivia',
        display_name: 'Olivia Martin',
        user_ulid: 'user_olivia',
        avatar_url: 'https://i.pravatar.cc/100?img=16'
      },
      created_at: BigInt(Date.now() - 86400000 * 80) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 80) * BigInt(1_000_000)
    },
    {
      key: 'user_paul',
      data: {
        user_handle: 'paul',
        display_name: 'Paul Thompson',
        user_ulid: 'user_paul',
        avatar_url: 'https://i.pravatar.cc/100?img=17'
      },
      created_at: BigInt(Date.now() - 86400000 * 85) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 85) * BigInt(1_000_000)
    },
    {
      key: 'user_quinn',
      data: {
        user_handle: 'quinn',
        display_name: 'Quinn Garcia',
        user_ulid: 'user_quinn',
        avatar_url: 'https://i.pravatar.cc/100?img=18'
      },
      created_at: BigInt(Date.now() - 86400000 * 90) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 90) * BigInt(1_000_000)
    },
    {
      key: 'user_ruby',
      data: {
        user_handle: 'ruby',
        display_name: 'Ruby Martinez',
        user_ulid: 'user_ruby',
        avatar_url: 'https://i.pravatar.cc/100?img=19'
      },
      created_at: BigInt(Date.now() - 86400000 * 95) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 95) * BigInt(1_000_000)
    },
    {
      key: 'user_sam',
      data: {
        user_handle: 'sam',
        display_name: 'Sam Rodriguez',
        user_ulid: 'user_sam',
        avatar_url: 'https://i.pravatar.cc/100?img=20'
      },
      created_at: BigInt(Date.now() - 86400000 * 100) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 100) * BigInt(1_000_000)
    },
    {
      key: 'user_tara',
      data: {
        user_handle: 'tara',
        display_name: 'Tara Lewis',
        user_ulid: 'user_tara',
        avatar_url: 'https://i.pravatar.cc/100?img=21'
      },
      created_at: BigInt(Date.now() - 86400000 * 105) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 105) * BigInt(1_000_000)
    },
    {
      key: 'user_uma',
      data: {
        user_handle: 'uma',
        display_name: 'Uma Lee',
        user_ulid: 'user_uma',
        avatar_url: 'https://i.pravatar.cc/100?img=22'
      },
      created_at: BigInt(Date.now() - 86400000 * 110) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 110) * BigInt(1_000_000)
    },
    {
      key: 'user_victor',
      data: {
        user_handle: 'victor',
        display_name: 'Victor Walker',
        user_ulid: 'user_victor',
        avatar_url: 'https://i.pravatar.cc/100?img=23'
      },
      created_at: BigInt(Date.now() - 86400000 * 115) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 115) * BigInt(1_000_000)
    },
    {
      key: 'user_wendy',
      data: {
        user_handle: 'wendy',
        display_name: 'Wendy Hall',
        user_ulid: 'user_wendy',
        avatar_url: 'https://i.pravatar.cc/100?img=24'
      },
      created_at: BigInt(Date.now() - 86400000 * 120) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 120) * BigInt(1_000_000)
    },
    {
      key: 'user_xander',
      data: {
        user_handle: 'xander',
        display_name: 'Xander Allen',
        user_ulid: 'user_xander',
        avatar_url: 'https://i.pravatar.cc/100?img=25'
      },
      created_at: BigInt(Date.now() - 86400000 * 125) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 125) * BigInt(1_000_000)
    },
    {
      key: 'user_yara',
      data: {
        user_handle: 'yara',
        display_name: 'Yara Young',
        user_ulid: 'user_yara',
        avatar_url: 'https://i.pravatar.cc/100?img=26'
      },
      created_at: BigInt(Date.now() - 86400000 * 130) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 130) * BigInt(1_000_000)
    },
    {
      key: 'demo_user',
      data: {
        user_handle: 'demo_user',
        display_name: 'Demo User',
        user_ulid: 'demo_user',
        avatar_url: 'https://i.pravatar.cc/100?img=3'
      },
      created_at: BigInt(Date.now() - 86400000 * 135) * BigInt(1_000_000),
      updated_at: BigInt(Date.now() - 86400000 * 135) * BigInt(1_000_000)
    }
  ] as UserDocument[]
  },

  // Tag-specific data
  tag: {
    // Preview tag document
    previewTag: {
      key: '___PREVIEW_DATA___',
      data: {
        tag_handle: 'preview-mode',
        description: 'Currently displaying sample data. Select or create a real tag to see live information and interact with the platform.',
        tag_ulid: '___PREVIEW_DATA___',
        owner_ulid: 'demo_user',
        reputation_threshold: 10,
        vote_reward: 0.1,
        min_users_for_threshold: 5,
        time_periods: [
          { months: 1, multiplier: 0.95 },
          { months: 3, multiplier: 0.90 },
          { months: 6, multiplier: 0.80 }
        ]
      }
    } as TagDocument,

    // Tag stats
    stats: {
      totalUsers: 47,
      trustedUsers: 23,
      totalVotes: 156
    }
  }
};

// Legacy export for backward compatibility
export const dummyProfileData = dummyData.profile; 