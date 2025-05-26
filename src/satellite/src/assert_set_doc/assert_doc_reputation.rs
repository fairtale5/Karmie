use crate::logger;
use junobuild_satellite::AssertSetDocContext;
use junobuild_utils::decode_doc_data;
use crate::utils::structs::ReputationData;
use crate::processors::document_keys::{validate_reputation_key, format_reputation_key};

/// Validates a reputation document before creation or update
/// 
/// This function performs validation of reputation documents:
/// 1. Verifies collection name is "reputations"
/// 2. Decodes and validates the basic reputation data structure
/// 3. Validates key format using document_keys validation src/satellite/src/processors/document_keys.rs
/// 4. Validates field constraints (voting rewards non-negative, vote weight in range)
/// 
/// # Arguments
/// * `context` - The validation context containing:
///   - caller: The Principal ID of the user making the request
///   - collection: Must be "reputations"
///   - key: The document key in format: usr_{ulid}_tag_{ulid}_
///   - data.data.proposed.data: The binary data of the proposed document
/// 
/// # Returns
/// * `Result<(), String>` - Ok if validation passes, Err with detailed message if it fails
pub fn validate_reputation_document(context: &AssertSetDocContext) -> Result<(), String> {
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

    // Step 3: Validate the key format using document_keys validation
    // This ensures the key follows our standardized format: usr_{user.data.key}_tag_{tag.data.ulid}_
    validate_reputation_key(&context.data.key)
        .map_err(|e| {
            logger!("error", "[validate_reputation_document] Invalid reputation key format: {}", e);
            format!("Invalid reputation key format: {}", e)
        })?;
    
    // Step 3.1: Verify the key matches the data
    // Generate the expected key from the data and compare with actual key
    let expected_key = format_reputation_key(&rep_data.owner_ulid, &rep_data.tag_ulid)
        .map_err(|e| {
            logger!("error", "[validate_reputation_document] Failed to format reputation key: {}", e);
            format!("Failed to format reputation key: {}", e)
        })?;
    
    if context.data.key != expected_key {
        let err_msg = format!(
            "[validate_reputation_document] Key does not match data. Expected: {}, Got: {}",
            expected_key, context.data.key
        );
        logger!("error", "{}", err_msg);
        return Err(err_msg);
    }

    // Step 4: Validate field constraints
    // Check that the values are within acceptable ranges

    // 4.1: Log total_basis_reputation (can be negative or positive)
    logger!("debug", "[validate_reputation_document] Total basis reputation: {}", rep_data.reputation_basis );

    // 4.2: Validate voting rewards (must be non-negative)
    if rep_data.reputation_rewards < 0.0 {
        let err_msg = format!(
            "[validate_reputation_document] Total voting rewards reputation cannot be negative (got: {})",
            rep_data.reputation_rewards
        );
        logger!("error", "{}", err_msg);
        return Err(err_msg);
    }

    // 4.3: Validate vote weight (must be between 0.0 and 1.0)
    // Use vote_weight.value() to access the underlying f64 value for comparison
    let weight_value = rep_data.vote_weight.value();
    if weight_value < 0.0 || weight_value > 1.0 {
        let err_msg = format!(
            "[validate_reputation_document] Vote weight must be between 0.0 and 1.0 (got: {})",
            weight_value
        );
        logger!("error", "{}", err_msg);
        return Err(err_msg);
    }

    logger!("info", "[validate_reputation_document] Successfully validated reputation document: key={}", context.data.key );

    Ok(())
}
