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

/// Types of key segments that can be queried
/// 
/// Each variant represents a different part of a document key that can be
/// searched for. The search pattern will be constructed as: "{prefix}_{query}_"
#[derive(Debug)]
pub enum KeySegment {
    User,    // matches usr_{query}_
    Tag,     // matches tag_{query}_
    Target,  // matches tar_{query}_
    Handle,  // matches hdl_{query}_
    Key,     // matches key_{query}_
}

impl KeySegment {
    /// Get the prefix string for this key segment type
    fn as_prefix(&self) -> &'static str {
        match self {
            KeySegment::User => "usr",
            KeySegment::Tag => "tag",
            KeySegment::Target => "tar",
            KeySegment::Handle => "hdl",
            KeySegment::Key => "key",
        }
    }
}

/// Query documents by key segment pattern
/// Uses efficient key-based indexing without loading full collection
/// 
/// # DEPRECATION NOTICE
/// This function is being phased out in favor of query_doc_by_key().
/// query_doc_by_key() provides more flexibility and control over key patterns,
/// while maintaining the same efficiency benefits.
/// 
/// Reasons for deprecation:
/// - Less flexible than query_doc_by_key for complex queries
/// - Harder to compose multiple key segments
/// - More rigid pattern structure
/// - query_doc_by_key is more universal and maintainable
/// 
/// Please use query_doc_by_key() for new code.
/// 
/// # Memory Efficiency
/// - Uses key-based queries instead of description field filtering
/// - Only loads matching documents instead of entire collection
/// - Follows Juno's recommended patterns for efficient queries
/// 
/// # Arguments
/// * `collection` - Collection name ("users", "tags", "votes", "reputations")
/// * `segment` - Which part of the key to search (usr, tag, tar, hdl, key)
/// * `query` - The value to search for
/// 
/// # Returns
/// * `Result<ListResults<Doc>, String>` - Matching documents or error
/// 
/// # Examples
/// ```rust
/// // Find user by handle
/// query_doc("users", KeySegment::Handle, "johndoe")?;
/// // Searches for pattern: "hdl_johndoe_"
/// 
/// // Find votes by target user
/// query_doc("votes", KeySegment::Target, "01ARZ3NDEKTSV4RRFFQ69G5FAV")?;
/// // Searches for pattern: "tar_01ARZ3NDEKTSV4RRFFQ69G5FAV_"
/// 
/// // Find tag by handle
/// query_doc("tags", KeySegment::Handle, "technical")?;
/// // Searches for pattern: "hdl_technical_"
/// ```
pub fn query_doc(
    collection: &str,
    segment: KeySegment,
    query: &str
) -> Result<ListResults<Doc>, String> {
    // Construct the key pattern: prefix_query_
    let key_pattern = format!("{}_{}_", segment.as_prefix(), query);
    
    logger!("debug", "[query_doc] Querying collection={}, segment={:?}, pattern={}", 
        collection, segment, key_pattern);

    // Use Juno's efficient key-based query with canister ID for admin access
    list_docs_store(
        ic_cdk::id(),  // Use canister's ID for admin/controller access
        collection.to_string(),
        &ListParams {
            matcher: Some(ListMatcher {
                key: Some(key_pattern.clone()),
                ..Default::default()
            }),
            ..Default::default()
        },
    ).map_err(|e| {
        logger!("error", "[query_doc] Query failed: collection={}, pattern={}, error={}", 
            collection, key_pattern, e);
        e
    })
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_segment_prefixes() {
        assert_eq!(KeySegment::User.as_prefix(), "usr");
        assert_eq!(KeySegment::Tag.as_prefix(), "tag");
        assert_eq!(KeySegment::Target.as_prefix(), "tar");
        assert_eq!(KeySegment::Handle.as_prefix(), "hdl");
        assert_eq!(KeySegment::Key.as_prefix(), "key");
    }

    #[test]
    fn test_query_pattern_format() {
        let test_cases = vec![
            (KeySegment::User, "123", "usr_123_"),
            (KeySegment::Handle, "john", "hdl_john_"),
            (KeySegment::Tag, "456", "tag_456_"),
            (KeySegment::Target, "789", "tar_789_"),
            (KeySegment::Key, "abc", "key_abc_"),
        ];

        for (segment, query, expected) in test_cases {
            let pattern = format!("{}_{}_", segment.as_prefix(), query);
            assert_eq!(pattern, expected);
        }
    }
} 