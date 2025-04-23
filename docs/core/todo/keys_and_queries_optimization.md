# Document Key Optimization Plan

## Main Problem

Currently, we query documents using the description field, which requires loading the entire table into memory first. This is not scalable as the dataset grows.

The key field is the only field that can be queried without loading the table into memory, making it much more efficient for queries. We need to move our query patterns from description-based to key-based.

## Solution Overview

### Key Concepts

1. **Document Keys vs ULID Fields**
   - Document keys are composite strings used for efficient querying
   - ULID fields in `data` are pure ULIDs used for references
   - Keys can include additional info (e.g., username) for readability
   - ULIDs ensure type safety and validation where needed

2. **Type Safety**
   ```typescript
   // In types.ts
   type ULID = string & { readonly __brand: unique symbol };
   
   // Example document structure
   interface Document {
     key: string;           // Composite key for querying
     data: {
       usr_key: ULID;      // Pure ULID for references
       other_key: ULID;    // Pure ULID for references
       // ... other fields
     }
   }
   ```

### 1. Reputation Documents

**Current (Problem):**
- Uses description field for queries
- Allows duplicate reputation documents
- No standardized key format

**New Solution:**
- Document key format: `USR_{ulid}_TAG_{ulid}`
- Example: `USR_01ARZ3NDEKTSV4RRFFQ69G5FAV_TAG_01ARZ3NDEKTSV4RRFFQ69G5FAW`
- Data fields:
  ```typescript
  interface ReputationData {
    usr_key: ULID;    // Pure ULID for user reference
    tag_key: ULID;    // Pure ULID for tag reference
    // ... other fields
  }
  ```
- Benefits:
  - Can query without loading table into memory
  - Prevents duplicate reputation documents
  - Type-safe ULID references in data (26 characters, Crockford Base32)
  - Natural chronological sorting
  - Easy validation using ULID library

### 2. Vote Documents

**Current (Problem):**
- Uses description field for queries
- Complex query patterns
- No standardized key format

**New Solution:**
- Document key format: `USR_{ulid}_TAG_{ulid}_TAR_{ulid}_KEY_{ulid}`
- Example: `USR_01ARZ3NDEKTSV4RRFFQ69G5FAV_TAG_01ARZ3NDEKTSV4RRFFQ69G5FAW_TAR_01ARZ3NDEKTSV4RRFFQ69G5FAX_KEY_01ARZ3NDEKTSV4RRFFQ69G5FAY`
- Data fields:
  ```typescript
  interface VoteData {
    usr_key: ULID;    // Pure ULID for user reference
    tag_key: ULID;    // Pure ULID for tag reference
    tar_key: ULID;    // Pure ULID for target reference
    vote_key: ULID;   // Pure ULID for this vote
    // ... other fields
  }
  ```
- Query patterns:
  - Find votes by user in tag: Search for `USR_{ulid}_TAG_{ulid}`
  - Find votes for target in tag: Search for `TAG_{ulid}_TAR_{ulid}`
  - Natural chronological sorting by default

### 3. Standard Format for All Documents

All document keys will follow these standards:
- User keys: `usr_{ulid}_usrName_{string}_`
- Tag keys: `usr_{ulid}_tag_{ulid}_tagName_{string}_`
- Reputation keys: `usr_{ulid}_tag_{ulid}`
- Vote keys: `usr_{ulid}_tag_{ulid}_tar_{ulid}_key_{ulid}`

Note: All ULIDs are 26 characters using Crockford's Base32 encoding, providing both uniqueness and chronological sorting.
Important: 
- ULIDs themselves must be UPPERCASE (e.g., `01ARZ3NDEKTSV4RRFFQ69G5FAV`) for consistency between frontend and backend
- Key segment prefixes use camelCase (e.g., `usr_`, `usrName_`, `tagName_`, etc.)
- Names in keys (username, tagname) are lowercase for efficient querying

### 4. Detailing for each document type

