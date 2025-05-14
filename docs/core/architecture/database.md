# Database Schema

This document defines the database schema for the Reputator project.

## Document Keys and Principals

### Document Keys
- All document keys are generated using ULID when documents are created
- Keys are unique identifiers within a collection, but have no special meaning
- Keys are used only for document lookup and referencing
- Example: `"user_123"`, `"tag_456"`, `"vote_789"`

### Principal IDs
- Principal IDs are automatically set by Juno to identify document owners
- They come from the user's Internet Identity authentication
- Used for access control when collection security is set to "owner only"
- Not used for document references (use document keys instead)
- Example: `"2vxsx-fae"` (actual principals are much longer)

## Collections

### Important Note for Test Phase
During the initial test phase, all documents will be created by the same user (single-user testing environment). The document's author will be stored in the `description` field along with other metadata. This approach will change in production to use proper multi-user authentication.

### Users Collection

Collection name: `users`

#### Permissions
- Read: public
- Write: private
- Memory: stable
- Mutable Permissions: true

#### Document Structure
```typescript
interface UserDocument {
    // Standard Juno fields (automatically managed)
    key: string;                // Format: _prn_{principal}_usr_{ulid}_hdl_{handle}_ generated with src/satellite/src/processors/document_keys.rs
    description: string;        // currently not used
    owner: Principal;           // Automatically set to user's Internet Identity Principal
    created_at: bigint;         // Creation timestamp in nanoseconds
    updated_at: bigint;         // Last update timestamp in nanoseconds
    version: bigint;            // Document version for concurrency control
    data: {                     // User-specific data
        user_key: ULID;         // Pure ULID for references
        username: string;       // Unique username (must be unique across all users)
        display_name: string;   // Display name (not required to be unique)
    }
}
```

#### Validation Rules
1. **Username Validation**
   - Length: 3-30 characters
   - Allowed characters: alphanumeric, hyphen
   - Must be unique across all users (case-insensitive)
   - Stored in lowercase format in the key field
   - Stored in original case in the data.username field

2. **Display Name Validation**
   - Non-empty after trimming
   - Maximum length: 100 characters
   - No character restrictions

3. **Document Key Format**
   - Format: `usr_{ulid}_hdl_{usernameHandle}_`
   - ULID: 26 characters, Crockford Base32
   - Handle: Lowercase, sanitized version (username)

4. **Description Format** // no longer used.

5. **Production Mode Rules**
   - One account per Internet Identity
   - Owner field uses Principal ID instead of document key
   - Stricter validation rules apply

#### Notes
- `username` must be unique across all users
- `display_name` is not required to be unique
- Username uniqueness is enforced by backend validation
- Frontend should handle validation errors gracefully
- All timestamps are in nanoseconds
- Version is required for updates to prevent concurrent modifications
- ULID provides chronological sorting capability

Example User Document:
```typescript
{
    key: "usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_hdl_johndoe_",
    description: "",
    owner: Principal.fromText("..."),
    created_at: 1234567890n,
    updated_at: 1234567890n,
    version: 1n,
    data: {
        user_key: "01ARZ3NDEKTSV4RRFFQ69G5FAV", // Pure ULID for user reference
        username: "johndoe", // Lowercase, sanitized username
        display_name: "John Doe" // Display name in original case
    }
}
```

### Tags Collection

Collection name: `tags`

#### Permissions
- Read: public
- Write: managed
- Memory: stable
- Mutable Permissions: true

#### Document Structure
```typescript
interface TagDocument {
    // Standard Juno fields (automatically managed)
    key: string;                // Format: usr_{userUlid}_tag_{tagUlid}_hdl_{tagHandle}_ generated with formatTagKey() in src/satellite/src/processors/document_keys.rs
    description: string;        // currently not used
    owner: Principal;           // Automatically set to document creator's Principal
    created_at: bigint;         // Creation timestamp in nanoseconds
    updated_at: bigint;         // Last update timestamp in nanoseconds
    version: bigint;            // Document version for concurrency control
    data: {                     // Tag-specific data
        name: string;           // Display name of the tag (original case preserved)
        description: string;    // Longer description of the tag's purpose
        user_key: ULID;         // Pure ULID of the user who created the tag
        tag_key: ULID;          // Pure ULID of this tag
        time_periods: Array<{   // Time periods for vote decay multipliers
            months: number;     // Duration in months (1-999)
            multiplier: number; // Weight multiplier (0.05-1.5)
        }>;
        reputation_threshold: number;       // Minimum reputation needed for voting power
        vote_reward: number;                // Reputation points given for casting votes
        min_users_for_threshold: number;    // Minimum users needed before vote rewards are restricted
    }
}
```

