# Database System Integrity Check

This document provides a comprehensive analysis of all database collections, comparing the Rust structs in `structs.rs` with the database schema documentation in `database.md`.

## Complete Collection Analysis

### 1. Data Structure Overview

The system contains four main collections:
- **Users**: User profiles and authentication information
- **Tags**: Categorization tags with reputation configuration
- **Votes**: User-to-user votes with category and value
- **Reputations**: Calculated reputation scores by user and tag

### 2. Collection-by-Collection Analysis

#### 2.1 Users Collection

| Field in structs.rs | Field in database.md | Status | Notes |
|---------------------|----------------------|--------|-------|
| `User.key` | ✓ `key` | Compatible | |
| `User.description` | ✓ `description` | Compatible | Format aligned |
| `User.owner` | ✓ `owner` | Compatible | |
| `User.created_at` | ✓ `created_at` | Compatible | Type difference: u64 vs bigint |
| `User.updated_at` | ✓ `updated_at` | Compatible | Type difference: u64 vs bigint |
| `User.version` | ✓ `version` | Compatible | Type difference: u64 vs bigint |
| `User.data` | ✓ `data` | Compatible | Contains `UserData` |
| `UserData.username` | ✓ `data.username` | Compatible | |
| `UserData.display_name` | ✓ `data.display_name` | Compatible | |

**Format Notes:**
- `description` format now aligned between structs.rs and database.md: `[owner:{principal}],[username:{name}]`

**Structural Notes:**
- In structs.rs, `UserData` is a separate struct, while in database.md it's nested in `data` field
- Removed `principal` field from database.md as it's not needed

#### 2.2 Tags Collection

| Field in structs.rs | Field in database.md | Status | Notes |
|---------------------|----------------------|--------|-------|
| `Tag.key` | ✓ `key` | Compatible | |
| `Tag.description` | ✓ `description` | Compatible | Format aligned |
| `Tag.owner` | ✓ `owner` | Compatible | |
| `Tag.created_at` | ✓ `created_at` | Compatible | Type difference: u64 vs bigint |
| `Tag.updated_at` | ✓ `updated_at` | Compatible | Type difference: u64 vs bigint |
| `Tag.version` | ✓ `version` | Compatible | Type difference: u64 vs bigint |
| `Tag.data` | ✓ `data` | Compatible | Contains `TagData` |
| `TagData.author_key` | ✓ `data.author_key` | Compatible | Added to database.md |
| `TagData.name` | ✓ `data.name` | Compatible | |
| `TagData.description` | ✓ `data.description` | Compatible | |
| `TagData.time_periods` | ✓ `data.time_periods` | Compatible | |
| `TagData.reputation_threshold` | ✓ `data.reputation_threshold` | Compatible | |
| `TagData.vote_reward` | ✓ `data.vote_reward` | Compatible | |
| `TagData.min_users_for_threshold` | ✓ `data.min_users_for_threshold` | Compatible | |

**Format Notes:**
- `description` format now aligned between structs.rs and database.md: `[name:{name}][owner:{key}]`

**Structural Notes:**
- In structs.rs, `TagData` is a separate struct, while in database.md it's nested in `data` field
- Removed `vote_weight` field from database.md as it's not needed in Tags collection
- Added `author_key` field to database.md

#### 2.3 Votes Collection

| Field in structs.rs | Field in database.md | Status | Notes |
|---------------------|----------------------|--------|-------|
| `Vote.key` | ✓ `key` | Compatible | |
| `Vote.description` | ✓ `description` | Compatible | Format aligned |
| `Vote.owner` | ✓ `owner` | Compatible | |
| `Vote.created_at` | ✓ `created_at` | Compatible | Type difference: u64 vs bigint |
| `Vote.updated_at` | ✓ `updated_at` | Compatible | Type difference: u64 vs bigint |
| `Vote.version` | ✓ `version` | Compatible | Type difference: u64 vs bigint |
| `Vote.data` | ✓ `data` | Compatible | Contains `VoteData` |
| `VoteData.author_key` | ✓ `data.author_key` | Compatible | |
| `VoteData.target_key` | ✓ `data.target_key` | Compatible | |
| `VoteData.tag_key` | ✓ `data.tag_key` | Compatible | |
| `VoteData.value` | ✓ `data.value` | Compatible | |
| `VoteData.weight` | ✓ `data.weight` | Compatible | |
| `VoteData.created_at` | ✓ `data.created_at` | Compatible | Type difference: u64 vs bigint |

**Format Notes:**
- `description` format now aligned between structs.rs and database.md: `[owner:{VoteData.author_key}],[target:{VoteData.target_key}][tag:{VoteData.tag_key}]`

**Structural Notes:**
- In structs.rs, `VoteData` is a separate struct, while in database.md it's nested in `data` field
- All fields between VoteData and database.md vote data match

#### 2.4 Reputations Collection

