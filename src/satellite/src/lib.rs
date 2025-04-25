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

//============================================================================
// Utility Imports
// ===========================================================================


use junobuild_utils::{decode_doc_data, encode_doc_data};

// Import our utility modules
use crate::utils::{
    normalize::normalize_username,
    validation::{validate_username, validate_display_name, validate_tag_name},
    structs::{Vote, VoteData, Tag, Reputation, UserData, TagData, TimePeriod, ReputationData},
    reputation_calculations::{
        calculate_user_reputation, get_user_reputation_data,
        calculate_and_store_vote_weight,
        get_period_multiplier,
    },
    description_helpers::{DocumentDescription, create_vote_description, validate_description}
};

// =============================================================================
// Module Declarations
// =============================================================================

mod utils;
mod assert_set_doc;
mod validation;

// Use the moved validation function
use assert_set_doc::{
    validate_user_document,
    validate_vote_document,
    validate_tag_document,
    validate_reputation_document,
};

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
    logger!("debug", "[on_set_doc] on_set_doc triggered");
    
    match context.data.collection.as_str() {
        "votes" => {
            logger!("debug", "[on_set_doc - Votes] Processing New Vote");
            process_vote(&context).await
        },
        "users" | "tags" => {
            // No side effects needed for users or tags
            logger!("debug", "No hooks defined for collection: {}", context.data.collection);
            Ok(())
        }
        _ => {
            let err_msg = format!("Unknown collection: {}", context.data.collection);
            logger!("error", "{}", err_msg);
            Err(err_msg)
        }
    }
}

/// Process a vote document after it has been created or updated
async fn process_vote(context: &OnSetDocContext) -> Result<(), String> {
    // Access the vote document metadata directly
    let vote_doc = &context.data.data.after;
    
    // Only decode the inner data field when needed
    let vote_data: VoteData = decode_doc_data(&vote_doc.data)
        .map_err(|e| {
            logger!("error", "[process_vote] Failed to decode vote data: {}", e);
            e.to_string()
        })?;
    
    // Log the vote details in a human-readable format
    logger!("info", "[process_vote] Processing new vote: author={} voted {} on target={} in tag={}",
        vote_data.author_key,
        vote_data.value,
        vote_data.target_key,
        vote_data.tag_key
    );
    
    // Ensure tag_key is not empty - this is critical to prevent later errors
    if vote_data.tag_key.is_empty() {
        let err_msg = "Tag key cannot be empty";
        logger!("error", "[process_vote] {}", err_msg);
        return Err(err_msg.to_string());
    }
    
    // Step 1: Calculate and store the voting user's vote weight
    logger!("info", "Step 1/3: Calculating vote weight for author: {}", vote_data.author_key);
    let vote_weight = calculate_and_store_vote_weight(&vote_data.author_key, &vote_data.tag_key).await
        .map_err(|e| {
            logger!("error", "[process_vote] Failed to calculate vote weight: {}", e);
            e.to_string()
        })?;
    logger!("info", "[process_vote] Step 1/3 COMPLETE: Vote weight for author={}: {}", vote_data.author_key, vote_weight);
    
    // Step 2: Calculate reputation for the voting user (author)
    logger!("info", "[process_vote] Step 2/3: Calculating reputation for author: {}", vote_data.author_key);
    let author_rep = calculate_user_reputation(&vote_data.author_key, &vote_data.tag_key).await
        .map_err(|e| {
            logger!("error", "[process_vote] Failed to calculate author reputation: {}", e);
            e.to_string()
        })?;
    logger!("info", "[process_vote] Step 2/3 COMPLETE: Author={}: basisR={}, voteR={}, totalR={}, voting_power={}",
        vote_data.author_key, 
        author_rep.total_basis_reputation,
        author_rep.total_voting_rewards_reputation,
        author_rep.last_known_effective_reputation,
        author_rep.has_voting_power
    );
    
    // Step 3: Calculate reputation for the target user
    logger!("info", "[process_vote] Step 3/3: Calculating reputation for target: {}", vote_data.target_key);
    let target_rep = calculate_user_reputation(&vote_data.target_key, &vote_data.tag_key).await
        .map_err(|e| {
            logger!("error", "[process_vote] Failed to calculate target reputation: {}", e);
            e.to_string()
        })?;
    logger!("info", "[process_vote] Step 3/3 COMPLETE: Target={}: basisR={}, voteR={}, totalR={}, voting_power={}",
        vote_data.target_key, 
        target_rep.total_basis_reputation,
        target_rep.total_voting_rewards_reputation,
        target_rep.last_known_effective_reputation,
        target_rep.has_voting_power
    );

    logger!("info", "[process_vote] Completed - author={}, target={}, tag={}, vote_value={}, vote_weight={}",
        vote_data.author_key, 
        vote_data.target_key, 
        vote_data.tag_key, 
        vote_data.value, 
        vote_weight
    );
    
    Ok(())
}

/// Configuration flag for playground mode
pub const IS_PLAYGROUND: bool = true;  // Set to false for production