Example Tag Document:
```typescript
{
    key: "usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW_hdl_technicalskills_",
    description: "",
    owner: Principal.fromText("..."),
    created_at: 1234567890n,
    updated_at: 1234567890n,
    version: 1n,
    data: {
        name: "Technical-Skills",               // Display name of the tag (original case preserved, no spaces or special characters, just alphanumeric and dashes)	
        description: "Technical expertise and knowledge",   // Description of the tag's purpose, created by the author user.
        user_key: "01ARZ3NDEKTSV4RRFFQ69G5FAV",  // Pure ULID of the user who created the tag
        tag_key: "01ARZ3NDEKTSV4RRFFQ69G5FAW",  // Pure ULID of this tag
        time_periods: [
            { months: 1, multiplier: 1.5 },     // Period 1: First month
            { months: 2, multiplier: 1.2 },     // Period 2: Months 2-3
            { months: 3, multiplier: 1.1 },     // Period 3: Months 4-6
            { months: 6, multiplier: 1.0 },     // Period 4: Months 7-12
            { months: 12, multiplier: 0.95 },   // Period 5: Months 13-24
            { months: 12, multiplier: 0.75 },   // Period 6: Months 25-36
            { months: 12, multiplier: 0.55 },   // Period 7: Months 37-48
            { months: 999, multiplier: 0.25 }   // Period 8: Months 49+ (treated as infinity)
        ],
        reputation_threshold: 10,               // Users need 10 reputation to get voting power
        vote_reward: 0.1,                       // Users get 0.1 reputation for each vote they cast
        min_users_for_threshold: 5              // Need 5 users to reach threshold before restricting rewards
    }
}
```
#### Validation Rules
1. **Name Validation**
   - Length: 3-30 characters
   - Allowed characters: alphanumeric, hyphen, no spaces, underscores, or special characters
   - Must be unique across all tags (case-insensitive)
   - Stored in lowercase format in the key field
   - Stored in original case in the data.name field

2. **Document Key Format**
   - Format: `usr_{userUlid}_tag_{tagUlid}_hdl_{handle}_`
   - First ULID: Creator's user identifier (must be uppercase)
   - Second ULID: Tag's unique identifier (must be uppercase)
   - Handle: Lowercase, sanitized version of tag name for easy querying (tagName)

3. **Production Mode Rules**
   - Stricter validation rules apply
   - Owner field uses Principal ID
   - Tag names must be unique system-wide

#### Notes
- `name` must be unique across all tags
- Tag name uniqueness is enforced by backend validation
- Frontend should handle validation errors gracefully
- All timestamps are in nanoseconds
- Version is required for updates to prevent concurrent modifications
- ULID provides chronological sorting capability
- The tag name is included in the key to facilitate uniqueness checks and queries

### Votes Collection

Collection name: `votes`

#### Permissions
- Read: public
- Write: private
- Memory: stable
- Mutable Permissions: false

#### Document Structure
```typescript
interface VoteDocument {
    // Standard Juno fields (automatically managed)
    key: string;                // Format: usr_{ulid}_tag_{ulid}_tar_{ulid}_key_{ulid}_ generated with src/satellite/src/processors/document_keys.rs
    description: string;        // currently not used
    owner: Principal;           // Automatically set to document creator's Principal
    created_at: bigint;         // Creation timestamp in nanoseconds
    updated_at: bigint;         // Last update timestamp in nanoseconds
    version: bigint;            // Document version for concurrency control
    data: {                     // Vote-specific data
        user_key: ULID;         // Pure ULID of the user casting the vote
        tag_key: ULID;          // Pure ULID of the tag being voted on
        target_key: ULID;       // Pure ULID of the target user receiving the vote
        vote_key: ULID;         // Pure ULID for this specific vote
        value: number;          // Vote value (+1 for upvote, -1 for downvote, some tags may allow other values)
        weight: number;         // Vote weight (default: 1.0)
    }
}
```

#### Validation Rules

1. **Vote Value Validation**
   - Standard tags: Only +1 (upvote) or -1 (downvote) allowed
   - Special tags: May allow additional values based on tag configuration
   - No fractional values allowed
   - Value must be an integer

