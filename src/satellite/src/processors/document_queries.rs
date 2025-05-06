/*!
 * Query Helper Functions
 * 
 * This module provides efficient key-based query functions that follow Juno's
 * recommended patterns for memory-efficient document retrieval.
 * 
 * Key Features:
 * - Uses key-based queries instead of description field filtering
 * 
 * - Avoids loading entire collections into memory
 * - Provides type-safe query segments
 * - Standardized key patterns for consistent querying
 * 
 * Key Patterns:
 * - Users:  usr_{ulid}_hdl_{handle}_
 * - Tags:   usr_{ulid}_tag_{ulid}_hdl_{handle}_
 * - Votes:  usr_{ulid}_tag_{ulid}_tar_{ulid}_key_{ulid}_
 * 
 * See docs/core/architecture/database.md for more details on query patterns
 * and memory efficiency considerations.
 */

use junobuild_satellite::list_docs_store;
use junobuild_shared::types::list::{ListMatcher, ListParams, ListResults};
use junobuild_satellite::Doc;
use crate::logger;
use ic_cdk;

/// Query documents by exact key pattern
/// 
/// This general-purpose function allows searching for documents using any key pattern.
/// It performs an efficient key-based query without loading the entire collection.
/// 
/// # Arguments
/// * `collection` - The collection to search in ("users", "tags", "votes", "reputations")
/// * `key_pattern` - The exact key pattern to search for
/// 
/// # Returns
/// * `Result<ListResults<Doc>, String>` - Matching documents or error
/// 
/// # Examples
/// ```rust
/// // Find all votes by a user in a tag
/// query_doc_by_key("votes", &format!("usr_{}_tag_{}_", user_key, tag_key))?;
/// 
/// // Find all votes for a target user in a tag
/// query_doc_by_key("votes", &format!("tag_{}_tar_{}_", tag_key, target_key))?;
/// 
/// // Find a specific vote by its key
/// query_doc_by_key("votes", &format!("key_{}_", vote_key))?;
/// 
/// // Find a specific user by handle
/// query_doc_by_key("users", &format!("hdl_{}_", handle))?;
/// 
/// // Find all tags created by a user
/// query_doc_by_key("tags", &format!("usr_{}_", user_key))?;
/// 
/// // Find a specific tag by its ID (matches anywhere in key)
/// query_doc_by_key("tags", &format!("tag_{}_", tag_key))?;
/// ```
pub fn query_doc_by_key(
    collection: &str,
    key_pattern: &str
) -> Result<ListResults<Doc>, String> {
    logger!("debug", "[query_doc_by_key] Querying collection={} with key pattern: {}", 
        collection, key_pattern);

    // Use Juno's efficient key-based query
    list_docs_store(
        ic_cdk::id(),  // Use canister's ID for admin/controller access
        collection.to_string(),
        &ListParams {
            matcher: Some(ListMatcher {
                key: Some(key_pattern.to_string()),
                ..Default::default()
            }),
            ..Default::default()
        },
    ).map_err(|e| {
        logger!("error", "[query_doc_by_key] Query failed: collection={}, pattern={}, error={}", 
            collection, key_pattern, e);
        e
    })
} 