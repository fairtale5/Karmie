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
 * - description: User profile description (optional)
 */
export interface UserData {
    user_handle: string;    // Unique username/handle (required)
    display_name: string;  // Display name (required)
    user_ulid: string;      // ULID for this user (required, string)
    avatar_url: string;    // Avatar URL (required, can be empty string)
    description?: string;   // User profile description (optional)
}

/**
 * Tag data interface
 *
 * - owner_ulid: ULID of the creator (references Users collection) (required, string)
 * - tag_ulid: ULID for this tag (required, string)
 * - tag_handle: Tag handle (required)
 * - description: Description of the tag's purpose
 * - time_periods: Array of decay rules that define how votes gain bonuses or decay based on the age of the vote.
 *   Each period specifies:
 *   - months: How long until this decay rule takes effect (1-999 months)
 *   - multiplier: How much the reputation decays (0.05-100)
 *     - multiplier < 1: Reputation decays by (1 - multiplier)%
 *     - multiplier = 1: No decay
 *     - multiplier > 1: Reputation increases by (multiplier - 1)%
 * - reputation_threshold: Minimum reputation needed for voting power
 * - vote_reward: Reputation points given for casting votes
 * - min_users_for_threshold: Minimum users needed before vote rewards are restricted
 */
export interface TagData {
    owner_ulid?: string;         // ULID key of the creator (references Users collection)
    tag_ulid?: string;          // ULID for this tag (required)
    tag_handle?: string;     // Tag handle (required)
    description?: string;    // Description of the tag's purpose 
    time_periods: Array<{   // Array of decay rules that define how reputation scores decrease over time
        months: number;     // How long until this decay rule takes effect (1-999 months)
        multiplier: number; // How much the reputation decays (0.05-100)
                          // < 1: Decay by (1 - multiplier)%
                          // = 1: No decay
                          // > 1: Increase by (multiplier - 1)%
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
    owner_ulid?: string;      // User key who cast the vote (references Users collection)
    target_ulid?: string;    // User key being voted on (references Users collection)
    tag_ulid?: string;       // Tag key this vote is for (references Tags collection)
    vote_ulid?: string;      // ULID for this specific vote (generated internally)
    value?: number;         // Vote value (+1 for upvote, -1 for downvote)
    weight?: number;        // Vote weight (default: 1.0)
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
    owner_ulid: string;                       // User this reputation is for (references Users collection)
    tag_ulid: string;                        // Tag this reputation is for (references Tags collection)
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