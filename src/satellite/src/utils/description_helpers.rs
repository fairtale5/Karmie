use crate::utils::structs::{UserData, TagData, VoteData, ReputationData};
use candid::Principal;

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
pub fn create_user_description(user: &UserData, owner: &Principal, is_playground: bool) -> String {
    let mut desc = DocumentDescription::new();
    desc.add_owner(if is_playground { &user.key } else { &owner.to_string() })
        .add_field("username", &user.handle);
    desc.build()
}

/// Helper functions for tag document descriptions
pub fn create_tag_description(tag: &TagData, owner: &Principal, is_playground: bool) -> String {
    let mut desc = DocumentDescription::new();
    desc.add_owner(if is_playground { &tag.key } else { &owner.to_string() })
        .add_field("name", &tag.name);
    desc.build()
}

/// Helper functions for vote document descriptions
pub fn create_vote_description(vote: &VoteData, owner: &Principal, is_playground: bool) -> String {
    let mut desc = DocumentDescription::new();
    desc.add_owner(if is_playground { &vote.author_key } else { &owner.to_string() })
        .add_field("target", &vote.target_key)
        .add_field("tag", &vote.tag_key);
    desc.build()
}

/// Helper functions for reputation document descriptions
pub fn create_reputation_description(reputation: &ReputationData, owner: &Principal, is_playground: bool) -> String {
    let mut desc = DocumentDescription::new();
    desc.add_owner(if is_playground { &reputation.user_key } else { &owner.to_string() })
        .add_field("tag", &reputation.tag_key);
    desc.build()
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
        let user_data = UserData {
            handle: "john_doe".to_string(),
            display_name: "John Doe".to_string(),
            key: "user_123".to_string(),
        };
        let owner = Principal::from_text("2vxsx-fae").unwrap();

        // Test playground format
        let playground_desc = create_user_description(&user_data, &owner, true);
        assert!(playground_desc.contains("owner:user_123"));

        // Test production format
        let production_desc = create_user_description(&user_data, &owner, false);
        assert!(production_desc.contains("owner:2vxsx-fae"));
    }
} 