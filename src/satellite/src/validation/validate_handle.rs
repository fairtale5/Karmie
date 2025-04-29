use crate::logger;

/// Validates a handle (username or tag) string against expected format
/// 
/// Requirements:
/// - Length between 3 and 20 characters
/// - Only alphanumeric characters and underscores
/// - Must start with a letter
/// - No consecutive underscores
/// - No trailing underscore
pub fn validate_handle(username: &str) -> Result<(), String> {
    if username.len() < 3 || username.len() > 20 {
        logger!("error", "[validate_handle] Invalid character length: {}", username);
        return Err("Username must be between 3 and 20 characters.".to_string());
    }

    let first_char = username.chars().next().unwrap();
    if !first_char.is_alphabetic() {
        logger!("error", "[validate_handle] Must start with a letter: {}", username);
        return Err("Username must start with a letter.".to_string());
    }

    let mut prev_char = '_';
    for c in username.chars() {
        if !c.is_alphanumeric() && c != '_' {
            logger!("error", "[validate_handle] Invalid characters: {} in {}", c, username);
            return Err("Username can only contain alphanumeric characters and underscores.".to_string());
        }
        if c == '_' && prev_char == '_' {
            logger!("error", "[validate_handle] Consecutive underscores in: {}", username);
            return Err("Username cannot contain consecutive underscores.".to_string());
        }
        prev_char = c;
    }

    if prev_char == '_' {
        logger!("error", "[validate_handle] Ends with underscore: {}", username);
        return Err("Username cannot end with an underscore.".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_handle() {
        // Valid usernames
        assert!(validate_handle("john").is_ok());
        assert!(validate_handle("john_doe").is_ok());
        assert!(validate_handle("john123").is_ok());

        // Invalid usernames
        assert!(validate_handle("jo").is_err()); // Too short
        assert!(validate_handle("johndoejohndoejohndoe1").is_err()); // Too long
        assert!(validate_handle("123john").is_err()); // Starts with number
        assert!(validate_handle("john__doe").is_err()); // Consecutive underscores
        assert!(validate_handle("john_").is_err()); // Ends with underscore
        assert!(validate_handle("john@doe").is_err()); // Invalid character
    }
} 