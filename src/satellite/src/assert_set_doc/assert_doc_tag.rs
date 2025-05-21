use junobuild_satellite::AssertSetDocContext;
use junobuild_utils::decode_doc_data;
use junobuild_shared::types::list::{ListMatcher, ListParams};
use crate::{
    validation::{validate_handle, validate_tag_date_struct},
    utils::structs::TagData,
    list_docs,
    logger,
    utils::normalize::normalize_handle,
    processors::document_queries::query_doc_by_key,
};

/// Validates a tag document before creation or update
/// 
/// This function performs comprehensive validation of tag documents:
/// 1. Decodes and validates the basic tag data structure
/// 2. Validates tag name format and restrictions (using username validation)
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
pub fn validate_tag_document(context: &AssertSetDocContext) -> Result<(), String> {
    // Step 1: Decode and validate the basic tag data structure
    let tag_data: TagData = decode_doc_data(&context.data.data.proposed.data)
        .map_err(|e| {
            logger!("error", "[assert_set_doc] Failed to decode tag data: key={}, error={}", context.data.key, e);
            format!("Invalid tag data format: {}", e)
        })?;
    
    // Step 2: Validate tag name format using username validation patterns
    // This treats the tag's short name like a username with same constraints
    validate_handle(&tag_data.tag_handle)
        .map_err(|e| {
            let err_msg = format!("[assert_set_doc] Tag name validation failed: {}", e);
            logger!("error", "{}", err_msg);
            err_msg
        })?;

    // Check for tag name uniqueness using normalized handle (normalized so that names like john123 and JOHN123)
    let normalized_name = normalize_handle(&tag_data.tag_handle);
    
    // Query for existing tags with this handle
    let existing_tags = query_doc_by_key("tags", &format!("hdl_{}_", normalized_name))?;

    // Check if any tags were found (excluding the current document if it's an update)
    if !existing_tags.items.is_empty() {
        for (doc_key, doc) in existing_tags.items {
            // Check if this is an update by looking at current data
            let is_update = context.data.data.current.is_some();
    
            // For updates, skip if we're looking at the current document
            if is_update && doc_key == context.data.key {
                logger!("debug", "[validate_tag_document] Skipping current document during update check");
                continue;
            }
        
            // If we get here, we found a duplicate tag
            let err_msg = format!("Tag with name '{}' already exists", tag_data.tag_handle);
            logger!("error", "[validate_tag_document] {}", err_msg);
            return Err(err_msg);
        }
    }

    // Step 3: Validate description length (max 1024 characters)
    if tag_data.description.len() > 1024 {
        let err_msg = format!(
            "[validate_tag_document] Tag description cannot exceed 1024 characters (current length: {})",
            tag_data.description.len()
        );
        logger!("error", "{}", err_msg);
        return Err(err_msg);
    }

    // Step 4: Validate time periods using the struct validation function
    validate_tag_date_struct(&tag_data.time_periods)?;

    // Step 5: Validate vote reward (0.0 to 1.0)
    if tag_data.vote_reward < 0.0 || tag_data.vote_reward > 1.0 {
        let err_msg = format!(
            "[validate_tag_document] Vote reward must be between 0.0 and 1.0 (got: {})",
            tag_data.vote_reward
        );
        logger!("error", "{}", err_msg);
        return Err(err_msg);
    }

    // Step 6: Validate minimum users (must be greater than 0)
    if tag_data.min_users_for_threshold == 0 {
        let err_msg = format!(
            "[validate_tag_document] Minimum users must be greater than 0 (got: {})",
            tag_data.min_users_for_threshold
        );
        logger!("error", "{}", err_msg);
        return Err(err_msg);
    }

    Ok(())
}
