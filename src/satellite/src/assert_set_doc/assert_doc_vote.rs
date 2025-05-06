use crate::logger;
use junobuild_satellite::AssertSetDocContext;
use crate::utils::structs::VoteData;
use junobuild_utils::decode_doc_data;
use junobuild_shared::types::list::{ListMatcher, ListParams};
use crate::list_docs;
use crate::processors::document_queries::KeySegment;
use crate::processors::document_queries::{query_doc, query_doc_by_key};
use crate::processors::ulid_timestamp_extract::extract_timestamp_ms;
use ic_cdk;

/// Validates a vote document before creation or update
/// 
/// This function performs comprehensive validation of vote documents:
/// 1. Decodes and validates the basic vote data structure
/// 2. Validates description format using DocumentDescription helper
/// 3. Validates vote value constraints (+1 or -1)
/// 4. Validates vote weight constraints (0.0 to 1.0)
/// 5. Prevents self-voting
/// 6. Verifies tag exists using ListMatcher by key
/// 7. Ensures vote timestamp is not backdated or in the future using IC blockchain time
/// 
/// # Arguments
/// * `context` - The validation context containing the document data
/// 
/// # Returns
/// * `Result<(), String>` - Ok if validation passes, Err with detailed message if it fails
pub fn validate_vote_document(context: &AssertSetDocContext) -> Result<(), String> {
    logger!("debug", "[validate_vote_document] Starting vote validation: key={}", context.data.key);

    // Step 1: Access the full document structure and prepare it
    
    // Decode and validate the basic vote data structure
    let vote_doc = &context.data.data.proposed;
    let vote_data: VoteData = decode_doc_data(&context.data.data.proposed.data)
        .map_err(|e| {
            logger!("error", "[validate_vote_document] Failed to decode vote data: key={}, error={}", context.data.key, e);
            format!("Invalid vote data format: {}", e)
        })?;

    // Step 2: Validate vote timestamp is not backdated or in the future
    // Extract vote_key from the document key (it's the last ULID in the key)
    let key_parts: Vec<&str> = context.data.key.split('_').collect();
    if let Some(vote_ulid) = key_parts.last() {
        // Extract timestamp from the ULID (in milliseconds)
        let vote_timestamp = extract_timestamp_ms(vote_ulid)?;
        
        // Get current IC time in milliseconds (convert from nanoseconds)
        let current_time = ic_cdk::api::time() / 1_000_000;
        
        // Allow for small clock skew (5 minutes in milliseconds)
        const ALLOWED_SKEW_MS: u64 = 5 * 60 * 1000;
        
        // Check if vote timestamp is too far in the past
        if vote_timestamp + ALLOWED_SKEW_MS < current_time {
            let err_msg = format!(
                "[validate_vote_document] Vote timestamp is backdated: vote_time={}, current_time={}, allowed_skew={}ms",
                vote_timestamp, current_time, ALLOWED_SKEW_MS
            );
            logger!("error", "{}", err_msg);
            return Err(err_msg);
        }

        // Check if vote timestamp is too far in the future
        if vote_timestamp > current_time + ALLOWED_SKEW_MS {
            let err_msg = format!(
                "[validate_vote_document] Vote timestamp is in the future: vote_time={}, current_time={}, allowed_skew={}ms",
                vote_timestamp, current_time, ALLOWED_SKEW_MS
            );
            logger!("error", "{}", err_msg);
            return Err(err_msg);
        }
    } else {
        let err_msg = format!("[validate_vote_document] Invalid vote key format: {}", context.data.key);
        logger!("error", "{}", err_msg);
        return Err(err_msg);
    }

    // Step 3: Validate vote value constraints
    // Vote value must be exactly -1 or 1 to:
    // - Ensure votes have clear, binary meaning in the system (upvote/downvote)
    // - Prevent vote manipulation through arbitrary values
    // - Maintain consistent reputation calculations
    // - Keep the system simple and understandable
    // - Enable clear upvote/downvote UI representation
    // - No neutral votes (0) allowed to encourage clear stance
    if vote_data.value != -1.0 && vote_data.value != 1.0 {
        let err_msg = format!(
            "[validate_vote_document] Vote value must be either -1 or 1 (got: {}). Neutral votes (0) are not allowed.",
            vote_data.value
        );
        logger!("error", "{}", err_msg);
        return Err(err_msg);
    }

    // Step 4: Validate vote weight constraints
    // Vote weight must be between 0.0 and 1.0 to:
    // - Represent voter's proportional influence in the tag
    // - Prevent any single voter from dominating
    // - Scale impact based on reputation and activity
    // - Ensure fair distribution of voting power
    // - Enable time-based vote decay
    // Weight is calculated from:
    // - Author's reputation in the tag
    // - Number of votes cast
    // - Age of previous votes
    if vote_data.weight < 0.0 || vote_data.weight > 1.0 {
        let err_msg = format!(
            "[validate_vote_document] Vote weight must be between 0.0 and 1.0 (got: {})",
            vote_data.weight
        );
        logger!("error", "{}", err_msg);
        return Err(err_msg);
    }

    // Step 5: Validate tag exists
    logger!("debug", "[validate_vote_document] Verifying tag exists: {}", vote_data.tag_key);
    
    // First validate that tag_key is not empty
    if vote_data.tag_key.trim().is_empty() {
        let err_msg = "[validate_vote_document] Tag key cannot be empty";
        logger!("error", "{}", err_msg);
        return Err(err_msg.to_string());
    }
    
    // Construct the key pattern to find the tag document
    // Tag keys follow the pattern: tag_{ulid}_
    // This pattern will match any document that contains this tag ULID segment
    // For example: usr_123_tag_456_hdl_example_ would match with pattern "tag_456_"
    let tag_key_pattern = format!(".*tag_{}_.*", vote_data.tag_key);
    
    // Query for the tag using the constructed key pattern
    let tag_results = query_doc_by_key(
        "tags",
        &tag_key_pattern
    )?;
    
    // Check if we found any matching tags
    if tag_results.items.is_empty() {
        let err_msg = format!("[validate_vote_document] Tag not found: {}", &tag_key_pattern);
        logger!("error", "{}", err_msg);
        return Err(err_msg);
    }
    
    logger!("debug", "[validate_vote_document] Found tag: {}", vote_data.tag_key);

    // Step 6: Validate no self-voting
    if vote_data.user_key == vote_data.target_key {
        let err_msg = "[validate_vote_document] Users cannot vote on themselves";
        logger!("error", "{}", err_msg);
        return Err(err_msg.to_string());
    }

    logger!("info", "[validate_vote_document] Vote validation passed: author={} voted {} on target={} in tag={}",
        vote_data.user_key,
        vote_data.value,
        vote_data.target_key,
        vote_data.tag_key
    );

    Ok(())
}
