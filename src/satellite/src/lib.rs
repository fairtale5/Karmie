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

// Import our utility functions
use crate::utils::normalize::normalize_username;
use crate::utils::validation::{validate_username, validate_display_name};

// =============================================================================
// Module Declarations
// =============================================================================

mod utils;

// =============================================================================
// Type Definitions
// =============================================================================

// Represents user data in the system
// 
// This struct defines the structure of user documents in the Juno datastore.
// It includes basic user information that can be serialized and deserialized.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserData {
    /// User's unique handle/username
    pub handle: String,
    /// User's display name (not necessarily unique)
    pub display_name: String,
}

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

#[on_set_doc(collections = ["users"])]
async fn on_set_doc(context: OnSetDocContext) -> Result<(), String> {
    // Decode the document data
    let user_data: UserData = decode_doc_data(&context.data.data.after.data)?;
    
    // Log the start of user data update
    log_user!("Updating user data for key: {}", context.data.key);
    
    // Normalize and validate the handle
    let normalized_handle = normalize_username(&user_data.handle);
    
    // Handle validation results
    validate_username(&normalized_handle).map_err(|e| e.to_string())?;
    validate_display_name(&user_data.display_name).map_err(|e| e.to_string())?;

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
            return Err("Username already exists".to_string());
        }
    }

    // Update the document with normalized handle and description
    let updated_data = UserData {
        handle: normalized_handle.clone(),
        display_name: user_data.display_name.clone(),
    };
    
    // Log the successful update
    log_user!("Successfully updated user {} with handle {}", context.data.key, normalized_handle);
    
    let encoded_data = encode_doc_data(&updated_data)?;
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
    ).map_err(|e| e.to_string())?;

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
    log_user!("Validating user data for key: {}", context.data.key);

    // Step 1: Basic Data Validation
    // ----------------------------
    let user_data: UserData = decode_doc_data(&context.data.data.proposed.data)?;
    
    // Validate username and display name
    validate_username(&user_data.handle).map_err(|e| {
        log_user!("Username validation failed for key {}: {}", context.data.key, e);
        e.to_string()
    })?;
    validate_display_name(&user_data.display_name).map_err(|e| {
        log_user!("Display name validation failed for key {}: {}", context.data.key, e);
        e.to_string()
    })?;

    // Step 2: Username Uniqueness Check
    // -------------------------------
    // Normalize the username for consistent comparison
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
        // If username exists in any other document, log and return error
        log_user!("Username '{}' already exists (key: {})", normalized_handle, context.data.key);
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
                log_user!("User already has an account (caller: {}, attempted key: {})", 
                    context.caller, context.data.key);
                return Err("User already has an account in the system".to_string());
            }
        }
    }
    */

    // Log successful validation
    log_user!("Successfully validated user data for key: {}", context.data.key);
    
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

include_satellite!();
