// use junobuild_satellite::list_docs;
// use junobuild_shared::types::list::{ListMatcher, ListParams};
// use junobuild_utils::encode_doc_data;
use serde::{Deserialize, Serialize};
use candid::{Principal, CandidType};

/// Represents a user in the system with their profile information
/// See format standards in docs/core/architecture/database.md
#[derive(Debug, Serialize, Deserialize)]
pub struct User {

    /// Unique identifier for the user document
    /// Generated using nanoid() by Juno when the document is created
    /// This is NOT the Principal ID - it's just a unique document identifier
    pub key: String,

    /// Description field for filtering/search 
    /// Format: 
    /// Playground: owner={key(nanoid)};username={name};
    /// Production: owner={owner(principal)};username={name};
    /// Note: owner field uses Principal ID, not document key
    /// See: docs/core/architecture/database.md#users-collection
    pub description: String,

    /// Principal ID of the document owner
    /// This is automatically set by Juno to the Principal of the creating user
    /// Used for access control when security is set to "owner only"
    pub owner: Principal,

    /// Creation timestamp in nanoseconds
    pub created_at: u64,

    /// Last update timestamp in nanoseconds
    pub updated_at: u64,

    /// Document version for concurrency control
    pub version: u64,

    /// User-specific profile data
    pub data: UserData,
}

/// Contains the core user profile information
#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    
    /// Unique username (must be unique across all users)
    pub username: String,

    /// Display name (not required to be unique)
    pub display_name: String,
    
    /// ULID for this user, stored separately from the formatted key
    /// This is the raw ULID without username and prefixes
    pub usr_key: Option<String>,
}

/// Represents a tag that can be used for categorizing votes and reputation
/// See format standards in docs/core/architecture/database.md
#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {

    /// Unique identifier for the tag document
    /// Generated using nanoid() by Juno when the document is created
    /// This is NOT the Principal ID - it's just a unique document identifier
    pub key: String,

    /// Description field for filtering/search
    /// Format:
    /// Playground: owner={TagData.author_key(nanoid)};name={TagData.name};
    /// Production: owner={owner(principal)};name={TagData.name};
    /// Note: owner field uses Principal ID, not document key
    /// See: docs/core/architecture/database.md#tags-collection
    pub description: String,

    /// Principal ID of the document owner
    /// This is automatically set by Juno to the Principal of the creating user
    pub owner: Principal,

    /// Creation timestamp in nanoseconds
    pub created_at: u64,

    /// Last update timestamp in nanoseconds
    pub updated_at: u64,

    /// Document version for concurrency control
    pub version: u64,

    /// Tag-specific configuration data
    pub data: TagData,
}

/// Contains the configuration and settings for a tag
#[derive(Debug, Serialize, Deserialize)]
pub struct TagData {

    /// User key who created the tag
    pub author_key: String,

    /// Display name of the tag
    pub name: String,

    /// Description of the tag's purpose
    pub description: String,

    /// Time periods for vote decay multipliers
    pub time_periods: Vec<TimePeriod>,

    /// Minimum reputation needed for voting power (whole number)
    pub reputation_threshold: f64,

    /// Reputation points given for casting a vote (e.g., 0.1)
    pub vote_reward: f64,

    /// Minimum number of users that need to reach threshold before vote rewards are restricted
    pub min_users_for_threshold: u32,

}

/// Represents a vote cast by one user on another
/// See format standards in docs/core/architecture/database.md
#[derive(Debug, Serialize, Deserialize)]
pub struct Vote {

    /// Unique identifier for the vote document
    /// Generated using nanoid() by Juno when the document is created
    /// This is NOT the Principal ID - it's just a unique document identifier
    pub key: String,

    /// Description field for filtering/search
    /// Format:
    /// Playground: owner={VoteData.author_key};target={VoteData.target_key};tag={VoteData.tag_key};
    /// Production: owner={owner.principal};target={VoteData.target_key};tag={VoteData.tag_key};
    /// See: docs/core/architecture/database.md#votes-collection
    pub description: String,

