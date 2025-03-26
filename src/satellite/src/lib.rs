/*!
 * Juno Satellite Implementation for Reputator
 * 
 * This module implements the core functionality for the Reputator satellite,
 * focusing on comprehensive data validation and reputation management.
 * 
 * # Available Features
 * - Document Management (create, update, delete)
 * - Asset Management (upload, delete)
 * - Custom Validation Logic
 * - Event Hooks
 * 
 * # Data Validation Architecture
 * The validation system follows a two-phase approach:
 * 
 * 1. Pre-Creation Validation (`assert_set_doc`):
 *    - Runs BEFORE document creation/update
 *    - Validates all data constraints
 *    - Enforces business rules
 *    - Prevents invalid data from being stored
 *    - Collection-specific validation:
 *      - Users: Username format, uniqueness, one account per identity
 *      - Votes: Value constraints, weight limits, no self-voting
 *      - Tags: Name requirements, time periods, reputation settings
 *      - Reputations: Score constraints, weight limits
 * 
 * 2. Post-Creation Processing (`on_set_doc`):
 *    - Runs AFTER successful document creation/update
 *    - Handles data processing and updates:
 *      - For votes: Recalculates reputation scores
 *      - For users/tags: No additional processing needed
 *    - Updates related documents as needed
 *    - Maintains system consistency
 * 
 * # Playground vs Production Mode
 * 
 * The system operates in two modes, controlled by `IS_PLAYGROUND`:
 * 
 * Playground Mode (`IS_PLAYGROUND = true`):
 * - Single user creates all documents
 * - Uses document key in description for ownership
 * - Relaxed validation for testing
 * - Format: [owner:{key}],[field:{value}]
 * 
 * Production Mode (`IS_PLAYGROUND = false`):
 * - Each user creates their own documents
 * - Uses Juno's Principal ID for ownership
 * - Strict validation rules
 * - Format: [field:{value}]
 * 
 * # Document Formats
 * 
 * Each collection uses specific description formats:
 * 
 * Users Collection:
 * ```text
 * [owner:{key}],[username:{normalized_username}]
 * ```
 * 
 * Votes Collection:
 * ```text
 * [owner:{id}],[author:{key}],[target:{key}],[tag:{key}]
 * ```
 * 
 * Tags Collection:
 * ```text
 * [owner:{id}],[name:{normalized_name}]
 * ```
 * 
 * Reputations Collection:
 * ```text
 * [owner:{id}],[user:{key}],[tag:{key}]
 * ```
 * 
 * # Logging Standards
 * 
 * This module uses structured logging with consistent prefixes:
 * 
 * - Error Format:
 *   ```text
 *   [ERROR] [Function - Collection] Detailed message with context
 *   ```
 * 
 * - Info Format:
 *   ```text
 *   [INFO] [Function - Collection] Operation result with key data
 *   ```
 * 
 * - Debug Format:
 *   ```text
 *   [DEBUG] [Function - Collection] Step-by-step operation tracking
 *   ```
 * 
 * See utils/logging.rs for the complete logging system documentation.
 */

// =============================================================================
// AVAILABLE IMPORTS FROM junobuild_macros:
// These are the only macro decorators available:
// =============================================================================
// These imports are kept as reference for all available macro decorators.
// Currently we only use on_set_doc and assert_set_doc, but others are
// documented for future use.

// Import all available macro decorators from junobuild_macros
#[allow(unused_imports)]
use junobuild_macros::{
    assert_delete_asset,   // For asserting asset deletion
    assert_delete_doc,     // For asserting document deletion
    assert_set_doc,        // For asserting document creation/update
    assert_upload_asset,   // For asserting asset upload
    on_delete_asset,       // For handling asset deletion
    on_delete_doc,         // For handling document deletion
    on_delete_filtered_assets,  // For handling filtered asset deletion
    on_delete_filtered_docs,    // For handling filtered document deletion
    on_delete_many_assets,      // For handling batch asset deletion
    on_delete_many_docs,        // For handling batch document deletion
    on_set_doc,                 // For handling document creation/update
    on_set_many_docs,           // For handling batch document creation/update
    on_upload_asset,            // For handling asset upload
};

// =============================================================================
// AVAILABLE IMPORTS FROM junobuild_satellite:
// These are the only context types and utilities available:
// =============================================================================
// These imports provide the necessary types and utilities for working with
// Juno's satellite features.

#[allow(unused_imports)]
use junobuild_satellite::{
    include_satellite,           // Required macro for Juno integration
    AssertDeleteAssetContext,    // Context for asset deletion assertion
    AssertDeleteDocContext,      // Context for document deletion assertion
    AssertSetDocContext,         // Context for document creation/update assertion
    AssertUploadAssetContext,    // Context for asset upload assertion
    OnDeleteAssetContext,        // Context for asset deletion handler
    OnDeleteDocContext,          // Context for document deletion handler
    OnDeleteFilteredAssetsContext,  // Context for filtered asset deletion
    OnDeleteFilteredDocsContext,    // Context for filtered document deletion
    OnDeleteManyAssetsContext,      // Context for batch asset deletion
    OnDeleteManyDocsContext,        // Context for batch document deletion
    OnSetDocContext,                // Context for document creation/update
    OnSetManyDocsContext,           // Context for batch document creation/update
    OnUploadAssetContext,           // Context for asset upload handler
};

