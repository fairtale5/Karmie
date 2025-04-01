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
    key: string;              // Unique identifier generated with nanoid() by Juno
    
    // Description format: [owner:documentKey],[username:username]
    // - owner: The key of this user document being created/edited
    // - username: The username field from the document data (not a key)
    // Example: [owner:user_123],[username:john_doe]
    description: string;      
    
    owner: Principal;         // Automatically set to user's Internet Identity Principal
    created_at: bigint;      // Creation timestamp in nanoseconds
    updated_at: bigint;      // Last update timestamp in nanoseconds
    version: bigint;         // Document version for concurrency control
    
    // User-specific data
    data: {
        username: string;     // Unique username (must be unique across all users)
        display_name: string; // Display name (not required to be unique)
    }
}
```

#### Notes
- `username` must be unique across all users
- `display_name` is not required to be unique
- The `description` field is managed automatically by the backend using standard Juno hooks
- All timestamps are in nanoseconds
- `version` is required for updates to prevent concurrent modifications

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
    key: string;              // Unique identifier generated with nanoid()
    
    // Description format: [owner:creatorUserKey],[name:tagName]
    // - owner: The document key of the user who is creating the tag
    // - name: The name field provided in the frontend (not a key)
    // Example: [owner:user_123],[name:technical_skills]
    description: string;      
    
    owner: Principal;         // Automatically set to document creator's Principal
    created_at: bigint;      // Creation timestamp in nanoseconds
    updated_at: bigint;      // Last update timestamp in nanoseconds
    version: bigint;         // Document version for concurrency control
    
    // Tag-specific data
    data: {
        author_key: string;   // User key of the creator (references Users collection)
        name: string;         // Display name of the tag
        description: string;  // Description of the tag's purpose
        
        // Time periods for vote decay multipliers
        time_periods: Array<{
            months: number;     // Duration in months (1-999)
            multiplier: number; // Weight multiplier (0.05-1.5)
        }>;
        
        reputation_threshold: number;     // Minimum reputation needed for voting power
        vote_reward: number;              // Reputation points given for casting votes
        min_users_for_threshold: number;  // Minimum users needed before vote rewards are restricted
    }
}
```