2. **Document Key Format**
   - Format: `usr_{ulid}_tag_{ulid}_tar_{ulid}_key_{ulid}_`
   - First ULID: Voter's identifier (must be uppercase)
   - Second ULID: Tag identifier (must be uppercase)
   - Third ULID: Target user identifier (must be uppercase)
   - Fourth ULID: Vote identifier (must be uppercase) followed by an underscore to signal the end of the key.
   - All parts must be present and properly formatted

3. **Production Mode Rules**
   - Cannot vote on self (usr_key cannot equal tar_key)

### Notes
- All timestamps are in nanoseconds
- ULID provides chronological sorting capability
- Vote impact is determined by voter's reputation at calculation time
- Vote weight is stored in voter's reputation document, not in vote document
- Some tags may have special validation rules for vote values

Example Vote Document:
```typescript
{
    key: "usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW_tar_01ARZ3NDEKTSV4RRFFQ69G5FAX_key_01ARZ3NDEKTSV4RRFFQ69G5FAY_",
    description: "",
    owner: Principal.fromText("..."),
    created_at: 1234567890n,
    updated_at: 1234567890n,
    version: 1n,
    data: {
        user_key: "01ARZ3NDEKTSV4RRFFQ69G5FAV",      // Pure ULID of the user casting the vote
        tag_key: "01ARZ3NDEKTSV4RRFFQ69G5FAW",      // Pure ULID of the tag being voted on
        target_key: "01ARZ3NDEKTSV4RRFFQ69G5FAX",      // Pure ULID of the target user receiving the vote
        vote_key: "01ARZ3NDEKTSV4RRFFQ69G5FAY",     // Pure ULID for this specific vote
        value: 1,                                   // Vote value (usually +1 for upvote, -1 for downvote)
        weight: 1.0                                 // Vote weight (default: 1.0)
    }
}
```

### Reputations Collection

Collection name: `reputations`

#### Permissions
- Read: public
- Write: controllers
- Memory: stable
- Mutable Permissions: false

#### Document Structure

```typescript
interface ReputationDocument {
    // Standard Juno fields (automatically managed)
    key: string;                // Format: usr_{ulid}_tag_{ulid}_ generated with src/satellite/src/processors/document_keys.rs
    description: string;        // currently not used
    owner: Principal;           // Set to canister Principal (ic_cdk::id())
    created_at: bigint;         // Creation timestamp in nanoseconds
    updated_at: bigint;         // Last update timestamp in nanoseconds
    version: bigint;            // Document version for concurrency control
    data: {                     // Reputation-specific data (see below)
        user_key: string;                     // ULID of the user this reputation is for
        tag_key: string;                     // ULID of the tag this reputation is for
        basis_reputation: number;            // Reputation from received votes
        voting_rewards_reputation: number;   // Reputation from casting votes
        effective_reputation: number;        // Final/cached reputation score (used as the user's reputation in this tag)
        last_calculation: bigint;            // Timestamp of last reputation calculation (nanoseconds)
        vote_weight: number;                 // User's voting weight in this tag (0.0 to 1.0)
        has_voting_power: boolean;           // Whether the user has sufficient reputation to have voting power
        vote_weight_value: number;           // User's voting weight as a float (redundant with vote_weight, for compatibility)
    }
}
```

#### Field Descriptions

- **user_key**: ULID of the user this reputation is for (uppercase, no prefix)
- **tag_key**: ULID of the tag this reputation is for (uppercase, no prefix)
- **basis_reputation**: Reputation points earned from received votes
- **voting_rewards_reputation**: Reputation points earned from casting votes (vote rewards)
- **effective_reputation**: The final, cached reputation score for this user in this tag (used for all calculations and display)
- **last_calculation**: Timestamp (nanoseconds) of the last time this reputation was recalculated
- **vote_weight**: User's voting weight in this tag (float, 0.0 to 1.0)
- **has_voting_power**: Boolean indicating if the user meets the threshold for voting power in this tag
- **vote_weight_value**: User's voting weight as a float (may be redundant with `vote_weight`)

#### Notes

- Only the fields above are present in the actual code and persisted documents.
- Fields such as `activity_score`, `received_votes`, `cast_votes`, `last_vote_at`, and `last_received_at` are **not** present in the current implementation and should not be relied upon.
- The `effective_reputation` field in code serves the same purpose as the `reputation` field described in earlier documentation.
- All timestamps are in nanoseconds.
- ULIDs are always uppercase and without prefixes in the data fields.

