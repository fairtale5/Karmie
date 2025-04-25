# Transition Plan: Description Validation to Key Validation

## Overview

This document outlines the strategy for transitioning our validation approach from using the `description` field to using the `key` field, based on the new approaches in `docs/core/architecture/database.md` and `docs/core/todo/keys_and_queries_optimization.md`.

## File Renaming Strategy

### Description Helpers to Key Validators

| Current File | New File | Notes |
|--------------|----------|-------|
| `src/satellite/src/utils/description_helpers.rs` | `src/satellite/src/utils/key_validators.rs` | Main utility file for key validation |

## Function Refactoring

### 1. Key Structure and Validators

#### Current `DocumentDescription` Class
Currently provides methods for creating, building, and validating descriptions.

#### New `DocumentKey` Class
Will provide methods for creating, validating, and parsing structured keys.

```rust
pub struct DocumentKey {
    segments: Vec<(String, String)>,  // (prefix, value) pairs
}

impl DocumentKey {
    /// Creates a new empty key
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    /// Adds a segment to the key
    pub fn add_segment(&mut self, prefix: &str, value: &str) -> &mut Self {
        self.segments.push((prefix.to_string(), value.to_string()));
        self
    }

    /// Adds a user ULID segment
    pub fn add_user_ulid(&mut self, ulid: &str) -> &mut Self {
        self.add_segment("usr", ulid)
    }

    /// Adds a tag ULID segment
    pub fn add_tag_ulid(&mut self, ulid: &str) -> &mut Self {
        self.add_segment("tag", ulid)
    }

    /// Adds a target ULID segment
    pub fn add_target_ulid(&mut self, ulid: &str) -> &mut Self {
        self.add_segment("tar", ulid)
    }

    /// Adds a vote ULID segment
    pub fn add_vote_ulid(&mut self, ulid: &str) -> &mut Self {
        self.add_segment("key", ulid)
    }

    /// Adds a username segment
    pub fn add_username(&mut self, username: &str) -> &mut Self {
        let sanitized = self.sanitize_key(username).to_lowercase();
        self.add_segment("usrName", &sanitized)
    }

    /// Adds a tag name segment
    pub fn add_tagname(&mut self, tagname: &str) -> &mut Self {
        let sanitized = self.sanitize_key(tagname).to_lowercase();
        self.add_segment("tagName", &sanitized)
    }

    /// Builds the key string
    pub fn build(&self) -> String {
        let mut result = String::new();
        for (i, (prefix, value)) in self.segments.iter().enumerate() {
            result.push_str(&format!("{}_{}", prefix, value));
            if i < self.segments.len() - 1 {
                result.push('_');
            }
        }
        result
    }

    /// Parses a key string into segments
    pub fn parse(key: &str) -> Result<Self, String> {
        // Implementation
    }

    /// Gets a segment value by prefix
    pub fn get_segment(&self, prefix: &str) -> Option<&str> {
        // Implementation
    }

    /// Sanitizes a key by removing special characters
    pub fn sanitize_key(&self, key: &str) -> String {
        // Same implementation as current DocumentDescription::sanitize_key
    }
}
```

### 2. Key Creation Functions

Rename and update the following key creation functions:

| Current Function | New Function |
|------------------|--------------|
| `create_user_description` | `create_user_key` |
| `create_tag_description` | `create_tag_key` |
| `create_vote_description` | `create_vote_key` |
| `create_reputation_description` | `create_reputation_key` |

#### Example Implementation:

```rust
/// Creates a document key for a user document
/// 
/// # Document Structure
/// ```
/// key: String,         // Format: usr_{ulid}_usrName_{username}_
/// data: {
///     username: String,  // User's username
///     display_name: String,
///     usr_key: ULID     // Pure ULID for references
/// }
/// ```
pub fn create_user_key(user: &User, owner: &Principal, is_playground: bool) -> String {
    let mut key = DocumentKey::new();
    key.add_user_ulid(&user.data.usr_key)
       .add_username(&user.data.username);
    
    // Add trailing underscore for consistency
    format!("{}_", key.build())
}
```

### 3. Name Validation Functions

Retain and enhance the name validation functions from the current code. These functions will now be used during key validation as well as data validation.

```rust
/// Validates a username
/// 
/// Rules:
/// - Length: 3-30 characters
/// - Allowed characters: alphanumeric, hyphen
/// - Must be unique across all users (case-insensitive)
/// 
/// # Arguments
/// * `username` - The username to validate
/// 
/// # Returns
/// * `Result<(), String>` - Ok if validation passes, Err with message if it fails
pub fn validate_username(username: &str) -> Result<(), String> {
    // Check length
    if username.len() < 3 || username.len() > 30 {
        return Err(format!(
            "Username must be between 3 and 30 characters (got: {})",
            username.len()
        ));
    }
    
    // Check allowed characters using regex
    let username_pattern = Regex::new(r"^[a-zA-Z0-9-_]+$").unwrap();
    if !username_pattern.is_match(username) {
        return Err(String::from(
            "Username can only contain letters, numbers, hyphens and underscores"
        ));
    }
    
    // Uniqueness is checked separately with key-based queries
    
    Ok(())
}

