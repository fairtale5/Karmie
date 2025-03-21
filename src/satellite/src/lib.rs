/*!
 * Juno Satellite Implementation
 * 
 * This module implements the core functionality for a Juno satellite,
 * including document management, asset handling, and custom business logic.
 * 
 * # Available Features
 * - Document Management (create, update, delete)
 * - Asset Management (upload, delete)
 * - Custom Validation Logic
 * - Event Hooks
 */

// =============================================================================
// Available Macro Imports
// =============================================================================
// These imports are kept as reference for all available macro decorators.
// Currently we only use on_set_doc and assert_set_doc, but others are
// documented for future use.

use junobuild_macros::{
    // Currently Active Macros
    // ----------------------
    assert_set_doc,                 // For asserting document creation/update
    on_set_doc,                     // For handling document creation/update

    // Available but Currently Unused Macros (kept as reference)
    // ------------------------------------------------------
    // assert_delete_asset,         // For asserting asset deletion
    // assert_delete_doc,           // For asserting document deletion
    // assert_upload_asset,         // For asserting asset upload
    // on_delete_asset,             // For handling asset deletion
    // on_delete_doc,               // For handling document deletion
    // on_delete_filtered_assets,   // For handling filtered asset deletion
    // on_delete_filtered_docs,     // For handling filtered document deletion
    // on_delete_many_assets,       // For handling batch asset deletion
    // on_delete_many_docs,         // For handling batch document deletion
    // on_set_many_docs,            // For handling batch document creation/update
    // on_upload_asset,             // For handling asset upload
};

// =============================================================================
// Available Context Types and Utilities
// =============================================================================
// These imports provide the necessary types and utilities for working with
// Juno's satellite features.

use junobuild_satellite::{
    // Currently Active Types
    // --------------------
    include_satellite,              // Required macro for Juno integration
    AssertSetDocContext,            // Context for document creation/update assertion
    OnSetDocContext,                // Context for document creation/update
    set_doc_store,                  // Function to store documents
    SetDoc,                         // Document type for setting data

    // Available but Currently Unused Types (kept as reference)
    // ---------------------------------------------------
    // AssertDeleteAssetContext,    // Context for asset deletion assertion
    // AssertDeleteDocContext,      // Context for document deletion assertion
    // AssertUploadAssetContext,    // Context for asset upload assertion
    // OnDeleteAssetContext,        // Context for asset deletion handler
    // OnDeleteDocContext,          // Context for document deletion handler
    // OnDeleteFilteredAssetsContext,  // Context for filtered asset deletion
    // OnDeleteFilteredDocsContext, // Context for filtered document deletion
    // OnDeleteManyAssetsContext,   // Context for batch asset deletion
    // OnDeleteManyDocsContext,     // Context for batch document deletion
    // OnSetManyDocsContext,        // Context for batch document creation/update
    // OnUploadAssetContext,        // Context for asset upload handler
};

// =============================================================================
// Juno Shared Types
// =============================================================================
// Types for working with Juno's list functionality

use junobuild_shared::types::list::{ListMatcher, ListParams};
use ic_cdk::api::call::CallResult;
use ic_cdk_macros::*;
use candid::Principal;

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

use junobuild_utils::{decode_doc_data, encode_doc_data};
use serde::{Deserialize, Serialize};

// Import our utility modules
use crate::utils::{
    normalize::normalize_username,
    validation::{validate_username, validate_display_name},
    structs::{Vote, Tag, Reputation, UserData},
    reputation_calculations::{
        calculate_user_reputation, get_user_reputation_data,
        calculate_and_store_vote_weight
    },
    logging::{log_error, log_warn, log_info, log_debug}
};

// =============================================================================
// Module Declarations
// =============================================================================

mod utils;

// =============================================================================
// Active Hooks and Assertions
// =============================================================================

// Handles document creation/update in the "users" collection
// 
// This hook is triggered whenever a document is created or updated in the
// "users" collection. It performs the following tasks:
// 1. Validates the user data
// 2. Normalizes the username
// 3. Checks for username uniqueness
// 4. Updates the document with normalized data

