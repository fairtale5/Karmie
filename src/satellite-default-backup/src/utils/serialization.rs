/*!
 * Data serialization utilities
 * 
 * This module provides functions for serializing and deserializing data
 * between Rust types and Juno's document format. It handles the conversion
 * of document data to and from JSON format.
 * 
 * Note: These implementations are kept as reference and for testing purposes.
 * In production code, we use the implementations from junobuild_utils.
 * 
 * The main functions in this module are:
 * - decode_doc_data: Deserializes Juno document data into Rust types
 * - encode_doc_data: Serializes Rust types into Juno document data
 */

use serde::de::DeserializeOwned;
use serde_json::Value;
use serde::Serialize;

/// Decodes document data from Juno's format into a specified Rust type.
/// 
/// This is a reference implementation. In production, use junobuild_utils::decode_doc_data.
#[allow(dead_code)]
pub fn decode_doc_data<T>(data: &Value) -> Result<T, String>
where
    T: DeserializeOwned,
{
    // Attempt to deserialize the data into the specified type
    serde_json::from_value(data.clone())
        .map_err(|e| format!("Failed to decode document data: {}", e))
}

/// Encodes a Rust type into Juno's document data format.
/// 
/// This is a reference implementation. In production, use junobuild_utils::encode_doc_data.
#[allow(dead_code)]
pub fn encode_doc_data<T>(data: &T) -> Result<Value, String>
where
    T: Serialize,
{
    serde_json::to_value(data)
        .map_err(|e| format!("Failed to encode document data: {}", e))
}

// Test module is conditionally compiled only when running tests
#[cfg(test)]
mod tests {
    use super::*;

    // Test data structure that represents a typical document payload
    // Note: We derive all necessary traits:
    // - Clone: For making copies of the data
    // - Debug: For debug printing
    // - PartialEq: For equality comparison in tests
    // - Serialize: For converting to JSON
    // - Deserialize: For converting from JSON
    #[derive(Debug)]
    struct TestData {
        field1: String,
        field2: i32,
    }

    // Implement the required traits manually to avoid proc-macro issues
    impl Clone for TestData {
        fn clone(&self) -> Self {
            TestData {
                field1: self.field1.clone(),
                field2: self.field2,
            }
        }
    }

    impl PartialEq for TestData {
        fn eq(&self, other: &Self) -> bool {
            self.field1 == other.field1 && self.field2 == other.field2
        }
    }

    // Implement serialization manually
    impl Serialize for TestData {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::ser::SerializeStruct;
            let mut state = serializer.serialize_struct("TestData", 2)?;
            state.serialize_field("field1", &self.field1)?;
            state.serialize_field("field2", &self.field2)?;
            state.end()
        }
    }

    // Implement deserialization manually
    impl<'de> Deserialize<'de> for TestData {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            use serde::de::{self, Visitor};
            use std::fmt;

            struct TestDataVisitor;

            impl<'de> Visitor<'de> for TestDataVisitor {
                type Value = TestData;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("struct TestData")
                }

                fn visit_map<V>(self, mut map: V) -> Result<TestData, V::Error>
                where
                    V: de::MapAccess<'de>,
                {
                    let mut field1 = None;
                    let mut field2 = None;

                    while let Some(key) = map.next_key()? {
                        match key {
                            "field1" => {
                                if field1.is_some() {
                                    return Err(de::Error::duplicate_field("field1"));
                                }
                                field1 = Some(map.next_value()?);
                            }
                            "field2" => {
                                if field2.is_some() {
                                    return Err(de::Error::duplicate_field("field2"));
                                }
                                field2 = Some(map.next_value()?);
                            }
                            _ => {
                                return Err(de::Error::unknown_field(key, &["field1", "field2"]));
                            }
                        }
                    }

                    let field1 = field1.ok_or_else(|| de::Error::missing_field("field1"))?;
                    let field2 = field2.ok_or_else(|| de::Error::missing_field("field2"))?;

                    Ok(TestData { field1, field2 })
                }
            }

            deserializer.deserialize_struct("TestData", &["field1", "field2"], TestDataVisitor)
        }
    }

    #[test]
    fn test_decode_doc_data() {
        // Test case 1: Valid data
        let json_data = serde_json::json!({
            "field1": "test",
            "field2": 42
        });

        let result: TestData = decode_doc_data(&json_data).unwrap();
        assert_eq!(result.field1, "test");
        assert_eq!(result.field2, 42);

        // Test case 2: Invalid data (type mismatch)
        let invalid_data = serde_json::json!({
            "field1": 123,  // Should be string
            "field2": "not a number"  // Should be integer
        });
        let result: Result<TestData, _> = decode_doc_data(&invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_encode_doc_data() {
        // Create test data
        let test_data = TestData {
            field1: "test".to_string(),
            field2: 42,
        };

        // Test encoding
        let encoded = encode_doc_data(&test_data).unwrap();
        
        // Verify the encoded data
        assert_eq!(encoded["field1"], "test");
        assert_eq!(encoded["field2"], 42);

        // Test round-trip
        let decoded: TestData = decode_doc_data(&encoded).unwrap();
        assert_eq!(decoded, test_data);
    }
} 