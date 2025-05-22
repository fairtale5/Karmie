// Standard Juno document interface
// Temporarily comment out ULID import until we implement serialization
// import type { ULID } from './keys/ulid_types';

// Import the Doc type from the Juno SDK
import type { Doc } from '@junobuild/core';

/**
 * User data interface
 *
 * - user_handle: Unique username/handle (required)
 * - display_name: Display name (required)
 * - user_key: ULID for this user (required, string)
 * - avatar_url: Avatar URL (required, can be empty string)
 */
export interface UserData {
    user_handle: string;    // Unique username/handle (required)
    display_name: string;  // Display name (required)
    user_key: string;      // ULID for this user (required, string)
    avatar_url: string;    // Avatar URL (required, can be empty string)
}

/**
 * Tag data interface
 *
 * - user_key: ULID of the creator (required, string)
 * - tag_key: ULID for this tag (required, string)
 * - tag_handle: Tag handle (required)
 * - description: Description of the tag's purpose
 * - time_periods: Array of time period objects
 * - reputation_threshold: Minimum reputation needed for voting power
 * - vote_reward: Reputation points given for casting votes
 * - min_users_for_threshold: Minimum users needed before vote rewards are restricted
 */
export interface TagData {
    user_key?: string;         // ULID key of the creator (references Users collection)
    tag_key?: string;          // ULID for this tag (required)
    tag_handle?: string;     // Tag handle (required)
    description?: string;    // Description of the tag's purpose 
    time_periods: Array<{   // Array of time period objects
        months: number;     // Duration in months (1-999)
        multiplier: number; // Weight multiplier (0.25-1.5)
    }>;
    reputation_threshold?: number;    // Minimum reputation needed for voting power
    vote_reward?: number;             // Reputation points given for casting votes
    min_users_for_threshold?: number; // Minimum users needed before vote rewards are restricted
}

/**
 * Vote Document
 * 
 * Represents a vote from one user to another in a specific tag context.
 * 
 * Description format:
 * owner={author_key};tag={tag_key};target={target_key};
 * 
 * Example: "owner=user_123;tag=tag_789;target=user_456;"
 */
export interface VoteData {
    user_key: string;       // User key who cast the vote (references Users collection)
    target_key: string;     // User key being voted on (references Users collection)
    tag_key: string;        // Tag key this vote is for (references Tags collection)
    value: number;        // Vote value (+1 for upvote, -1 for downvote)
    weight: number;       // Vote weight (default: 1.0)
    created_at?: bigint;  // Creation timestamp in nanoseconds
}

/**
 * Reputation data interface
 * 
 * Description field format:
 * owner={user_key};tag={tag_key};
 * 
 * Example: "owner=user_123;tag=tag_789;"
 * 
 * This allows querying:
 * - By user: owner=user_123;
 * - By tag: tag=tag_789;
 * - By exact combination: owner=user_123;tag=tag_789;
 */
export interface ReputationData {
    user_key: string;                       // User this reputation is for (references Users collection)
    tag_key: string;                        // Tag this reputation is for (references Tags collection)
    reputation_basis: number;               // Reputation from received votes
    reputation_rewards: number;             // Reputation from casting votes
    reputation_total_effective: number;     // Final reputation score (cached value)
    last_calculation: bigint;               // When the reputation was last calculated
    vote_weight: number;                    // User's vote weight (0.0 to 1.0)
    has_voting_power: boolean;              // Whether user has sufficient reputation
}

// --- Canonical document types using the SDK's Doc<T> ---

/** A user document as returned by Juno SDK */
export type UserDocument = Doc<UserData>;
/** A tag document as returned by Juno SDK */
export type TagDocument = Doc<TagData>;
/** A vote document as returned by Juno SDK */
export type VoteDocument = Doc<VoteData>;
/** A reputation document as returned by Juno SDK */
export type ReputationDocument = Doc<ReputationData>;

// Optionally, you can define a generic alias for all your documents:
export type AppDocument<T> = Doc<T>; 