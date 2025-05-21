use crate::logger;

/// Validates a description string against expected format
/// 
/// Requirements:
/// - Length between 0 and 1024 characters
/// - No leading or trailing whitespace
/// - No consecutive whitespace characters
pub fn validate_description(description: &str) -> Result<(), String> {
    if description.len() > 1024 {
        let err_msg = format!("Description must be 1024 characters or less (current length: {})", description.len());
        logger!("error", "[validate_description] {}", err_msg);
        return Err(err_msg);
    }

    if description.trim() != description {
        let err_msg = "Description must not contain leading or trailing whitespace.".to_string();
        logger!("error", "[validate_description] {}", err_msg);
        return Err(err_msg);
    }

    let mut prev_char = ' ';
    for c in description.chars() {
        if c.is_whitespace() && prev_char.is_whitespace() {
            let err_msg = "Description must not contain consecutive whitespace characters.".to_string();
            logger!("error", "[validate_description] {}", err_msg);
            return Err(err_msg);
        }
        prev_char = c;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_description() {
        // Valid descriptions
        assert!(validate_description("").is_ok());
        assert!(validate_description("A valid description").is_ok());
        assert!(validate_description("Description with numbers 123").is_ok());
        assert!(validate_description("Description with symbols !@#").is_ok());
        assert!(validate_description(&"x".repeat(1024)).is_ok()); // Maximum length

        // Invalid descriptions
        assert!(validate_description(" leading space").is_err());
        assert!(validate_description("trailing space ").is_err());
        assert!(validate_description("double  space").is_err());
        assert!(validate_description(&"x".repeat(1025)).is_err()); // Too long
    }
} 