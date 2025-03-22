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

#### Document Structure
```typescript
interface UserDocument {
    // Standard Juno fields (automatically managed)
    key: string;              // Generated with nanoid() by Juno
    description: string;      // Format: [owner:{principal}],[username:{name}]
    owner: Principal;         // Automatically set to user's Internet Identity Principal
    created_at: bigint;      // Automatically set on creation (nanoseconds)
    updated_at: bigint;      // Automatically updated on changes (nanoseconds)
    version: bigint;         // Automatically managed for concurrency control

    // User-specific data
    data: {
        handle: string;       // Unique username
        display_name: string; // Display name (not unique)
        principal: Principal; // User's Internet Identity Principal
    }
}
```

#### Permissions
- Read: public
- Write: private
- Memory: stable
- Mutable Permissions: true

#### Notes
- `handle` must be unique across all users
- `display_name` is not required to be unique
- The `description` field is managed automatically by the backend
- All timestamps are in nanoseconds
- `version` is required for updates to prevent concurrent modifications

### Tags Collection

Collection name: `tags`

#### Document Structure
```typescript
interface TagDocument {
    // Standard Juno fields (automatically managed)
    key: string;              // Generated with nanoid()
    description: string;      // Optional field for filtering/search
    owner: Principal;         // Automatically set to document creator's Principal
    created_at: bigint;      // Automatically set on creation (nanoseconds)
    updated_at: bigint;      // Automatically updated on changes (nanoseconds)
    version: bigint;         // Automatically managed for concurrency control

    // Tag-specific data
    data: {
        name: string;     // Display name
        description: string; // Description of the tag
        time_periods: Array<{
            months: number;    // Duration in months (1-999)
            multiplier: number; // Weight multiplier (0.25-1.5)
        }>;
        reputation_threshold: number;  // Minimum reputation needed for voting power (whole number)
        vote_reward: number;          // Reputation points given for casting a vote (e.g., 0.1)
        min_users_for_threshold: number; // Minimum number of users that need to reach threshold
                                        // before vote rewards are restricted
        vote_weight: number;          // Weight multiplier for votes (default: 1.0)
    }
}
```

Example Tag Document:
```typescript
{
    key: "tag_123",
    description: "Technical expertise and knowledge",
    owner: Principal.fromText("..."),
    created_at: 1234567890n,
    updated_at: 1234567890n,
    version: 1n,
    data: {
        name: "Technical Skills",
        description: "Technical expertise and knowledge",
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
        min_users_for_threshold: 5,  // Need 5 users to reach threshold before restricting rewards
        vote_weight: 1.0            // Default vote weight multiplier
    }
}
```

#### Permissions
- Read: public
- Write: managed
- Memory: stable
- Mutable Permissions: true

### Votes Collection

Collection name: `votes`

#### Document Structure
```typescript
interface VoteDocument {
    // Standard Juno fields (automatically managed)
    key: string;              // Generated with nanoid()
    description: string;      // Format: "author:{author_key},target:{target_key},tag:{tag_key}"
    owner: Principal;         // Automatically set to document creator's Principal
    created_at: bigint;      // Automatically set on creation (nanoseconds)
    updated_at: bigint;      // Automatically updated on changes (nanoseconds)
    version: bigint;         // Automatically managed for concurrency control

    // Vote-specific data
    data: {
        author_key: string;   // User key who cast the vote
        target_key: string;   // User key being voted on
        tag_key: string;      // Tag this vote is for
        value: number;        // Vote value (+1 for upvote, -1 for downvote)
        weight: number;       // Vote weight (default: 1.0)
        created_at: bigint;   // Creation timestamp in nanoseconds
    }
}
```

#### Permissions
- Read: public
- Write: private
- Memory: stable
- Mutable Permissions: false

### Reputations Collection

Collection name: `reputations`

