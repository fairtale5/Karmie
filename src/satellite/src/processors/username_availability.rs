/*!
 * Username Availability Checker using Full Collection Scan
 * 
 * This module provides functionality to check username availability using
 * the approach of filtering documents by their userDocument.data.user_handle field
 * 
 * **Performance Note**: This approach loads ALL user documents into memory and
 * filters them, which is less efficient than key-based queries but allows for
 * mutable usernames. Suitable for development and small user bases (<1000 users).
 * 
 * **Purpose**: This is a testing implementation to compare performance against the
 * current handle-in-key approach before implementing the full users_handles
 * index collection architecture.
 */

use junobuild_satellite;
use junobuild_utils::decode_doc_data;
use crate::utils::structs::UserData;
use crate::logger;
use ic_cdk;

/// Checks if a username is available using full collection scan
/// 
/// This function demonstrates the new approach to username availability checking
/// by filtering documents by their data.user_handle field instead of using key patterns.
/// 
/// **Performance Note**: This approach loads ALL user documents into memory and
/// filters them, which is less efficient than key-based queries but allows for
/// mutable usernames. Suitable for development and small user bases (<1000 users).
/// 
/// **Purpose**: This is a testing endpoint to compare performance against the
/// current handle-in-key approach before implementing the full users_handles
/// index collection architecture.
/// 
/// # Arguments
/// * `username` - The username to check for availability
/// 
/// # Returns
/// * `Result<bool, String>` - Returns true if username is available (not taken),
///   false if username is already taken, or error message if operation fails
/// 
/// # Process
/// 1. Validates input username format and length
/// 2. Normalizes username to lowercase for case-insensitive comparison
/// 3. Uses list_docs_store to load ALL user documents
/// 4. Filters documents to find any with matching user_handle
/// 5. Returns availability status based on search results
/// 
/// # Errors
/// - Returns error if username is empty or invalid format
/// - Returns error if list_docs_store fails
/// - Returns error if document data decoding fails
pub async fn check_username_availability_v2(username: String) -> Result<bool, String> {
    // NOTE: Logs commented out since query functions don't persist logs to Juno console
    // logger!("debug", "[check_username_availability_v2] Checking availability for username: {}", username);
    
    // Input validation
    if username.is_empty() {
        let err_msg = "[check_username_availability_v2] Username cannot be empty";
        // logger!("error", "{}", err_msg);
        return Err(err_msg.to_string());
    }
    
    if username.len() < 3 {
        let err_msg = "[check_username_availability_v2] Username must be at least 3 characters";
        // logger!("error", "{}", err_msg);
        return Err(err_msg.to_string());
    }
    
    // Normalize username for case-insensitive comparison
    let normalized_username = username.trim().to_lowercase();
    // logger!("debug", "[check_username_availability_v2] Normalized username: {}", normalized_username);
    
    // Use list_docs_store to get ALL user documents (this loads everything into memory)
    let list_docs_result = junobuild_satellite::list_docs_store(
        ic_cdk::id(),  // Use canister's ID for admin/controller access
        "users".to_string(),
        &junobuild_shared::types::list::ListParams {
            matcher: None,
            paginate: None,
            order: None,
            owner: None,
        }
    );
    
    match list_docs_result {
        Ok(docs_result) => {
            // logger!("debug", "[check_username_availability_v2] Loaded {} user documents for filtering", docs_result.items.len());
            
            // Filter through all documents to find matching usernames
            for (doc_key, doc) in &docs_result.items {
                // Decode the document data to access the user_handle field
                match decode_doc_data::<UserData>(&doc.data) {
                    Ok(user_data) => {
                        let existing_handle = user_data.user_handle.trim().to_lowercase();
                        // logger!("debug", "[check_username_availability_v2] Checking document key={}, handle={}", doc_key, existing_handle);
                        
                        if existing_handle == normalized_username {
                            // logger!("info", "[check_username_availability_v2] Username '{}' is TAKEN (found in document: {})", username, doc_key);
                            return Ok(false); // Username is taken
                        }
                    },
                    Err(_e) => {
                        // logger!("error", "[check_username_availability_v2] Failed to decode user data for document {}: {}", doc_key, e);
                        // Continue checking other documents instead of failing completely
                        continue;
                    }
                }
            }
            
            // If we get here, no matching username was found
            // logger!("info", "[check_username_availability_v2] Username '{}' is AVAILABLE (checked {} documents)", username, docs_result.items.len());
            Ok(true) // Username is available
        },
        Err(e) => {
            let err_msg = format!("[check_username_availability_v2] Failed to list user documents: {}", e);
            // logger!("error", "{}", err_msg);
            Err(err_msg)
        }
    }
} 