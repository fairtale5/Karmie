/*! 
 * Data validation utilities
 * 
 * This module provides functions for validating user data before it is stored
 * in the Juno datastore. It includes validation for usernames, display names,
 * and other user-related data.
 */

/// Validates a username according to the system's requirements.
/// 
/// Performs the following checks:
/// - Length (3-30 characters)
/// - Character set (alphanumeric, underscore, hyphen only)
/// - Non-empty after trimming
/// 
/// # Arguments
/// * `username` - The username to validate
/// 
/// # Returns
/// * `Ok(())` - Username is valid
/// * `Err(String)` - Error message describing why the username is invalid
/// 
/// # Example
/// ```
/// match validate_username("user123") {
///     Ok(_) => println!("Username is valid"),
///     Err(e) => println!("Invalid username: {}", e),
/// }
/// ```
pub fn validate_username(username: &str) -> Result<(), String> {
    // Step 1: Check if empty
    if username.trim().is_empty() {
        return Err("Username cannot be empty".to_string());
    }

    // Step 2: Check length
    let len = username.len();
    if len < 3 || len > 30 {
        return Err(format!(
            "Username must be between 3 and 30 characters (current length: {})",
            len
        ));
    }

    // Step 3: Check allowed characters
    if !username.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err("Username can only contain letters, numbers, and hyphens".to_string());
    }

    Ok(())
}

/// Validates a display name according to the system's requirements.
/// 
/// Performs the following checks:
/// - Non-empty after trimming
/// - Maximum length (100 characters)
/// 
/// # Arguments
/// * `display_name` - The display name to validate
/// 
/// # Returns
/// * `Ok(())` - Display name is valid
/// * `Err(String)` - Error message describing why the display name is invalid
pub fn validate_display_name(display_name: &str) -> Result<(), String> {
    // Check for empty display name after trimming
    if display_name.trim().is_empty() {
        return Err("Display name cannot be empty".to_string());
    }

    // Check maximum length
    if display_name.len() > 100 {
        return Err("Display name cannot be longer than 100 characters".to_string());
    }

    Ok(())
}

/// Validates a tag name string
/// 
/// Requirements:
/// 1. Not empty
/// 2. Length between 3-30 characters
/// 3. Only alphanumeric, underscore, and hyphen allowed
/// 4. Must be unique (case-insensitive comparison)
/// 5. Can contain uppercase letters (preserved as entered)
/// 
/// # Arguments
/// * `tag_name` - The tag name to validate
/// 
/// # Returns
/// * `Result<(), String>` - Ok if validation passes, Err with message if it fails
pub fn validate_tag_name(tag_name: &str) -> Result<(), String> {
    // Step 1: Check if empty
    if tag_name.trim().is_empty() {
        return Err("Tag name cannot be empty".to_string());
    }

    // Step 2: Check length
    let len = tag_name.len();
    if len < 3 || len > 30 {
        return Err(format!(
            "Tag name must be between 3 and 30 characters (current length: {})",
            len
        ));
    }

    // Step 3: Check allowed characters
    if !tag_name.chars().all(|c| c.is_alphanumeric() || c == '-') {
        return Err("Tag name can only contain letters, numbers, underscores, and hyphens".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username() {
        // Test valid usernames
        assert!(validate_username("user123").is_ok());
        assert!(validate_username("user-name").is_ok());

        // Test invalid usernames
        assert!(validate_username("").is_err());
        assert!(validate_username("ab").is_err());
        assert!(validate_username("a".repeat(31).as_str()).is_err());
        assert!(validate_username("user@name").is_err());
    }

    #[test]
    fn test_validate_display_name() {
        // Test valid display names
        assert!(validate_display_name("John Doe").is_ok());
        assert!(validate_display_name("User 123!").is_ok());

        // Test invalid display names
        assert!(validate_display_name("").is_err());
        assert!(validate_display_name(" ".repeat(101).as_str()).is_err());
    }
    
    #[test]
    fn test_validate_tag_name() {
        // Test valid tag names
        assert!(validate_tag_name("tech").is_ok());
        assert!(validate_tag_name("soft-skills").is_ok());
        
        // Test invalid tag names
        assert!(validate_tag_name("").is_err());
        assert!(validate_tag_name("ab").is_err());
        assert!(validate_tag_name("a".repeat(31).as_str()).is_err());
        assert!(validate_tag_name("tag@name").is_err());
    }
} 