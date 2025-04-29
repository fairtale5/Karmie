use junobuild_satellite::AssertSetDocContext;
use junobuild_utils::decode_doc_data;
use junobuild_shared::types::list::{ListMatcher, ListParams};
use crate::{
    validation::{validate_handle, validate_display_name},
    utils::structs::UserData,
    processors::document_keys::create_user_key,
};
use crate::list_docs;
use crate::logger;
use crate::IS_PLAYGROUND;

/// Validates a user document before creation or update
/// 
/// This function performs comprehensive validation of user documents:
/// 1. Validates the document key field format by comparing with a freshly generated comparisson key
/// 2. Validates username format and restrictions
/// 3. Validates display name format and restrictions
/// 4. Ensures username uniqueness across the system
/// 5. Enforces one-document-per-identity rule in production mode
/// 
/// # Arguments
/// * `context` - The validation context containing:
///   - caller: The Principal ID of the user making the request
///   - collection: Must be "users"
///   - key: The document key (ULID-based format)
///   - data: The proposed document data
/// 
/// # Returns
/// * `Result<(), String>` - Ok if validation passes, Err with detailed message if it fails
pub fn assert_doc_user(context: &AssertSetDocContext) -> Result<(), String> {
    // Step 1: Decode and validate user data
    let user_data: UserData = decode_doc_data(&context.data.data.proposed.data)
        .map_err(|e| format!("Failed to decode user data: {}", e))?;

    // Step 2: Validate document key format
    // First validate that the key is a valid ULID
    crate::processors::ulid_generator::validate_ulid(&context.data.key)?;
    
    // Then check if the formatted key matches what was provided
    let expected_key = crate::processors::document_keys::format_user_key(&context.data.key, &user_data.username)?;
    if expected_key != context.data.key {
        return Err(format!(
            "Invalid document key format. Expected: {}, Got: {}", 
            expected_key, 
            context.data.key
        ));
    }

    // Step 3: Validate username format and restrictions
    validate_handle(&user_data.username)
        .map_err(|e| {
            let err_msg = format!("[assert_set_doc] Username validation failed: {}", e);
            logger!("error", "{}", err_msg);
            e
        })?;

    // Step 4: Validate display name format and restrictions
    validate_display_name(&user_data.display_name)
        .map_err(|e| {
            let err_msg = format!("[assert_set_doc] Display name validation failed: {}", e);
            logger!("error", "{}", err_msg);
            e
        })?;

    // Step 5: Ensure username uniqueness using direct key-based lookup
    // This is more efficient than loading the entire table into memory first
    let normalized_username = crate::processors::document_keys::sanitize_for_key(&user_data.username);
    let username_key_part = format!("_usrName_{}_", normalized_username);
    
    logger!("debug", "[validate_user_document] Checking username uniqueness with key pattern: {}", username_key_part);

    // Check if we're updating an existing document
    let is_update = context.data.data.current.is_some();
    
    // Get documents that have this username pattern in their key
    // This lookup happens directly at the database level without loading everything into memory
    let results = junobuild_satellite::list_docs_store(
        context.caller,
        String::from("users"),
        &ListParams {
            matcher: Some(ListMatcher {
                key: Some(username_key_part),  // Match the username part in the key
                ..Default::default()
            }),
            ..Default::default()
        },
    )?;

    // Check if we found any documents with this username
    if results.items_length > 0 {
        // If this is an update, it's ok if we found our own document
        for (existing_key, _) in results.items {
            if !is_update || existing_key != context.data.key {
                // Found another document with this username
                let err_msg = format!(
                    "[assert_set_doc] Username '{}' is already taken. Please choose a different username.",
                    user_data.username
                );
                logger!("error", "{}", err_msg);
                return Err(err_msg);
            }
        }
    } else {
        // No existing document with this username - this is good!
        logger!("debug", "[assert_set_doc] Username '{}' is available", user_data.username);
    }

    // Step 6: In production mode, enforce one-document-per-identity rule
    if !IS_PLAYGROUND {
        // In production mode, we can search by owner field (Principal ID)
        // This is a built-in Juno field, no need for description
        let principal_string = context.caller.to_string();
        
        // Use list_docs to find any documents owned by this principal
        let owner_params = ListParams {
            matcher: Some(ListMatcher {
                // Juno doesn't provide a way to search by owner directly in matcher,
                // so we need to fetch all documents and check owner manually
                ..Default::default()
            }),
            ..Default::default()
        };

        let existing_docs = list_docs(String::from("users"), owner_params);

        // Check if any existing documents are owned by this principal (excluding this document if it's an update)
        for (doc_key, doc) in existing_docs.items {
            if doc_key != context.data.key && doc.owner.to_string() == principal_string {
                let err_msg = format!("[validate_user_document] Users can only have one account in production mode. key={}", context.data.key);
                logger!("error", "{}", err_msg);
                return Err(err_msg.to_string());
            }
        }
    }

    Ok(())
}
