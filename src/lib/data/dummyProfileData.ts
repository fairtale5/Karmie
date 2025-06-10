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
        target_ulid: 'demo_user', // demo user receiving vote
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
        owner_ulid: 'demo_user', // demo user casting vote
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
        target_ulid: 'demo_user', // demo user receiving vote
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
        owner_ulid: 'demo_user', // demo user casting vote
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
    },

    // Top users for tags (extended to 20+ users)
    topUsers: [
      {
        userDocument: {
          key: 'usr_demo_user_1',
          data: {
            user_handle: 'alice_crypto',
            user_ulid: 'demo_user_1',
            display_name: 'Alice Crypto',
            avatar_url: 'https://i.pravatar.cc/100?img=1'
          }
        },
        reputationDocument: {
          key: 'rep_demo_1',
          data: {
            owner_ulid: 'demo_user_1',
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 250,
            has_voting_power: true,
            reputation_basis: 220,
            reputation_rewards: 30,
            last_calculation: '2024-01-15T10:30:00.000Z',
            vote_weight: 1.0,
            severity: 'low'
          }
        },
        score: 250,
        isTrusted: true
      },
      {
        userDocument: {
          key: 'usr_demo_user_2',
          data: {
            user_handle: 'bob_dev',
            user_ulid: 'demo_user_2',
            display_name: 'Bob Developer',
            avatar_url: 'https://i.pravatar.cc/100?img=2'
          }
        },
        reputationDocument: {
          key: 'rep_demo_2',
          data: {
            owner_ulid: 'demo_user_2',
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 230,
            has_voting_power: true,
            reputation_basis: 210,
            reputation_rewards: 20,
            last_calculation: '2024-01-15T10:30:00.000Z',
            vote_weight: 0.9,
            severity: 'low'
          }
        },
        score: 230,
        isTrusted: true
      },
      {
        userDocument: {
          key: 'usr_demo_user_3',
          data: {
            user_handle: 'carol_ui',
            user_ulid: 'demo_user_3',
            display_name: 'Carol Designer',
            avatar_url: 'https://i.pravatar.cc/100?img=4'
          }
        },
        reputationDocument: {
          key: 'rep_demo_3',
          data: {
            owner_ulid: 'demo_user_3',
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 210,
            has_voting_power: true,
            reputation_basis: 190,
            reputation_rewards: 20,
            last_calculation: '2024-01-15T10:30:00.000Z',
            vote_weight: 0.8,
            severity: 'low'
          }
        },
        score: 210,
        isTrusted: true
      },
      {
        userDocument: {
          key: 'usr_demo_user_4',
          data: {
            user_handle: 'david_smart',
            user_ulid: 'demo_user_4',
            display_name: 'David Contracts',
            avatar_url: 'https://i.pravatar.cc/100?img=7'
          }
        },
        reputationDocument: {
          key: 'rep_demo_4',
          data: {
            owner_ulid: 'demo_user_4',
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 195,
            has_voting_power: true,
            reputation_basis: 180,
            reputation_rewards: 15,
            last_calculation: '2024-01-15T10:30:00.000Z',
            vote_weight: 0.8,
            severity: 'low'
          }
        },
        score: 195,
        isTrusted: true
      },
      {
        userDocument: {
          key: 'usr_demo_user_5',
          data: {
            user_handle: 'eve_protocol',
            user_ulid: 'demo_user_5',
            display_name: 'Eve Protocol',
            avatar_url: 'https://i.pravatar.cc/100?img=8'
          }
        },
        reputationDocument: {
          key: 'rep_demo_5',
          data: {
            owner_ulid: 'demo_user_5',
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 180,
            has_voting_power: true,
            reputation_basis: 165,
            reputation_rewards: 15,
            last_calculation: '2024-01-15T10:30:00.000Z',
            vote_weight: 0.7,
            severity: 'low'
          }
        },
        score: 180,
        isTrusted: true
      },
      {
        userDocument: {
          key: 'usr_demo_user_6',
          data: {
            user_handle: 'frank_defi',
            user_ulid: 'demo_user_6',
            display_name: 'Frank DeFi',
            avatar_url: 'https://i.pravatar.cc/100?img=9'
          }
        },
        reputationDocument: {
          key: 'rep_demo_6',
          data: {
            owner_ulid: 'demo_user_6',
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 165,
            has_voting_power: true,
            reputation_basis: 150,
            reputation_rewards: 15,
            last_calculation: '2024-01-15T10:30:00.000Z',
            vote_weight: 0.7,
            severity: 'low'
          }
        },
        score: 165,
        isTrusted: true
      },
      {
        userDocument: {
          key: 'usr_demo_user_7',
          data: {
            user_handle: 'grace_dao',
            user_ulid: 'demo_user_7',
            display_name: 'Grace DAO',
            avatar_url: 'https://i.pravatar.cc/100?img=10'
          }
        },
        reputationDocument: {
          key: 'rep_demo_7',
          data: {
            owner_ulid: 'demo_user_7',
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 150,
            has_voting_power: true,
            reputation_basis: 140,
            reputation_rewards: 10,
            last_calculation: '2024-01-15T10:30:00.000Z',
            vote_weight: 0.6,
            severity: 'medium'
          }
        },
        score: 150,
        isTrusted: true
      },
      {
        userDocument: {
          key: 'usr_demo_user_8',
          data: {
            user_handle: 'henry_nft',
            user_ulid: 'demo_user_8',
            display_name: 'Henry NFT',
            avatar_url: 'https://i.pravatar.cc/100?img=11'
          }
        },
        reputationDocument: {
          key: 'rep_demo_8',
          data: {
            owner_ulid: 'demo_user_8',
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 140,
            has_voting_power: true,
            reputation_basis: 130,
            reputation_rewards: 10,
            last_calculation: '2024-01-15T10:30:00.000Z',
            vote_weight: 0.6,
            severity: 'medium'
          }
        },
        score: 140,
        isTrusted: true
      },
      {
        userDocument: {
          key: 'usr_demo_user_9',
          data: {
            user_handle: 'iris_web3',
            user_ulid: 'demo_user_9',
            display_name: 'Iris Web3',
            avatar_url: 'https://i.pravatar.cc/100?img=12'
          }
        },
        reputationDocument: {
          key: 'rep_demo_9',
          data: {
            owner_ulid: 'demo_user_9',
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 130,
            has_voting_power: true,
            reputation_basis: 120,
            reputation_rewards: 10,
            last_calculation: '2024-01-15T10:30:00.000Z',
            vote_weight: 0.5,
            severity: 'medium'
          }
        },
        score: 130,
        isTrusted: true
      },
      {
        userDocument: {
          key: 'usr_demo_user_10',
          data: {
            user_handle: 'jack_metaverse',
            user_ulid: 'demo_user_10',
            display_name: 'Jack Metaverse',
            avatar_url: 'https://i.pravatar.cc/100?img=13'
          }
        },
        reputationDocument: {
          key: 'rep_demo_10',
          data: {
            owner_ulid: 'demo_user_10',
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 125,
            has_voting_power: true,
            reputation_basis: 115,
            reputation_rewards: 10,
            last_calculation: '2024-01-15T10:30:00.000Z',
            vote_weight: 0.5,
            severity: 'medium'
          }
        },
        score: 125,
        isTrusted: true
      },
      {
        userDocument: {
          key: 'usr_demo_user_11',
          data: {
            user_handle: 'kelly_gamefi',
            user_ulid: 'demo_user_11',
            display_name: 'Kelly GameFi',
            avatar_url: 'https://i.pravatar.cc/100?img=14'
          }
        },
        reputationDocument: {
          key: 'rep_demo_11',
          data: {
            owner_ulid: 'demo_user_11',
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 120,
            has_voting_power: true,
            reputation_basis: 110,
            reputation_rewards: 10,
            last_calculation: '2024-01-15T10:30:00.000Z',
            vote_weight: 0.5,
            severity: 'medium'
          }
        },
        score: 120,
        isTrusted: true
      },
      {
        userDocument: {
          key: 'usr_demo_user_12',
          data: {
            user_handle: 'liam_validator',
            user_ulid: 'demo_user_12',
            display_name: 'Liam Validator',
            avatar_url: 'https://i.pravatar.cc/100?img=15'
          }
        },
        reputationDocument: {
          key: 'rep_demo_12',
          data: {
            owner_ulid: 'demo_user_12',
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 115,
            has_voting_power: true,
            reputation_basis: 105,
            reputation_rewards: 10,
            last_calculation: '2024-01-15T10:30:00.000Z',
            vote_weight: 0.4,
            severity: 'medium'
          }
        },
        score: 115,
        isTrusted: true
      },
      {
        userDocument: {
          key: 'usr_demo_user_13',
          data: {
            user_handle: 'mia_bridge',
            user_ulid: 'demo_user_13',
            display_name: 'Mia Bridge',
            avatar_url: 'https://i.pravatar.cc/100?img=16'
          }
        },
        reputationDocument: {
          key: 'rep_demo_13',
          data: {
            owner_ulid: 'demo_user_13',
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 110,
            has_voting_power: false,
            reputation_basis: 100,
            reputation_rewards: 10,
            last_calculation: '2024-01-15T10:30:00.000Z',
            vote_weight: 0.4,
            severity: 'medium'
          }
        },
        score: 110,
        isTrusted: false
      },
      {
        userDocument: {
          key: 'usr_demo_user_14',
          data: {
            user_handle: 'noah_oracle',
            user_ulid: 'demo_user_14',
            display_name: 'Noah Oracle',
            avatar_url: 'https://i.pravatar.cc/100?img=17'
          }
        },
        reputationDocument: {
          key: 'rep_demo_14',
          data: {
            owner_ulid: 'demo_user_14',
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 105,
            has_voting_power: false,
            reputation_basis: 95,
            reputation_rewards: 10,
            last_calculation: '2024-01-15T10:30:00.000Z',
            vote_weight: 0.4,
            severity: 'medium'
          }
        },
        score: 105,
        isTrusted: false
      },
      {
        userDocument: {
          key: 'usr_demo_user_15',
          data: {
            user_handle: 'olivia_layer2',
            user_ulid: 'demo_user_15',
            display_name: 'Olivia Layer2',
            avatar_url: 'https://i.pravatar.cc/100?img=18'
          }
        },
        reputationDocument: {
          key: 'rep_demo_15',
          data: {
            owner_ulid: 'demo_user_15',
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 100,
            has_voting_power: false,
            reputation_basis: 90,
            reputation_rewards: 10,
            last_calculation: '2024-01-15T10:30:00.000Z',
            vote_weight: 0.3,
            severity: 'medium'
          }
        },
        score: 100,
        isTrusted: false
      },
      {
        userDocument: {
          key: 'usr_demo_user_16',
          data: {
            user_handle: 'paul_zk',
            user_ulid: 'demo_user_16',
            display_name: 'Paul ZK',
            avatar_url: 'https://i.pravatar.cc/100?img=19'
          }
        },
        reputationDocument: {
          key: 'rep_demo_16',
          data: {
            owner_ulid: 'demo_user_16',
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 95,
            has_voting_power: false,
            reputation_basis: 85,
            reputation_rewards: 10,
            last_calculation: '2024-01-15T10:30:00.000Z',
            vote_weight: 0.3,
            severity: 'medium'
          }
        },
        score: 95,
        isTrusted: false
      },
      {
        userDocument: {
          key: 'usr_demo_user_17',
          data: {
            user_handle: 'quinn_rollup',
            user_ulid: 'demo_user_17',
            display_name: 'Quinn Rollup',
            avatar_url: 'https://i.pravatar.cc/100?img=20'
          }
        },
        reputationDocument: {
          key: 'rep_demo_17',
          data: {
            owner_ulid: 'demo_user_17',
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 90,
            has_voting_power: false,
            reputation_basis: 80,
            reputation_rewards: 10,
            last_calculation: '2024-01-15T10:30:00.000Z',
            vote_weight: 0.3,
            severity: 'medium'
          }
        },
        score: 90,
        isTrusted: false
      },
      {
        userDocument: {
          key: 'usr_demo_user_18',
          data: {
            user_handle: 'ruby_staking',
            user_ulid: 'demo_user_18',
            display_name: 'Ruby Staking',
            avatar_url: 'https://i.pravatar.cc/100?img=21'
          }
        },
        reputationDocument: {
          key: 'rep_demo_18',
          data: {
            owner_ulid: 'demo_user_18',
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 85,
            has_voting_power: false,
            reputation_basis: 75,
            reputation_rewards: 10,
            last_calculation: '2024-01-15T10:30:00.000Z',
            vote_weight: 0.3,
            severity: 'medium'
          }
        },
        score: 85,
        isTrusted: false
      },
      {
        userDocument: {
          key: 'usr_demo_user_19',
          data: {
            user_handle: 'sam_consensus',
            user_ulid: 'demo_user_19',
            display_name: 'Sam Consensus',
            avatar_url: 'https://i.pravatar.cc/100?img=22'
          }
        },
        reputationDocument: {
          key: 'rep_demo_19',
          data: {
            owner_ulid: 'demo_user_19',
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 80,
            has_voting_power: false,
            reputation_basis: 70,
            reputation_rewards: 10,
            last_calculation: '2024-01-15T10:30:00.000Z',
            vote_weight: 0.2,
            severity: 'high'
          }
        },
        score: 80,
        isTrusted: false
      },
      {
        userDocument: {
          key: 'usr_demo_user_20',
          data: {
            user_handle: 'tara_yield',
            user_ulid: 'demo_user_20',
            display_name: 'Tara Yield',
            avatar_url: 'https://i.pravatar.cc/100?img=23'
          }
        },
        reputationDocument: {
          key: 'rep_demo_20',
          data: {
            owner_ulid: 'demo_user_20',
            tag_ulid: '___PREVIEW_DATA___',
            reputation_total_effective: 75,
            has_voting_power: false,
            reputation_basis: 65,
            reputation_rewards: 10,
            last_calculation: '2024-01-15T10:30:00.000Z',
            vote_weight: 0.2,
            severity: 'high'
          }
        },
        score: 75,
        isTrusted: false
      }
    ],

    // Recent votes for tags (extended to 22 votes)
    recentVotes: [
      {
        key: 'preview_vote_1',
        data: {
          owner_ulid: 'demo_user_1',
          target_ulid: 'demo_user_2',
          value: 1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_2',
        data: {
          owner_ulid: 'demo_user_3',
          target_ulid: 'demo_user_1',
          value: -1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_3',
        data: {
          owner_ulid: 'demo_user_2',
          target_ulid: 'demo_user_3',
          value: 1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_4',
        data: {
          owner_ulid: 'demo_user_4',
          target_ulid: 'demo_user_5',
          value: 1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_5',
        data: {
          owner_ulid: 'demo_user_5',
          target_ulid: 'demo_user_6',
          value: -1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_6',
        data: {
          owner_ulid: 'demo_user_6',
          target_ulid: 'demo_user_7',
          value: 1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_7',
        data: {
          owner_ulid: 'demo_user_7',
          target_ulid: 'demo_user_8',
          value: 1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_8',
        data: {
          owner_ulid: 'demo_user_8',
          target_ulid: 'demo_user_9',
          value: -1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_9',
        data: {
          owner_ulid: 'demo_user_9',
          target_ulid: 'demo_user_10',
          value: 1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_10',
        data: {
          owner_ulid: 'demo_user_10',
          target_ulid: 'demo_user_11',
          value: 1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_11',
        data: {
          owner_ulid: 'demo_user_11',
          target_ulid: 'demo_user_12',
          value: -1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_12',
        data: {
          owner_ulid: 'demo_user_12',
          target_ulid: 'demo_user_13',
          value: 1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_13',
        data: {
          owner_ulid: 'demo_user_13',
          target_ulid: 'demo_user_14',
          value: 1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_14',
        data: {
          owner_ulid: 'demo_user_14',
          target_ulid: 'demo_user_15',
          value: -1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_15',
        data: {
          owner_ulid: 'demo_user_15',
          target_ulid: 'demo_user_16',
          value: 1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_16',
        data: {
          owner_ulid: 'demo_user_16',
          target_ulid: 'demo_user_17',
          value: 1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_17',
        data: {
          owner_ulid: 'demo_user_17',
          target_ulid: 'demo_user_18',
          value: -1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_18',
        data: {
          owner_ulid: 'demo_user_18',
          target_ulid: 'demo_user_19',
          value: 1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_19',
        data: {
          owner_ulid: 'demo_user_19',
          target_ulid: 'demo_user_20',
          value: 1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_20',
        data: {
          owner_ulid: 'demo_user_20',
          target_ulid: 'demo_user_1',
          value: -1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_21',
        data: {
          owner_ulid: 'demo_user_1',
          target_ulid: 'demo_user_15',
          value: 1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      },
      {
        key: 'preview_vote_22',
        data: {
          owner_ulid: 'demo_user_8',
          target_ulid: 'demo_user_3',
          value: 1,
          tag_ulid: '___PREVIEW_DATA___'
        }
      }
    ] as VoteDocument[],

    // User data map for tags (extended with all 20 users)
    userData: new Map([
      ['demo_user_1', {
        key: 'usr_demo_user_1',
        data: {
          user_handle: 'alice_crypto',
          user_ulid: 'demo_user_1',
          display_name: 'Alice Crypto',
          avatar_url: 'https://i.pravatar.cc/100?img=1'
        }
      }],
      ['demo_user_2', {
        key: 'usr_demo_user_2',
        data: {
          user_handle: 'bob_dev',
          user_ulid: 'demo_user_2',
          display_name: 'Bob Developer',
          avatar_url: 'https://i.pravatar.cc/100?img=2'
        }
      }],
      ['demo_user_3', {
        key: 'usr_demo_user_3',
        data: {
          user_handle: 'carol_ui',
          user_ulid: 'demo_user_3',
          display_name: 'Carol Designer',
          avatar_url: 'https://i.pravatar.cc/100?img=4'
        }
      }],
      ['demo_user_4', {
        key: 'usr_demo_user_4',
        data: {
          user_handle: 'david_smart',
          user_ulid: 'demo_user_4',
          display_name: 'David Contracts',
          avatar_url: 'https://i.pravatar.cc/100?img=7'
        }
      }],
      ['demo_user_5', {
        key: 'usr_demo_user_5',
        data: {
          user_handle: 'eve_protocol',
          user_ulid: 'demo_user_5',
          display_name: 'Eve Protocol',
          avatar_url: 'https://i.pravatar.cc/100?img=8'
        }
      }],
      ['demo_user_6', {
        key: 'usr_demo_user_6',
        data: {
          user_handle: 'frank_defi',
          user_ulid: 'demo_user_6',
          display_name: 'Frank DeFi',
          avatar_url: 'https://i.pravatar.cc/100?img=9'
        }
      }],
      ['demo_user_7', {
        key: 'usr_demo_user_7',
        data: {
          user_handle: 'grace_dao',
          user_ulid: 'demo_user_7',
          display_name: 'Grace DAO',
          avatar_url: 'https://i.pravatar.cc/100?img=10'
        }
      }],
      ['demo_user_8', {
        key: 'usr_demo_user_8',
        data: {
          user_handle: 'henry_nft',
          user_ulid: 'demo_user_8',
          display_name: 'Henry NFT',
          avatar_url: 'https://i.pravatar.cc/100?img=11'
        }
      }],
      ['demo_user_9', {
        key: 'usr_demo_user_9',
        data: {
          user_handle: 'iris_web3',
          user_ulid: 'demo_user_9',
          display_name: 'Iris Web3',
          avatar_url: 'https://i.pravatar.cc/100?img=12'
        }
      }],
      ['demo_user_10', {
        key: 'usr_demo_user_10',
        data: {
          user_handle: 'jack_metaverse',
          user_ulid: 'demo_user_10',
          display_name: 'Jack Metaverse',
          avatar_url: 'https://i.pravatar.cc/100?img=13'
        }
      }],
      ['demo_user_11', {
        key: 'usr_demo_user_11',
        data: {
          user_handle: 'kelly_gamefi',
          user_ulid: 'demo_user_11',
          display_name: 'Kelly GameFi',
          avatar_url: 'https://i.pravatar.cc/100?img=14'
        }
      }],
      ['demo_user_12', {
        key: 'usr_demo_user_12',
        data: {
          user_handle: 'liam_validator',
          user_ulid: 'demo_user_12',
          display_name: 'Liam Validator',
          avatar_url: 'https://i.pravatar.cc/100?img=15'
        }
      }],
      ['demo_user_13', {
        key: 'usr_demo_user_13',
        data: {
          user_handle: 'mia_bridge',
          user_ulid: 'demo_user_13',
          display_name: 'Mia Bridge',
          avatar_url: 'https://i.pravatar.cc/100?img=16'
        }
      }],
      ['demo_user_14', {
        key: 'usr_demo_user_14',
        data: {
          user_handle: 'noah_oracle',
          user_ulid: 'demo_user_14',
          display_name: 'Noah Oracle',
          avatar_url: 'https://i.pravatar.cc/100?img=17'
        }
      }],
      ['demo_user_15', {
        key: 'usr_demo_user_15',
        data: {
          user_handle: 'olivia_layer2',
          user_ulid: 'demo_user_15',
          display_name: 'Olivia Layer2',
          avatar_url: 'https://i.pravatar.cc/100?img=18'
        }
      }],
      ['demo_user_16', {
        key: 'usr_demo_user_16',
        data: {
          user_handle: 'paul_zk',
          user_ulid: 'demo_user_16',
          display_name: 'Paul ZK',
          avatar_url: 'https://i.pravatar.cc/100?img=19'
        }
      }],
      ['demo_user_17', {
        key: 'usr_demo_user_17',
        data: {
          user_handle: 'quinn_rollup',
          user_ulid: 'demo_user_17',
          display_name: 'Quinn Rollup',
          avatar_url: 'https://i.pravatar.cc/100?img=20'
        }
      }],
      ['demo_user_18', {
        key: 'usr_demo_user_18',
        data: {
          user_handle: 'ruby_staking',
          user_ulid: 'demo_user_18',
          display_name: 'Ruby Staking',
          avatar_url: 'https://i.pravatar.cc/100?img=21'
        }
      }],
      ['demo_user_19', {
        key: 'usr_demo_user_19',
        data: {
          user_handle: 'sam_consensus',
          user_ulid: 'demo_user_19',
          display_name: 'Sam Consensus',
          avatar_url: 'https://i.pravatar.cc/100?img=22'
        }
      }],
      ['demo_user_20', {
        key: 'usr_demo_user_20',
        data: {
          user_handle: 'tara_yield',
          user_ulid: 'demo_user_20',
          display_name: 'Tara Yield',
          avatar_url: 'https://i.pravatar.cc/100?img=23'
        }
      }]
    ])
  }
};

// Legacy export for backward compatibility
export const dummyProfileData = dummyData.profile; 