Example Tag Document:
```typescript
{
    key: "tag_123",
    description: "[owner:user_123],[name:Technical Skills]",
    owner: Principal.fromText("..."),
    created_at: 1234567890n,
    updated_at: 1234567890n,
    version: 1n,
    data: {
        author_key: "user_123",
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
        min_users_for_threshold: 5   // Need 5 users to reach threshold before restricting rewards
    }
}
```

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
    key: string;              // Unique identifier generated with nanoid()
    
    // Description format: [owner:voterUserKey],[target:targetUserKey],[tag:tagKey]
    // - owner: The document key of the user casting the vote
    // - target: The document key of the user being voted on
    // - tag: The document key of the tag this vote belongs to
    // Example: [owner:user_123],[target:user_456],[tag:tag_789]
    description: string;      
    
    owner: Principal;         // Automatically set to document creator's Principal
    created_at: bigint;      // Creation timestamp in nanoseconds
    updated_at: bigint;      // Last update timestamp in nanoseconds
    version: bigint;         // Document version for concurrency control
    
    // Vote-specific data
    data: {
        author_key: string;   // User key who cast the vote (references Users collection)
        target_key: string;   // User key being voted on (references Users collection)
        tag_key: string;      // Tag key this vote is for (references Tags collection)
        value: number;        // Vote value (+1 for upvote, -1 for downvote)
        weight: number;       // Vote weight (default: 1.0)
        created_at: bigint;   // Creation timestamp in nanoseconds
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
    key: string;              // Unique identifier generated with nanoid()
    
    // Description format: [owner:userKey],[tag:tagKey]
    // - owner: The document key of the user this reputation belongs to
    // - tag: The document key of the tag this reputation is for
    // Example: [owner:user_123],[tag:tag_789]
    description: string;      
    
    owner: Principal;         // Automatically set to document creator's Principal
    created_at: bigint;      // Creation timestamp in nanoseconds
    updated_at: bigint;      // Last update timestamp in nanoseconds
    version: bigint;         // Document version for concurrency control
    
    // Reputation-specific data
    data: {
        user_key: string;     // User this reputation is for (references Users collection)
        tag_key: string;      // Tag this reputation is for (references Tags collection)
        
        total_basis_reputation: number;          // Reputation from received votes
        total_voting_rewards_reputation: number; // Reputation from casting votes
        last_known_effective_reputation: number; // Final reputation score (cached value)
        
        last_calculation: bigint;  // When the reputation was last calculated (timestamp in nanoseconds)
        vote_weight: number;       // The user's vote weight (0.0 to 1.0, where 1.0 = 100%)
        has_voting_power: boolean; // Whether the user has sufficient reputation to have voting power
    }
}
```

#### Notes
- Each document represents one user's reputation in one tag
- Reputation calculations are tag-specific
- Cached scores are updated only when needed
- Other tags' reputations remain untouched during updates
- The `vote_weight` field has special handling:
  - In the Rust code: Implemented as a custom `VoteWeight` struct with validation to ensure values are between 0.0 and 1.0
  - In the database: Stored directly as a number between 0.0 and 1.0

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

# Document Description Field Standards

## Format Standard
All document descriptions follow a consistent format with field-value pairs:
`field1=value1;field2=value2;field3=value3;`

Rules:
- Each field-value pair ends with a semicolon `;`
- Use equal sign `=` to separate field from value
- Use consistent field order per collection
- All values are document keys unless specified otherwise

## Collection-Specific Formats

### Users Collection
Format: `owner=documentKey;username=username;`
- `owner`: The key of this user document being created/edited
- `username`: The username field from the document data (not a key)

Example: `owner=user_123;username=john_doe;`

### Tags Collection
Format: `owner=creatorUserKey;name=tagName;`
- `owner`: The document key of the user who is creating the tag
- `name`: The name field provided in the frontend (not a key)

Example: `owner=user_123;name=technical_skills;`

### Votes Collection
Format: `owner=voterUserKey;target=targetUserKey;tag=tagKey;`
- `owner`: The document key of the user casting the vote
- `target`: The document key of the user being voted on
- `tag`: The document key of the tag this vote belongs to

Example: `owner=user_123;target=user_456;tag=tag_789;`

### Reputations Collection
Format: `owner=userKey;tag=tagKey;`
- `owner`: The document key of the user this reputation belongs to
- `tag`: The document key of the tag this reputation is for

Example: `owner=user_123;tag=tag_789;`

## Important Implementation Notes

1. **Description Field Generation**:
   ```rust
   // Example: Creating a vote description in Rust
   let description = format!(
       "owner={};target={};tag={};",
       vote.data.author_key,
       vote.data.target_key,
       vote.data.tag_key
   );
   ```

2. **Handling in Collection Hooks**:
   ```rust
   // Example: In on_set_doc hook for votes collection
   fn on_set_doc_votes(doc: &mut Document) -> Result<(), String> {
       // Access the data fields directly
       if let Some(data) = doc.data.as_object_mut() {
           // Get fields from the document data
           let author_key = data.get("author_key").and_then(|v| v.as_str()).unwrap_or("");
           let target_key = data.get("target_key").and_then(|v| v.as_str()).unwrap_or("");
           let tag_key = data.get("tag_key").and_then(|v| v.as_str()).unwrap_or("");
           
           // Set the description using the fields
           doc.description = format!("owner={};target={};tag={};", author_key, target_key, tag_key);
       }
       Ok(())
   }
   ```

3. **Querying with Partial Matches**:
   ```typescript
   // Example: Find all votes for a specific tag, regardless of author or target
   const { items } = await listDocs({
       collection: "votes",
       filter: {
           matcher: {
               description: `tag=${tagKey};`
           }
       }
   });
   ```

## Benefits of Field=Value; Format

1. **Clear Field Boundaries**: 
   - Each field-value pair ends with a semicolon
   - Prevents issues with values containing delimiters

2. **Partial Matching**: 
   - Can match on any subset of fields
   - Easier to construct search patterns

3. **Consistent Pattern**:
   - Same pattern across all collections
   - Easier to generate programmatically
   - Easier to parse in queries

4. **Developer-Friendly**:
   - Intuitive mapping to underlying data structures
   - Self-documenting format
   - Easy to debug and understand

## Description Field Pattern Matching

The `ListMatcher` in Juno provides powerful pattern matching capabilities for description fields. This is particularly useful when querying documents that need to match multiple criteria.

### Basic Pattern Matching

```rust
// Simple single-field match
let results = list_docs_store(
    caller,
    String::from("votes"),
    ListParams {
        matcher: Some(ListMatcher {
            description: Some(format!("tag={};", tag_key)),
            ..Default::default()
        }),
        ..Default::default()
    },
);
```

### Multiple Field Matching 

To match multiple fields:

```rust
// Match documents where both owner AND tag match
let results = list_docs_store(
    caller,
    String::from("reputations"),
    ListParams {
        matcher: Some(ListMatcher {
            // Will match description containing both owner=123; AND tag=456;
            description: Some(format!("owner={};tag={};", user_key, tag_key)),
            ..Default::default()
        }),
        ..Default::default()
    },
);
```

### Benefits of ListMatcher

1. **Database-Level Filtering**: 
   - Pattern matching happens at the database level
   - More efficient than filtering in application code
   - Reduces data transfer

2. **Flexible Matching**:
   - Support for exact string matching
   - Can match partial patterns
   - Order-independent matching

3. **Clear Intent**:
   - Pattern matching logic is explicit
   - Easy to understand and maintain
   - Self-documenting queries

4. **Performance**:
   - Optimized for description field queries
   - Efficient for large datasets
   - Minimizes memory usage

### Best Practices

1. **Use Proper Formatting**:
   ```rust
   // Good: Clear field boundaries
   format!("owner={};tag={};", user_key, tag_key)
   
   // Bad: Unclear boundaries
   format!("owner={} tag={}", user_key, tag_key)
   ```

2. **Choose Appropriate Logic**:
   ```rust
   // Match documents for a specific field
   format!("tag={};", tag_key)
   
   // Match documents with multiple criteria
   format!("owner={};tag={};", user_key, tag_key)
   ```

3. **Consider Access Control**:
   ```rust
   // System-level operations
   list_docs_store(ic_cdk::id(), ...)
   
   // User-level operations
   list_docs_store(ic_cdk::caller(), ...)
   ```

4. **Handle Results Appropriately**:
   ```rust
   // Use first() when expecting single result
   if let Some((doc_key, doc)) = results.items.first() {
       // Process single document
   }
   
   // Iterate when expecting multiple results
   for (doc_key, doc) in results.items {
       // Process each document
   }
   ```
