# Frontend Refactor: set_doc Functions and User Key Format

## Current Tasklist

- [x] Create per-type frontend validation toggles in src/lib/settings.ts
- [x] Create src/lib/set_doc/ folder for set_doc functions
- [x] Implement setDocUser for user document creation (new key format: _prn_{principal}_usr_{userULID}_hdl_{username}_)
- [x] Implement setDocTag for tag document creation
- [x] Implement setDocVote for vote document creation
- [ ] Update admin page to use setDocUser for user creation
- [ ] Update onboarding page to use setDocUser for user creation
- [ ] Add frontend validation logic (toggleable per type)
- [ ] Update documentation in database.md and this file to reflect new user key format and set_doc usage
- [ ] Remove any legacy user creation logic from frontend

---

# Document Key Optimization Plan

## ToDo List

### Completed Tasks
- [x] Add ULID library (ulid/javascript) to frontend
- [x] Create ULID types (`src/lib/keys/ulid_types.ts`)
- [x] Create ULID generation function (`src/lib/keys/create_ulid.ts`)
- [x] Implement key formatting utilities:
  - [x] `formatUserKey` in `src/lib/keys/format_key_user.ts`
  - [x] `formatTagKey` in `src/lib/keys/format_key_tag.ts`
  - [x] `formatReputationKey` in `src/lib/keys/format_key_reputation.ts`
  - [x] `formatVoteKey` in `src/lib/keys/format_key_vote.ts`
- [x] Update User document creation in admin panel to use new key format

### Current Priority: Frontend Implementation
- [x] Update Tag document creation in admin panel to use new key format 
  - [x] Follow implementation steps in `docs/core/temp/tag_implementation_plan.md`
  - [x] Remove usage of `nanoid()` in tag creation
  - [x] Update tag form to include author selection
  - [x] Modify `saveTag()` to use `formatTagKey`
- [x] Update Vote document creation in admin panel to use new key format
  - [x] Remove usage of `nanoid()` in vote creation
  - [x] Modify `saveVote()` to use `formatVoteKey`
- [x] Update document queries in frontend to use key-based search instead of description-based
  - [x] Update `loadUserReputations` to use key-based filtering with pattern `tag_{tagUlid}`
  - [x] Update `loadVotes` to use key-based filtering with pattern `tag_{tagUlid}`
  - [x] Update `loadTags` to use key-based filtering with pattern `usr_{userUlid}`
  - [x] Ensure all queries are optimized for key-based search to improve efficiency

### Next Phase: Backend Changes
- [x] Add ULID library (dylanhart/ulid-rs) to backend
  - [x] Add `ulid` crate to Cargo.toml with proper features
  - [x] Implement comprehensive unit tests for ULID functionality
- [x] Create processors directory structure
  - [x] Implement `/src/satellite/src/processors/mod.rs` for module exports
  - [x] Implement `/src/satellite/src/processors/ulid_generator.rs` with:
    - [x] `generate_ulid()` function using IC's time and random functions
    - [x] `validate_ulid()` function with format checking
    - [x] Comprehensive tests for generation and validation
- [x] Implement document key management
  - [x] Create `/src/satellite/src/processors/document_keys.rs` with:
    - [x] Common utilities (`sanitize_for_key()`, `parse_key()`)
    - [x] Key generation for all document types (`create_user_key()`, etc.)
    - [x] Key validation for all document types (`validate_user_key()`, etc.)
    - [x] Comprehensive tests for each function
- [x] Update `src/satellite/src/utils/structs.rs` to include new ULID fields
  - [x] Add `usr_key` optional field to `UserData` for backward compatibility
  - [x] Add `usr_key` and `tag_key` ULID fields to `TagData`
  - [x] Add ULID fields to `VoteData` (`usr_key`, `tag_key_ulid`, `tar_key`, `vote_key`)
  - [x] Add ULID fields to `ReputationData` including new fields for enhanced functionality
  - [x] Update documentation for all struct fields
- [x] Restructure validation system
  - [x] Move from `/utils/validation.rs` to `/validation/` modular approach
  - [x] Create specialized validation modules (username, display_name, etc.)
  - [x] Update imports and references throughout codebase
