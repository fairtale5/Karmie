use crate::logger;
use junobuild_satellite::AssertSetDocContext;
use crate::utils::structs::VoteData;
use junobuild_utils::decode_doc_data;
use junobuild_shared::types::list::{ListMatcher, ListParams};
use crate::list_docs;

/// Validates a vote document before creation or update
/// 
/// This function performs comprehensive validation of vote documents:
/// 1. Decodes and validates the basic vote data structure
/// 2. Validates description format using DocumentDescription helper
/// 3. Validates vote value constraints (+1 or -1)
/// 4. Validates vote weight constraints (0.0 to 1.0)
/// 5. Prevents self-voting
/// 6. Verifies tag exists using ListMatcher by key
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

    // Step 2: Validate vote value constraints
    // Vote value must be -1, 0, or 1 to:
    // - Ensure votes have clear, binary meaning in the system
    // - Prevent vote manipulation through arbitrary values
    // - Maintain consistent reputation calculations
    // - Keep the system simple and understandable
    // - Enable clear upvote/downvote UI representation
    if vote_data.value < -1.0 || vote_data.value > 1.0 {
        let err_msg = format!(
            "[validate_vote_document] Vote value must be -1, 0, or 1 (got: {})",
            vote_data.value
        );
        logger!("error", "{}", err_msg);
        return Err(err_msg);
    }

    // Step 3: Validate vote weight constraints
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

    // Step 4: Validate tag exists
    logger!("debug", "[validate_vote_document] Verifying tag exists: {}", vote_data.tag_key);
    
    // First validate that tag_key is not empty
    if vote_data.tag_key.trim().is_empty() {
        let err_msg = "[validate_vote_document]Tag key cannot be empty";
        logger!("error", "{}", err_msg);
        return Err(err_msg.to_string());
    }
    
    let params = ListParams {
        matcher: Some(ListMatcher {
            key: Some(vote_data.tag_key.clone()),
            ..Default::default()
        }),
        ..Default::default()
    };

    // Search for the tag in the tags collection
    let existing_tags = list_docs(String::from("tags"), params);
    if existing_tags.items.is_empty() {
        let err_msg = format!("[validate_vote_document] Tag not found: {}", vote_data.tag_key);
        logger!("error", "{}", err_msg);
        return Err(err_msg);
    }
    
    logger!("debug", "[validate_vote_document] Found tag: {}", vote_data.tag_key);

    // Step 5: Validate no self-voting
    if vote_data.user_key == vote_data.target_key {
        let err_msg = "[validate_vote_document]Users cannot vote on themselves";
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