#[on_set_doc(collections = ["users", "votes", "tags"])]
async fn on_set_doc(context: OnSetDocContext) -> Result<(), String> {
    match context.data.collection.as_str() {
        "users" => {
            // Decode the document data
            let user_data: UserData = decode_doc_data(&context.data.data.after.data)
                .map_err(|e| {
                    log_error(&format!("[on_set_doc - Users] Failed to decode user data: {}", e));
                    e.to_string()
                })?;
            
            // Log the start of user data update
            log_debug(&format!("[on_set_doc - Users] Processing update for key: {}", context.data.key));
            
            // Normalize and validate the handle
            let normalized_handle = normalize_username(&user_data.handle);
            
            // Handle validation results
            validate_username(&normalized_handle).map_err(|e| {
                log_error(&format!("[on_set_doc - Users] Username validation failed: {}", e));
                e.to_string()
            })?;
            validate_display_name(&user_data.display_name).map_err(|e| {
                log_error(&format!("[on_set_doc - Users] Display name validation failed: {}", e));
                e.to_string()
            })?;

            // Check for handle uniqueness using the description field
            let username_search = format!("username:{}", normalized_handle);
            let matcher = ListMatcher {
                description: Some(username_search.clone()),
                ..Default::default()
            };

            let params = ListParams {
                matcher: Some(matcher),
                ..Default::default()
            };

            let existing_docs = list_docs(String::from("users"), params);
            for doc in existing_docs.items {
                if doc.0 != context.data.key {
                    log_error(&format!("[on_set_doc - Users] Username conflict: {} already exists", normalized_handle));
                    return Err("Username already exists".to_string());
                }
            }

            // Update the document with normalized handle and description
            let updated_data = UserData {
                handle: normalized_handle.clone(),
                display_name: user_data.display_name.clone(),
            };
            
            // Log the successful update
            log_info(&format!("[on_set_doc - Users] Successfully updated user {} with handle {}", context.data.key, normalized_handle));
            
            let encoded_data = encode_doc_data(&updated_data)
                .map_err(|e| {
                    log_error(&format!("[on_set_doc - Users] Failed to encode updated data: {}", e));
                    e.to_string()
                })?;

            let doc = SetDoc {
                data: encoded_data,
                description: Some(username_search),
                version: context.data.data.after.version,
            };

            set_doc_store(
                context.caller,
                context.data.collection,
                context.data.key,
                doc,
            ).map_err(|e| {
                log_error(&format!("[on_set_doc - Users] Failed to store document: {}", e));
                e.to_string()
            })?;
        }
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
        "tags" => {
            log_debug("[on_set_doc - Tags] Processing tag update (no reputation recalculation needed)");
        }
        _ => {
            log_error(&format!("[on_set_doc] Unknown collection: {}", context.data.collection));
            return Err(format!("Unknown collection: {}", context.data.collection));
        }
    }
    
    Ok(())
}

