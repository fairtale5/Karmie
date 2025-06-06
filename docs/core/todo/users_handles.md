# Users Handles Index Collection Implementation

## **Problem Statement**

Currently, the Reputator system stores usernames directly in the user document keys using this format:
```
usr_{ulid}_hdl_{handle}_
```

This approach has several critical limitations:

### **Issues with Current Approach:**
1. **Immutable Usernames**: Juno document keys cannot be changed after creation, making username changes impossible
2. **Locked-in Architecture**: Once a username is embedded in a key, the user is permanently tied to that handle
3. **No Username Updates**: Users cannot rebrand, fix typos, or change their identity without creating entirely new accounts
4. **Inflexible User Experience**: Other platforms allow username changes - we cannot offer this basic feature

**Note**: The current system is actually very efficient for queries - embedding handles in keys provides O(1) username lookups and excellent performance. The issue isn't with query efficiency (which is great), but rather with the inability to modify usernames after account creation. We cannot use `document.data.user_handle` for queries because that would require loading entire collections, which would be truly inefficient.

### **Solution: Index Collection**

We're implementing a dedicated `users_handles` collection that provides:
- **Fast Username Lookups**: O(1) username existence checks
- **Efficient Conflict Detection**: Direct key-based queries instead of full collection scans
- **Mutable Usernames**: Users can change handles while maintaining data consistency
- **Scalable Architecture**: Index grows linearly and queries remain constant time
- **Simplified Keys**: User documents use clean ULID-only keys
- **Atomic Operations**: Handle changes are managed through Juno's serverless functions

### **New Architecture:**
```
Users Collection:        usr_{ulid}_
Users Handles Collection: hdl_{handle}_usr_{ulid}_
```

This separation allows efficient username operations while maintaining data integrity and enabling future username changes.

---

## Complete Implementation Plan

### **File Structure:**
```
src/satellite/src/
├── on_set_doc/
│   ├── mod.rs                    # Module exports
│   └── users_handles.rs           # Main user handle logic
├── assert_set_doc/
│   ├── mod.rs                   # Add users_handles validation
│   └── assert_doc_users_handles.rs  # Validation logic
├── utils/
│   └── structs.rs               # Add UserHandleDoc struct
└── lib.rs                       # Updated with new imports
```

---

## **1. Main Implementation: `src/satellite/src/on_set_doc/users_handles.rs`**

