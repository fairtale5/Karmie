//! Document Key Management
//! 
//! This module provides utilities for generating, validating, and parsing
//! document keys for various Juno collections. It follows the format specified
//! in the database schema documentation, utilizing ULIDs for unique identifiers.
//!
//! Each document type has specific key format requirements:
//! - Users: `usr_{ulid}_usrName_{username}_`
//! - Tags: `usr_{ulid}_tag_{ulid}_tagName_{tagName}_`
//! - Reputations: `usr_{ulid}_tag_{ulid}`
//! - Votes: `usr_{ulid}_tag_{ulid}_tar_{ulid}_key_{ulid}_`
//!
//! This module ensures consistent key formatting across backend and frontend.

use crate::processors::ulid_generator::{generate_ulid, validate_ulid};
use ic_cdk;
use std::collections::HashMap;
use regex::Regex;

/// Type alias for ULID strings
pub type ULID = String;

// ===== Common Utility Functions =====

/// Sanitizes a string for use in document keys
/// 
/// Converts to lowercase, removes spaces and special characters
/// Keeps alphanumeric and hyphen characters
///
/// # Arguments
/// * `input` - The string to sanitize
///
/// # Returns
/// * `String` - Sanitized string
pub fn sanitize_for_key(input: &str) -> String {
    // Remove spaces, special characters, and convert to lowercase
    let alphanumeric_hyphen = input
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>()
        .to_lowercase();
    
    alphanumeric_hyphen
}

/// Extracts components from a document key
///
/// # Arguments
/// * `key` - The document key to parse
///
/// # Returns
/// * `Result<HashMap<String, String>, String>` - Map of components or error
pub fn parse_key(key: &str) -> Result<HashMap<String, String>, String> {
    let mut components = HashMap::new();
    
    // Split the key by underscore
    let parts: Vec<&str> = key.split('_').collect();
    
    // Basic validation
    if parts.len() < 3 {
        return Err("Key has insufficient components".to_string());
    }
    
    // Start extracting components
    let mut i = 0;
    
    // Extract components based on pattern recognition
    while i < parts.len() - 1 {
        match parts[i] {
            "usr" => {
                if i + 1 < parts.len() && validate_ulid(parts[i+1]).is_ok() {
                    components.insert("usr_key".to_string(), parts[i+1].to_string());
                    i += 2;
                } else {
                    return Err("Invalid usr_key format".to_string());
                }
            },
            "hdl" => {
                // Extract handle (might span multiple parts due to underscores in handle)
                let mut handle = parts[i+1].to_string();
                i += 2;
                // Keep collecting until we hit another known component
                while i < parts.len() && !["usr", "tag", "tar", "key", "hdl"].contains(&parts[i]) {
                    handle.push('_');
                    handle.push_str(parts[i]);
                    i += 1;
                }
                components.insert("handle".to_string(), handle);
            },
            "tag" => {
                if i + 1 < parts.len() && validate_ulid(parts[i+1]).is_ok() {
                    components.insert("tag_key".to_string(), parts[i+1].to_string());
                    i += 2;
                } else {
                    return Err("Invalid tag_key format".to_string());
                }
            },
            "tar" => {
                if i + 1 < parts.len() && validate_ulid(parts[i+1]).is_ok() {
                    components.insert("tar_key".to_string(), parts[i+1].to_string());
                    i += 2;
                } else {
                    return Err("Invalid tar_key format".to_string());
                }
            },
            "key" => {
                if i + 1 < parts.len() && validate_ulid(parts[i+1]).is_ok() {
                    components.insert("vote_key".to_string(), parts[i+1].to_string());
                    i += 2;
                } else {
                    return Err("Invalid vote_key format".to_string());
                }
            },
            _ => {
                // Skip unknown parts
                i += 1;
            }
        }
    }
    
    Ok(components)
}

// ===== Document-Specific Key Generation Functions =====

/// Create a new user document key with generated ULID
/// 
/// # Arguments
/// * `handle` - User's handle (username)
/// 
/// # Returns
/// * `Result<String, String>` - Formatted key or error
/// 
/// # Example
/// ```rust
/// // Creates: "usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_hdl_johndoe_"
/// let key = create_user_key("johndoe").await?;
/// ```
pub async fn create_user_key(handle: &str) -> Result<String, String> {
    let user_ulid = generate_ulid().await;
    format_user_key(&user_ulid, handle)
}

