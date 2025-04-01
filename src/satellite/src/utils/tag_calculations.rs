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
use crate::utils::logging::{log_error, log_info, log_with_prefix};
use crate::utils::description_helpers;

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
        log_error(&err_msg); // This is a fatal error - we can't proceed without the tag
        err_msg
    })?;

    // Decode to TagData instead of Tag
    let tag_data: TagData = decode_doc_data(&tag_doc.data)?;
    let threshold = tag_data.reputation_threshold;
    
    // Log the threshold we're using
    log_info(&format!("[get_active_users_count] Tag={} has reputation_threshold={}", tag_key, threshold));

    // Step 2: Get all reputations for this tag
    // Query all reputation documents for this tag with proper description filter
    // We need to use a description-based filter that matches all reputations with this tag
    
    // Create properly formatted description using the DocumentDescription helper
    // Since we want to match any reputation document with this tag, we only filter by the tag field
    let mut desc = description_helpers::DocumentDescription::new();
    desc.add_field("tag", tag_key);
    let description_filter = desc.build();
    
    let reputations: ListResults<_> = list_docs(
        String::from("reputations"),
        ListParams {
            matcher: Some(ListMatcher {
                description: Some(description_filter),
                ..Default::default()
            }),
            ..Default::default()
        },
    );
    
    // Log how many reputation documents we found
    log_info(&format!("[get_active_users_count] Found {} total reputation documents for tag={}", 
        reputations.items.len(), tag_key));

    // Step 3: Count users above threshold
    let mut active_users = 0;
    let mut inactive_users = 0;
    
    for (doc_key, doc) in &reputations.items {
        match decode_doc_data::<ReputationData>(&doc.data) {
            Ok(rep_data) => {
                if rep_data.last_known_effective_reputation >= threshold {
                    // Count active user
                    active_users += 1;
                    log_info(&format!(
                        "[get_active_users_count] ACTIVE: user={}, rep={}, threshold={}",
                        rep_data.user_key, rep_data.last_known_effective_reputation, threshold
                    ));
                } else {
                    // Count inactive user
                    inactive_users += 1;
                    log_info(&format!(
                        "[get_active_users_count] INACTIVE: user={}, rep={}, threshold={}",
                        rep_data.user_key, rep_data.last_known_effective_reputation, threshold
                    ));
                }
            },
            Err(e) => {
                // Use log_with_prefix for non-fatal errors - we can continue processing other documents
                log_with_prefix("ERROR", &format!("Failed to decode reputation data for document {}: {}", doc_key, e));
            }
        }
    }

    log_info(&format!(
        "[get_active_users_count] RESULT: tag={} has {} active users, {} inactive users (threshold={})",
        tag_key, active_users, inactive_users, threshold
    ));
    
    Ok(active_users)
} 