- [x] Update backend validation for document types:
  - [x] User document format and validation:
    - [x] Simplify user key validation in `assert_doc_user.rs`
    - [x] Implement key-based validation instead of regex pattern matching
    - [x] Remove description validation in favor of key validation
    - [x] Update username uniqueness check to use key-based lookup
    - [x] Implement direct `get_doc` with partial key matching for efficiency
  - [x] Tag document format and validation
  - [x] Vote document format and validation
  - [x] Reputation document format and validation
- [x] Update API interface
  - [x] Add public API functions in `lib.rs` for key management:
    - [x] `create_document_key_for_user`
    - [x] `create_document_key_for_tag`
    - [x] `create_document_key_for_reputation`
    - [x] `create_document_key_for_vote`
    - [x] `validate_document_key`
- [x] Enhance Type Safety for ULID
  - [x] Create dedicated ULID newtype for stronger type checking
    - [x] Define newtype in `/src/satellite/src/processors/ulid_type.rs`
    - [x] Add deserialization logic for ULID type
    - [x] Implement proper error handling for invalid ULID conversions
    - [x] Add timestamp validation for ULID (enforcing valid date range)
    - [x] Add character set validation (Crockford Base32)
  - [ ] Update struct definitions to use the new ULID type
    - [ ] Modify `UserData.usr_key` to use `Option<ULID>` instead of `Option<String>`
    - [ ] Update `TagData` fields to use ULID type
    - [ ] Update `VoteData` fields to use ULID type 
    - [ ] Update `ReputationData` fields to use ULID type
  - [ ] Update document key functions to use the new ULID type
    - [ ] Modify `generate_ulid()` to return ULID instead of String
    - [ ] Update key generation functions to accept ULID type
    - [ ] Update key validation functions to work with ULID type
  - [x] Add comprehensive tests for ULID type safety
    - [x] Test serialization/deserialization
    - [x] Test validation during construction
    - [x] Test error handling for invalid ULIDs
- [x] Update document creation flows for Reputations (see `docs/core/temp/reputation_implementation_plan.md`and reputation_calculations.rs)
- [x] Update query patterns to use key-based search
  - [x] Implement user document partial key matching in `validate_user_document`
  - [x] Update `list_docs` usage to prefer key-based filtering
  - [x] Implement key-based search for reputation documents
  - [x] Implement key-based search for vote documents
  - [x] Implement key-based search for tag documents
- [x] Implement clean transition approach
  - [x] Create utilities for dual-query (both key and description) for transition period
  - [x] Implement validation that supports both old and new formats

### Documentation & Testing
- [x] Update `docs/core/architecture/database.md` to reflect the new document layouts:
  - [x] Update key format descriptions for all collections
  - [x] Document transition approach from description-based to key-based queries
  - [x] Update example documents to show new ULID format
  - [x] Remove outdated query examples and add new key-based query examples
- [x] Create implementation guide for team
  - [x] Document new key validation approach
  - [x] Provide examples of key-based queries
  - [x] Explain performance benefits of the new approach
- [x] Testing
  - [x] Add comprehensive unit tests for ULID generation
  - [x] Add comprehensive unit tests for document key validation
  - [x] Test performance impact of key-based vs. description-based queries
  - [x] Add integration tests for key-based document operations
- [x] Define clear monitoring strategy for new format adoption
  - [x] Create logging for key format issues during transition
  - [x] Implement metrics for tracking key-based query performance

### Migration Considerations
- [x] Develop a strategy for gradually migrating existing documents (if needed)
- [x] Create a timeline for full transition to new format
- [x] Define error handling approach for mixed-format operations

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
- Document key format: `usr_{ulid}_tag_{ulid}`
- Example: `usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW`
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
- Document key format: `usr_{ulid}_tag_{ulid}_tar_{ulid}_key_{ulid}`
- Example: `usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW_tar_01ARZ3NDEKTSV4RRFFQ69G5FAX_key_01ARZ3NDEKTSV4RRFFQ69G5FAY`
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
  - Find votes by user in tag: Search for `usr_{ulid}_tag_{ulid}`
  - Find votes for target in tag: Search for `tag_{ulid}_tar_{ulid}`
  - Natural chronological sorting by default

### 3. Standard Format for All Documents

All document keys will follow these standards:
- User keys: `usr_{ulid}_hdl_{usernameHandle}_`
- Tag keys: `usr_{ulid}_tag_{ulid}_hdl_{tagHandle}_`
- Reputation keys: `usr_{ulid}_tag_{ulid}`
- Vote keys: `usr_{ulid}_tag_{ulid}_tar_{ulid}_key_{ulid}`