```rust
/*!
 * User Handle Management for users_handles Collection
 * 
 * This module manages the users_handles index collection that provides efficient
 * username lookups and handle uniqueness enforcement.
 * 
 * Collection: users_handles
 * Key Format: hdl_{userHandle}_usr_{userULID}_
 * Purpose: Fast username lookups and conflict detection
 */

use junobuild_satellite::{OnSetDocContext, set_doc_store, delete_doc_store, SetDoc, DelDoc};
use junobuild_utils::{decode_doc_data, encode_doc_data};
use crate::utils::structs::{UserData, UserHandleDoc};
use crate::processors::document_queries::query_doc_by_key;
use crate::utils::logging::logger;

/// Handles user document updates and manages the users_handles index collection
/// 
/// This function is called after a user document is created or updated.
/// It maintains the users_handles collection for efficient username lookups.
/// 
/// Process:
/// 1. Check if handle changed (early exit if unchanged)
/// 2. Delete old handle documents for this user
/// 3. Create new handle document with updated handle
/// 
/// # Arguments
/// * `context` - The document update context from Juno
/// 
/// # Returns
/// * `Result<(), String>` - Success or detailed error message
pub async fn update_users_handles_index(context: &OnSetDocContext) -> Result<(), String> {
    logger!("debug", "[update_users_handles_index] Starting user handle update processing");
    
    // Decode before and after user data
    let before_data: UserData = decode_doc_data(&context.data.data.before.data)
        .map_err(|e| {
            let err_msg = format!("[update_users_handles_index] Failed to decode before data: {}", e);
            logger!("error", "{}", err_msg);
            err_msg
        })?;
    
    let after_data: UserData = decode_doc_data(&context.data.data.after.data)
        .map_err(|e| {
            let err_msg = format!("[update_users_handles_index] Failed to decode after data: {}", e);
            logger!("error", "{}", err_msg);
            err_msg
        })?;
    
    // 1. Early exit if handle didn't change
    if before_data.user_handle == after_data.user_handle {
        logger!("debug", "[update_users_handles_index] Handle unchanged ({}), skipping users_handles update", 
            after_data.user_handle);
        return Ok(());
    }
    
    logger!("info", "[update_users_handles_index] Handle changed from '{}' to '{}' for user={}", 
        before_data.user_handle, after_data.user_handle, after_data.user_ulid);
    
    let user_ulid = &after_data.user_ulid;
    let old_handle = &before_data.user_handle;
    let new_handle = &after_data.user_handle;
    
    // 2. Query and delete all old handle documents for this user
    // Pattern matches: hdl_{oldHandle}_usr_{userULID}_
    let old_handle_pattern = format!("hdl_{}_usr_{}_", old_handle, user_ulid);
    logger!("debug", "[update_users_handles_index] Querying for old handle documents with pattern: {}", old_handle_pattern);
    
    let existing_handles = query_doc_by_key("users_handles", &old_handle_pattern)
        .map_err(|e| {
            let err_msg = format!("[update_users_handles_index] Failed to query old handle documents: {}", e);
            logger!("error", "{}", err_msg);
            err_msg
        })?;
    
    // Delete all found documents (handles cleanup if multiple exist)
    for (doc_key, doc) in existing_handles.items {
        delete_doc_store(
            context.caller,
            "users_handles".to_string(),
            doc_key.clone(),
            DelDoc { version: doc.version }
        ).map_err(|e| {
            let err_msg = format!("[update_users_handles_index] Failed to delete old handle document {}: {}", doc_key, e);
            logger!("error", "{}", err_msg);
            err_msg
        })?;
        
        logger!("debug", "[update_users_handles_index] Deleted old handle document: {}", doc_key);
    }
    
    // 3. Create new handle document
    let new_handle_key = format!("hdl_{}_usr_{}_", new_handle, user_ulid);
    let handle_doc = UserHandleDoc {
        user_ulid: user_ulid.clone(),
    };
    
    set_doc_store(
        context.caller,
        "users_handles".to_string(),
        new_handle_key.clone(),
        SetDoc {
            data: encode_doc_data(&handle_doc)
                .map_err(|e| {
                    let err_msg = format!("[update_users_handles_index] Failed to encode handle document: {}", e);
                    logger!("error", "{}", err_msg);
                    err_msg
                })?,
            description: None,
            version: None,
        }
    ).map_err(|e| {
        let err_msg = format!("[update_users_handles_index] Failed to create new handle document {}: {}", new_handle_key, e);
        logger!("error", "{}", err_msg);
        err_msg
    })?;
    
    logger!("info", "[update_users_handles_index] Successfully updated handle: old='{}' new='{}' key={}", 
        old_handle, new_handle, new_handle_key);
    
    Ok(())
}
```

---

## **2. Module Export: `src/satellite/src/on_set_doc/mod.rs`**

```rust
/*!
 * On Set Document Hook Modules
 * 
 * This module exports all on_set_doc hook implementations for different collections.
 */

pub mod users_handles;

pub use users_handles::update_users_handles_index;
```

---

## **3. Data Structure: Add to `src/satellite/src/utils/structs.rs`**

```rust
// Add this struct to the existing structs.rs file

/// Document structure for the users_handles collection
/// 
/// This collection provides efficient username lookups and handle uniqueness.
/// Each document maps a username to a user's ULID for fast resolution.
/// 
/// Key Format: hdl_{userHandle}_usr_{userULID}_
/// Purpose: Username -> ULID mapping for efficient queries
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserHandleDoc {
    /// The ULID of the user who owns this handle
    pub user_ulid: String,
}
```

---

## **4. Validation: `src/satellite/src/assert_set_doc/assert_doc_users_handles.rs`**