/// Description formats for Juno documents in the system
/// All documents use a standardized description field format pattern to enable filtering
/// 
/// ## Description Field Format
/// 
/// All description fields follow a consistent pattern:
/// * - Format: field1=value1;field2=value2;
/// 
/// The concrete implementation depends on the collection:
/// 
/// ### Users Collection
/// * owner=key;username=normalized_username;
/// 
/// ### Votes Collection
/// * owner=id;author=key;target=key;tag=key;
/// 
/// ### Tags Collection
/// * owner=id;name=normalized_name;
/// 
/// ### Reputations Collection
/// * owner=id;user=key;tag=key;
/// 
#[assert_set_doc(collections = ["users", "votes", "tags", "reputations"])]
fn assert_set_doc(context: AssertSetDocContext) -> Result<(), String> {
    let result = match context.data.collection.as_str() {
        "users" => {
            logger!("debug", "[assert_set_doc] Validating user document: key={}", context.data.key);
            validate_user_document(&context)
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
        _ => {
            // This should never happen because we're specifying collections in the decorator
            let err_msg = format!("Unexpected collection reached validation: {}", context.data.collection);
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
                "[validate_time_periods]Multiplier for period {} must be between 0.05 and 10.0 (got: {})",
                i + 1, period.multiplier
            );
            logger!("error", "{}", err_msg);
            return Err(err_msg);
        }

        // Validate multiplier step increments (0.05) with floating-point tolerance
        // We multiply by 100 to work with integers and avoid floating-point issues
        let multiplier_int = (period.multiplier * 100.0).round();
        let remainder = multiplier_int % 5.0;
        if remainder > 0.000001 { // Allow for small floating-point rounding errors
            let err_msg = format!(
                "[validate_time_periods] Multiplier for period {} must use 0.05 step increments (got: {})",
                i + 1, period.multiplier
            );
            logger!("error", "{}", err_msg);
            return Err(err_msg);
        }

        // Validate month duration is greater than 0
        if period.months == 0 {
            let err_msg = format!(
                "[validate_time_periods] Months for period {} must be greater than 0 (got: {})",
                i + 1, period.months
            );
            logger!("error", "{}", err_msg);
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
    logger!("debug", "[get_user_reputation] Fetching reputation for user={}, tag={}", user_key, tag_key);
    
    // Input validation
    if user_key.is_empty() {
        let err_msg = "[get_user_reputation] User key cannot be empty";
        logger!("error", "{}", err_msg);
        return Err(err_msg.to_string());
    }
    if tag_key.is_empty() {
        let err_msg = "[get_user_reputation] Tag key cannot be empty";
        logger!("error", "{}", err_msg);
        return Err(err_msg.to_string());
    }

    // Check if user has reputation in this tag
    let reputation_key = format!("{}_{}", user_key, tag_key);
    let reputation_doc = junobuild_satellite::get_doc(
        String::from("reputations"),  // Collection name first
        reputation_key,               // Document key second
    );
    
    // Match on the result instead of using map_err
    match reputation_doc {
        Some(doc) => {
            let reputation_data: ReputationData = decode_doc_data(&doc.data)
                .map_err(|e| format!("Failed to decode reputation data: {}", e))?;
    
            logger!("info", "[get_user_reputation] Successfully retrieved reputation: user={}, tag={}, value={}", 
                user_key, tag_key, reputation_data.last_known_effective_reputation);
    
            Ok(reputation_data.last_known_effective_reputation)
        },
        None => {
            let err_msg = format!("[get_user_reputation] User {} has no reputation in tag {}", user_key, tag_key);
            logger!("error", "{}", err_msg);
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
    logger!("debug", "[get_user_reputation_full] Fetching complete reputation data for user={}, tag={}", user_key, tag_key);
    
    // Input validation
    if user_key.is_empty() {
        let err_msg = "[get_user_reputation_full] User key cannot be empty";
        logger!("error", "{}", err_msg);
        return Err(err_msg.to_string());
    }
    if tag_key.is_empty() {
        let err_msg = "[get_user_reputation_full] Tag key cannot be empty";
        logger!("error", "{}", err_msg);
        return Err(err_msg.to_string());
    }

    // Check if user has reputation in this tag
    let reputation_key = format!("{}_{}", user_key, tag_key);
    let reputation_doc = junobuild_satellite::get_doc(
        String::from("reputations"),  // Collection name first
        reputation_key,               // Document key second
    );
    
    // Match on the result instead of using map_err
    match reputation_doc {
        Some(doc) => {
            let reputation_data: ReputationData = decode_doc_data(&doc.data)
                .map_err(|e| format!("Failed to decode reputation data: {}", e))?;
    
            logger!("info", "[get_user_reputation_full] Successfully retrieved complete reputation data: user={}, tag={}", user_key, tag_key);
    
            Ok(reputation_data)
        },
        None => {
            let err_msg = format!("[get_user_reputation_full] User {} has no reputation in tag {}", user_key, tag_key);
            logger!("error", "{}", err_msg);
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
    logger!("debug", "[recalculate_reputation] Starting recalculation for user={}, tag={}", user_key, tag_key);
    
    // Input validation
    if user_key.is_empty() {
        let err_msg = "[recalculate_reputation] User key cannot be empty";
        logger!("error", "{}", err_msg);
        return Err(err_msg.to_string());
    }
    if tag_key.is_empty() {
        let err_msg = "[recalculate_reputation] Tag key cannot be empty";
        logger!("error", "{}", err_msg);
        return Err(err_msg.to_string());
    }

    // Attempt to calculate reputation
    let reputation_data = calculate_user_reputation(&user_key, &tag_key).await
        .map_err(|e| {
            let err_msg = format!("[recalculate_reputation] Failed to calculate reputation: {}", e);
            logger!("error", "{}", err_msg);
            err_msg
        })?;
    
    logger!("info", "[recalculate_reputation] Successfully recalculated reputation: user={}, tag={}, value={}", 
        user_key, 
        tag_key, 
        reputation_data.last_known_effective_reputation
    );
    
    Ok(reputation_data.last_known_effective_reputation)
}

include_satellite!();