// use junobuild_satellite::list_docs;
// use junobuild_shared::types::list::{ListMatcher, ListParams};
// use junobuild_utils::encode_doc_data;
use serde::{Deserialize, Serialize};
use candid::{Principal, CandidType};

/// Represents a user in the system with their profile information
/// See format standards in docs/core/architecture/database.md
#[derive(Debug, Serialize, Deserialize)]
pub struct User {

    /// Unique identifier for the user document created using src/satellite/src/processors/document_keys.rs
    /// Users: `usr_{ulid}_usrName_{username}_`
    /// This is NOT the Principal ID - it's just a unique document identifier
    pub key: String,

    /// Description field, currently unused for user documents
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
    
    /// ULID for this user, stored separately from the formatted key
    /// This is the raw ULID without username and prefixes
    /// Optional for backward compatibility during transition
    pub usr_key: Option<String>,

    /// Unique username (must be unique across all users)
    pub username: String,

    /// Display name (not required to be unique)
    pub display_name: String,
    
}

/// Represents a tag that can be used for categorizing votes and reputation
/// See format standards in docs/core/architecture/database.md
#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {

    /// Unique identifier key for the tag document
    /// Generated using src/satellite/src/processors/document_keys.rs
    /// Tags: `usr_{ulid}_tag_{ulid}_tagName_{tagName}_`
    /// This is NOT the Principal ID - it's just a unique document identifier
    pub key: String,

    /// Description of the tag.
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

    /// ULID for the user who created this tag
    /// This is the raw ULID without prefixes, stored as uppercase
    pub usr_key: String,

    /// ULID for this tag
    /// This is the raw ULID without prefixes, stored as uppercase
    pub tag_key: String,

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
    /// Generated using src/satellite/src/processors/document_keys.rs
    /// Votes: `usr_{ulid}_tag_{ulid}_tar_{ulid}_key_{ulid}_`
    /// This is NOT the Principal ID - it's just a unique document identifier
    pub key: String,

    /// Description field, currently unused for vote documents
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

    /// ULID for the user who cast this vote
    /// This is the raw ULID without prefixes, stored as uppercase
    pub usr_key: String,

    /// ULID for the target user receiving the vote
    /// This is the raw ULID without prefixes, stored as uppercase
    pub tar_key: String,

    /// ULID for the tag this vote is for
    /// This is the raw ULID without prefixes, stored as uppercase
    pub tag_key: String,
    
    /// ULID for this specific vote
    /// This is the raw ULID without prefixes, stored as uppercase
    pub vote_key: String,

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
    /// Generated using src/satellite/src/processors/document_keys.rs
    /// Reputations: `usr_{ulid}_tag_{ulid}`
    /// This is NOT the Principal ID - it's just a unique document identifier
    pub key: String,

    /// Description field, currently unused for reputation documents
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

    /// ULID for the user this reputation is for
    /// This is the raw ULID without prefixes, stored as uppercase
    pub usr_key: String,

    /// ULID for the tag this reputation is for
    /// This is the raw ULID without prefixes, stored as uppercase
    pub tag_key: String,

    /// Reputation from received votes
    pub reputation_basis: f64,

    /// Reputation from casting votes
    pub reputation_rewards: f64,

    /// Final reputation score (cached value)
    pub reputation_total_effective: f64,

    /// When we last calculated
    pub last_calculation: u64,

    /// The user's vote weight (0.0 to 1.0, where 1.0 = 100%)
    /// Access the value using vote_weight.value() method
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