// =============================================================================
// Juno Shared Types
// =============================================================================
// Types for working with Juno's list functionality

use junobuild_shared::types::list::{ListMatcher, ListParams};
use ic_cdk_macros::*;

// IMPORTANT NOTE:
// Any additional functionality needed (like data serialization, string manipulation, etc.)
// must be either:
// 1. Imported from external crates (e.g., serde for serialization)
// 2. Implemented manually in our codebase
// 3. Or imported from junobuild_utils if available (needs to be verified)

// For example, we'll need to add:
// - serde for data serialization
// - Our own utility functions for things like username normalization
// - Any additional helper functions we need for our business logic

// All the available hooks and assertions for your Datastore and Storage are scaffolded by default in this `lib.rs` module.
// However, if you don't have to implement all of them, for example to improve readability or reduce unnecessary logic,
// you can selectively enable only the features you need.
//
// To do this, disable the default features in your `Cargo.toml` and explicitly specify only the ones you want to use.
//
// For example, if you only need `on_set_doc`, configure your `Cargo.toml` like this:
//
// [dependencies]
// junobuild-satellite = { version = "0.0.22", default-features = false, features = ["on_set_doc"] }
//
// With this setup, only `on_set_doc` must be implemented with custom logic,
// and other hooks and assertions can be removed. They will not be included in your Satellite.

//===========================================================================
// Utility Imports
// ===========================================================================

#[allow(unused_imports)]
use junobuild_utils::{decode_doc_data, encode_doc_data};

// Import our utility modules
use crate::utils::{
    normalize::normalize_username,
    validation::{validate_username, validate_display_name, validate_tag_name},
    structs::{Vote, Tag, Reputation, UserData, TagData, TimePeriod, ReputationData},
    reputation_calculations::{
        calculate_user_reputation, get_user_reputation_data,
        calculate_and_store_vote_weight
    },
    logging::{log_error, log_warn, log_info, log_debug},
    description_helpers::{DocumentDescription, create_vote_description, validate_description}
};

// =============================================================================
// Module Declarations
// =============================================================================

mod utils;

// =============================================================================
// Active Hooks and Assertions
// =============================================================================

/// Handles document updates for users, votes, and tags collections
/// 
/// This function is called by Juno AFTER a document is created or updated.
/// It handles side effects that should occur after successful document creation:
/// - For votes: Triggers reputation recalculation
/// - For users/tags: No side effects needed
/// 
/// Note: This function does NOT handle validation. All validation is done in assert_set_doc
/// which runs BEFORE document creation.
#[on_set_doc(collections = ["users", "votes", "tags"])]
async fn on_set_doc(context: OnSetDocContext) -> Result<(), String> {
    match context.data.collection.as_str() {
        "votes" => {
            log_debug("[on_set_doc - Votes] Processing new vote");
            
            // Decode the vote data
            let vote: Vote = decode_doc_data(&context.data.data.after.data)
                .map_err(|e| {
                    log_error(&format!("[on_set_doc - Votes] Failed to decode vote data: {}", e));
                    e.to_string()
                })?;
            
            // Step 1: Calculate and store the voting user's vote weight
            log_debug(&format!("[on_set_doc - Votes] Calculating vote weight for author: {}", vote.data.author_key));
            calculate_and_store_vote_weight(&vote.data.author_key, &vote.data.tag_key).await
                .map_err(|e| {
                    log_error(&format!("[on_set_doc - Votes] Failed to calculate vote weight: {}", e));
                    e.to_string()
                })?;
            
            // Step 2: Calculate reputation for the voting user (author)
            log_debug(&format!("[on_set_doc - Votes] Calculating reputation for author: {}", vote.data.author_key));
            calculate_user_reputation(&vote.data.author_key, &vote.data.tag_key).await
                .map_err(|e| {
                    log_error(&format!("[on_set_doc - Votes] Failed to calculate author reputation: {}", e));
                    e.to_string()
                })?;
            
            // Step 3: Calculate target's reputation
            log_debug(&format!("[on_set_doc - Votes] Calculating reputation for target: {}", vote.data.target_key));
            calculate_user_reputation(&vote.data.target_key, &vote.data.tag_key).await
                .map_err(|e| {
                    log_error(&format!("[on_set_doc - Votes] Failed to calculate target reputation: {}", e));
                    e.to_string()
                })?;

            log_info(&format!(
                "[on_set_doc - Votes] Successfully processed vote: author={}, target={}, tag={}",
                vote.data.author_key, vote.data.target_key, vote.data.tag_key
            ));
        }
        "users" | "tags" => {
            // No side effects needed for users or tags
            log_debug(&format!("[on_set_doc] No side effects needed for collection: {}", context.data.collection));
        }
        _ => {
            log_error(&format!("[on_set_doc] Unknown collection: {}", context.data.collection));
            return Err(format!("Unknown collection: {}", context.data.collection));
        }
    }
    
    Ok(())
}

