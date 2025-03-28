use candid::Principal;
use crate::utils::structs::{User, UserData, Tag, TagData, Vote, VoteData, Reputation, ReputationData};
use junobuild_satellite::get_doc;
use regex::Regex;
use lazy_static::lazy_static;

/// Helper struct for creating and parsing document descriptions
/// This follows the new format standard defined in docs/core/architecture/database.md
pub struct DocumentDescription {
    fields: Vec<(String, String)>,
}

impl DocumentDescription {
    /// Creates a new empty description
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
        }
    }

    /// Adds a field to the description
    pub fn add_field(&mut self, name: &str, value: &str) -> &mut Self {
        self.fields.push((name.to_string(), value.to_string()));
        self
    }

    /// Adds an owner field, handling both key and Principal formats
    pub fn add_owner(&mut self, value: &str) -> &mut Self {
        // If the value looks like a Principal (contains "-"), use it as is
        // Otherwise, treat it as a key
        self.add_field("owner", value)
    }

    /// Builds the description string in the new format: [field1:{value1}],[field2:{value2}]
    pub fn build(&self) -> String {
        let mut result = String::new();
        for (i, (name, value)) in self.fields.iter().enumerate() {
            result.push_str(&format!("[{}:{}]", name, value));
            if i < self.fields.len() - 1 {
                result.push(',');
            }
        }
        result
    }

    /// Parses a description string into fields
    pub fn parse(description: &str) -> Result<Self, String> {
        let mut fields = Vec::new();
        let mut current_pos = 0;

        while let Some(start) = description[current_pos..].find('[') {
            if let Some(end) = description[current_pos + start..].find(']') {
                let field_str = &description[current_pos + start + 1..current_pos + start + end];
                if let Some(separator) = field_str.find(':') {
                    let name = &field_str[..separator];
                    let value = &field_str[separator + 1..];
                    fields.push((name.to_string(), value.to_string()));
                }
                current_pos += start + end + 1;
                // Skip the comma if present
                if description[current_pos..].starts_with(',') {
                    current_pos += 1;
                }
            } else {
                break;
            }
        }

        Ok(Self { fields })
    }

    /// Gets a field value by name
    pub fn get_field(&self, name: &str) -> Option<&str> {
        self.fields.iter()
            .find(|(field_name, _)| field_name == name)
            .map(|(_, value)| value.as_str())
    }

    /// Gets the owner value, which could be either a key or Principal
    pub fn get_owner(&self) -> Option<&str> {
        self.get_field("owner")
    }
}

/// Helper functions for user document descriptions
/// Creates a document description for a user document
/// 
/// # Document Structure
/// ```
/// key: String,         // Document's unique identifier
/// data: {
///     username: String,  // User's username
///     display_name: String
/// }
/// ```
/// 
/// # Description Format
/// Playground mode: [owner:{key}],[username:{data.username}]
/// Production mode: [owner:{Principal}],[username:{data.username}]
/// 
/// This function is called from assert_set_doc when validating new/updated user documents.
/// The document key comes from:
/// 1. For new documents: Generated with nanoid() in the frontend
/// 2. For updates: The existing document's key
pub fn create_user_description(user: &User, owner: &Principal, is_playground: bool) -> String {
    let mut desc = DocumentDescription::new();
    let owner_str = if is_playground {
        user.key.clone()  // Using document key in playground mode
    } else {
        owner.to_string()
    };
    desc.add_owner(&owner_str)
        .add_field("username", &user.data.username);
    desc.build()
}

/// Helper functions for tag document descriptions
/// Creates a document description for a tag document
/// 
/// # Document Structure
/// ```
/// key: String,           // Document's unique identifier
/// data: {
///     name: String,      // Tag's display name
///     author_key: String // Reference to Users collection
///     description: String,
///     time_periods: Vec<TimePeriod>,
///     reputation_threshold: f64,
///     vote_reward: f64,
///     min_users_for_threshold: u32
/// }
/// ```
/// 
/// # Description Format
/// Playground mode: [owner:{key}],[name:{data.name}]
/// Production mode: [owner:{Principal}],[name:{data.name}]
pub fn create_tag_description(tag: &Tag, owner: &Principal, is_playground: bool) -> String {
    let mut desc = DocumentDescription::new();
    let owner_str = if is_playground {
        tag.key.clone()  // Using document key in playground mode
    } else {
        owner.to_string()
    };
    desc.add_owner(&owner_str)
        .add_field("name", &tag.data.name);
    desc.build()
}

