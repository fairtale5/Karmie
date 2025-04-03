// Standard Juno document interface
export interface JunoDocument<T> {
    key: string;           // Unique identifier generated with nanoid()
    description: string;   // Description for searching/filtering
    owner: string;         // Principal ID of document owner
    created_at: bigint;    // Creation timestamp in nanoseconds
    updated_at: bigint;    // Last update timestamp in nanoseconds
    version: bigint;       // Document version for concurrency control
    data: T;               // The actual document data - varies by collection
}

/**
 * User data interface
 * 
 * Description field format:
 * owner={key};username={name};
 * 
 * Example: "owner=user_123;username=john_doe;"
 * 
 * This allows querying:
 * - By user key: owner=user_123;
 * - By username: username=john_doe;
 */
export interface UserData {
    username: string;         // Unique username
    display_name: string;     // Display name (not required to be unique)
}

/**
 * Tag data interface
 * 
 * Description field format:
 * owner={author_key};name={name};
 * 
 * Example: "owner=user_123;name=technical_skills;"
 * 
 * This allows querying:
 * - By author: owner=user_123;
 * - By tag name: name=technical_skills;
 */
export interface TagData {
    author_key: string;     // User key of the creator (references Users collection)
    name: string;           // Display name of the tag
    description: string;    // Description of the tag's purpose
    time_periods: Array<{
        months: number;     // Duration in months (1-999)
        multiplier: number; // Weight multiplier (0.25-1.5)
    }>;
    reputation_threshold: number;    // Minimum reputation needed for voting power
    vote_reward: number;             // Reputation points given for casting votes
    min_users_for_threshold: number; // Minimum users needed before vote rewards are restricted
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
    author_key: string;    // User key who cast the vote (references Users collection)
    target_key: string;    // User key being voted on (references Users collection)
    tag_key: string;       // Tag key this vote is for (references Tags collection)
    value: number;         // Vote value (+1 for upvote, -1 for downvote)
    weight: number;        // Vote weight (default: 1.0)
    created_at: bigint;    // Creation timestamp in nanoseconds
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
    user_key: string;      // User this reputation is for (references Users collection)
    tag_key: string;       // Tag this reputation is for (references Tags collection)
    total_basis_reputation: number;          // Reputation from received votes
    total_voting_rewards_reputation: number; // Reputation from casting votes
    last_known_effective_reputation: number; // Final reputation score (cached value)
    last_calculation: bigint;                // When the reputation was last calculated
    vote_weight: number;                     // User's vote weight (0.0 to 1.0)
    has_voting_power: boolean;               // Whether user has sufficient reputation
}

// Helper type definitions for each collection
export type UserDocument = JunoDocument<UserData>;
export type TagDocument = JunoDocument<TagData>;
export type VoteDocument = JunoDocument<VoteData>;
export type ReputationDocument = JunoDocument<ReputationData>; 