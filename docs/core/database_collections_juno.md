# Database Collections in Juno

This document defines how to work with documents in the Juno datastore.

## Document Format

When setting a document:
```typescript
// Format for setDoc()
{
    collection: string,  // The collection name (e.g. "users")
    doc: {
        key: string,    // Document identifier
        data: any,      // Your document data
        description?: string,  // Optional, max 1024 chars
        version?: bigint      // Required for updates
    }
}
```

When getting a document:
```typescript
// What getDoc() returns
// nobody knows the return format so far. need to document this
```

## Users Collection

Collection name: `users`

### Complete Document Structure
```typescript
interface UserDocument {
    // Standard Juno fields
    key: string;
    description: string;     // Format: "username:normalized_handle"
    owner: Principal;
    created_at: bigint;
    updated_at: bigint;
    version: bigint;

    // User-specific data
    data: {
        handle: string;      // Unique identifier for the user
        displayName: string; // Display name (not unique)
    }
}
```

### Notes
- `handle` must be unique across all users
- `displayName` is not required to be unique
- The `description` field is managed automatically by the backend
- All timestamps are in nanoseconds
- `version` is required for updates to prevent concurrent modifications 

---

### Collections Format for Each Database Type


+ ### Users Collection
+ ```typescript
+ // Format for setDoc()
+ {
+     collection: "users",
+     doc: {
+         key: string,              // Generated with nanoid()
+         data: {
+             username: string,     // The user's username
+             displayName: string,  // The user's display name
+         },
+         description?: string,     // Will be set by backend: "username:normalized_value"
+         version?: bigint         // Required only for updates
+     }
+ }
+ ```

## Tags Collection

Collection name: `tags`

```typescript
{
    collection: "tags",
    doc: {
        key: string,              // Generated with nanoid()
        data: {
            name: string,         // The tag name (e.g., "#teamplay", "#crypto")
            description: string,  // Description of what this tag represents
            founding_members: string[]  // Array of user IDs (up to 100) invited as founding members
        },
        description?: string,     // Will be set by backend: "name:normalized_tag"
        version?: bigint         // Required only for updates
    }
}
```

Votes Collection

```typescript
{
    collection: "votes",
    doc: {
        key: string,              // Generated with nanoid()
        data: {
            target: string,       // User key of the target being voted on
            is_positive: boolean, // true for positive vote, false for negative
            weight?: number      // Optional weight of the vote
        },
        description?: string,     // Will be set by backend: "author:key,target:key"
        version?: bigint         // Required only for updates
    }
}
```

Reputations Collection
```typescript
{
    collection: "reputations",
    doc: {
        key: string,              // Generated with nanoid()
        data: {
            user_id: string,      // The user whose reputation this is
            tag_id: string,       // The tag this reputation is for
            voting_power: number, // Current voting power for this tag
            votes_received: number // Total number of votes received for this tag
        },
        description?: string,     // Will be set by backend: "user:key,tag:key"
        version?: bigint         // Required only for updates
    }
}