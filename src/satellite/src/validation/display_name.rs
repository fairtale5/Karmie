use crate::utils::structs::ValidationError;

/// Validates a display name string against expected format
/// 
/// Requirements:
/// - Length between 1 and 50 characters
/// - No leading/trailing whitespace
/// - No consecutive whitespace
pub fn validate_display_name(display_name: &str) -> Result<(), ValidationError> {
    if display_name.len() < 1 || display_name.len() > 50 {
        return Err(ValidationError::InvalidLength);
    }

    if display_name.trim() != display_name {
        return Err(ValidationError::InvalidFormat);
    }

    let mut prev_char = ' ';
    for c in display_name.chars() {
        if c.is_whitespace() && prev_char.is_whitespace() {
            return Err(ValidationError::InvalidFormat);
        }
        prev_char = c;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_display_name() {
        // Valid display names
        assert!(validate_display_name("John").is_ok());
        assert!(validate_display_name("John Doe").is_ok());
        assert!(validate_display_name("John Doe III").is_ok());

        // Invalid display names
        assert!(validate_display_name("").is_err()); // Empty
        assert!(validate_display_name(" John").is_err()); // Leading space
        assert!(validate_display_name("John ").is_err()); // Trailing space
        assert!(validate_display_name("John  Doe").is_err()); // Consecutive spaces
    }
} 