/// Configuration flag for playground mode
pub const IS_PLAYGROUND: bool = true;  // Set to false for production

#[cfg(not(feature = "assert_set_doc"))]
fn assert_set_doc(context: AssertSetDocContext) -> Result<(), String> {
    ic_cdk::println!("[CRITICAL DEBUG] assert_set_doc CALLED for collection: {}, key: {}", 
        context.data.collection, context.data.key);
    
    log_debug(&format!("[assert_set_doc] Starting validation for collection: {}, key: {}", 
        context.data.collection, context.data.key));

    let result = match context.data.collection.as_str() {
        "users" => validate_user_document(&context),
        "votes" => validate_vote_document(&context),
        "tags" => validate_tag_document(&context),
        "reputations" => validate_reputation_document(&context),
        _ => {
            let err_msg = format!("Unknown collection: {}", context.data.collection);
            log_error(&format!("[assert_set_doc] {}", err_msg));
            Err(err_msg)
        }
    };
    
    if result.is_err() {
        ic_cdk::println!("[CRITICAL DEBUG] assert_set_doc FAILED with error: {:?}", result.as_ref().err());
    } else {
        ic_cdk::println!("[CRITICAL DEBUG] assert_set_doc PASSED validation");
    }
    
    result
}

/// Validates a user document before creation or update
/// 
/// This function performs comprehensive validation of user documents:
/// 1. Decodes and validates the basic user data structure
/// 2. Validates username format and restrictions
/// 3. Validates display name format and restrictions
/// 4. Validates description format and referenced documents
/// 5. Ensures username uniqueness across the system
/// 6. Enforces one-document-per-identity rule in production mode
/// 
/// # Arguments
/// * `context` - The validation context containing:
///   - caller: The Principal ID of the user making the request
///   - collection: Must be "users"
///   - key: The document key (nanoid-generated)
///   - data: The proposed document data
/// 
/// # Returns
/// * `Result<(), String>` - Ok if validation passes, Err with detailed message if it fails
fn validate_user_document(context: &AssertSetDocContext) -> Result<(), String> {
    // Step 1: Decode and validate the basic user data structure
    // This ensures the document contains all required fields in the correct format
    let user_data: UserData = decode_doc_data(&context.data.data.proposed.data)
        .map_err(|e| {
            let err_msg = format!("[assert_set_doc] Failed to decode user data: key={}, error={}", 
                context.data.key, e);
            log_error(&err_msg);
            format!("Invalid user data format: {}", e)
        })?;
    
    // Step 2: Validate username format and restrictions
    validate_username(&user_data.username)
        .map_err(|e| {
            let err_msg = format!("[assert_set_doc] Username validation failed: {}", e);
            log_error(&err_msg);
            e
        })?;

    // Step 3: Validate display name format and restrictions
    validate_display_name(&user_data.display_name)
        .map_err(|e| {
            let err_msg = format!("[assert_set_doc] Display name validation failed: {}", e);
            log_error(&err_msg);
            e
        })?;

    // Step 4: Validate description format and referenced documents
    if let Some(description) = &context.data.data.proposed.description {
        // Since validate_description is async, we'll validate the format synchronously
        // and leave the document existence check to the on_set_doc hook
        let mut desc = DocumentDescription::new();
        if IS_PLAYGROUND {
            desc.add_owner(&context.data.key);
        } else {
            desc.add_owner(&context.caller.to_string());
        }
        desc.add_field("username", &user_data.username);
        
        let expected_description = desc.build();
        if description != &expected_description {
            let err_msg = format!(
                "Invalid description format. Expected: {}, Got: {}",
                expected_description, description
            );
            log_error(&format!("[assert_set_doc] {}", err_msg));
            return Err(err_msg);
        }
    } else {
        let err_msg = "Description field is required for user documents";
        log_error(&format!("[assert_set_doc] {} key={}", err_msg, context.data.key));
        return Err(err_msg.to_string());
    }

    // Step 5: Ensure username uniqueness across the system
    // First, normalize the username to lowercase for comparison
    let normalized_username = user_data.username.to_lowercase();
    
    // Build the search query to find any document with this username
    // The pattern will match [username:name] anywhere in the description string
    // This works regardless of whether it's at the start, middle, or end
    let params = ListParams {
        matcher: Some(ListMatcher {
            description: Some(format!(".*\\[username:{}\\].*", normalized_username)),
            ..Default::default()
        }),
        ..Default::default()
    };

    // Call list_docs and handle potential errors
    let existing_users = list_docs(String::from("users"), params);

    // Check if we found any existing users with this normalized username
    // Exclude the current document if we're updating
    for (doc_key, _) in existing_users.items {
        if doc_key != context.data.key {
            let err_msg = format!(
                "Username '{}' is already taken. Please choose a different username.",
                user_data.username
            );
            log_error(&format!("[assert_set_doc] {} key={}, username={}", err_msg, context.data.key, user_data.username));
            return Err(err_msg);
        }
    }

    // Step 6: In production mode, enforce one-document-per-identity rule
    if !IS_PLAYGROUND {
        // In production mode, we search by Principal ID in the description
        let mut desc = DocumentDescription::new();
        desc.add_owner(&context.caller.to_string());
        
        let owner_params = ListParams {
            matcher: Some(ListMatcher {
                description: Some(desc.build()),
                ..Default::default()
            }),
            ..Default::default()
        };

        // Call list_docs and handle potential errors
        let existing_docs = list_docs(String::from("users"), owner_params);

        // Check if we found any existing documents owned by this user
        // Exclude the current document if we're updating
        for (doc_key, _) in existing_docs.items {
            if doc_key != context.data.key {
                let err_msg = "Users can only have one account in production mode";
                log_error(&format!("[assert_set_doc] {} key={}", err_msg, context.data.key));
                return Err(err_msg.to_string());
            }
        }
    }

    Ok(())
}