#### Example Reputation Document

```typescript
{
    key: "usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW_",
    description: "",
    owner: Principal.fromText("..."),  // Canister Principal
    created_at: 1234567890n,
    updated_at: 1234567890n,
    version: 1n,
    data: {
        user_key: "01ARZ3NDEKTSV4RRFFQ69G5FAV",
        tag_key: "01ARZ3NDEKTSV4RRFFQ69G5FAW",
        basis_reputation: 10.0,
        voting_rewards_reputation: 2.5,
        effective_reputation: 12.5,
        last_calculation: 1234567890n,
        vote_weight: 0.85,
        has_voting_power: true,
        vote_weight_value: 0.85
    }
}
```

#### Validation Rules

1. **Document Key Format**
   - Format: `usr_{ulid}_tag_{ulid}_`
   - First ULID: User identifier (must be uppercase)
   - Second ULID: Tag identifier (must be uppercase)
   - All parts must be present and properly formatted

2. **Reputation Score Rules**
   - Must be a non-negative number
   - Calculated based on received votes and their weights
   - Updated whenever user receives a vote in the tag

3. **Vote Weight Rules**
   - Must be a non-negative number
   - Calculated based on:
     - User's reputation in the tag
     - Activity score in the tag
     - Time since last vote
   - Updated before each vote is cast
   - Stored to ensure consistent vote impact

4. **Activity Score Rules**
   - Must be a non-negative number
   - Increases with voting activity
   - Decays over time without activity
   - Affects vote weight calculation

5. **Production Mode Rules**
   - One reputation document per user-tag combination
   - Cannot be manually created or deleted
   - Updates only through system functions
   - All fields must be present and valid

#### Notes
- All timestamps are in nanoseconds
- ULID provides chronological sorting capability
- Vote weight is recalculated before each vote
- Activity score helps prevent reputation farming
- System maintains consistency between votes and reputation
- Uses exponential backoff for creation retries:
  - Max attempts: 3
  - Initial delay: 100ms
  - Backoff: 100ms -> 200ms -> 400ms

## Description Field Queries

The description field uses a consistent bracket format that enables powerful querying capabilities. Here are some common query patterns:



## Important Notes

1. **Document Size Limits**
   - Maximum document size: 2MB
   - Maximum description length: 1024 characters
   - Maximum batch operation size: 100 documents

2. **Key Generation**
   - All document keys are generated using ULID
   - Keys are unique within a collection and provide chronological sorting capability
   - Keys are used for document references
   - Do not use Principal IDs as keys

3. **Principal IDs**
   - Set automatically by Juno on document creation
   - Come from Internet Identity authentication
   - Used for access control
   - Not used for document references