```rust
/*!
 * Validation logic for users_handles collection documents
 * 
 * This module validates users_handles documents to ensure:
 * - Users can only create handle docs for their own ULID
 * - Handle matches their current user document
 * - No conflicts with existing handles
 * - Users don't have multiple handle documents
 */

use junobuild_satellite::AssertSetDocContext;
use junobuild_utils::decode_doc_data;
use crate::utils::structs::UserHandleDoc;
use crate::utils::logging::logger;

/// Validates users_handles document creation/update
/// 
/// Validation Rules:
/// 1. Users can only create handle docs for their own ULID
/// 2. Handle must match their current user document  
/// 3. Prevent handle conflicts with existing handles
/// 4. Ensure users don't have more than one document in collection
/// 
/// # Arguments
/// * `context` - The validation context from Juno
/// 
/// # Returns
/// * `Result<(), String>` - Success or detailed validation error
pub fn validate_users_handles_document(_context: &AssertSetDocContext) -> Result<(), String> {
    logger!("debug", "[validate_users_handles_document] Starting validation for users_handles document");
    
    // TODO: Implement validation rules:
    // 
    // 1. Extract user ULID from document key format: hdl_{handle}_usr_{ulid}_
    // 2. Verify caller matches the user ULID in the key
    // 3. Decode UserHandleDoc and verify user_ulid matches key
    // 4. Query users collection to verify handle matches user's current handle
    // 5. Query users_handles collection to ensure no conflicts
    // 6. Ensure user doesn't have multiple handle documents
    //    (This blocks write if deletion of old handle failed)
    
    logger!("debug", "[validate_users_handles_document] Validation passed (TODO: implement actual validation)");
    Ok(())
}
```

---

## **5. Update Assert Module: `src/satellite/src/assert_set_doc/mod.rs`**

```rust
// Add to existing mod.rs file
pub mod assert_doc_users_handles;

// Add to existing exports
pub use assert_doc_users_handles::validate_users_handles_document;
```

---

## **6. Updated `src/satellite/src/lib.rs`**

```rust
// Add to module declarations (around line 200)
mod on_set_doc;

// Add to imports (around line 230)
use on_set_doc::update_users_handles_index;
use assert_set_doc::validate_users_handles_document;

// Update the on_set_doc hook (replace existing function around line 260)
#[on_set_doc(collections = ["users", "votes", "tags"])]
async fn on_set_doc(context: OnSetDocContext) -> Result<(), String> {
    logger!("debug", "[on_set_doc] on_set_doc triggered");
    
    match context.data.collection.as_str() {
        "users" => {
            logger!("debug", "[on_set_doc - Users] Processing user update");
            update_users_handles_index(&context).await
        },
        "votes" => {
            logger!("debug", "[on_set_doc - Votes] Processing New Vote");
            process_vote(&context).await
        },
        "tags" => {
            // No side effects needed for tags
            logger!("debug", "No hooks defined for collection: {}", context.data.collection);
            Ok(())
        }
        _ => {
            let err_msg = format!("Unknown collection: {}", context.data.collection);
            logger!("error", "[on_set_doc] {}", err_msg);
            Err(err_msg)
        }
    }
}

// Update assert_set_doc (replace existing function around line 360)
#[assert_set_doc(collections = ["users", "votes", "tags", "reputations", "users_handles"])]
fn assert_set_doc(context: AssertSetDocContext) -> Result<(), String> {
    let result = match context.data.collection.as_str() {
        "users" => {
            logger!("debug", "[assert_set_doc] Validating user document: key={}", context.data.key);
            assert_doc_user(&context)
        },
        "votes" => {
            logger!("debug", "[assert_set_doc] Validating vote document: key={}", context.data.key);
            validate_vote_document(&context)
        },
        "tags" => {
            logger!("debug", "[assert_set_doc] Validating tag document: key={}", context.data.key);
            validate_tag_document(&context)
        },
        "reputations" => {
            logger!("debug", "[assert_set_doc] Validating reputation document: key={}", context.data.key);
            validate_reputation_document(&context)
        },
        "users_handles" => {
            logger!("debug", "[assert_set_doc] Validating users_handles document: key={}", context.data.key);
            validate_users_handles_document(&context)
        },
        _ => {
            let err_msg = format!("Unexpected collection for validation: {}", context.data.collection);
            logger!("error", "[assert_set_doc] {}", err_msg);
            Err(err_msg)
        }
    };
    
    // Log the validation result
    match &result {
        Ok(_) => logger!("info", "[assert_set_doc] Validation passed for {}", context.data.key),
        Err(e) => logger!("error", "[assert_set_doc] Validation failed for {}: {}", context.data.key, e),
    }
    
    result
}
```