    /// Principal ID of the document owner
    pub owner: Principal,

    /// Creation timestamp in nanoseconds
    pub created_at: u64,

    /// Last update timestamp in nanoseconds
    pub updated_at: u64,

    /// Document version for concurrency control
    pub version: u64,

    /// Vote-specific data
    pub data: VoteData,
}

/// Contains the core vote information
#[derive(Debug, Serialize, Deserialize)]
pub struct VoteData {

    /// User key who cast the vote (nanoid-generated document key)
    pub author_key: String,

    /// User key being voted on (nanoid-generated document key)
    pub target_key: String,

    /// Tag key this vote is for (nanoid-generated document key)
    pub tag_key: String,

    /// Vote value (+1 for upvote, -1 for downvote)
    pub value: f64,

    /// Vote weight (default: 1.0)
    pub weight: f64,
}

/// Represents a user's reputation in a specific tag
/// See format standards in docs/core/architecture/database.md
#[derive(Debug, Serialize, Deserialize)]
pub struct Reputation {

    /// Unique identifier for the reputation document
    /// Generated using nanoid() by Juno when the document is created
    /// This is NOT the Principal ID - it's just a unique document identifier
    pub key: String,

    /// Description field for filtering/search
    /// Format: 
    /// Playground: owner={ReputationData.user_key};tag={ReputationData.tag_key};
    /// Production: owner={owner};tag={ReputationData.tag_key};
    /// See: docs/core/architecture/database.md#reputations-collection
    pub description: String,

    /// Principal ID of the document owner
    pub owner: Principal,

    /// Creation timestamp in nanoseconds
    pub created_at: u64,

    /// Last update timestamp in nanoseconds
    pub updated_at: u64,

    /// Document version for concurrency control
    pub version: u64,

    /// Reputation-specific data
    pub data: ReputationData,
}

/// Contains the reputation calculation results for a user in a tag
#[derive(Debug, Serialize, Deserialize, Clone, CandidType)]
pub struct ReputationData {

    /// The user this reputation is for
    pub user_key: String,

    /// The tag this reputation is for
    pub tag_key: String,

    /// Reputation from received votes
    pub total_basis_reputation: f64,

    /// Reputation from casting votes
    pub total_voting_rewards_reputation: f64,

    /// Final reputation score (cached value)
    pub last_known_effective_reputation: f64,

    /// When we last calculated
    pub last_calculation: u64,

    /// The user's vote weight (0.0 to 1.0, where 1.0 = 100%)
    pub vote_weight: VoteWeight,

    /// Whether the user has sufficient reputation to have voting power (above threshold)
    pub has_voting_power: bool,
}

/// Represents a vote weight with constraints (0.0 to 1.0)
#[derive(Debug, Clone, CandidType)]
pub struct VoteWeight(f64);

impl VoteWeight {
    /// Creates a new VoteWeight, ensuring it's between 0.0 and 1.0
    pub fn new(value: f64) -> Result<Self, String> {
        if value < 0.0 || value > 1.0 {
            return Err("Vote weight must be between 0.0 and 1.0".to_string());
        }
        Ok(VoteWeight(value))
    }

    /// Returns the underlying f64 value
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Serialize for VoteWeight {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_f64(self.0)
    }
}

impl<'de> Deserialize<'de> for VoteWeight {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = f64::deserialize(deserializer)?;
        VoteWeight::new(value).map_err(serde::de::Error::custom)
    }
}



/// Contains information about a vote author used in reputation calculations
#[derive(Debug, Clone)]
pub struct AuthorInfo {

    /// The author's current effective reputation in the tag
    pub effective_reputation: f64,

    /// The author's vote weight in this tag
    pub vote_weight: VoteWeight,

    /// Whether the author's votes are currently active
    pub votes_active: bool,
}

/// Represents a time period for vote decay multipliers
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimePeriod {

    /// Duration in months (1-999)
    pub months: u32,

    /// Weight multiplier (0.25-1.5)
    pub multiplier: f64,
}