/// Format a user key with existing ULID
/// 
/// # Arguments
/// * `ulid` - ULID for the user, must be uppercase
/// * `handle` - Username, will be sanitized
///
/// # Returns
/// * `Result<String, String>` - Formatted key or error
pub fn format_user_key(ulid: &str, handle: &str) -> Result<String, String> {
    validate_ulid(ulid)?;
    let sanitized_handle = sanitize_for_key(handle);
    
    if sanitized_handle.len() < 3 || sanitized_handle.len() > 30 {
        return Err("Handle must be between 3 and 30 characters".to_string());
    }
    
    // Format: usr_ULID_hdl_handle_
    Ok(format!("usr_{}_hdl_{}_", ulid, sanitized_handle))
}

/// Create a tag document key
/// 
/// # Arguments
/// * `user_ulid` - ULID of the user creating the tag
/// * `tag_name` - Name of the tag
/// 
/// # Returns
/// * `Result<String, String>` - Formatted key or error
pub async fn create_tag_key(user_ulid: &str, tag_name: &str) -> Result<String, String> {
    let tag_ulid = generate_ulid().await;
    format_tag_key(user_ulid, &tag_ulid, tag_name)
}

/// Format a tag key with existing ULIDs
/// 
/// # Arguments
/// * `user_ulid` - ULID of the user creating the tag
/// * `tag_ulid` - ULID for the tag
/// * `tag_name` - Name of the tag
/// 
/// # Returns
/// * `Result<String, String>` - Formatted key or error
pub fn format_tag_key(user_ulid: &str, tag_ulid: &str, tag_name: &str) -> Result<String, String> {
    validate_ulid(user_ulid)?;
    validate_ulid(tag_ulid)?;
    let sanitized_name = sanitize_for_key(tag_name);
    
    if sanitized_name.len() < 3 || sanitized_name.len() > 30 {
        return Err("Tag name must be between 3 and 30 characters".to_string());
    }
    
    // Format: usr_ULID_tag_ULID_hdl_name_
    Ok(format!("usr_{}_tag_{}_hdl_{}_", user_ulid, tag_ulid, sanitized_name))
}

/// Creates a reputation document key with generated ULID for the reputation
///
/// # Arguments
/// * `user_ulid` - ULID of the user
/// * `tag_ulid` - ULID of the tag
///
/// # Returns
/// * `Result<String, String>` - Formatted key or error message
pub async fn create_reputation_key(user_ulid: &str, tag_ulid: &str) -> Result<String, String> {
    format_reputation_key(user_ulid, tag_ulid)
}

/// Format a reputation key with existing ULIDs
///
/// Use this function for validation and key formatting that doesn't require 
/// async operations or new ULIDs generation.
///
/// Format: usr_{userUlid}_tag_{tagUlid}_
///
/// # Arguments
/// * `user_ulid` - ULID of the user
/// * `tag_ulid` - ULID of the tag
///
/// # Returns
/// * `Result<String, String>` - Formatted key or error message
pub fn format_reputation_key(user_ulid: &str, tag_ulid: &str) -> Result<String, String> {
    // Validate ULIDs
    validate_ulid(user_ulid)?;
    validate_ulid(tag_ulid)?;
    
    // Format key with trailing underscore
    Ok(format!("usr_{}_tag_{}_", user_ulid, tag_ulid))
}

/// Creates a vote document key
///
/// Format: usr_{userUlid}_tag_{tagUlid}_tar_{targetUlid}_key_{voteUlid}_
///
/// # Arguments
/// * `user_ulid` - ULID of the voter
/// * `tag_ulid` - ULID of the tag
/// * `target_ulid` - ULID of the target user
/// * `vote_ulid` - Optional ULID for the vote, generated if None
///
/// # Returns
/// * `Result<String, String>` - Formatted key or error message
pub async fn create_vote_key(
    user_ulid: &str, 
    tag_ulid: &str, 
    target_ulid: &str, 
    vote_ulid: Option<&str>
) -> Result<String, String> {
    // Validate ULIDs
    validate_ulid(user_ulid)?;
    validate_ulid(tag_ulid)?;
    validate_ulid(target_ulid)?;
    
    // Validate or generate vote ULID
    let vote_id = match vote_ulid {
        Some(id) => {
            validate_ulid(id)?;
            id.to_string()
        },
        None => generate_ulid().await
    };
    
    // Format key
    Ok(format!("usr_{}_tag_{}_tar_{}_key_{}_", 
        user_ulid, tag_ulid, target_ulid, vote_id))
}