/// Helper functions for vote document descriptions
/// Creates a document description for a vote document
/// 
/// # Document Structure
/// ```
/// key: String,           // Document's unique identifier
/// data: {
///     author_key: String // Reference to Users collection
///     target_key: String // Reference to Users collection
///     tag_key: String    // Reference to Tags collection
///     value: f64,        // Vote value (+1 or -1)
///     weight: f64,       // Vote weight (0.0 to 1.0)
/// }
/// ```
/// 
/// # Description Format
/// Playground mode: [owner:{key}],[target:{data.target_key}],[tag:{data.tag_key}]
/// Production mode: [owner:{Principal}],[target:{data.target_key}],[tag:{data.tag_key}]
pub fn create_vote_description(vote: &Vote, owner: &Principal, is_playground: bool) -> String {
    let mut desc = DocumentDescription::new();
    let owner_str = if is_playground { 
        vote.key.clone()  // Using document key in playground mode
    } else { 
        owner.to_string() 
    };
    desc.add_owner(&owner_str)
        .add_field("target", &vote.data.target_key)
        .add_field("tag", &vote.data.tag_key);
    desc.build()
}

/// Helper functions for reputation document descriptions
/// Creates a document description for a reputation document
/// 
/// # Document Structure
/// ```
/// key: String,           // Document's unique identifier
/// data: {
///     user_key: String   // Reference to Users collection
///     tag_key: String    // Reference to Tags collection
///     total_basis_reputation: f64,
///     total_voting_rewards_reputation: f64,
///     last_known_effective_reputation: f64,
///     last_calculation: u64,
///     vote_weight: VoteWeight,
///     has_voting_power: bool
/// }
/// ```
/// 
/// # Description Format
/// Playground mode: [owner:{key}],[tag:{data.tag_key}]
/// Production mode: [owner:{Principal}],[tag:{data.tag_key}]
pub fn create_reputation_description(reputation: &Reputation, owner: &Principal, is_playground: bool) -> String {
    let mut desc = DocumentDescription::new();
    let owner_str = if is_playground { 
        reputation.key.clone()  // Using document key in playground mode
    } else { 
        owner.to_string() 
    };
    desc.add_owner(&owner_str)
        .add_field("tag", &reputation.data.tag_key);
    desc.build()
}

lazy_static! {
    // Regex patterns for validating description formats
    static ref USER_DESC_PATTERN: Regex = Regex::new(r"^\[owner:([^,\]]+)\],\[username:([^,\]]+)\]$").unwrap();
    static ref TAG_DESC_PATTERN: Regex = Regex::new(r"^\[owner:([^,\]]+)\],\[name:([^,\]]+)\]$").unwrap();
    static ref VOTE_DESC_PATTERN: Regex = Regex::new(r"^\[owner:([^,\]]+)\],\[target:([^,\]]+)\],\[tag:([^,\]]+)\]$").unwrap();
    static ref REP_DESC_PATTERN: Regex = Regex::new(r"^\[owner:([^,\]]+)\],\[tag:([^,\]]+)\]$").unwrap();
}

