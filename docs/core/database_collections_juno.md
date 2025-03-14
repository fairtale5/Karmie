# Database Collections in Juno

This document defines how to work with documents in the Juno datastore.
For more info, check /docs/juno/docs/build/datastore/development.md

## Document Format

### Setting a Document
```typescript
// Format for setDoc()
{
    collection: string,  // The collection name (e.g., "users")
    doc: {
        key: string,    // Document identifier (use nanoid())
        data: any,      // Your document data
        description?: string,  // Optional, max 1024 chars
        version?: bigint      // Required for updates
    }
}
```

### Retrieving Documents
```typescript
// Single document
const doc = await getDoc({
    collection: "collection_name",
    key: "document_key"
});

// Multiple documents
const { items, items_length, matches_length } = await listDocs({
    collection: "collection_name",
    filter: {
        matcher: {
            key?: string,          // Regex for document keys
            description?: string,   // Regex for descriptions
            createdAt?: ListTimestampMatcher,
            updatedAt?: ListTimestampMatcher
        },
        paginate: {
            startAfter?: string,
            limit?: number
        },
        order: {
            desc: boolean,
            field: "keys" | "updated_at" | "created_at"
        },
        owner?: string | Principal
    }
});
```

## Collections

### Users Collection

Collection name: `users`

#### Complete Document Structure
```typescript
interface UserDocument {
    // Standard Juno fields
    key: string;              // Generated with nanoid()
    description: string;      // Format: "username:normalized_handle"
    owner: Principal;         // Document owner's Principal ID
    created_at: bigint;      // Timestamp in nanoseconds
    updated_at: bigint;      // Timestamp in nanoseconds
    version: bigint;         // Required for updates

    // User-specific data
    data: {
        handle: string;      // Unique identifier for the user
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

#### Complete Document Structure
```typescript
interface TagDocument {
    // Standard Juno fields
    key: string;              // Generated with nanoid()
    description: string;      // Format: "tag:normalized_tag_name"
    owner: Principal;         // Document owner's Principal ID
    created_at: bigint;      // Timestamp in nanoseconds
    updated_at: bigint;      // Timestamp in nanoseconds
    version: bigint;         // Required for updates

    // Tag-specific data
    data: {
        name: string;         // The tag name (e.g., "#teamplay", "#crypto")
        description: string;  // Description of what this tag represents
        founding_members: string[]; // Array of user keys (up to 100)
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

#### Complete Document Structure
```typescript
interface VoteDocument {
    // Standard Juno fields
    key: string;              // Generated with nanoid()
    description: string;      // Format: "author:{author_id},target:{target_id}"
    owner: Principal;         // Document owner's Principal ID
    created_at: bigint;      // Timestamp in nanoseconds
    updated_at: bigint;      // Timestamp in nanoseconds
    version: bigint;         // Required for updates

    // Vote-specific data
    data: {
        tag: string;             // The tag this vote applies to
        target: string;          // User key of the vote recipient
        is_positive: boolean;    // true for positive, false for negative
        weight: number;          // Weighted vote value (based on voter reputation)
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

#### Complete Document Structure
```typescript
interface ReputationDocument {
    // Standard Juno fields
    key: string;              // Generated with nanoid()
    description: string;      // Format: "user:{user_id},tag:{tag_id}"
    owner: Principal;         // Document owner's Principal ID
    created_at: bigint;      // Timestamp in nanoseconds
    updated_at: bigint;      // Timestamp in nanoseconds
    version: bigint;         // Required for updates

    // Reputation-specific data
    data: {
        tag: string;             // The tag key this reputation is for
        reputation_score: number; // Computed reputation after applying decay
        voting_power: number;    // Power multiplier for this tag
        last_decay_update: bigint; // Last time decay was applied

        votes_by_period: {       // Aggregated votes before decay, grouped by period
            [period: string]: {  // Format: "YYYY-MM" (e.g., "2025-03")
                positive: number; // Total positive vote weight before decay
                negative: number; // Total negative vote weight before decay
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

## Usage Examples

### Creating a Document
```typescript
import { setDoc } from "@junobuild/core";
import { nanoid } from "nanoid";

await setDoc({
    collection: "users",
    doc: {
        key: nanoid(),
        data: {
            handle: "alice",
            display_name: "Alice in Cryptoland"
        }
    }
});
```

### Updating a Document
```typescript
await setDoc({
    collection: "users",
    doc: {
        key: existingDoc.key,
        data: {
            handle: "alice_updated",
            display_name: "Alice Updated"
        },
        version: existingDoc.version  // Required for updates
    }
});
```

### Querying Documents
```typescript
// List all documents in a collection
const allDocs = await listDocs({
    collection: "users"
});

// Query with filters
const filteredDocs = await listDocs({
    collection: "users",
    filter: {
        matcher: {
            description: "username:alice"
        }
    }
});

// Paginated query
const paginatedDocs = await listDocs({
    collection: "users",
    filter: {
        paginate: {
            limit: 10,
            startAfter: "last_doc_key"
        },
        order: {
            desc: true,
            field: "created_at"
        }
    }
});
```

### Batch Operations
```typescript
import { setManyDocs } from "@junobuild/core";

const updates = [
    {
        collection: "votes",
        doc: {
            key: nanoid(),
            data: {
                tag: "tag_1",
                target: "user_1",
                is_positive: true,
                weight: 1
            }
        }
    },
    {
        collection: "votes",
        doc: {
            key: nanoid(),
            data: {
                tag: "tag_1",
                target: "user_2",
                is_positive: false,
                weight: 1
            }
        }
    }
];

await setManyDocs({ docs: updates });
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

4. **Permissions**
   - public: Anyone can access
   - private: Only document owner can access
   - managed: Owner and controllers can access
   - controllers: Only controllers can access