/// Validates a tag name
/// 
/// Rules:
/// - Length: 3-30 characters
/// - Allowed characters: alphanumeric, hyphen
/// - Must be unique across all tags (case-insensitive)
/// - No spaces allowed
/// 
/// # Arguments
/// * `tag_name` - The tag name to validate
/// 
/// # Returns
/// * `Result<(), String>` - Ok if validation passes, Err with message if it fails
pub fn validate_tag_name(tag_name: &str) -> Result<(), String> {
    // Check length
    if tag_name.len() < 3 || tag_name.len() > 30 {
        return Err(format!(
            "Tag name must be between 3 and 30 characters (got: {})",
            tag_name.len()
        ));
    }
    
    // Check allowed characters using regex
    let tag_pattern = Regex::new(r"^[a-zA-Z0-9-_]+$").unwrap();
    if !tag_pattern.is_match(tag_name) {
        return Err(String::from(
            "Tag name can only contain letters, numbers, hyphens and underscores"
        ));
    }
    
    // Check for spaces
    if tag_name.contains(" ") {
        return Err(String::from("Tag name cannot contain spaces"));
    }
    
    // Uniqueness is checked separately with key-based queries
    
    Ok(())
}

/// Validates a display name
/// 
/// Rules:
/// - Non-empty after trimming
/// - Maximum length: 100 characters
/// - No character restrictions
/// 
/// # Arguments
/// * `display_name` - The display name to validate
/// 
/// # Returns
/// * `Result<(), String>` - Ok if validation passes, Err with message if it fails
pub fn validate_display_name(display_name: &str) -> Result<(), String> {
    let trimmed = display_name.trim();
    
    // Check non-empty
    if trimmed.is_empty() {
        return Err(String::from("Display name cannot be empty"));
    }
    
    // Check length
    if trimmed.len() > 100 {
        return Err(format!(
            "Display name cannot exceed 100 characters (got: {})",
            trimmed.len()
        ));
    }
    
    Ok(())
}

/// Checks uniqueness of username using key-based query
/// 
/// # Arguments
/// * `username` - The username to check
/// * `current_key` - The current document key (for updates)
/// 
/// # Returns
/// * `Result<(), String>` - Ok if unique, Err with message if not unique
pub async fn check_username_uniqueness(username: &str, current_key: Option<&str>) -> Result<(), String> {
    // Sanitize and lowercase the username
    let sanitized = DocumentKey::sanitize_key(username).to_lowercase();
    
    // Build the key pattern to search for username in keys
    let pattern = format!("usrName_{}_", sanitized);
    
    let params = ListParams {
        matcher: Some(ListMatcher {
            key: Some(pattern),
            ..Default::default()
        }),
        ..Default::default()
    };

    let docs = list_docs(String::from("users"), params).await?;
    
    // For updates, exclude the current document
    if let Some(current) = current_key {
        if docs.items.iter().any(|(key, _)| key != current) {
            return Err(format!("Username '{}' is already taken", username));
        }
    } else if !docs.items.is_empty() {
        // For new documents, any match means the username is taken
        return Err(format!("Username '{}' is already taken", username));
    }
    
    Ok(())
}