// ===== Document-Specific Key Validation Functions =====

/// Validates a user document key format
///
/// # Arguments
/// * `key` - The key to validate
///
/// # Returns
/// * `Result<(), String>` - Ok if valid, Err with message if invalid
pub fn validate_user_key(key: &str) -> Result<(), String> {
    let user_key_pattern = Regex::new(
        r"^usr_[0123456789ABCDEFGHJKMNPQRSTVWXYZ]{26}_usrName_[a-z0-9\-]+_$"
    ).unwrap();
    
    if !user_key_pattern.is_match(key) {
        return Err(format!("Invalid user key format: {}", key));
    }
    
    // Parse key to extract components
    let components = parse_key(key)?;
    
    // Ensure required components are present
    if !components.contains_key("usr_key") || !components.contains_key("handle") {
        return Err("User key missing required components".to_string());
    }
    
    // Username validation
    if let Some(handle) = components.get("handle") {
        if handle.len() < 3 || handle.len() > 30 {
            return Err("Handle must be between 3 and 30 characters".to_string());
        }
    }
    
    Ok(())
}

/// Validates a tag document key format
///
/// # Arguments
/// * `key` - The key to validate
///
/// # Returns
/// * `Result<(), String>` - Ok if valid, Err with message if invalid
pub fn validate_tag_key(key: &str) -> Result<(), String> {
    let tag_key_pattern = Regex::new(
        r"^usr_[0123456789ABCDEFGHJKMNPQRSTVWXYZ]{26}_tag_[0123456789ABCDEFGHJKMNPQRSTVWXYZ]{26}_hdl_[a-z0-9\-]+_$"
    ).unwrap();
    
    if !tag_key_pattern.is_match(key) {
        return Err(format!("Invalid tag key format: {}", key));
    }
    
    // Parse key to extract components
    let components = parse_key(key)?;
    
    // Ensure required components are present
    if !components.contains_key("usr_key") || 
       !components.contains_key("tag_key") || 
       !components.contains_key("tagname") {
        return Err("Tag key missing required components".to_string());
    }
    
    // Tag name validation
    if let Some(tagname) = components.get("tagname") {
        if tagname.len() < 3 || tagname.len() > 30 {
            return Err("Tag name must be between 3 and 30 characters".to_string());
        }
    }
    
    Ok(())
}

/// Validates a reputation document key format
///
/// # Arguments
/// * `key` - The key to validate
///
/// # Returns
/// * `Result<(), String>` - Ok if valid, Err with message if invalid
pub fn validate_reputation_key(key: &str) -> Result<(), String> {
    let reputation_key_pattern = Regex::new(
        r"^usr_[0123456789ABCDEFGHJKMNPQRSTVWXYZ]{26}_tag_[0123456789ABCDEFGHJKMNPQRSTVWXYZ]{26}_$"
    ).unwrap();
    
    if !reputation_key_pattern.is_match(key) {
        return Err(format!("Invalid reputation key format: {}", key));
    }
    
    // Parse key to extract components
    let components = parse_key(key)?;
    
    // Ensure required components are present
    if !components.contains_key("usr_key") || !components.contains_key("tag_key") {
        return Err("Reputation key missing required components".to_string());
    }
    
    Ok(())
}

