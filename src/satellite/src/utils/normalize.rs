/*! 
 * Username normalization utilities
 * 
 * This module provides functions for normalizing usernames to ensure consistent
 * comparison and storage. Normalization includes:
 * - Converting to lowercase
 * - Removing leading/trailing whitespace
 * - Handling special characters
 */

/// Normalizes a username for consistent comparison and storage.
/// 
/// This function performs the following transformations:
/// - Converts the username to lowercase
/// - Removes leading and trailing whitespace
/// - Ensures the username only contains allowed characters
/// 
/// # Arguments
/// * `username` - The username to normalize
/// 
/// # Returns
/// A normalized version of the username as a String
/// 
/// # Example
/// ```
/// let normalized = normalize_handle("  UserName123  ");
/// assert_eq!(normalized, "username123");
/// ```
pub fn normalize_handle(username: &str) -> String {
    // First trim any whitespace and convert to lowercase
    let normalized = username.trim().to_lowercase();
    
    // Filter out any characters that aren't alphanumeric, underscore, or hyphen
    normalized
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_username() {
        // Test basic normalization
        assert_eq!(normalize_handle("UserName"), "username");
        
        // Test whitespace handling
        assert_eq!(normalize_handle("  user  name  "), "username");
        
        // Test special characters
        assert_eq!(normalize_handle("user@name!123"), "username123");
        
        // Test allowed special characters
        assert_eq!(normalize_handle("user_name-123"), "user_name-123");
    }
} 