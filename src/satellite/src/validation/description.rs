use crate::logger;

/// Validates a description string against expected format
/// 
/// Requirements:
/// - Length between 0 and 500 characters
/// - No leading or trailing whitespace
/// - No consecutive whitespace characters
pub fn validate_description(description: &str) -> Result<(), String> {
    if description.len() > 200 {
        logger!("error", "[validate_description] Description too long: {}", description);
        return Err("Description must be 200 characters or less.".to_string());
    }

    if description.trim() != description {
        return Err("Description must not contain leading or trailing whitespace.".to_string());
    }

    let mut prev_char = ' ';
    for c in description.chars() {
        if c.is_whitespace() && prev_char.is_whitespace() {
            return Err("Description must not contain consecutive whitespace characters.".to_string());
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

        // Invalid descriptions
        assert!(validate_description(" leading space").is_err());
        assert!(validate_description("trailing space ").is_err());
        assert!(validate_description("double  space").is_err());
        assert!(validate_description(&"x".repeat(501)).is_err()); // Too long
    }
} 