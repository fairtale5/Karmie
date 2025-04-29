/*!
 * Tag-related calculations and utilities
 * 
 * This module provides functions for calculating tag-specific metrics
 * and handling tag-related operations.
 * 
 * IMPORTANT NOTE FOR TEST PHASE:
 * During the initial test/playground phase, all documents are created by the same user
 * (single-user testing environment). Therefore, we use the `description` field for 
 * document identification and relationships instead of the `owner` field.
 * 
 * Description field format standards
 * All documents in the system use a standardized description field format:
 * 
 * * - Reputation docs: owner=principal_or_key;tag=tag_key;
 * * - Vote docs: owner=principal_or_key;target=target_key;tag=tag_key;
 * * - Tag docs: owner=principal_or_key;name=tag_name;
 * 
 * In these formats, "owner" refers to the creator/controller of the document:
 * 
 * * - For votes: The user casting the vote
 * * - For tags: The user creating the tag
 * * - For reputations: The user whose reputation is being tracked
 * 
 * The "owner" field is implemented differently depending on mode:
 * 
 * * - Playground mode: Uses document key (e.g., owner=user_123;)
 * * - Production mode: Uses Principal ID (e.g., owner=2vxsx-fae;)
 * 
 * This approach will change in production to use proper multi-user authentication
 * where document ownership and relationships will be managed through the `owner` field
 * (Principal IDs) instead of the description field.
 * 
 * The owner field format depends on the mode:
 * - Playground mode: Uses document key (e.g., [owner:user_123])
 * - Production mode: Uses Principal ID (e.g., [owner:2vxsx-fae])
 */

use junobuild_satellite::{get_doc, list_docs};
use junobuild_shared::types::list::{ListMatcher, ListParams, ListResults};
use junobuild_utils::decode_doc_data;
use crate::utils::structs::{Tag, ReputationData, TagData};
use crate::logger;
use crate::utils::query_helpers::{query_doc, KeySegment};

/// Calculates the number of active users for a given tag
/// 
/// Active users are defined as users who have a reputation above
/// the tag's minimum reputation threshold.
/// 
/// # Arguments
/// * `tag_key` - The key of the tag to check
/// 
/// # Returns
/// * `Result<u32, String>` - The number of active users or an error message
pub async fn get_active_users_count(tag_key: &str) -> Result<u32, String> {
    // Step 1: Get tag configuration to find threshold
    let tag_doc = get_doc(
        String::from("tags"),      // Collection name first
        tag_key.to_string(),       // Document key second
    ).ok_or_else(|| {
        let err_msg = format!("Tag not found: {}", tag_key);
        logger!("error", "[get_active_users_count] {}", err_msg);
        err_msg
    })?;

    // Decode to TagData instead of Tag
    let tag_data: TagData = decode_doc_data(&tag_doc.data)?;
    let threshold = tag_data.reputation_threshold;
    
    // Log the threshold we're using
    logger!("info", "[get_active_users_count] Tag={} has reputation_threshold={}", tag_key, threshold);

    // Step 2: Get all reputations for this tag
    // Query all reputation documents for this tag 
    // Query for active users in this tag
    logger!("debug", "[get_active_users_count] Querying active users for tag={}", tag_key);
    
    // Use key-based query to find all users in this tag by using query_doc to find any document in 'reputations' collection that contains the tag key
    let results = query_doc(
        "reputations",
        KeySegment::Tag,
        tag_key
    )?;
    
    // Count users with active voting power
    let active_count = results.items.len();
    
    // Log how many reputation documents we found
    logger!("info", "[get_active_users_count] Found {} total reputation documents for tag={}", 
        reputations.items.len(), tag_key);

    // Step 3: Count users above threshold
    let mut active_users = 0;
    let mut inactive_users = 0;
    
    for (doc_key, doc) in results.items {
        match decode_doc_data::<ReputationData>(&doc.data) {
            Ok(rep_data) => {
                if rep_data.reputation_total_effective >= threshold {
                    // Count active user
                    active_users += 1;
                    logger!("info", "[get_active_users_count] ACTIVE: user={}, rep={}, threshold={}",
                        rep_data.usr_key, rep_data.reputation_total_effective, threshold);
                } else {
                    // Count inactive user
                    inactive_users += 1;
                    logger!("info", "[get_active_users_count] INACTIVE: user={}, rep={}, threshold={}",
                        rep_data.usr_key, rep_data.reputation_total_effective, threshold);
                }
            },
            Err(e) => {
                // Log error but continue processing other documents - non-fatal error
                logger!("error", "Failed to decode reputation data for document: {} | Error: {}", doc_key, e);
            }
        }
    }
    
    logger!("info", "[get_active_users_count] RESULT: tag={} has {} active users, {} inactive users (threshold={})",
        tag_key,
        active_users,
        inactive_users,
        threshold
    );
    
    Ok(active_users)
}