#### Document Structure
```typescript
interface ReputationDocument {
    // Standard Juno fields (automatically managed)
    key: string;              // Generated with nanoid()
    description: string;      // Format: "user:{user_key},tag:{tag_key},author:{author_key}"
    owner: Principal;         // Automatically set to document creator's Principal
    created_at: bigint;      // Automatically set on creation (nanoseconds)
    updated_at: bigint;      // Automatically updated on changes (nanoseconds)
    version: bigint;         // Automatically managed for concurrency control

    // Reputation-specific data
    data: {
        user_key: string;     // The user this reputation is for
        tag_key: string;      // The tag this reputation is for
        total_basis_reputation: number;  // Reputation from received votes
        total_voting_rewards_reputation: number;  // Reputation from casting votes
        last_known_effective_reputation: number;  // Final reputation score (cached value)
        last_calculation: number;  // When we last calculated (timestamp in nanoseconds)
        vote_weight: number;      // The user's vote weight (0.0 to 1.0, where 1.0 = 100%)
        has_voting_power: boolean; // Whether the user has sufficient reputation to have voting power (above threshold)
    }
}
```

#### Permissions
- Read: public
- Write: managed
- Memory: stable
- Mutable Permissions: false

#### Notes
- Each document represents one user's reputation in one tag
- Reputation calculations are tag-specific
- Cached scores are updated only when needed
- Other tags' reputations remain untouched during updates

## Query Examples

### Get User's Reputation in a Tag
```typescript
const { items } = await listDocs({
    collection: "reputations",
    filter: {
        matcher: {
            description: `user:${userKey},tag:${tagKey}`
        }
    }
});
```

### Get All Votes by a User
```typescript
const { items } = await listDocs({
    collection: "votes",
    filter: {
        matcher: {
            description: `author:${userKey}`
        }
    }
});
```

### Get All Votes for a User in a Tag
```typescript
const { items } = await listDocs({
    collection: "votes",
    filter: {
        matcher: {
            description: `target:${userKey},tag:${tagKey}`
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

4. **Timestamps**
   - All timestamps are in nanoseconds
   - Use `Date.now() * 1_000_000` to convert from JavaScript

5. **Version Control**
   - Required for updates to prevent concurrent modifications
   - Must match the current document version
   - Automatically incremented after successful updates
   - Only need to provide version when updating documents

6. **Automatically Managed Fields**
   - `owner`: Set to document creator's Principal
   - `created_at`: Set on document creation
   - `updated_at`: Updated on document changes
   - `version`: Managed for concurrency control
   - Only need to provide `version` when updating documents

7. **Test Phase Considerations**
   - All documents created by same user during testing
   - Author stored in description field
   - Will change to proper multi-user system later 

# Document Description Field Standards

## Format Standard
All document descriptions follow this format:
`[field1:{value1}][field2:{value2}][field3:{value3}]`

Rules:
- Use brackets to wrap each field-value pair
- No spaces
- Consistent field order per collection
- Use 'owner' consistently for document ownership

## Collection-Specific Formats

### Users Collection
Format: `,username:{name},owner:{key},`
Example: `,username:john_doe,owner:user_123,`

### Tags Collection
Format: `[name:{name}][owner:{key}]`
Example: `[name:technical_skills][owner:user_123]`

### Votes Collection
Format: `[owner:{key}][target:{key}][tag:{key}]`
Example: `[owner:user_123][target:user_456][tag:tag_789]`

### Reputations Collection
Format: `[owner:{key}][tag:{key}]`
Example: `[owner:user_123][tag:tag_789]`

## Playground vs Production Mode

### Playground Mode
- Uses description field for ownership lookup
- Single Juno user creates all documents
- Ownership tracked via description field
- Format allows for simulated multi-user testing

### Production Mode
- Uses Juno's native owner field
- Each user creates their own documents
- Ownership tracked via Juno's Principal ID
- Description field still maintained for compatibility

## Important Notes
1. The bracket format ensures:
   - Clear field boundaries
   - No issues with special characters in values
   - Reliable pattern matching for queries
   - Easy visual debugging

2. Query Examples:
   - Find by tag: `[tag:tag_123]`
   - Find by owner: `[owner:user_123]`
   - Find by multiple fields: Pattern match on `[owner:user_123][tag:tag_456]`

3. Format Benefits:
   - Values can contain any character (except `]`)
   - No need for escaping special characters
   - Exact field matching without false positives
   - Future-proof for nested structures if needed