Note: All ULIDs are 26 characters using Crockford's Base32 encoding, providing both uniqueness and chronological sorting.
Important: 
- ULIDs themselves must be UPPERCASE (e.g., `01ARZ3NDEKTSV4RRFFQ69G5FAV`) for consistency between frontend and backend
- Key segment prefixes use camelCase (e.g., `usr_`, `hdl_`, etc.)
- Handles in keys (username, tagname) are lowercase for efficient querying

### 4. Detailing for each document type

#### 4.1 User Documents

  - Example: `usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_hdl_johndoe_`
  - ULID: Unique identifier for the user (must be uppercase)
  - Username: Sanitized username for readability and querying
  - Validation:
    - Username: 3-30 chars, alphanumeric + hyphen, can be both upper or lower case, no spaces.
    - Username uniqueness enforced by backend. Uniqueness ignores case. This means that `johndoe` and `JohnDoe` are considered the same username.
      - In the user key, the username is stored in lowercase, for efficient queries. Example: `johndoe`
      - In the data.username field, the username is stored in the original case. Example: `JohnDoe`
      - One account per identity in production mode
    - Display name: Non-empty, max 50 chars

#### 4.2 Tag Documents

  - Example: `usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW_hdl_technicalskills_`
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

#### 5. Other Optimizations:
- [x] Refactor `assert_doc_user.rs` to extract the username uniqueness check into a reusable function
  - [x] Implement a function (e.g., `check_key_uniqueness(collection: &str, key_pattern: &str) -> Result<bool, String>`) that can be called from anywhere to check for uniqueness of any string in any collection by key search
  - [x] Replace the inline username uniqueness logic in `assert_doc_user.rs` with a call to this new function

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
     // Example key: usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW
     // Benefits:
     // 1. Prefix (usr_, tag_) makes key type immediately identifiable
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
           key: `^usr_${userUlid}_tag_${tagUlid}`  // Prefix match
         }
       }
     });

     // Find all votes for a target in a tag
     const targetVotes = await listDocs({
       collection: "votes",
       filter: {
         matcher: {
           key: `tag_${tagUlid}_tar_${targetUlid}`  // Partial match
         }
       }
     });

     // Combine with pagination and sorting
     const paginatedVotes = await listDocs({
       collection: "votes",
       filter: {
         matcher: {
           key: `usr_${userUlid}`
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

    // this is a bad example, because it is filtering in memory. the key should be matched at the same level as the collection
    async fn find_user_votes(user_ulid: &str, tag_ulid: &str) -> Result<Vec<Vote>, String> {
        let results = list_docs(
            String::from("votes"),
            ListParams {
                matcher: Some(ListMatcher {
                    key: Some(format!("^usr_{}_tag_{}", user_ulid, tag_ulid)),
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

## Implementation Progress

### March-April 2023
- Added ULID libraries to frontend and backend
- Created key formatting utilities
- Implemented document key validation
- Updated document creation flows

### May-June 2023
- Restructured validation system
- Updated API interface for key management
- Created comprehensive tests for key validation
- Added documentation for new key formats

### July-August 2023
- Updated all backend validation for document types
- Implemented key-based search for all documents
- Removed redundant description-based queries
- Added performance optimizations

### September-October 2023 
- Created query helpers for partial key matching
- Updated reputation calculation system to use key-based queries
- Completed transition from description-based to key-based queries
- Fixed code cleanliness issues and enhanced security measures

### Recent Changes
- Implemented `format_reputation_key` for consistent key generation
- Created general-purpose `query_doc_by_key` function to replace specialized query functions
- Fixed type errors in `get_doc_store` calls by passing strings directly instead of references
- Fixed pattern matching inconsistencies when retrieving documents
- Improved error handling with consistent logging
- Removed redundant fields from data structures
- Enhanced type safety with the `VoteWeight` wrapper type

## Next Steps

1. Clean up remaining unused imports and variables
2. Optimize remaining usage of `list_docs` with key-based filtering
3. Monitor performance to ensure the optimizations are effective
4. Document lessons learned for future key-based designs

## Questions to Resolve

1. Retry strategy for reputation documents:
   - How many retries? (Currently set to 3)
   - Delay between retries? (Currently exponential: 100ms -> 200ms -> 400ms)
   - Error handling after max retries?