---

## **Summary:**
- **Collection Name:** `users_handles`
- **Key Format:** `hdl_{userHandle}_usr_{userULID}_`
- **Optimized Flow:** Early exit if handle unchanged, efficient cleanup, atomic operations
- **File Structure:** Properly organized in backend directory structure
- **Validation:** Placeholder for comprehensive validation rules

---

## **Comprehensive Impact Analysis: Required Changes**

After implementing the `users_handles` index collection, multiple parts of the system need updates to support the new architecture. Here's a complete breakdown:

### **1. Backend Database Schema Changes**

#### **Users Collection Key Format**
```rust
// OLD FORMAT:
usr_{ulid}_hdl_{handle}_

// NEW FORMAT:
usr_{ulid}_
```

**Files Affected:**
- `src/satellite/src/processors/document_keys.rs` - Update `create_user_key()` function
- `src/satellite/src/assert_set_doc/assert_doc_user.rs` - Update key validation logic
- `src/satellite/src/validation/` - All user-related validation functions

#### **Query Pattern Changes**
- All backend queries that rely on handle-in-key patterns need updating
- Replace handle-based key queries with users_handles index lookups

### **2. Backend Helper Functions**

**Files Requiring Updates:**
- `src/satellite/src/processors/document_keys.rs`:
  - `create_user_key()` - Remove handle parameter, use ULID-only format
  - `validate_user_key()` - Update validation for new format
  - `extract_handle_from_user_key()` - DELETE (no longer possible)
  
- `src/satellite/src/processors/document_queries.rs`:
  - Add `query_user_by_handle()` - Query users_handles then lookup user
  - Update any functions that extract handles from user keys

### **3. Backend Validation Changes**

**Files Requiring Updates:**
- `src/satellite/src/assert_set_doc/assert_doc_user.rs`:
  - Remove handle validation from user key format
  - Add validation to check users_handles collection for conflicts
  - Update key format validation logic

- `src/satellite/src/validation/field_validation.rs`:
  - Update username availability checking to use users_handles collection
  - Modify handle uniqueness validation

### **4. Frontend Document Creation & Key Generation**

**Files Requiring Updates:**
- `src/lib/docs-crud/user_create.ts`:
  - Update key generation to use ULID-only format
  - Remove handle from key generation logic

- `src/lib/docs-crud/user_update.ts`:
  - Update to work with new key format
  - Handle username changes properly

- `src/lib/utils/document-keys.ts` (if exists):
  - Update user key generation functions
  - Remove handle-based key logic

### **5. Frontend Onboarding Changes**

**Files Requiring Updates:**
- `src/routes/onboarding/+page.svelte`:
  - Update username availability checking logic
  - Change from key-based queries to users_handles collection queries
  - Modify `checkUsername()` function to query new collection

- `src/lib/docs-crud/query_by_key.ts`:
  - Add helper for username availability checking
  - Create `checkUsernameAvailability()` function

### **6. Frontend Profile & User Lookup**

**Files Requiring Updates:**
- `src/routes/u/[handle]/+page.svelte`:
  - Update user lookup logic to use users_handles index
  - Change from direct key queries to two-step lookup process

- `src/routes/u/[handle]/+page.ts`:
  - Update server-side user lookup logic
  - Implement users_handles → users collection lookup

### **7. Tag Collection (Future Implementation)**

**Similar Changes Required:**
- Create `tags_handles` collection with format: `hdl_{tagName}_tag_{tagULID}_`
- Update tag keys from `usr_{userUlid}_tag_{tagUlid}_hdl_{handle}_` to `usr_{userUlid}_tag_{tagUlid}_`
- Create `src/satellite/src/on_set_doc/tags_handles.rs`
- Create `src/satellite/src/assert_set_doc/assert_doc_tags_handles.rs`
- Update all tag-related frontend and backend code similarly

### **8. Database Migration Requirements**

**Migration Strategy:**
1. **Phase 1**: Deploy users_handles collection support
2. **Phase 2**: Create migration script to populate users_handles from existing user documents
3. **Phase 3**: Update frontend to use new lookup methods
4. **Phase 4**: Update user key format (breaking change - requires careful coordination)
5. **Phase 5**: Clean up old code and helper functions

