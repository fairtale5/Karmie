/// Validates a reputation document before creation or update
/// 
/// This function performs validation of reputation documents:
/// 1. Verifies collection name is "reputations"
/// 2. Decodes and validates the basic reputation data structure
/// 3. Validates description format using DocumentDescription helper
/// 4. Validates field constraints (voting rewards non-negative, vote weight in range)
/// 
/// # Arguments
/// * `context` - The validation context containing:
///   - caller: The Principal ID of the user making the request
///   - collection: Must be "reputations"
///   - key: The document key (nanoid-generated)
///   - data.data.proposed.data: The binary data of the proposed document
///   - data.data.proposed.description: The description field
/// 
/// # Returns
/// * `Result<(), String>` - Ok if validation passes, Err with detailed message if it fails
fn validate_reputation_document(context: &AssertSetDocContext) -> Result<(), String> {
    // Step 1: Verify collection name
    // This ensures we're only validating documents in the correct collection
    if context.data.collection != "reputations" {
        let err_msg = format!(
            "[validate_reputation_document] Invalid collection: expected 'reputations', got '{}'",
            context.data.collection
        );
        logger!("error", "{}", err_msg);
        return Err(err_msg);
    }

    logger!("debug", "[validate_reputation_document] Validating reputation document: key={}", context.data.key );

    // Step 2: Decode and validate the basic reputation data structure
    // This ensures the document contains all required fields in the correct format
    // and that we can properly access the reputation data for further validation
    let rep_data: ReputationData = decode_doc_data(&context.data.data.proposed.data)
        .map_err(|e| {
            logger!("error", "[validate_reputation_document] Failed to decode reputation data: {}", e);
            format!("Failed to decode reputation data: {}", e)
        })?;

    // Step 3: Create and validate description using DocumentDescription helper
    // This ensures the description follows our standardized format:
    // - Playground mode: owner=user_key;tag=tag_key;
    // - Production mode: owner=principal_id;tag=tag_key;
    let mut desc = DocumentDescription::new();
    let caller_string = context.caller.to_string(); // Create a string that lives for the duration of the function
    desc.add_owner(if IS_PLAYGROUND {
        &rep_data.user_key
    } else {
        &caller_string
    })
    .add_field("tag", &rep_data.tag_key);

    let expected_description = desc.build();

    // Verify the description matches our expected format
    if let Some(actual_description) = &context.data.data.proposed.description {
        if actual_description != &expected_description {
            let err_msg = format!(
                "[validate_reputation_document] Invalid description format. Expected: {}, Got: {}",
                expected_description, actual_description
            );
            logger!("error", "{}", err_msg);
            return Err(err_msg);
        }
    } else {
        let err_msg = "[validate_reputation_document] Description field is required for reputation documents";
        logger!("error", "{}", err_msg);
        return Err(err_msg.to_string());
    }

    // Step 4: Validate field constraints
    // Check that the values are within acceptable ranges

    // 4.1: Log total_basis_reputation (can be negative or positive)
    logger!("debug", "[validate_reputation_document] Total basis reputation: {}", rep_data.total_basis_reputation );

    // 4.2: Validate voting rewards (must be non-negative)
    if rep_data.total_voting_rewards_reputation < 0.0 {
        let err_msg = format!(
            "[validate_reputation_document] Total voting rewards reputation cannot be negative (got: {})",
            rep_data.total_voting_rewards_reputation
        );
        logger!("error", "{}", err_msg);
        return Err(err_msg);
    }

    // 4.3: Validate vote weight (must be between 0.0 and 1.0)
    if rep_data.vote_weight.value() < 0.0 || rep_data.vote_weight.value() > 1.0 {
        let err_msg = format!(
            "[validate_reputation_document] Vote weight must be between 0.0 and 1.0 (got: {})",
            rep_data.vote_weight.value()
        );
        logger!("error", "{}", err_msg);
        return Err(err_msg);
    }

    logger!("info", "[validate_reputation_document] Successfully validated reputation document: key={}", context.data.key );

    Ok(())
}