/// Validates document creation/update in the "users" collection
/// 
/// This assertion is triggered before a document is created or updated
/// in the "users" collection. It ensures:
/// 1. The username and display name are valid
/// 2. The username is unique across all documents
/// 3. (Temporarily disabled) One document per owner limit
#[assert_set_doc(collections = ["users"])]
fn assert_set_doc(context: AssertSetDocContext) -> Result<(), String> {
    // Log the start of validation
    log_debug(&format!("[assert_set_doc] Starting validation for key: {}", context.data.key));

    // Step 1: Basic Data Validation
    // ----------------------------
    let user_data: UserData = decode_doc_data(&context.data.data.proposed.data)
        .map_err(|e| {
            log_error(&format!("[assert_set_doc] Failed to decode user data: key={}, error={}", context.data.key, e));
            e.to_string()
        })?;
    
    // Validate username and display name
    validate_username(&user_data.handle).map_err(|e| {
        log_error(&format!("[assert_set_doc] Username validation failed: key={}, error={}", context.data.key, e));
        e.to_string()
    })?;
    validate_display_name(&user_data.display_name).map_err(|e| {
        log_error(&format!("[assert_set_doc] Display name validation failed: key={}, error={}", context.data.key, e));
        e.to_string()
    })?;

    // Step 2: Username Uniqueness Check
    // -------------------------------
    // Normalize the username for consistent comparison
    log_debug(&format!("[assert_set_doc] Checking username uniqueness for: {}", user_data.handle));
    
    let normalized_handle = normalize_username(&user_data.handle);
    let username_search = format!("username:{}", normalized_handle);
    
    // Search for any documents with this username
    let matcher = ListMatcher {
        description: Some(username_search),
        ..Default::default()
    };

    let params = ListParams {
        matcher: Some(matcher),
        ..Default::default()
    };

    // Check for username conflicts
    let existing_docs = list_docs(String::from("users"), params);
    for doc in existing_docs.items {
        // Skip our own document if this is an update
        if context.data.data.proposed.version.is_some() && doc.0 == context.data.key {
            continue;
        }
        log_error(&format!("[assert_set_doc] Username conflict: {} already exists (key: {})", normalized_handle, context.data.key));
        return Err("Username already exists".to_string());
    }

    // Step 3: One-Document-Per-Owner Check (Temporarily Disabled)
    // --------------------------------------------------------
    // Only check on CREATE operations (when version is not provided)
    /*
    if context.data.data.proposed.version.is_none() {
        let user_docs_params = ListParams {
            owner: Some(context.caller),
            ..Default::default()
        };

        let existing_user_docs = list_docs(String::from("users"), user_docs_params);
        for doc in existing_user_docs.items {
            if doc.0 != context.data.key {
                log_info(&format!("User already has an account (caller: {}, attempted key: {})", 
                    context.caller, context.data.key));
                return Err("User already has an account in the system".to_string());
            }
        }
    }
    */

    // Log successful validation
    log_info(&format!("[assert_set_doc] Successfully validated user data: key={}, handle={}", context.data.key, normalized_handle));
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

/// Gets a user's current effective reputation score in a specific tag
/// 
/// This function retrieves the user's cached reputation score from the reputations collection.
/// The score is a combination of:
/// - Basis reputation (from received votes)
/// - Voting rewards (if user is trusted or community is in bootstrap phase)
/// 
/// The reputation score is tag-specific and represents the user's standing in that tag's community.
/// A higher score indicates more influence in voting and content moderation.
/// 
/// # Arguments
/// * `user_key` - The unique identifier of the user
/// * `tag_key` - The unique identifier of the tag
/// 
/// # Returns
/// * `Result<f64, String>` - The user's effective reputation score or a detailed error message
/// 
/// # Errors
/// - Returns error if user_key or tag_key is empty
/// - Returns error if tag doesn't exist
/// - Returns error if reputation calculation fails
/// - Returns error with context about what specifically failed
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

    // Attempt to calculate reputation
    let reputation_data = calculate_user_reputation(&user_key, &tag_key).await
        .map_err(|e| {
            let err_msg = format!("Failed to calculate reputation: {}", e);
            log_error(&format!("[get_user_reputation] {}: user={}, tag={}", err_msg, user_key, tag_key));
            err_msg
        })?;
    
    log_info(&format!("[get_user_reputation] Successfully retrieved reputation: user={}, tag={}, value={}", 
        user_key, tag_key, reputation_data.last_known_effective_reputation));
    
    Ok(reputation_data.last_known_effective_reputation)
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
/// * `Result<f64, String>` - The newly calculated effective reputation score or a detailed error message
/// 
/// # Errors
/// - Returns error if user_key or tag_key is empty
/// - Returns error if user doesn't exist in the tag
/// - Returns error if tag doesn't exist
/// - Returns error if reputation calculation fails
/// - Returns error with context about what specifically failed
#[ic_cdk::update]
async fn recalculate_reputation(user_key: String, tag_key: String) -> Result<f64, String> {
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

    // Check if user exists in this tag
    match get_user_reputation_data(&user_key, &tag_key).await {
        Ok(None) => {
            let err_msg = format!("No reputation data found for user {} in tag {}", user_key, tag_key);
            log_error(&format!("[recalculate_reputation] {}", err_msg));
            return Err(err_msg);
        }
        Err(e) => {
            let err_msg = format!("Error checking user reputation data: {}", e);
            log_error(&format!("[recalculate_reputation] {}: user={}, tag={}", err_msg, user_key, tag_key));
            return Err(err_msg);
        }
        Ok(Some(_)) => {
            log_debug(&format!("[recalculate_reputation] User found, proceeding with recalculation"));
        }
    }
    
    // Trigger complete recalculation
    let rep_data = calculate_user_reputation(&user_key, &tag_key).await
        .map_err(|e| {
            let err_msg = format!("Failed to recalculate reputation: {}", e);
            log_error(&format!("[recalculate_reputation] {}: user={}, tag={}", err_msg, user_key, tag_key));
            err_msg
        })?;
    
    log_info(&format!("[recalculate_reputation] Successfully recalculated: user={}, tag={}, value={}", 
        user_key, tag_key, rep_data.last_known_effective_reputation));
    
    Ok(rep_data.last_known_effective_reputation)
}

include_satellite!();
