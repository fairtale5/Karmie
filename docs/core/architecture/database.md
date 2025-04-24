# Database Schema

This document defines the database schema for the Reputator project.

## Document Keys and Principals

### Document Keys
- All document keys are generated using [nanoid](https://github.com/ai/nanoid) when documents are created
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
    key: string;                // Format: usr_{ulid}_usrName_{username}_ generated with ulid() src/lib/keys/create_ulid.ts
    description: string;        // currently not used
    owner: Principal;           // Automatically set to user's Internet Identity Principal
    created_at: bigint;         // Creation timestamp in nanoseconds
    updated_at: bigint;         // Last update timestamp in nanoseconds
    version: bigint;            // Document version for concurrency control
    data: {                     // User-specific data
        usr_key: ULID;          // Pure ULID for references
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
   - Format: `usr_{ulid}_usrName_{username}_`
   - ULID: 26 characters, Crockford Base32
   - Username: Lowercase, sanitized version

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
    key: string;                // Format: usr_{userUlid}_tag_{tagUlid}_tagName_{tagName}_ generated with formatTagKey() 
    description: string;        // currently not used
    owner: Principal;           // Automatically set to document creator's Principal
    created_at: bigint;         // Creation timestamp in nanoseconds
    updated_at: bigint;         // Last update timestamp in nanoseconds
    version: bigint;            // Document version for concurrency control
    data: {                     // Tag-specific data
        name: string;           // Display name of the tag (original case preserved)
        description: string;    // Longer description of the tag's purpose
        usr_key: ULID;          // Pure ULID of the user who created the tag
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
    key: "usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW_tagName_technicalskills_",
    description: "",
    owner: Principal.fromText("..."),
    created_at: 1234567890n,
    updated_at: 1234567890n,
    version: 1n,
    data: {
        name: "Technical-Skills", // Display name of the tag (original case preserved, no spaces or special characters, just alphanumeric and dashes)	
        description: "Technical expertise and knowledge",
        usr_key: "01ARZ3NDEKTSV4RRFFQ69G5FAV", // Pure ULID of the user who created the tag
        tag_key: "01ARZ3NDEKTSV4RRFFQ69G5FAW", // Pure ULID of this tag
        time_periods: [
            { months: 1, multiplier: 1.5 },    // Period 1: First month
            { months: 2, multiplier: 1.2 },    // Period 2: Months 2-3
            { months: 3, multiplier: 1.1 },    // Period 3: Months 4-6
            { months: 6, multiplier: 1.0 },    // Period 4: Months 7-12
            { months: 12, multiplier: 0.95 },  // Period 5: Months 13-24
            { months: 12, multiplier: 0.75 },  // Period 6: Months 25-36
            { months: 12, multiplier: 0.55 },  // Period 7: Months 37-48
            { months: 999, multiplier: 0.25 }  // Period 8: Months 49+ (treated as infinity)
        ],
        reputation_threshold: 10,     // Users need 10 reputation to get voting power
        vote_reward: 0.1,            // Users get 0.1 reputation for each vote they cast
        min_users_for_threshold: 5   // Need 5 users to reach threshold before restricting rewards
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
   - Format: `usr_{userUlid}_tag_{tagUlid}_tagName_{tagName}_`
   - First ULID: Creator's user identifier (must be uppercase)
   - Second ULID: Tag's unique identifier (must be uppercase)
   - Tag Name: Lowercase, sanitized version of tag name for easy querying

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
    key: string;                // Format: usr_{ulid}_tag_{ulid}_tar_{ulid}_key_{ulid}_ generated with ulid()
    description: string;        // currently not used
    owner: Principal;           // Automatically set to document creator's Principal
    created_at: bigint;         // Creation timestamp in nanoseconds
    updated_at: bigint;         // Last update timestamp in nanoseconds
    version: bigint;            // Document version for concurrency control
    data: {                     // Vote-specific data
        usr_key: ULID;          // Pure ULID of the user casting the vote
        tag_key: ULID;          // Pure ULID of the tag being voted on
        tar_key: ULID;          // Pure ULID of the target user receiving the vote
        vote_key: ULID;         // Pure ULID for this specific vote
        value: number;          // Vote value (+1 for upvote, -1 for downvote, some tags may allow other values)
        created_at: bigint;     // Creation timestamp in nanoseconds
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
   - Fourth ULID: Vote identifier (must be uppercase)
   - All parts must be present and properly formatted

3. **Production Mode Rules**
   - Cannot vote on self (usr_key cannot equal tar_key)

### Notes
- All timestamps are in nanoseconds
- ULID provides chronological sorting capability
- Vote impact is determined by voter's reputation at calculation time
- Vote weight is stored in voter's reputation document, not in vote document
- Some tags may have special validation rules for vote values


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
    key: string;                // Format: usr_{ulid}_tag_{ulid}_ generated with ulid()
    description: string;        // currently not used
    owner: Principal;           // Automatically set to canister Principal (ic_cdk::id())
    created_at: bigint;         // Creation timestamp in nanoseconds
    updated_at: bigint;         // Last update timestamp in nanoseconds
    version: bigint;            // Document version for concurrency control
    data: {                     // Reputation-specific data
        usr_key: ULID;          // Pure ULID of the user this reputation is for
        tag_key: ULID;          // Pure ULID of the tag this reputation is for
        reputation: number;      // Current reputation score
        vote_weight: number;     // User's voting weight in this tag
        activity_score: number;  // Activity level in this tag
        received_votes: number;  // Count of votes received
        cast_votes: number;      // Count of votes cast
        last_vote_at: bigint;   // Timestamp of last vote cast
        last_received_at: bigint; // Timestamp of last vote received
    }
}
```

Example Reputation Document:

```typescript
{
    key: "usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW",
    description: "",
    owner: Principal.fromText("..."),  // Canister Principal
    created_at: 1234567890n,
    updated_at: 1234567890n,
    version: 1n,
    data: {
        usr_key: "01ARZ3NDEKTSV4RRFFQ69G5FAV",
        tag_key: "01ARZ3NDEKTSV4RRFFQ69G5FAW",
        reputation: 25.5,
        vote_weight: 0.85,
        activity_score: 0.75,
        received_votes: 12,
        cast_votes: 8,
        last_vote_at: 1234567890n,
        last_received_at: 1234567890n
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

### Exact Match Query
```typescript
// Find a specific user's reputation in a specific tag
const { items } = await listDocs({
    collection: "reputations",
    filter: {
        matcher: {
            description: `[owner:${userKey}],[tag:${tagKey}]`
        }
    }
});
```

### Partial Match Query
```typescript
// Find all votes for a specific tag, regardless of author or target
const { items } = await listDocs({
    collection: "votes",
    filter: {
        matcher: {
            description: `[tag:${tagKey}]`
        }
    }
});

// Find all votes by a specific user
const { items } = await listDocs({
    collection: "votes",
    filter: {
        matcher: {
            description: `[owner:${userKey}]`
        }
    }
});

// Find a user by username
const { items } = await listDocs({
    collection: "users",
    filter: {
        matcher: {
            description: `[username:${username}]`
        }
    }
});
```

## Important Notes

1. **Document Size Limits**
   - Maximum document size: 2MB
   - Maximum description length: 1024 characters
   - Maximum batch operation size: 100 documents

2. **Key Generation**
   - All document keys are generated using nanoid()
   - Keys are unique within a collection
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