4. **Owner Field Management**
   - Use `ic_cdk::id()` (canister's Principal ID) ONLY for:
     - Writing to the "reputations" collection (controller-only access)
     - Any other collections marked as "controller" access
     - When backend needs to write documents users can't access
   - Use `ic_cdk::caller()` (user's Principal ID) for:
     - All other collections where users should own their documents
     - When users need to manage their own data
     - To maintain proper ownership attribution
   - This ensures:
     - Proper access control for restricted collections
     - Correct document ownership tracking
     - Users can manage their own documents
     - Clear audit trail of who created/modified documents

5. **Timestamps**
   - All timestamps are in nanoseconds
   - Use `Date.now() * 1_000_000` to convert from JavaScript

6. **Version Control**
   - Required for updates to prevent concurrent modifications
   - Must match the current document version
   - Automatically incremented after successful updates
   - Only need to provide version when updating documents

7. **Automatically Managed Fields**
   - `owner`: Set to document creator's Principal
   - `created_at`: Set on document creation
   - `updated_at`: Updated on document changes
   - `version`: Managed for concurrency control
   - Only need to provide `version` when updating documents

8. **Test Phase Considerations**
   - All documents created by same user during testing
   - Author stored in description field
   - Will change to proper multi-user system later 

## Query Performance and Memory Management

### Memory Model
Juno's memory is not a traditional database - it's a growable memory space indexed by collection and keys. Think of it like a cake or bucket where:
- Data is organized first by collection
- Then indexed by keys within each collection
- Memory can be either heap (1GB max) or stable (400GB max)

### Query Methods and Performance

#### 1. Key-Based Queries
**Most Efficient - No Full Memory Load**
- `get_doc_store(collection, exact_key)`: Most efficient for single document retrieval
- `list_docs_store(collection, key_pattern)`: Efficient for partial key matching
- `query_doc(collection, segment, query)`: Type-safe key segment queries (recommended)
- These operate at the storage layer using the key index
- Example:
  ```rust
  // Efficient: Uses key index
  get_doc_store(
      caller,
      "users",
      "usr_123_hdl_johndoe_"
  )
  
  // Also efficient: Uses key pattern matching
  list_docs_store(
      caller,
      "users",
      ListParams {
          matcher: Some(ListMatcher {
              key: Some("usr_123_"), // Partial key match
              ..Default::default()
          })
      }
  )
  ```
  
The `query_doc` function is the recommended way to query documents:
```rust
// Find user by handle
query_doc("users", KeySegment::Handle, "johndoe")?;
// Searches for pattern: "hdl_johndoe_"

// Find votes by target user
query_doc("votes", KeySegment::Target, "01ARZ3NDEKTSV4RRFFQ69G5FAV")?;
// Searches for pattern: "tar_01ARZ3NDEKTSV4RRFFQ69G5FAV_"
```

Key Segment Types:
- `User`: Matches `usr_{query}_` pattern
- `Tag`: Matches `tag_{query}_` pattern
- `Target`: Matches `tar_{query}_` pattern
- `Handle`: Matches `hdl_{query}_` pattern
- `Key`: Matches `key_{query}_` pattern

Benefits of `query_doc`:
- Type-safe query segments prevent errors
- Consistent key pattern formatting
- Efficient key-based indexing
- No full collection memory load
- Built-in logging and error handling

#### 2. Description-Based Queries
**Least Efficient - Requires Full Memory Load**
- Queries using `description` field in ListMatcher
- Loads ALL documents into memory before filtering
- Can hit memory limits with large collections
- Example of what NOT to do:
  ```rust
  // Inefficient: Loads all documents then filters
  list_docs_store(
      caller,
      "users",
      ListParams {
          matcher: Some(ListMatcher {
              description: Some("username=john"), // Requires full memory load
              ..Default::default()
          })
      }
  )
  ```

### Best Practices

1. **Key Structure Design**
   - Include searchable fields in the key
   - Use consistent separators (underscores)
   - Follow the standard patterns:
     ```
     Users:  usr_{ulid}_hdl_{handle}_
     Tags:   usr_{ulid}_tag_{ulid}_hdl_{handle}_
     Votes:  usr_{ulid}_tag_{ulid}_tar_{ulid}_key_{ulid}_
     ```
   - Use the `query_doc` function for type-safe queries with partial key matching

2. **Query Optimization**
   - Use `query_doc` for segment-based queries (recommended)
   - Use `get_doc_store` for single document lookups
   - Use `list_docs_store` with key patterns only when needed
   - Avoid description-based filtering
   - Structure keys to support your most common queries

3. **Index Collections**
   - For complex queries, create separate collections as indexes
   - Use key patterns that support your query needs
   - Example structure for vote queries:
     ```
     Collection: vote_by_owner
     Key format: owner_{ownerUlid}_vote_{voteUlid}
     
     Collection: vote_by_tag
     Key format: tag_{tagUlid}_vote_{voteUlid}
     ```

4. **Memory Considerations**
   - Description field queries load entire collection
   - Key-based queries only load matching segments
   - Consider splitting large collections if needed
   - Monitor memory usage during development

### Query Method Comparison

| Method | Memory Usage | Performance | Use Case |
|--------|--------------|-------------|-----------|
| `get_doc_store` with exact key | Minimal | Fastest | Single document lookup |
| `list_docs_store` with key pattern | Partial | Fast | Filtered queries using key patterns |
| `list_docs_store` with description | Full Collection | Slow | Avoid for large collections |
| Multiple `get_doc_store` calls | Minimal | Fast but sequential | When you need specific documents |

### Memory Limits and Performance Tips

1. **Memory Limits**
   - Heap Memory: 1GB maximum
   - Stable Memory: 400GB maximum (minus heap size)
   - Document Size: 2MB maximum
   - Description Field: 1024 characters maximum
   - Batch Operations: 100 documents maximum

2. **Performance Tips**
   - Use key patterns instead of description fields for filtering
   - Structure keys to support your most common queries
   - Consider index collections for complex query patterns
   - Monitor memory usage during development
   - Use appropriate memory type (heap vs stable) based on access patterns