/// Validates a vote document key format
///
/// # Arguments
/// * `key` - The key to validate
///
/// # Returns
/// * `Result<(), String>` - Ok if valid, Err with message if invalid
pub fn validate_vote_key(key: &str) -> Result<(), String> {
    let vote_key_pattern = Regex::new(
        r"^usr_[0123456789ABCDEFGHJKMNPQRSTVWXYZ]{26}_tag_[0123456789ABCDEFGHJKMNPQRSTVWXYZ]{26}_tar_[0123456789ABCDEFGHJKMNPQRSTVWXYZ]{26}_key_[0123456789ABCDEFGHJKMNPQRSTVWXYZ]{26}_$"
    ).unwrap();
    
    if !vote_key_pattern.is_match(key) {
        return Err(format!("Invalid vote key format: {}", key));
    }
    
    // Parse key to extract components
    let components = parse_key(key)?;
    
    // Ensure required components are present
    if !components.contains_key("usr_key") || 
       !components.contains_key("tag_key") || 
       !components.contains_key("tar_key") ||
       !components.contains_key("vote_key") {
        return Err("Vote key missing required components".to_string());
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_key() {
        // Test parse user key
        let user_key = "usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_hdl_johndoe_";
        let components = parse_key(user_key).unwrap();
        assert_eq!(components.get("usr_key").unwrap(), "01ARZ3NDEKTSV4RRFFQ69G5FAV");
        assert_eq!(components.get("handle").unwrap(), "johndoe");
        
        // Test parse tag key
        let tag_key = "usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW_hdl_technical-skills_";
        let components = parse_key(tag_key).unwrap();
        assert_eq!(components.get("usr_key").unwrap(), "01ARZ3NDEKTSV4RRFFQ69G5FAV");
        assert_eq!(components.get("tag_key").unwrap(), "01ARZ3NDEKTSV4RRFFQ69G5FAW");
        assert_eq!(components.get("handle").unwrap(), "technical-skills");
        
        // Test parse reputation key
        let rep_key = "usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW_";
        let components = parse_key(rep_key).unwrap();
        assert_eq!(components.get("usr_key").unwrap(), "01ARZ3NDEKTSV4RRFFQ69G5FAV");
        assert_eq!(components.get("tag_key").unwrap(), "01ARZ3NDEKTSV4RRFFQ69G5FAW");
        
        // Test parse vote key
        let vote_key = "usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW_tar_01ARZ3NDEKTSV4RRFFQ69G5FAX_key_01ARZ3NDEKTSV4RRFFQ69G5FAY_";
        let components = parse_key(vote_key).unwrap();
        assert_eq!(components.get("usr_key").unwrap(), "01ARZ3NDEKTSV4RRFFQ69G5FAV");
        assert_eq!(components.get("tag_key").unwrap(), "01ARZ3NDEKTSV4RRFFQ69G5FAW");
        assert_eq!(components.get("tar_key").unwrap(), "01ARZ3NDEKTSV4RRFFQ69G5FAX");
        assert_eq!(components.get("vote_key").unwrap(), "01ARZ3NDEKTSV4RRFFQ69G5FAY");
        
        // Test invalid key
        let invalid_key = "invalid_key_format";
        assert!(parse_key(invalid_key).is_err());
    }
    
    #[tokio::test]
    async fn test_create_user_key() {
        // Test with handle
        let key = create_user_key("John Doe").await.unwrap();
        assert!(key.starts_with("usr_"));
        assert!(key.ends_with("_hdl_johndoe_"));
        assert!(validate_user_key(&key).is_ok());
        
        // Test with invalid handle (too short)
        let result = create_user_key("ab").await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_create_tag_key() {
        // Test with valid input
        let key = create_tag_key(
            "01ARZ3NDEKTSV4RRFFQ69G5FAV",
            "Technical-Skills"
        ).await.unwrap();
        
        assert!(key.starts_with("usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_"));
        assert!(key.ends_with("_hdl_technical-skills_"));
        assert!(validate_tag_key(&key).is_ok());
        
        // Test with another valid input
        let key = create_tag_key(
            "01ARZ3NDEKTSV4RRFFQ69G5FAV",
            "Programming"
        ).await.unwrap();
        
        assert!(key.starts_with("usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_"));
        assert!(key.ends_with("_hdl_programming_"));
        assert!(validate_tag_key(&key).is_ok());
        
        // Test with invalid tag name (too short)
        let result = create_tag_key(
            "01ARZ3NDEKTSV4RRFFQ69G5FAV",
            "py"
        ).await;
        assert!(result.is_err());
    }
    
    #[test]
    fn test_create_reputation_key() {
        // Test with valid ULIDs
        let key = create_reputation_key(
            "01ARZ3NDEKTSV4RRFFQ69G5FAV",
            "01ARZ3NDEKTSV4RRFFQ69G5FAW"
        ).unwrap();
        
        assert_eq!(
            key, 
            "usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW_"
        );
        assert!(validate_reputation_key(&key).is_ok());
        
        // Test with invalid ULID
        let result = create_reputation_key(
            "01ARZ3NDEKTSV4RRFFQ69G5FAV",
            "invalid"
        );
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_create_vote_key() {
        // Test with provided ULIDs
        let key = create_vote_key(
            "01ARZ3NDEKTSV4RRFFQ69G5FAV",
            "01ARZ3NDEKTSV4RRFFQ69G5FAW",
            "01ARZ3NDEKTSV4RRFFQ69G5FAX",
            Some("01ARZ3NDEKTSV4RRFFQ69G5FAY")
        ).await.unwrap();
        
        assert_eq!(
            key, 
            "usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW_tar_01ARZ3NDEKTSV4RRFFQ69G5FAX_key_01ARZ3NDEKTSV4RRFFQ69G5FAY_"
        );
        assert!(validate_vote_key(&key).is_ok());
        
        // Test with auto-generated vote ULID
        let key = create_vote_key(
            "01ARZ3NDEKTSV4RRFFQ69G5FAV",
            "01ARZ3NDEKTSV4RRFFQ69G5FAW",
            "01ARZ3NDEKTSV4RRFFQ69G5FAX",
            None
        ).await.unwrap();
        
        assert!(key.starts_with("usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW_tar_01ARZ3NDEKTSV4RRFFQ69G5FAX_key_"));
        assert!(key.ends_with("_"));
        assert!(validate_vote_key(&key).is_ok());
        
        // Test with invalid user ULID
        let result = create_vote_key(
            "invalid",
            "01ARZ3NDEKTSV4RRFFQ69G5FAW",
            "01ARZ3NDEKTSV4RRFFQ69G5FAX",
            None
        ).await;
        assert!(result.is_err());
    }
    
    #[test]
    fn test_parse_key() {
        // Test parse user key
        let user_key = "usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_usrName_johndoe_";
        let components = parse_key(user_key).unwrap();
        assert_eq!(components.get("usr_key").unwrap(), "01ARZ3NDEKTSV4RRFFQ69G5FAV");
        assert_eq!(components.get("username").unwrap(), "johndoe");
        
        // Test parse tag key
        let tag_key = "usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW_tagName_technical-skills_";
        let components = parse_key(tag_key).unwrap();
        assert_eq!(components.get("usr_key").unwrap(), "01ARZ3NDEKTSV4RRFFQ69G5FAV");
        assert_eq!(components.get("tag_key").unwrap(), "01ARZ3NDEKTSV4RRFFQ69G5FAW");
        assert_eq!(components.get("tagname").unwrap(), "technical-skills");
        
        // Test parse reputation key
        let rep_key = "usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW_";
        let components = parse_key(rep_key).unwrap();
        assert_eq!(components.get("usr_key").unwrap(), "01ARZ3NDEKTSV4RRFFQ69G5FAV");
        assert_eq!(components.get("tag_key").unwrap(), "01ARZ3NDEKTSV4RRFFQ69G5FAW");
        
        // Test parse vote key
        let vote_key = "usr_01ARZ3NDEKTSV4RRFFQ69G5FAV_tag_01ARZ3NDEKTSV4RRFFQ69G5FAW_tar_01ARZ3NDEKTSV4RRFFQ69G5FAX_key_01ARZ3NDEKTSV4RRFFQ69G5FAY_";
        let components = parse_key(vote_key).unwrap();
        assert_eq!(components.get("usr_key").unwrap(), "01ARZ3NDEKTSV4RRFFQ69G5FAV");
        assert_eq!(components.get("tag_key").unwrap(), "01ARZ3NDEKTSV4RRFFQ69G5FAW");
        assert_eq!(components.get("tar_key").unwrap(), "01ARZ3NDEKTSV4RRFFQ69G5FAX");
        assert_eq!(components.get("vote_key").unwrap(), "01ARZ3NDEKTSV4RRFFQ69G5FAY");
        
        // Test invalid key
        let invalid_key = "invalid_key_format";
        assert!(parse_key(invalid_key).is_err());
    }
    
    #[test]
    fn test_sanitize_for_key() {
        assert_eq!(sanitize_for_key("John Doe"), "johndoe");
        assert_eq!(sanitize_for_key("Technical-Skills!"), "technical-skills");
        assert_eq!(sanitize_for_key("Programming_123"), "programming123");
        assert_eq!(sanitize_for_key("   spaces   "), "spaces");
        assert_eq!(sanitize_for_key("UPPERCASE"), "uppercase");
        assert_eq!(sanitize_for_key("special@#$%^&*chars"), "specialchars");
    }
} 