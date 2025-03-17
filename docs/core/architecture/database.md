# Database Schema

This document defines the database schema for the Reputator project.

## Collections

### Important Note for Test Phase
During the initial test phase, all documents will be created by the same user (single-user testing environment). The document's author will be stored in the `description` field along with other metadata. This approach will change in production to use proper multi-user authentication.

### Users Collection

Collection name: `users`

#### Document Structure
```typescript
interface UserDocument {
    // Standard Juno fields
    key: string;              // Generated with nanoid()
    description: string;      // Format: "username:{normalized_handle},author:{author_key}"
    owner: Principal;         // Document owner's Principal ID
    created_at: bigint;      // Timestamp in nanoseconds
    updated_at: bigint;      // Timestamp in nanoseconds
    version: bigint;         // Required for updates

    // User-specific data
    data: {
        handle: string;       // Unique username
        display_name: string; // Display name (not unique)
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
    // Standard Juno fields
    key: string;              // Generated with nanoid()
    description: string;      // Format: "tag:{normalized_tag_name},author:{author_key}"
    owner: Principal;         // Document owner's Principal ID
    created_at: bigint;      // Timestamp in nanoseconds
    updated_at: bigint;      // Timestamp in nanoseconds
    version: bigint;         // Required for updates

    // Tag-specific data
    data: {
        name: string;         // The tag (e.g., "#teamwork", "#coding")
        description: string;  // What this tag represents
        
        // Optional: Custom decay rules for this tag
        decay_rules?: {
            time_brackets: Array<{
                name: string;        // e.g., "last_24h", "current_week"
                duration: number;    // Duration in milliseconds
                weight: number;      // Weight percentage (0-1)
            }>;
        }
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
    // Standard Juno fields
    key: string;              // Generated with nanoid()
    description: string;      // Format: "author:{author_key},target:{target_key},tag:{tag_key}"
    owner: Principal;         // Document owner's Principal ID
    created_at: bigint;      // Timestamp in nanoseconds
    updated_at: bigint;      // Timestamp in nanoseconds
    version: bigint;         // Required for updates

    // Vote-specific data
    data: {
        author_key: string;   // User key who cast the vote
        target_key: string;   // User key being voted on
        tag_key: string;      // Tag this vote is for
        is_positive: boolean; // true = upvote, false = downvote
        
        // Store these for historical tracking
        author_reputation: number;  // Author's reputation at time of voting
        weight: number;            // Calculated initial vote weight
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
    // Standard Juno fields
    key: string;              // Generated with nanoid()
    description: string;      // Format: "user:{user_key},tag:{tag_key},author:{author_key}"
    owner: Principal;         // Document owner's Principal ID
    created_at: bigint;      // Timestamp in nanoseconds
    updated_at: bigint;      // Timestamp in nanoseconds
    version: bigint;         // Required for updates

    // Reputation-specific data
    data: {
        user_key: string;     // The user this reputation is for
        tag_key: string;      // The tag this reputation is for
        reputation_score: number;  // Cached final score
        last_calculation: bigint;  // When we last calculated
        calculation_month: string; // "YYYY-MM" of last calculation
        
        // Store votes grouped by month for decay calculation
        votes_by_period: {
            [period: string]: {    // "YYYY-MM" format
                positive: number;   // Sum of weighted positive votes
                negative: number;   // Sum of weighted negative votes
            }
        }
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
- Votes are grouped by month for efficient decay calculation
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

2. **Timestamps**
   - All timestamps are in nanoseconds
   - Use `Date.now() * 1_000_000` to convert from JavaScript

3. **Version Control**
   - Required for updates to prevent concurrent modifications
   - Must match the current document version
   - Automatically incremented after successful updates

4. **Test Phase Considerations**
   - All documents created by same user during testing
   - Author stored in description field
   - Will change to proper multi-user system later 