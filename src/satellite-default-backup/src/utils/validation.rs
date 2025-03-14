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
/// - Length (3-50 characters)
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
    // Check for empty username after trimming
    if username.trim().is_empty() {
        return Err("Username cannot be empty".to_string());
    }

    // Check minimum length
    if username.len() < 3 {
        return Err("Username must be at least 3 characters long".to_string());
    }

    // Check maximum length
    if username.len() > 50 {
        return Err("Username cannot be longer than 50 characters".to_string());
    }

    // Check character set
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err("Username can only contain letters, numbers, underscores, and hyphens".to_string());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username() {
        // Test valid usernames
        assert!(validate_username("user123").is_ok());
        assert!(validate_username("user_name").is_ok());
        assert!(validate_username("user-name").is_ok());

        // Test invalid usernames
        assert!(validate_username("").is_err());
        assert!(validate_username("ab").is_err());
        assert!(validate_username("a".repeat(51).as_str()).is_err());
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
} 