#### 4.1 User Documents

  - Example: `usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_usrName_johndoe_`
  - ULID: Unique identifier for the user (must be uppercase)
  - Username: Sanitized username for readability
  - Validation:
    - Username: 3-30 chars, alphanumeric + hyphen, can be both upper or lower case, no spaces.
    - Username uniqueness enforced by backend. Uniqueness ignores case. This means that `johndoe` and `JohnDoe` are considered the same username.
      - In the user key, the username is stored in lowercase, for efficient queries. Example: `johndoe`
      - In the data.username field, the username is stored in the original case. Example: `JohnDoe`
      - One account per identity in production mode
    - Display name: Non-empty, max 50 chars

#### 4.2 Tag Documents

  - Example: `usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW_tagName_technicalskills_`
  - First ULID: Creator's user identifier (must be uppercase)
  - Second ULID: Tag's unique identifier (must be uppercase)
  - Tag name: Sanitized, no spaces, lowercase tag name for readability and querying
  - Validation:
    - Tag name: Must be unique (case-insensitive) 3-30 chars, alphanumeric + hyphen, can be both upper or lower case, no spaces.
    - Original case preserved in data.name field
    - Tag name follows same character restrictions as username

#### 4.3 Reputation Documents

  - Example: `usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW`
  - First ULID: User's identifier (must be uppercase)
  - Second ULID: Tag's identifier (must be uppercase)
  - No additional components needed as the combination of user and tag is unique

#### 4.4 Vote Documents

  - Example: `usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW_tar_01ARZ3NDEKTSV4RRFFQ69G5FAX_key_01ARZ3NDEKTSV4RRFFQ69G5FAY`
  - Components:
    - usr: Voter's ulid identifier (must be uppercase)
    - tag: Tag being voted on's ulid identifier (must be uppercase)
    - tar: Target user receiving the vote's ulid identifier (must be uppercase)
    - key: Unique vote ulid identifier (must be uppercase)
  - Format enables efficient querying by any combination of voter, tag, and target

## Required Updates

### Backend Changes

1. **ULID Generation**
   - Use IC's native features for ULID components:
     ```rust
     // ULID components:
     // - Timestamp (48 bits): from IC time
     // - Random (80 bits): from IC random number generator
     async fn generate_ulid() -> String {
         let timestamp = ic_cdk::api::time() / 1_000_000; // Convert ns to ms
         let random = ic_cdk::api::management_canister::main::raw_rand().await.unwrap().0;
         // Implementation using dylanhart/ulid-rs
         // Ensure uppercase output
         ulid.to_string().to_uppercase()
     }
     ```

   - Validation ensures ULIDs are correctly formatted:
     ```rust
     fn validate_ulid(ulid_str: &str) -> Result<(), String> {
         // Validation checks:
         // 1. Length must be exactly 26 chars
         // 2. All characters must be valid Crockford Base32
         // 3. Timestamp must be valid
         // 4. Must be uppercase
         if !ulid_str.len() == 26 {
             return Err("ULID must be exactly 26 characters".to_string());
         }
         if ulid_str != ulid_str.to_uppercase() {
             return Err("ULID must be uppercase".to_string());
         }
         if !ulid_str.chars().all(|c| "0123456789ABCDEFGHJKMNPQRSTVWXYZ".contains(c)) {
             return Err("ULID contains invalid characters".to_string());
         }
         Ok(())
     }
     ```

   - Key format benefits:
     ```rust
     // Example key: USR_01ARZ3NDEKTSV4RRFFQ69G5FAV_TAG_01ARZ3NDEKTSV4RRFFQ69G5FAW
     // Benefits:
     // 1. Prefix (USR_, TAG_) makes key type immediately identifiable
     // 2. Underscore separator is URL-safe and easy to split/parse
     // 3. ULID format (26 chars) is compact and includes timestamp
     // 4. Natural chronological sorting
     // 5. Case-insensitive
     ```

2. **Document Creation/Updates**
   - Update all document creation to use new key formats
   - Implement retry logic for reputation document creation with exponential backoff:
     - Max attempts: 3
     - Initial delay: 100ms
     - Backoff: 100ms -> 200ms -> 400ms
   - Add validation for new key formats
   - Rename `data.owner` to `data.user` in documents

3. **Query System**
   - Update all queries to use key-based search instead of description
   - Implement new query patterns for votes and reputations
   - Add validation for query parameters
   - Take advantage of natural chronological sorting