/// Checks uniqueness of tag name using key-based query
/// 
/// # Arguments
/// * `tag_name` - The tag name to check
/// * `current_key` - The current document key (for updates)
/// 
/// # Returns
/// * `Result<(), String>` - Ok if unique, Err with message if not unique
pub async fn check_tag_name_uniqueness(tag_name: &str, current_key: Option<&str>) -> Result<(), String> {
    // Sanitize and lowercase the tag name
    let sanitized = DocumentKey::sanitize_key(tag_name).to_lowercase();
    
    // Build the key pattern to search for tag name in keys
    let pattern = format!("tagName_{}_", sanitized);
    
    let params = ListParams {
        matcher: Some(ListMatcher {
            key: Some(pattern),
            ..Default::default()
        }),
        ..Default::default()
    };

    let docs = list_docs(String::from("tags"), params).await?;
    
    // For updates, exclude the current document
    if let Some(current) = current_key {
        if docs.items.iter().any(|(key, _)| key != current) {
            return Err(format!("Tag name '{}' is already taken", tag_name));
        }
    } else if !docs.items.is_empty() {
        // For new documents, any match means the tag name is taken
        return Err(format!("Tag name '{}' is already taken", tag_name));
    }
    
    Ok(())
}
```

### 4. Key Validation Functions

Rename and update the validation function:

| Current Function | New Function |
|------------------|--------------|
| `validate_description` | `validate_key` |

#### Example Implementation:

```rust
/// Validates a key string against expected format and referenced documents
pub async fn validate_key(collection: &str, key: &str) -> Result<(), String> {
    // Step 1: Validate format using regex
    let pattern = match collection {
        "users" => &*USER_KEY_PATTERN,
        "tags" => &*TAG_KEY_PATTERN,
        "votes" => &*VOTE_KEY_PATTERN,
        "reputations" => &*REP_KEY_PATTERN,
        _ => return Err(format!("Unknown collection: {}", collection))
    };

    if !pattern.is_match(key) {
        return Err(format!("Invalid key format for {}: {}", collection, key));
    }

    // Step 2: Extract and validate ULIDs
    let parsed_key = DocumentKey::parse(key)?;
    
    // Validate all ULIDs in the key
    for prefix in ["usr", "tag", "tar", "key"].iter() {
        if let Some(ulid_value) = parsed_key.get_segment(prefix) {
            if !validate_ulid(ulid_value) {
                return Err(format!("Invalid ULID in segment {}: {}", prefix, ulid_value));
            }
        }
    }
    
    // Step 3: Validate username or tag name if present
    if collection == "users" {
        if let Some(username) = parsed_key.get_segment("usrName") {
            // Validate username format
            validate_username(username)?;
        }
    } else if collection == "tags" {
        if let Some(tag_name) = parsed_key.get_segment("tagName") {
            // Validate tag name format
            validate_tag_name(tag_name)?;
        }
    }

    // Step 4: Validate referenced documents exist
    match collection {
        "votes" => {
            // Check user, tag, and target exist
            let usr_ulid = parsed_key.get_segment("usr")
                .ok_or_else(|| "Missing usr segment in vote key".to_string())?;
                
            let tag_ulid = parsed_key.get_segment("tag")
                .ok_or_else(|| "Missing tag segment in vote key".to_string())?;
                
            let tar_ulid = parsed_key.get_segment("tar")
                .ok_or_else(|| "Missing tar segment in vote key".to_string())?;
                
            // Verify referenced documents exist
            // This will use key-based lookup instead of description-based
            validate_referenced_document("users", usr_ulid).await?;
            validate_referenced_document("tags", tag_ulid).await?;
            validate_referenced_document("users", tar_ulid).await?;
        },
        "reputations" => {
            // Check user and tag exist
            let usr_ulid = parsed_key.get_segment("usr")
                .ok_or_else(|| "Missing usr segment in reputation key".to_string())?;
                
            let tag_ulid = parsed_key.get_segment("tag")
                .ok_or_else(|| "Missing tag segment in reputation key".to_string())?;
                
            // Verify referenced documents exist
            validate_referenced_document("users", usr_ulid).await?;
            validate_referenced_document("tags", tag_ulid).await?;
        },
        _ => {}
    }

    Ok(())
}

