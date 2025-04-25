use junobuild_satellite::AssertSetDocContext;
use junobuild_utils::decode_doc_data;
use junobuild_shared::types::list::{ListMatcher, ListParams};
use crate::utils::{
    validation::{validate_username, validate_display_name},
    structs::UserData,
    description_helpers::DocumentDescription,
};
use crate::list_docs;
use crate::logger;
use crate::IS_PLAYGROUND;

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
pub fn validate_user_document(context: &AssertSetDocContext) -> Result<(), String> {
    // Step 1: Decode and validate the basic user data structure
    // This ensures the document contains all required fields in the correct format
    let user_data: UserData = decode_doc_data(&context.data.data.proposed.data)
        .map_err(|e| {
            let err_msg = format!("[assert_set_doc] Failed to decode user data: key={}, error={}", 
                context.data.key, e);
            logger!("error", "{}", err_msg);
            format!("Invalid user data format: {}", e)
        })?;
    
    // Step 2: Validate username format and restrictions
    validate_username(&user_data.username)
        .map_err(|e| {
            let err_msg = format!("[assert_set_doc] Username validation failed: {}", e);
            logger!("error", "{}", err_msg);
            e
        })?;

    // Step 3: Validate display name format and restrictions
    validate_display_name(&user_data.display_name)
        .map_err(|e| {
            let err_msg = format!("[assert_set_doc] Display name validation failed: {}", e);
            logger!("error", "{}", err_msg);
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
            logger!("error", "[assert_set_doc] {}", err_msg);
            return Err(err_msg);
        }
    } else {
        let err_msg = "Description field is required for user documents";
        logger!("error", "[assert_set_doc] {} key={}", err_msg, context.data.key);
        return Err(err_msg.to_string());
    }

    // Step 5: Ensure username uniqueness across the system
    // First, normalize the username to lowercase for comparison
    let normalized_username = user_data.username.to_lowercase();
    
    // Sanitize the username to match how it's stored in the description
    let sanitized_username = crate::utils::description_helpers::DocumentDescription::sanitize_key(&normalized_username);
    
    // Build the search query to find any document with this username using the new format
    // The pattern will match username=name; in the description string
    let search_pattern = format!("username={};", sanitized_username);
    logger!("debug", "[validate_user_document] Searching for username with pattern: {}", search_pattern);
    
    let params = ListParams {
        matcher: Some(ListMatcher {
            description: Some(search_pattern),
            ..Default::default()
        }),
        ..Default::default()
    };

    // Call list_docs and handle potential errors
    let existing_users = list_docs(String::from("users"), params);
    logger!("info", "[validate_user_document] Found {} existing users with this username", existing_users.items.len());
    
    // When checking username uniqueness, we need to handle two cases:
    // 1. New user: Check that NO existing document has this username
    // 2. User update: Check that no OTHER document (except current) has this username
    let is_update = context.data.data.current.is_some();
    logger!("debug", "[validate_user_document] Document type: {}", if is_update { "updating existing user" } else { "creating new user" });
    
    for (doc_key, doc) in existing_users.items {
        logger!("debug", "[validate_user_document] Checking username uniqueness against document: key={}, description={:?}", doc_key, doc.description);
        
        // When updating an existing user, their current document will be in the results
        // Skip comparing against their own document to allow them to keep their username
        if is_update && doc_key == context.data.key {
            logger!("debug", "[validate_user_document] Skipping user's current document in username uniqueness check");
            continue;
        }
        
        // Parse the description using the DocumentDescription helper
        if let Some(desc_str) = &doc.description {
            if let Ok(desc) = crate::utils::description_helpers::DocumentDescription::parse(desc_str) {
                // Check if the username field matches
                if let Some(existing_username) = desc.get_field("username") {
                    // Now compare the sanitized username values
                    if existing_username == sanitized_username {
                        let err_msg = format!(
                            "[assert_set_doc] Username '{}' is already taken. Please choose a different username.",
                            user_data.username
                        );
                        logger!("error", "{}", err_msg);
                        return Err(err_msg);
                    }
                }
            }
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
                let err_msg = format!("[validate_user_document] Users can only have one account in production mode. key={}", context.data.key);
                logger!("error", "{}", err_msg);
                return Err(err_msg.to_string());
            }
        }
    }

    Ok(())
}
