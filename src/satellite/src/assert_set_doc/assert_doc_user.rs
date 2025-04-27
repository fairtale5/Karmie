use junobuild_satellite::AssertSetDocContext;
use junobuild_utils::decode_doc_data;
use junobuild_shared::types::list::{ListMatcher, ListParams};
use crate::{
    validation::{validate_username, validate_display_name},
    utils::{
        structs::UserData,
    },
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
pub async fn validate_user_document(context: &AssertSetDocContext) -> Result<(), String> {
    // Step 1: Decode the user data structure
    let user_data: UserData = decode_doc_data(&context.data.data.proposed.data)
        .map_err(|e| {
            let err_msg = format!("[assert_set_doc] Failed to decode user data: key={}, error={}", 
                context.data.key, e);
            logger!("error", "{}", err_msg);
            format!("Invalid user data format: {}", e)
        })?;
    
    // Step 2: Validate the document key by comparing with a freshly generated key
    // Extract the ULID from usr_key if available, otherwise this is invalid
    if let Some(usr_key) = &user_data.usr_key {
        // Generate a key based on the document data
        let expected_key = create_user_key(Some(usr_key), &user_data.username).await
            .map_err(|e| {
                let err_msg = format!("[assert_set_doc] Failed to generate comparisson key for validation: {}", e);
                logger!("error", "{}", err_msg);
                e
            })?;
        
        // Compare the generated key with the provided key
        if context.data.key != expected_key {
            let err_msg = format!(
                "[assert_set_doc] Document key mismatch. Provided: {}, Expected: {}",
                context.data.key, expected_key
            );
            logger!("error", "{}", err_msg);
            return Err(err_msg);
        }
        
        logger!("debug", "[assert_set_doc] Document key validated successfully: {}", context.data.key);
    } else {
        let err_msg = format!("[assert_set_doc] Missing usr_key field in user data");
        logger!("error", "{}", err_msg);
        return Err(err_msg);
    }
    
    // Step 3: Validate username format and restrictions
    validate_username(&user_data.username)
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
    if let Some(existing_doc) = junobuild_satellite::get_doc(String::from("users"), username_key_part) {
        // If we found a document with this username
        // Check if it's the same document (in case of update)
        if !is_update || existing_doc.key != context.data.key {
            // Found another document with the same username
            let err_msg = format!(
                "[assert_set_doc] Username '{}' is already taken. Please choose a different username.",
                user_data.username
            );
            logger!("error", "{}", err_msg);
            return Err(err_msg);
        }
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