/// Validates a vote document before creation or update
/// 
/// This function performs comprehensive validation of vote documents:
/// 1. Decodes and validates the basic vote data structure
/// 2. Validates description format using DocumentDescription helper
/// 3. Validates vote value constraints (+1 or -1)
/// 4. Validates vote weight constraints (0.0 to 1.0)
/// 5. Prevents self-voting
/// 6. Verifies tag exists
/// 
/// # Arguments
/// * `context` - The validation context containing the document data
/// 
/// # Returns
/// * `Result<(), String>` - Ok if validation passes, Err with detailed message if it fails
fn validate_vote_document(context: &AssertSetDocContext) -> Result<(), String> {
    log_debug(&format!(
        "[validate_vote_document] Validating vote document: key={}",
        context.data.key
    ));

    // Step 1: Decode and validate the basic vote data structure
    // This ensures the document contains all required fields in the correct format
    let vote: Vote = decode_doc_data(&context.data.data.proposed.data)
        .map_err(|e| {
            log_error(&format!(
                "[validate_vote_document] Failed to decode vote data: {}",
                e
            ));
            format!("Failed to decode vote data: {}", e)
        })?;

    // Step 2: Create and validate description using DocumentDescription helper
    // This ensures the description follows our standardized format for both playground and production modes
    let mut desc = DocumentDescription::new();
    let caller_string = context.caller.to_string(); // Create a string that lives for the duration of the function
    desc.add_owner(if IS_PLAYGROUND {
        &vote.data.author_key
    } else {
        &caller_string
    })
    .add_field("target", &vote.data.target_key)
    .add_field("tag", &vote.data.tag_key);

    let expected_description = desc.build();

    // Verify the description matches our expected format
    if let Some(actual_description) = &context.data.data.proposed.description {
        if actual_description != &expected_description {
            let err_msg = format!(
                "Invalid description format. Expected: {}, Got: {}",
                expected_description, actual_description
            );
            log_error(&format!("[validate_vote_document] {}", err_msg));
            return Err(err_msg);
        }
    } else {
        let err_msg = "Description field is required for vote documents";
        log_error(&format!("[validate_vote_document] {}", err_msg));
        return Err(err_msg.to_string());
    }

    // Step 3: Validate vote value constraints
    // Vote value must be either +1 (upvote) or -1 (downvote)
    // This ensures votes have clear, binary meaning in the system
    if vote.data.value != 1.0 && vote.data.value != -1.0 {
        let err_msg = format!("Vote value must be either +1 or -1, got: {}", vote.data.value);
        log_error(&format!("[validate_vote_document] {}", err_msg));
        return Err(err_msg);
    }

    // Step 4: Validate vote weight constraints
    // Vote weight represents the voter's influence, scaled from 0.0 to 1.0
    // This is calculated based on the voter's reputation in the tag
    if vote.data.weight < 0.0 || vote.data.weight > 1.0 {
        let err_msg = format!("Vote weight must be between 0.0 and 1.0, got: {}", vote.data.weight);
        log_error(&format!("[validate_vote_document] {}", err_msg));
        return Err(err_msg);
    }

    // Step 5: Prevent self-voting
    // Users cannot vote on their own content to prevent reputation manipulation
    if vote.data.author_key == vote.data.target_key {
        let err_msg = format!("Cannot vote on yourself (author_key: {})", vote.data.author_key);
        log_error(&format!("[validate_vote_document] {}", err_msg));
        return Err(err_msg);
    }

    // Step 6: Verify the tag exists
    // This ensures votes are only cast on valid tags
    let tag_doc = junobuild_satellite::get_doc(vote.data.tag_key.clone(), String::from("tags"));
    
    // Match on the result instead of using map_err
    let tag_doc = match tag_doc {
        Some(doc) => doc,
        None => {
            let err_msg = format!("Tag not found: {}", vote.data.tag_key);
            log_error(&format!("[validate_vote_document] {}", err_msg));
            return Err(err_msg);
        }
    };

    log_info(&format!(
        "[validate_vote_document] Successfully validated vote document: key={}",
        context.data.key
    ));

    Ok(())
}