/// Validates a description string against expected format and referenced documents
pub async fn validate_description(collection: &str, description: &str, document_key: &str) -> Result<(), String> {
    // Step 1: Validate format using regex
    let (pattern, field_names) = match collection {
        "users" => (&*USER_DESC_PATTERN, vec!["owner", "username"]),
        "tags" => (&*TAG_DESC_PATTERN, vec!["owner", "name"]),
        "votes" => (&*VOTE_DESC_PATTERN, vec!["owner", "target", "tag"]),
        "reputations" => (&*REP_DESC_PATTERN, vec!["owner", "tag"]),
        _ => return Err(format!("Unknown collection: {}", collection))
    };

    let captures = pattern.captures(description)
        .ok_or_else(|| format!("Invalid description format for {}: {}", collection, description))?;

    // Step 2: Extract and validate referenced documents
    for (i, field_name) in field_names.iter().enumerate() {
        let field_value = captures.get(i + 1)
            .ok_or_else(|| format!("Missing {} field in description", field_name))?
            .as_str();

        // Skip validation for non-reference fields (like username and name)
        if *field_name == "username" || *field_name == "name" {
            continue;
        }

        // For owner field in user documents, it must match the document key
        if collection == "users" && *field_name == "owner" && field_value != document_key {
            return Err(format!(
                "Owner field must match document key. Expected: {}, Got: {}",
                document_key, field_value
            ));
        }

        // For other reference fields, verify the referenced document exists
        // This section validates that any referenced documents (users, tags) actually exist
        // in their respective collections before allowing the reference to be created.
        let referenced_collection = match *field_name {
            "owner" | "target" => "users",  // References to users (either as owner or target)
            "tag" => "tags",               // References to tag documents
            _ => continue                  // Skip validation for non-reference fields
        };

        // Verify the referenced document exists in its collection
        // Parameters for get_doc:
        // 1. document_key: The unique identifier of the document we're looking for
        // 2. collection_name: The collection where we expect to find the document
        // Example: For a tag reference, we look for the tag_id in the "tags" collection
        if let None = junobuild_satellite::get_doc(field_value.to_string(), referenced_collection.to_string()) {
            return Err(format!(
                "Referenced {} document not found: {}",
                referenced_collection, field_value
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    #[test]
    fn test_description_builder_playground() {
        let mut desc = DocumentDescription::new();
        desc.add_owner("user_123")
            .add_field("tag", "tag_456");
        
        assert_eq!(desc.build(), "[owner:user_123],[tag:tag_456]");
    }

    #[test]
    fn test_description_builder_production() {
        let mut desc = DocumentDescription::new();
        desc.add_owner("2vxsx-fae")  // Example Principal ID
            .add_field("tag", "tag_456");
        
        assert_eq!(desc.build(), "[owner:2vxsx-fae],[tag:tag_456]");
    }

    #[test]
    fn test_description_parser() {
        // Test playground format
        let desc = DocumentDescription::parse("[owner:user_123],[tag:tag_456]").unwrap();
        assert_eq!(desc.get_owner(), Some("user_123"));
        assert_eq!(desc.get_field("tag"), Some("tag_456"));

        // Test production format
        let desc = DocumentDescription::parse("[owner:2vxsx-fae],[tag:tag_456]").unwrap();
        assert_eq!(desc.get_owner(), Some("2vxsx-fae"));
        assert_eq!(desc.get_field("tag"), Some("tag_456"));
    }

    #[test]
    fn test_user_description_formats() {
        // Create a complete User document like we'd get from Juno
        let user = User {
            key: "user_123".to_string(),  // This would normally be generated with nanoid
            description: String::new(),    // Will be generated by this function
            owner: Principal::anonymous(), // Not relevant for this test
            created_at: 0,                // Not relevant for this test
            updated_at: 0,                // Not relevant for this test
            version: 0,                   // Not relevant for this test
            data: UserData {
                username: "john_doe".to_string(),
            display_name: "John Doe".to_string(),
            },
        };
        let owner = Principal::from_text("2vxsx-fae").unwrap();

        // Test playground format - should use document key
        let playground_desc = create_user_description(&user, &owner, true);
        assert!(playground_desc.contains("owner:user_123"));  // Check for document key

        // Test production format - should use Principal ID
        let production_desc = create_user_description(&user, &owner, false);
        assert!(production_desc.contains("owner:2vxsx-fae")); // Check for Principal ID
    }

    #[test]
    fn test_tag_description_formats() {
        // Create a complete Tag document like we'd get from Juno
        let tag = Tag {
            key: "tag_123".to_string(),  // This would normally be generated with nanoid
            description: String::new(),   // Will be generated by this function
            owner: Principal::anonymous(), // Not relevant for this test
            created_at: 0,                // Not relevant for this test
            updated_at: 0,                // Not relevant for this test
            version: 0,                   // Not relevant for this test
            data: TagData {
                author_key: "author_456".to_string(),
                name: "test_tag".to_string(),
                description: "Test tag description".to_string(),
                time_periods: vec![],     // Not relevant for this test
                reputation_threshold: 0.0, // Not relevant for this test
                vote_reward: 0.0,         // Not relevant for this test
                min_users_for_threshold: 1, // Not relevant for this test
            },
        };
        let owner = Principal::from_text("2vxsx-fae").unwrap();

        // Test playground format - should use document key
        let playground_desc = create_tag_description(&tag, &owner, true);
        assert!(playground_desc.contains("owner:tag_123"));  // Check for document key
        assert!(playground_desc.contains("name:test_tag")); // Check for tag name

        // Test production format - should use Principal ID
        let production_desc = create_tag_description(&tag, &owner, false);
        assert!(production_desc.contains("owner:2vxsx-fae")); // Check for Principal ID
        assert!(production_desc.contains("name:test_tag")); // Check for tag name
    }

    #[test]
    fn test_vote_description_formats() {
        // Create a complete Vote document like we'd get from Juno
        let vote = Vote {
            key: "vote_123".to_string(),  // This would normally be generated with nanoid
            description: String::new(),    // Will be generated by this function
            owner: Principal::anonymous(), // Not relevant for this test
            created_at: 0,                // Not relevant for this test
            updated_at: 0,                // Not relevant for this test
            version: 0,                   // Not relevant for this test
            data: VoteData {
                author_key: "author_456".to_string(),
                target_key: "target_789".to_string(),
                tag_key: "tag_123".to_string(),
                value: 1.0,               // Not relevant for this test
                weight: 1.0,              // Not relevant for this test
            },
        };
        let owner = Principal::from_text("2vxsx-fae").unwrap();

        // Test playground format - should use document key
        let playground_desc = create_vote_description(&vote, &owner, true);
        assert!(playground_desc.contains("owner:vote_123"));   // Check for document key
        assert!(playground_desc.contains("target:target_789")); // Check for target key
        assert!(playground_desc.contains("tag:tag_123"));      // Check for tag key

        // Test production format - should use Principal ID
        let production_desc = create_vote_description(&vote, &owner, false);
        assert!(production_desc.contains("owner:2vxsx-fae"));  // Check for Principal ID
        assert!(production_desc.contains("target:target_789")); // Check for target key
        assert!(production_desc.contains("tag:tag_123"));      // Check for tag key
    }

    #[test]
    fn test_reputation_description_formats() {
        // Create a complete Reputation document like we'd get from Juno
        let reputation = Reputation {
            key: "rep_123".to_string(),   // This would normally be generated with nanoid
            description: String::new(),    // Will be generated by this function
            owner: Principal::anonymous(), // Not relevant for this test
            created_at: 0,                // Not relevant for this test
            updated_at: 0,                // Not relevant for this test
            version: 0,                   // Not relevant for this test
            data: ReputationData {
                user_key: "user_456".to_string(),
                tag_key: "tag_789".to_string(),
                total_basis_reputation: 0.0,        // Not relevant for this test
                total_voting_rewards_reputation: 0.0, // Not relevant for this test
                last_known_effective_reputation: 0.0, // Not relevant for this test
                last_calculation: 0,                // Not relevant for this test
                vote_weight: VoteWeight::new(0.0).unwrap(), // Not relevant for this test
                has_voting_power: false,            // Not relevant for this test
            },
        };
        let owner = Principal::from_text("2vxsx-fae").unwrap();

        // Test playground format - should use document key
        let playground_desc = create_reputation_description(&reputation, &owner, true);
        assert!(playground_desc.contains("owner:rep_123"));  // Check for document key
        assert!(playground_desc.contains("tag:tag_789"));    // Check for tag key

        // Test production format - should use Principal ID
        let production_desc = create_reputation_description(&reputation, &owner, false);
        assert!(production_desc.contains("owner:2vxsx-fae")); // Check for Principal ID
        assert!(production_desc.contains("tag:tag_789"));     // Check for tag key
    }

    #[test]
    fn test_description_patterns() {
        // User description tests
        assert!(USER_DESC_PATTERN.is_match("[owner:user_123],[username:john_doe]"));
        assert!(!USER_DESC_PATTERN.is_match("owner:user_123,username:john_doe")); // Missing brackets
        assert!(!USER_DESC_PATTERN.is_match("[owner:user_123][username:john_doe]")); // Missing comma

        // Tag description tests
        assert!(TAG_DESC_PATTERN.is_match("[owner:user_123],[name:technical_skills]"));
        assert!(!TAG_DESC_PATTERN.is_match("owner:user_123,name:technical_skills")); // Missing brackets
        assert!(!TAG_DESC_PATTERN.is_match("[owner:user_123][name:technical_skills]")); // Missing comma

        // Vote description tests
        assert!(VOTE_DESC_PATTERN.is_match("[owner:user_123],[target:user_456],[tag:tag_789]"));
        assert!(!VOTE_DESC_PATTERN.is_match("owner:user_123,target:user_456,tag:tag_789")); // Missing brackets
        assert!(!VOTE_DESC_PATTERN.is_match("[owner:user_123][target:user_456][tag:tag_789]")); // Missing commas

        // Reputation description tests
        assert!(REP_DESC_PATTERN.is_match("[owner:user_123],[tag:tag_789]"));
        assert!(!REP_DESC_PATTERN.is_match("owner:user_123,tag:tag_789")); // Missing brackets
        assert!(!REP_DESC_PATTERN.is_match("[owner:user_123][tag:tag_789]")); // Missing comma
    }
} 