| Field in structs.rs | Field in database.md | Status | Notes |
|---------------------|----------------------|--------|-------|
| `Reputation.key` | ✓ `key` | Compatible | |
| `Reputation.description` | ✓ `description` | Compatible | Format aligned |
| `Reputation.owner` | ✓ `owner` | Compatible | |
| `Reputation.created_at` | ✓ `created_at` | Compatible | Type difference: u64 vs bigint |
| `Reputation.updated_at` | ✓ `updated_at` | Compatible | Type difference: u64 vs bigint |
| `Reputation.version` | ✓ `version` | Compatible | Type difference: u64 vs bigint |
| `Reputation.data` | ✓ `data` | Compatible | Contains `ReputationData` |
| `ReputationData.user_key` | ✓ `data.user_key` | Compatible | |
| `ReputationData.tag_key` | ✓ `data.tag_key` | Compatible | |
| `ReputationData.total_basis_reputation` | ✓ `data.total_basis_reputation` | Compatible | |
| `ReputationData.total_voting_rewards_reputation` | ✓ `data.total_voting_rewards_reputation` | Compatible | |
| `ReputationData.last_known_effective_reputation` | ✓ `data.last_known_effective_reputation` | Compatible | |
| `ReputationData.last_calculation` | ✓ `data.last_calculation` | Compatible | Fixed type in database.md to bigint |
| `ReputationData.vote_weight` | ✓ `data.vote_weight` | Compatible | Implementation difference documented |
| `ReputationData.has_voting_power` | ✓ `data.has_voting_power` | Compatible | |

**Format Notes:**
- `description` format now aligned between structs.rs and database.md: `[owner:{ReputationData:user_key}],[tag:{ReputationData:tag_key}]`

**Structural Notes:**
- In structs.rs, `ReputationData` is a separate struct, while in database.md it's nested in `data` field
- In structs.rs, `vote_weight` is a custom `VoteWeight` struct with validation
- In database.md, `vote_weight` is described as a number, with a note explaining the implementation difference

### 3. Important Type Differences

| Type in structs.rs | Type in database.md | Affected Fields |
|-------------------|---------------------|-----------------|
| `u64` | `bigint` | All timestamp fields (created_at, updated_at, last_calculation) |
| `VoteWeight(struct)` | `number` | ReputationData.vote_weight |
| `f64` | `number` | All numeric values (reputation_threshold, vote_reward, etc.) |

### 4. Data Structure Naming Approach

In Juno's data model, all document data is stored in a field called `data`. In our structs.rs implementation:

1. We have separate struct types:
   - `UserData`
   - `TagData`
   - `VoteData`
   - `ReputationData`

2. These structs are stored in fields also called `data` in their parent structs.

This approach is correct and provides good type safety while maintaining compatibility with Juno's data model.

### 5. Description Field Format

Each collection now has consistent format for the description field between structs.rs and database.md:

| Collection | Format |
|------------|-------------------|
| Users | `[owner:{principal}],[username:{name}]` |
| Tags | `[name:{name}][owner:{key}]` |
| Votes | `[owner:{VoteData.author_key}],[target:{VoteData.target_key}][tag:{VoteData.tag_key}]` |
| Reputations | `[owner:{ReputationData:user_key}],[tag:{ReputationData:tag_key}]` |

## Required Updates

### 1. Type Consistency

Ensure consistent handling of:
- Timestamps (u64 vs bigint)
- VoteWeight (struct vs number)
- Numeric values (f64 vs number)

### 2. Documentation Completeness

Update the system_integrity_check.md to:
- Document these changes
- Provide guidance for developers
- Explain serialization/deserialization of special types

## Backend Function Requirements

### 1. Batch User Reputation Calculation

Implement a function to calculate reputations for multiple users at once:

```rust
// Function signature
async fn calculate_user_reputations(user_keys: Vec<String>, tag_key: String) -> Result<(), String>;
```

This function should:
- Accept an array of user keys and a single tag key
- Process each user's reputation efficiently
- Return success/failure status
- Handle any VoteWeight conversions properly

### 2. Batch User Reputation Retrieval

Implement a function to retrieve reputation data for multiple users at once:

```rust
// Function signature
async fn get_user_reputations(user_keys: Vec<String>, tag_key: String) -> Result<HashMap<String, ReputationSummary>, String>;

// ReputationSummary struct
struct ReputationSummary {
    last_known_effective_reputation: f64,
    vote_weight: VoteWeight,
    has_voting_power: bool
}
```

This function should:
- Accept an array of user keys and a single tag key
- Return essential reputation data for all users
- Handle VoteWeight conversions correctly
- Provide default values for users without existing reputation data

## Timeline

1. Documentation Updates: Complete
2. Backend Function Implementation: 2-3 days
3. Frontend Component Updates: 1-2 days
4. Testing: 1-2 days

Total estimated time: 4-7 days 