/// Validates a tag document before creation or update
/// 
/// This function performs comprehensive validation of tag documents:
/// 1. Decodes and validates the basic tag data structure
/// 2. Validates tag name format and restrictions
/// 3. Validates description length constraints
/// 4. Validates time period configuration
/// 5. Validates reputation and voting settings
/// 
/// # Arguments
/// * `context` - The validation context containing:
///   - caller: The Principal ID of the user making the request
///   - collection: Must be "tags"
///   - key: The document key (nanoid-generated)
///   - data: The proposed document data
/// 
/// # Returns
/// * `Result<(), String>` - Ok if validation passes, Err with detailed message if it fails
fn validate_tag_document(context: &AssertSetDocContext) -> Result<(), String> {
    // Step 1: Decode and validate the basic tag data structure
    let tag_data: TagData = decode_doc_data(&context.data.data.proposed.data)
        .map_err(|e| {
            log_error(&format!("[assert_set_doc] Failed to decode tag data: key={}, error={}", 
                context.data.key, e));
            format!("Invalid tag data format: {}", e)
        })?;
    
    // Step 2: Validate tag name format and uniqueness
    validate_tag_name(&tag_data.name)?;

    // Check for tag name uniqueness (case-insensitive)
    let normalized_name = tag_data.name.to_lowercase();
    
    // Build the search query using DocumentDescription helper
    let mut name_desc = DocumentDescription::new();
    name_desc.add_field("name", &normalized_name);
    let name_description = name_desc.build();
    
    let matcher = ListMatcher {
        description: Some(name_description),
        ..Default::default()
    };

    let params = ListParams {
        matcher: Some(matcher),
        ..Default::default()
    };

    let existing_tags = list_docs(String::from("tags"), params);

    // Check if we found any existing tags with this normalized name
    // Exclude the current document if we're updating
    for (doc_key, _) in existing_tags.items {
        if doc_key != context.data.key {
            let err_msg = format!(
                "Tag name '{}' is already taken (case-insensitive comparison)",
                tag_data.name
            );
            log_error(&format!("[assert_set_doc] {} key={}", err_msg, context.data.key));
            return Err(err_msg);
        }
    }

    // Step 3: Validate description length
    if tag_data.description.len() > 1024 {
        let err_msg = format!(
            "Tag description cannot exceed 1024 characters (current length: {})",
            tag_data.description.len()
        );
        log_error(&format!("[validate_tag_document] {}", err_msg));
        return Err(err_msg);
    }

    // Step 4: Validate time periods
    validate_time_periods(&tag_data.time_periods)?;

    // Step 5: Validate vote reward (0.0 to 1.0)
    if tag_data.vote_reward < 0.0 || tag_data.vote_reward > 1.0 {
        let err_msg = format!(
            "Vote reward must be between 0.0 and 1.0 (got: {})",
            tag_data.vote_reward
        );
        log_error(&format!("[validate_tag_document] {}", err_msg));
        return Err(err_msg);
    }

    // Step 6: Validate minimum users (must be greater than 0)
    if tag_data.min_users_for_threshold == 0 {
        let err_msg = format!(
            "Minimum users must be greater than 0 (got: {})",
            tag_data.min_users_for_threshold
        );
        log_error(&format!("[validate_tag_document] {}", err_msg));
        return Err(err_msg);
    }

    Ok(())
}

