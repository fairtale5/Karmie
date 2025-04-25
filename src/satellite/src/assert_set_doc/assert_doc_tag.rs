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
            logger!("error", "[assert_set_doc] Failed to decode tag data: key={}, error={}", context.data.key, e);
            format!("Invalid tag data format: {}", e)
        })?;
    
    // Step 2: Validate tag name format and uniqueness
    validate_tag_name(&tag_data.name)?;

    // Check for tag name uniqueness (case-insensitive)
    let normalized_name = tag_data.name.to_lowercase();
    
    // Sanitize the tag name to match how it's stored in the description
    let sanitized_name = crate::utils::description_helpers::DocumentDescription::sanitize_key(&normalized_name);
    
    // Build the search query to find any document with this tag name using the new format
    // The pattern will match name=tag_name; in the description string
    let search_pattern = format!("name={};", sanitized_name);
    logger!("debug", "[validate_tag_document] Searching for tag name with pattern: {}", search_pattern);
    
    let params = ListParams {
        matcher: Some(ListMatcher {
            description: Some(search_pattern),
            ..Default::default()
        }),
        ..Default::default()
    };

    // Call list_docs and handle potential errors
    let existing_tags = list_docs(String::from("tags"), params);
    logger!("info", "[validate_tag_document] Found {} existing tags with this name", existing_tags.items.len());
    
    // Check if we found any existing tags with this normalized name
    // For new tags (no key), we check all documents
    // For updates (has key), we exclude the current document
    let is_update = context.data.data.current.is_some();
    logger!("debug", "[validate_tag_document] Is this an update? {}", is_update);
    
    for (doc_key, doc) in existing_tags.items {
        logger!("debug", "[validate_tag_document] Checking document: key={}, description={:?}", doc_key, doc.description);
        
        // If this is an update and the document key matches, skip it
        if is_update && doc_key == context.data.key {
            logger!("debug", "[validate_tag_document] Skipping current document during update");
            continue;
        }
        
        // Parse the description using the DocumentDescription helper
        if let Some(desc_str) = &doc.description {
            if let Ok(desc) = crate::utils::description_helpers::DocumentDescription::parse(desc_str) {
                // Check if the name field matches
                if let Some(existing_name) = desc.get_field("name") {
                    // Now compare the sanitized name values
                    if existing_name == sanitized_name {
                        let err_msg = format!(
                            "[assert_set_doc] Tag name '{}' is already taken (case-insensitive comparison)",
                            tag_data.name
                        );
                        logger!("error", "{}", err_msg);
                        return Err(err_msg);
                    }
                }
            }
        }
    }

    // Step 3: Validate description length
    if tag_data.description.len() > 1024 {
        let err_msg = format!(
            "[validate_tag_document]Tag description cannot exceed 1024 characters (current length: {})",
            tag_data.description.len()
        );
        logger!("error", "{}", err_msg);
        return Err(err_msg);
    }

    // Step 4: Validate time periods
    validate_time_periods(&tag_data.time_periods)?;

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