4. **Juno List Matcher Integration**
   - Utilize Juno's built-in list matcher for efficient querying:
     ```typescript
     interface ListMatcher {
       key?: string;          // Regex pattern to match against document keys
       description?: string;  // Regex pattern to match against descriptions
       createdAt?: ListTimestampMatcher;
       updatedAt?: ListTimestampMatcher;
     }
     ```

   - Example query patterns:
     ```typescript
     // Find all votes by a user in a specific tag
     const userVotes = await listDocs({
       collection: "votes",
       filter: {
         matcher: {
           key: `^USR_${userUlid}_TAG_${tagUlid}`  // Prefix match
         }
       }
     });

     // Find all votes for a target in a tag
     const targetVotes = await listDocs({
       collection: "votes",
       filter: {
         matcher: {
           key: `TAG_${tagUlid}_TAR_${targetUlid}`  // Partial match
         }
       }
     });

     // Combine with pagination and sorting
     const paginatedVotes = await listDocs({
       collection: "votes",
       filter: {
         matcher: {
           key: `USR_${userUlid}`
         },
         paginate: {
           startAfter: lastKey,
           limit: 10
         },
         order: {
           desc: true,
           field: "created_at"
         }
       }
     });
     ```

   - Key benefits:
     1. Database-level filtering (more efficient than client-side)
     2. Supports both exact and partial key matches
     3. Can combine multiple search criteria
     4. Built-in pagination and sorting
     5. No table-wide scans required
     6. Works with both document keys and descriptions

   - Implementation patterns:
     ```rust
     // Backend implementation using Rust
     use junobuild_satellite::list_docs;
     use junobuild_shared::types::list::{ListMatcher, ListParams};

     async fn find_user_votes(user_ulid: &str, tag_ulid: &str) -> Result<Vec<Vote>, String> {
         let results = list_docs(
             String::from("votes"),
             ListParams {
                 matcher: Some(ListMatcher {
                     key: Some(format!("^USR_{}_TAG_{}", user_ulid, tag_ulid)),
                     ..Default::default()
                 }),
                 ..Default::default()
             },
         ).await?;
         
         Ok(results.items)
     }
     ```

   - Performance considerations:
     1. Use key-based queries whenever possible
     2. Leverage prefix matching for hierarchical keys
     3. Combine with pagination for large result sets
     4. Take advantage of natural ULID chronological sorting
     5. Use appropriate index patterns for common queries

### Frontend Changes

1. **ULID Implementation**
   - Add ULID library:
     ```typescript
     import { ulid } from 'ulid'
     
     // Simple key generation - ulid() generates uppercase by default
     function generateDocumentKey(prefix: string): string {
         return `${prefix}_${ulid()}`
     }
     
     // Validation - enforce uppercase
     function validateUlid(ulidStr: string): boolean {
         if (ulidStr !== ulidStr.toUpperCase()) {
             return false;
         }
         return /^[0123456789ABCDEFGHJKMNPQRSTVWXYZ]{26}$/.test(ulidStr)
     }
     ```

2. **Document Creation**
   - Update all document creation flows to use new key format
   - Add validation for ULID fields
   - Update any UI components that display/handle keys

### Migration Plan

1. **New Documents**
   - All new documents will use new ULID format
   - Implement validation for new format
   - Take advantage of chronological sorting

2. **Existing Documents**
   - No need to migrate existing UUID documents
   - New ULID format only applies to new documents

## Next Steps

1. Add ULID libraries:
   - Frontend: ulid/javascript
   - Backend: dylanhart/ulid-rs
2. Create ULID generation/validation functions
3. Update document creation flows
4. Update query patterns
5. Test performance impact
6. Document new patterns for team

## Questions to Resolve

1. Retry strategy for reputation documents:
   - How many retries? (Currently set to 3)
   - Delay between retries? (Currently exponential: 100ms -> 200ms -> 400ms)
   - Error handling after max retries?

2. Migration strategy:
   - How to handle mixed format queries?
   - Documentation for mixed format period?
   - Monitoring strategy for new format adoption?