/// Validates a reputation document before creation or update
/// 
/// This function performs comprehensive validation of reputation documents:
/// 1. Decodes and validates the basic reputation data structure
/// 2. Validates description format using DocumentDescription helper
/// 3. Validates total basis reputation (from received votes)
/// 4. Validates voting rewards reputation (must be non-negative)
/// 5. Validates effective reputation calculation consistency
/// 6. Validates vote weight constraints (between 0.0 and 1.0)
/// 
/// # Arguments
/// * `context` - The validation context containing:
///   - caller: The Principal ID of the user making the request
///   - collection: Must be "reputations"
///   - key: The document key (nanoid-generated)
///   - data: The proposed document data
/// 
/// # Returns
/// * `Result<(), String>` - Ok if validation passes, Err with detailed message if it fails
fn validate_reputation_document(context: &AssertSetDocContext) -> Result<(), String> {
    log_debug(&format!(
        "[validate_reputation_document] Validating reputation document: key={}",
        context.data.key
    ));

    // Step 1: Decode and validate the basic reputation data structure
    // This ensures the document contains all required fields in the correct format
    // and that we can properly access the reputation data for further validation
    let reputation: Reputation = decode_doc_data(&context.data.data.proposed.data)
        .map_err(|e| {
            log_error(&format!(
                "[validate_reputation_document] Failed to decode reputation data: {}",
                e
            ));
            format!("Failed to decode reputation data: {}", e)
        })?;

    // Step 2: Create and validate description using DocumentDescription helper
    // This ensures the description follows our standardized format:
    // - Playground mode: [owner:{user_key}][tag:{tag_key}]
    // - Production mode: [owner:{principal_id}][tag:{tag_key}]
    let mut desc = DocumentDescription::new();
    let caller_string = context.caller.to_string(); // Create a string that lives for the duration of the function
    desc.add_owner(if IS_PLAYGROUND {
        &reputation.data.user_key
    } else {
        &caller_string
    })
    .add_field("tag", &reputation.data.tag_key);

    let expected_description = desc.build();

    // Verify the description matches our expected format
    if let Some(actual_description) = &context.data.data.proposed.description {
        if actual_description != &expected_description {
            let err_msg = format!(
                "Invalid description format. Expected: {}, Got: {}",
                expected_description, actual_description
            );
            log_error(&format!("[validate_reputation_document] {}", err_msg));
            return Err(err_msg);
        }
    } else {
        let err_msg = "Description field is required for reputation documents";
        log_error(&format!("[validate_reputation_document] {}", err_msg));
        return Err(err_msg.to_string());
    }

    // Step 3: Validate total basis reputation
    // Basis reputation (from received votes) can be negative or positive:
    // - Positive: User has received more upvotes or higher-weighted upvotes
    // - Negative: User has received more downvotes or higher-weighted downvotes
    // - This is the raw vote-based reputation before voting rewards
    log_debug(&format!(
        "[validate_reputation_document] Total basis reputation: {}",
        reputation.data.total_basis_reputation
    ));

    // Step 4: Validate voting rewards constraints
    // Voting rewards must be non-negative because:
    // - They represent participation rewards
    // - They help bootstrap new communities
    // - They incentivize active participation
    if reputation.data.total_voting_rewards_reputation < 0.0 {
        let err_msg = format!(
            "Total voting rewards reputation cannot be negative (got: {})",
            reputation.data.total_voting_rewards_reputation
        );
        log_error(&format!("[validate_reputation_document] {}", err_msg));
        return Err(err_msg);
    }

    // Step 5: Validate effective reputation calculation consistency
    // The effective reputation:
    // - Can be negative (when heavily downvoted)
    // - Should match basis + rewards when above threshold
    // - Should match only basis when below threshold
    // - Is used to determine voting power and privileges
    let expected_effective = if reputation.data.has_voting_power {
        reputation.data.total_basis_reputation + reputation.data.total_voting_rewards_reputation
    } else {
        reputation.data.total_basis_reputation
    };

    if (reputation.data.last_known_effective_reputation - expected_effective).abs() > 0.000001 {
        let err_msg = format!(
            "Effective reputation calculation mismatch. Expected: {}, Got: {}",
            expected_effective,
            reputation.data.last_known_effective_reputation
        );
        log_error(&format!("[validate_reputation_document] {}", err_msg));
        return Err(err_msg);
    }

    // Step 6: Validate vote weight constraints
    // Vote weight must be between 0.0 and 1.0 to:
    // - weight represents how much that vote is part of the user's total 100% votes
    // - it is a percentage (0-1)
    if reputation.data.vote_weight.value() < 0.0 || reputation.data.vote_weight.value() > 1.0 {
        let err_msg = format!(
            "Vote weight must be between 0.0 and 1.0 (got: {})",
            reputation.data.vote_weight.value()
        );
        log_error(&format!("[validate_reputation_document] {}", err_msg));
        return Err(err_msg);
    }

    log_info(&format!(
        "[validate_reputation_document] Successfully validated reputation document: key={}",
        context.data.key
    ));

    Ok(())
}

