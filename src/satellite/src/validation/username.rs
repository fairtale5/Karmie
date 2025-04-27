use crate::utils::structs::ValidationError;

/// Validates a username string against expected format
/// 
/// Requirements:
/// - Length between 3 and 20 characters
/// - Only alphanumeric characters and underscores
/// - Must start with a letter
/// - No consecutive underscores
/// - No trailing underscore
pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    if username.len() < 3 || username.len() > 20 {
        return Err(ValidationError::InvalidLength);
    }

    let first_char = username.chars().next().unwrap();
    if !first_char.is_alphabetic() {
        return Err(ValidationError::InvalidFormat);
    }

    let mut prev_char = '_';
    for c in username.chars() {
        if !c.is_alphanumeric() && c != '_' {
            return Err(ValidationError::InvalidCharacter);
        }
        if c == '_' && prev_char == '_' {
            return Err(ValidationError::InvalidFormat);
        }
        prev_char = c;
    }

    if prev_char == '_' {
        return Err(ValidationError::InvalidFormat);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username() {
        // Valid usernames
        assert!(validate_username("john").is_ok());
        assert!(validate_username("john_doe").is_ok());
        assert!(validate_username("john123").is_ok());

        // Invalid usernames
        assert!(validate_username("jo").is_err()); // Too short
        assert!(validate_username("johndoejohndoejohndoe1").is_err()); // Too long
        assert!(validate_username("123john").is_err()); // Starts with number
        assert!(validate_username("john__doe").is_err()); // Consecutive underscores
        assert!(validate_username("john_").is_err()); // Ends with underscore
        assert!(validate_username("john@doe").is_err()); // Invalid character
    }
} 