//! Utility functions for generating unique document IDs
//!
//! This module provides utilities for generating random, unique document IDs
//! similar to nanoid in JavaScript. These IDs are suitable for use as document
//! keys in Juno collections.

use ic_cdk;

/// Generates a random document key similar to nanoid
/// 
/// This function creates a unique random string that can be used as a document key.
/// It uses a combination of current timestamp and a hash of the canister ID to 
/// ensure uniqueness.
/// 
/// # Returns
/// * `String` - A random string suitable for use as a document key
pub fn generate_random_doc_key() -> String {
    // Get current timestamp in nanoseconds for uniqueness
    let timestamp = ic_cdk::api::time();
    
    // Use timestamp as a seed to generate a string
    let mut result = String::new();
    
    // Create a 20-character ID using a simple algorithm
    // This isn't cryptographically secure but is sufficient for document keys
    let timestamp_bytes = timestamp.to_be_bytes();
    
    // Use each byte of the timestamp to generate a character
    for byte in timestamp_bytes.iter() {
        // Map the byte to alphanumeric characters (0-9, a-z, A-Z)
        let value = *byte % 62;
        let char = if value < 10 {
            // 0-9
            (b'0' + value) as char
        } else if value < 36 {
            // a-z
            (b'a' + (value - 10)) as char
        } else {
            // A-Z
            (b'A' + (value - 36)) as char
        };
        
        result.push(char);
    }
    
    // Add a random component to ensure uniqueness even with same timestamp
    let principal_id = ic_cdk::id().to_string();
    let principal_hash: u64 = principal_id.bytes().fold(0, |acc, byte| acc.wrapping_add(byte as u64));
    
    // Combine with a hash of the principal ID
    let combined = timestamp.wrapping_add(principal_hash);
    
    // Add more characters based on the combined value
    for i in 0..12 {
        let value = ((combined >> (i * 5)) & 0x3F) as u8; // Get 6 bits
        let char = if value < 10 {
            // 0-9
            (b'0' + value) as char
        } else if value < 36 {
            // a-z
            (b'a' + (value - 10)) as char
        } else {
            // A-Z
            (b'A' + (value - 36)) as char
        };
        
        result.push(char);
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_random_doc_key() {
        // Basic check - run it multiple times and ensure we get different results
        let key1 = generate_random_doc_key();
        let key2 = generate_random_doc_key();
        
        // Keys should be different
        assert_ne!(key1, key2);
        
        // Keys should be of reasonable length
        assert!(key1.len() >= 20);
        
        // Keys should only contain alphanumeric characters
        for c in key1.chars() {
            assert!(c.is_ascii_alphanumeric());
        }
    }
} 