//! Utility functions for generating unique document IDs
//!
//! This module provides utilities for generating random, unique document IDs
//! similar to nanoid in JavaScript. These IDs are suitable for use as document
//! keys in Juno collections.

use ic_cdk;
use hex::encode;

/// Generates a unique document key using cryptographically secure random bytes
/// 
/// Uses IC's raw_rand function to get random bytes and converts them to a hex string
/// 
/// # Returns
/// * `String` - A unique random ID string
pub async fn generate_random_doc_key() -> String {
    get_random_hex().await
}

/// Gets random bytes from IC and converts them to a hex string
async fn get_random_hex() -> String {
    let random_bytes = ic_cdk::api::management_canister::main::raw_rand()
        .await
        .map(|result| result.0)
        .unwrap_or_default();
    
    encode(random_bytes) // Converts bytes to hex
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_generate_random_doc_key() {
        // Basic check - run it multiple times and ensure we get different results
        let key1 = generate_random_doc_key().await;
        let key2 = generate_random_doc_key().await;
        
        // Keys should be different
        assert_ne!(key1, key2);
        
        // Keys should be of reasonable length
        assert!(key1.len() >= 20);
        
        // Keys should only contain hexadecimal characters
        for c in key1.chars() {
            assert!(c.is_ascii_hexdigit());
        }
    }
} 