/// Validates time periods configuration for tags
/// 
/// Time periods define how reputation ages over time in a tag.
/// The configuration must follow specific rules to ensure:
/// - Proper coverage of different time spans
/// - Reasonable reputation decay
/// - System stability
/// 
/// Requirements:
/// 1. At least 1 period must be defined
/// 2. Maximum 10 periods allowed
/// 3. Last period must have 999 months duration
/// 4. Valid multiplier values and increments
/// 
/// # Arguments
/// * `periods` - Array of TimePeriod structs to validate
/// 
/// # Returns
/// * `Result<(), String>` - Ok if validation passes, Err with detailed message if it fails
fn validate_time_periods(periods: &[TimePeriod]) -> Result<(), String> {
    // Step 1: Validate array length
    if periods.is_empty() {
        return Err("Tag must have at least 1 time period".to_string());
    }
    if periods.len() > 10 {
        return Err(format!(
            "Tag cannot have more than 10 time periods (got: {})",
            periods.len()
        ));
    }

    // Step 2: Validate last period is "infinity" (999 months)
    let last_period = periods.last().unwrap();
    if last_period.months != 999 {
        return Err(format!(
            "Last period must be 999 months (got: {})",
            last_period.months
        ));
    }

    // Step 3: Validate each period's configuration
    for (i, period) in periods.iter().enumerate() {
        // Validate multiplier range (0.05 to 10.0)
        if period.multiplier < 0.05 || period.multiplier > 10.0 {
            let err_msg = format!(
                "Multiplier for period {} must be between 0.05 and 10.0 (got: {})",
                i + 1, period.multiplier
            );
            log_error(&format!("[validate_time_periods] {}", err_msg));
            return Err(err_msg);
        }

        // Validate multiplier step increments (0.05)
        let remainder = (period.multiplier * 100.0) % 5.0;
        if remainder != 0.0 {
            let err_msg = format!(
                "Multiplier for period {} must use 0.05 step increments (got: {})",
                i + 1, period.multiplier
            );
            log_error(&format!("[validate_time_periods] {}", err_msg));
            return Err(err_msg);
        }

        // Validate month duration is greater than 0
        if period.months == 0 {
            let err_msg = format!(
                "Months for period {} must be greater than 0 (got: {})",
                i + 1, period.months
            );
            log_error(&format!("[validate_time_periods] {}", err_msg));
            return Err(err_msg);
        }
    }

    Ok(())
}

// =============================================================================
// Available Hooks and Assertions (Currently Disabled)
// =============================================================================
// These hooks and assertions are available but not currently used.
// They are kept as reference for future implementation.

/*
// Document Management Hooks
// ------------------------

#[on_set_many_docs]
async fn on_set_many_docs(_context: OnSetManyDocsContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_doc]
async fn on_delete_doc(_context: OnDeleteDocContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_many_docs]
async fn on_delete_many_docs(_context: OnDeleteManyDocsContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_filtered_docs]
async fn on_delete_filtered_docs(_context: OnDeleteFilteredDocsContext) -> Result<(), String> {
    Ok(())
}

// Asset Management Hooks
// --------------------

#[on_upload_asset]
async fn on_upload_asset(_context: OnUploadAssetContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_asset]
async fn on_delete_asset(_context: OnDeleteAssetContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_many_assets]
async fn on_delete_many_assets(_context: OnDeleteManyAssetsContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_filtered_assets]
async fn on_delete_filtered_assets(_context: OnDeleteFilteredAssetsContext) -> Result<(), String> {
    Ok(())
}

// Additional Assertions
// -------------------

#[assert_delete_doc]
fn assert_delete_doc(_context: AssertDeleteDocContext) -> Result<(), String> {
    Ok(())
}

#[assert_upload_asset]
fn assert_upload_asset(_context: AssertUploadAssetContext) -> Result<(), String> {
    Ok(())
}

#[assert_delete_asset]
fn assert_delete_asset(_context: AssertDeleteAssetContext) -> Result<(), String> {
    Ok(())
}
*/

// =============================================================================
// Satellite Integration
// =============================================================================
// This macro must be included at the end of the file to properly register
// all hooks and assertions with the Juno ecosystem.

/// Gets a user's last known reputation score in a specific tag
/// 
/// This function retrieves the user's cached reputation score from the reputations collection.
/// It does NOT recalculate the reputation - use recalculate_reputation() for that.
/// 
/// The reputation score is tag-specific and represents the user's standing in that tag's community.
/// A higher score indicates more influence in voting and content moderation.
/// 
/// # Arguments
/// * `user_key` - The unique identifier of the user
/// * `tag_key` - The unique identifier of the tag
/// 
/// # Returns
/// * `Result<f64, String>` - The user's last known reputation score or a detailed error message
/// 
/// # Errors
/// - Returns error if user_key or tag_key is empty
/// - Returns error if tag doesn't exist
/// - Returns error if user has no reputation in this tag

#[query]
async fn get_user_reputation(user_key: String, tag_key: String) -> Result<f64, String> {
    log_debug(&format!("[get_user_reputation] Fetching reputation for user={}, tag={}", user_key, tag_key));
    
    // Input validation
    if user_key.is_empty() {
        let err_msg = "User key cannot be empty";
        log_error(&format!("[get_user_reputation] {}", err_msg));
        return Err(err_msg.to_string());
    }
    if tag_key.is_empty() {
        let err_msg = "Tag key cannot be empty";
        log_error(&format!("[get_user_reputation] {}", err_msg));
        return Err(err_msg.to_string());
    }

    // Check if user has reputation in this tag
    let reputation_key = format!("{}_{}", user_key, tag_key);
    let reputation_doc = junobuild_satellite::get_doc(reputation_key, String::from("reputations"));
    
    // Match on the result instead of using map_err
    match reputation_doc {
        Some(doc) => {
            let reputation_data: ReputationData = decode_doc_data(&doc.data)
                .map_err(|e| format!("Failed to decode reputation data: {}", e))?;
    
            log_info(&format!("[get_user_reputation] Successfully retrieved reputation: user={}, tag={}, value={}", 
                user_key, tag_key, reputation_data.last_known_effective_reputation));
    
            Ok(reputation_data.last_known_effective_reputation)
        },
        None => {
            let err_msg = format!("User {} has no reputation in tag {}", user_key, tag_key);
            log_error(&format!("[get_user_reputation] {}", err_msg));
            Err(err_msg)
        }
    }
}