/// Helper function to validate a referenced document exists
async fn validate_referenced_document(collection: &str, ulid: &str) -> Result<(), String> {
    // Implementation based on key search instead of description search
    // This will search for documents where the key contains the given ULID
    let params = ListParams {
        matcher: Some(ListMatcher {
            key: Some(format!("{}_{}", match collection {
                "users" => "usr",
                "tags" => "tag",
                _ => return Err(format!("Unsupported collection for reference: {}", collection))
            }, ulid)),
            ..Default::default()
        }),
        ..Default::default()
    };

    let docs = list_docs(collection.to_string(), params).await?;
    if docs.items.is_empty() {
        return Err(format!("Referenced {} document not found: {}", collection, ulid));
    }

    Ok(())
}
```

### 5. Comprehensive Document Validation

The complete document validation will combine both key and data validation:

```rust
/// Validates a user document before creation or update
pub async fn validate_user_document(context: &AssertSetDocContext) -> Result<(), String> {
    // Step 1: Validate key format
    validate_key("users", &context.data.key).await?;
    
    // Step 2: Decode and validate user data
    let user_data: UserData = decode_doc_data(&context.data.data.proposed.data)
        .map_err(|e| format!("Invalid user data format: {}", e))?;
    
    // Step 3: Validate username format (already done in key validation, but still validate data)
    validate_username(&user_data.username)?;
    
    // Step 4: Validate display name
    validate_display_name(&user_data.display_name)?;
    
    // Step 5: Check username uniqueness (using the key field)
    let is_update = context.data.data.before.is_some();
    check_username_uniqueness(
        &user_data.username,
        if is_update { Some(&context.data.key) } else { None }
    ).await?;
    
    // Step 6: Validate ULID in data matches ULID in key
    let parsed_key = DocumentKey::parse(&context.data.key)?;
    if let Some(key_ulid) = parsed_key.get_segment("usr") {
        if user_data.usr_key != key_ulid {
            return Err(format!(
                "ULID mismatch: '{}' in key does not match '{}' in data",
                key_ulid, user_data.usr_key
            ));
        }
    }
    
    Ok(())
}
```

### 5. Regex Pattern Updates

Update regex patterns to match key formats:

```rust
lazy_static! {
    // Updated regex patterns for the new key format
    static ref USER_KEY_PATTERN: Regex = Regex::new(r"^usr_[0-9A-Z]{26}_usrName_[a-z0-9_]+_$").unwrap();
    static ref TAG_KEY_PATTERN: Regex = Regex::new(r"^usr_[0-9A-Z]{26}_tag_[0-9A-Z]{26}_tagName_[a-z0-9_]+_$").unwrap();
    static ref VOTE_KEY_PATTERN: Regex = Regex::new(r"^usr_[0-9A-Z]{26}_tag_[0-9A-Z]{26}_tar_[0-9A-Z]{26}_key_[0-9A-Z]{26}_$").unwrap();
    static ref REP_KEY_PATTERN: Regex = Regex::new(r"^usr_[0-9A-Z]{26}_tag_[0-9A-Z]{26}$").unwrap();
}
```

## Implementation Plan

### Phase 1: Preparation

1. Create backup of current validation code
2. Update ULID validation and utility functions
3. Create new key format validation regex patterns
4. Write unit tests for new key validation functions

### Phase 2: File Renaming and Function Updates

1. Rename `description_helpers.rs` to `key_validators.rs`
2. Update `DocumentDescription` to `DocumentKey` class
3. Rename and reimplement all description creation functions to key creation functions
4. Rename and reimplement `validate_description` to `validate_key`
5. Update unit tests for all renamed functions

### Phase 3: Backend Integration

1. Update import statements in all files that use the description helpers
2. Update `lib.rs` to use the new key validation functions
3. Update `assert_set_doc` to validate keys instead of descriptions
4. Update document creation functions to use key validation
5. Run comprehensive tests on backend validations

### Phase 4: Frontend Integration

1. Update frontend key generation to match backend expectations
2. Update document creation to include proper key formats
3. Update query patterns to use key-based search
4. Test frontend-backend integration

## API Changes

### Old API (Description-based):

```rust
// Creating a description
let description = create_user_description(&user, &owner, is_playground);

// Validating a description
validate_description("users", &description, &document_key).await?;

// Using DocumentDescription class
let mut desc = DocumentDescription::new();
desc.add_owner(&owner_str)
    .add_field("username", &sanitized_username);
let description = desc.build();
```

### New API (Key-based):

```rust
// Creating a key
let key = create_user_key(&user, &owner, is_playground);

// Validating a key
validate_key("users", &key).await?;

// Using DocumentKey class
let mut key = DocumentKey::new();
key.add_user_ulid(&user.data.usr_key)
   .add_username(&user.data.username);
let document_key = format!("{}_", key.build());
```

## Testing Strategy

1. Unit tests for individual components:
   - ULID validation
   - Key format validation
   - Referenced document validation

2. Integration tests:
   - Document creation with proper keys
   - Document updates with key validation
   - Query operations using key-based search

3. System tests:
   - End-to-end validation of key-based operations
   - Performance comparison with description-based approaches

## Rollback Plan

1. Keep old description_helpers.rs file as backup
2. Implement feature flag to toggle between description and key validation
3. Monitor system performance and error rates after deployment
4. Maintain compatibility with existing documents during transition

## Migration Strategy for Existing Documents

For existing documents, we'll keep the ability to process both description-based and key-based formats during the transition period:

1. For read operations: 
   - Try key-based lookup first
   - Fall back to description-based if key lookup fails

2. For write operations:
   - Always use new key format for new documents
   - Validate existing documents using their original format

## Timeline and Milestones

1. Phase 1 (Preparation): 1 day
   - Create backups and initial implementation of key validators

2. Phase 2 (File Renaming): 1-2 days
   - Rename files and update core functions

3. Phase 3 (Backend Integration): 2-3 days
   - Update backend to use new validation approach

4. Phase 4 (Frontend Integration): 2-3 days
   - Update frontend to use key-based operations

Total Estimated Time: 6-9 days 