### **9. Testing & Validation**

**Test Files Requiring Updates:**
- All user creation/update tests
- Username validation tests  
- Profile lookup tests
- Onboarding flow tests

### **10. Documentation Updates**

#### **Database Schema Documentation** (`docs/core/architecture/database.md`)
- **Users Collection**: Update key format from `usr_{ulid}_hdl_{handle}_` to `usr_{ulid}_`
- **New users_handles Collection**: Add complete schema documentation:
  ```typescript
  // Collection: users_handles
  // Key Format: hdl_{userHandle}_usr_{userULID}_
  // Purpose: Username → ULID mapping for efficient queries
  interface UserHandleDocument {
      key: string;        // hdl_{handle}_usr_{ulid}_
      data: {
          user_ulid: string;  // ULID of the user who owns this handle
      }
  }
  ```
- **Tags Collection**: Update key format from `usr_{userUlid}_tag_{tagUlid}_hdl_{handle}_` to `usr_{userUlid}_tag_{tagUlid}_`
- **New tags_handles Collection**: Add documentation for future implementation
- **Query Performance Examples**: Update to show new lookup patterns
- **Memory Usage Patterns**: Document two-step lookup performance characteristics

#### **Backend Type Definitions** (`src/satellite/src/utils/structs.rs`)
- **Add New Structs**:
  ```rust
  /// Document structure for users_handles collection
  #[derive(Serialize, Deserialize, Clone, Debug)]
  pub struct UserHandleDocument {
      pub key: String,
      pub description: String,
      pub owner: Principal,
      pub created_at: u64,
      pub updated_at: u64,
      pub version: u64,
      pub data: UserHandleData,
  }
  
  #[derive(Serialize, Deserialize, Clone, Debug)]
  pub struct UserHandleData {
      pub user_ulid: String,
  }
  
  /// Future: TagHandleDocument and TagHandleData
  ```
- **Update Existing Structs**: Remove any references to handle-in-key logic
- **Add Documentation**: Explain relationship between main collections and handle indexes

#### **Frontend Type Definitions** (`src/lib/types.ts`)
- **Add New Interfaces**:
  ```typescript
  /** users_handles collection document */
  export interface UserHandleData {
      user_ulid: string;  // ULID of the user who owns this handle
  }
  export type UserHandleDocument = Doc<UserHandleData>;
  
  /** Future: TagHandleData and TagHandleDocument */
  ```
- **Update JSDoc**: Explain new handle lookup architecture
- **Add Examples**: Show how to perform username availability checks and user lookups

#### **API Reference Documentation** (`docs/resources/ic_and_juno_api_reference.md`)
- **Update Query Examples**: Replace old key-based patterns with users_handles lookups
- **Add Handle Management Patterns**: Document username availability checking
- **Update Error Handling**: Document new validation error types
- **Migration Examples**: Show before/after code patterns

#### **Implementation Documentation** (`docs/implementation/`)
- **User Management Guide**: Update with new handle architecture
- **Migration Strategy**: Document step-by-step migration process
- **Performance Considerations**: Document lookup performance changes
- **Troubleshooting**: Add common issues with handle management

#### **Integration Guides**
- **Frontend Integration**: Update examples to use new lookup patterns
- **Backend Integration**: Update server-side code examples
- **Testing Guides**: Update test patterns for new architecture

### **Critical Considerations:**

1. **Breaking Changes**: The key format change is breaking - requires coordinated deployment
2. **Data Migration**: Existing user documents need users_handles entries created
3. **Rollback Strategy**: Plan for reverting if issues arise during migration
4. **Performance Impact**: Two-step lookups (handle → ULID → user) vs direct key access
5. **Consistency**: Ensure users_handles index stays in sync with users collection

### **Recommended Implementation Order:**

1. ✅ Implement users_handles collection and hooks (current task)
2. 🔄 Update backend helper functions for dual support
3. 🔄 Update frontend to use users_handles for lookups
4. 🔄 Create migration script for existing data
5. 🔄 Update user key format (coordinated deployment)
6. 🔄 Remove old code and cleanup
7. 🔄 Implement same pattern for tags_handles
8. 🔄 Full system testing and validation
