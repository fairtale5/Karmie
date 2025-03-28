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
 * Description field formats (see docs/core/architecture/database.md):
 * - Reputation docs: [owner:{principal_or_key}][tag:{tag_key}]
 * - Vote docs: [owner:{principal_or_key}][target:{target_key}][tag:{tag_key}]
 * - Tag docs: [owner:{principal_or_key}][name:{tag_name}]
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

    // Step 3: Count users above threshold
    let active_count = reputations.items.iter()
        .filter_map(|(doc_key, doc)| {  // doc_key is the document's key in the collection
            match decode_doc_data(&doc.data) {
                Ok(rep_data) => {
                    let rep_data: ReputationData = rep_data;
                    if rep_data.last_known_effective_reputation >= threshold {
                        Some(1)
                    } else {
                        None
                    }
                },
                Err(e) => {
                    // Use log_with_prefix for non-fatal errors - we can continue processing other documents
                    log_with_prefix("ERROR", &format!("Failed to decode reputation data for document {}: {}", doc_key, e));
                    None
                }
            }
        })
        .sum();

    log_info(&format!("Found {} active users for tag {}", active_count, tag_key));
    Ok(active_count)
} 