/// Gets a user's complete reputation data for a specific tag
/// 
/// This function retrieves all reputation data components for a user in a tag,
/// including basis reputation, voting rewards, effective reputation, and trust status.
/// 
/// Use this function when you need the complete reputation profile, such as for
/// detailed analysis, admin dashboards, or user profiles.
/// 
/// # Arguments
/// * `user_key` - The unique identifier of the user
/// * `tag_key` - The unique identifier of the tag
/// 
/// # Returns
/// * `Result<ReputationData, String>` - The complete reputation data or a detailed error message
/// 
/// # Errors
/// - Returns error if user_key or tag_key is empty
/// - Returns error if tag doesn't exist
/// - Returns error if user has no reputation in this tag
#[query]
async fn get_user_reputation_full(user_key: String, tag_key: String) -> Result<ReputationData, String> {
    log_debug(&format!("[get_user_reputation_full] Fetching complete reputation data for user={}, tag={}", user_key, tag_key));
    
    // Input validation
    if user_key.is_empty() {
        let err_msg = "User key cannot be empty";
        log_error(&format!("[get_user_reputation_full] {}", err_msg));
        return Err(err_msg.to_string());
    }
    if tag_key.is_empty() {
        let err_msg = "Tag key cannot be empty";
        log_error(&format!("[get_user_reputation_full] {}", err_msg));
        return Err(err_msg.to_string());
    }

    // Check if user has reputation in this tag
    let reputation_key = format!("{}_{}", user_key, tag_key);
    let reputation_doc = junobuild_satellite::get_doc(reputation_key, String::from("reputations"));
    
    // Match on the result instead of using map_err
    match reputation_doc {
        Some(doc) => {
            let reputation_data: ReputationData = decode_doc_data(&doc.data)
                .map_err(|e| format!("Failed to decode reputation data: {}", e))?;
    
            log_info(&format!("[get_user_reputation_full] Successfully retrieved complete reputation data: user={}, tag={}", 
                user_key, tag_key));
    
            Ok(reputation_data)
        },
        None => {
            let err_msg = format!("User {} has no reputation in tag {}", user_key, tag_key);
            log_error(&format!("[get_user_reputation_full] {}", err_msg));
            Err(err_msg)
        }
    }
}

/// Forces a recalculation of a user's reputation in a specific tag
/// 
/// This function triggers a complete recalculation of the user's reputation, including:
/// 1. Basis reputation from all received votes
/// 2. Voting rewards from all votes cast
/// 3. Trust status based on current tag thresholds
/// 4. Final effective reputation score
/// 
/// Use this function when you need to ensure the reputation score is current,
/// such as after significant changes to the voting system or tag configuration.
/// 
/// # Process
/// 1. Validates input parameters
/// 2. Verifies user exists in the tag
/// 3. Triggers complete reputation recalculation
/// 4. Updates all reputation components in storage
/// 
/// # Arguments
/// * `user_key` - The unique identifier of the user
/// * `tag_key` - The unique identifier of the tag
/// 
/// # Returns
/// * `Result<f64, String>` - The updated effective reputation score or a detailed error message
/// 
/// # Errors
/// - Returns error if user_key or tag_key is empty
/// - Returns error if tag doesn't exist
/// - Returns error if reputation calculation fails
/// - Returns error with context about what specifically failed
#[ic_cdk::update]
#[candid::candid_method(update)]
pub async fn recalculate_reputation(user_key: String, tag_key: String) -> Result<f64, String> {
    log_debug(&format!("[recalculate_reputation] Starting recalculation for user={}, tag={}", user_key, tag_key));
    
    // Input validation
    if user_key.is_empty() {
        let err_msg = "User key cannot be empty";
        log_error(&format!("[recalculate_reputation] {}", err_msg));
        return Err(err_msg.to_string());
    }
    if tag_key.is_empty() {
        let err_msg = "Tag key cannot be empty";
        log_error(&format!("[recalculate_reputation] {}", err_msg));
        return Err(err_msg.to_string());
    }

    // Attempt to calculate reputation
    let reputation_data = calculate_user_reputation(&user_key, &tag_key).await
        .map_err(|e| {
            let err_msg = format!("Failed to calculate reputation: {}", e);
            log_error(&format!("[recalculate_reputation] {}: user={}, tag={}", err_msg, user_key, tag_key));
            err_msg
        })?;
    
    log_info(&format!("[recalculate_reputation] Successfully recalculated reputation: user={}, tag={}, value={}", 
        user_key, tag_key, reputation_data.last_known_effective_reputation));
    
    Ok(reputation_data.last_known_effective_reputation)
}

include_satellite!();