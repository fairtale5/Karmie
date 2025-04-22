# Validation Architecture and Builder Pattern

## Overview
This document outlines the proposed validation architecture for the Reputator system, which includes:
1. A modular validation system with clear separation of concerns
2. Common validation traits and utilities
3. Document-specific validation implementations
4. A builder pattern for complex validation rules

## Proposed Directory Structure
```
src/satellite/src/validations/
├── mod.rs          // Exports all validation modules
├── common.rs       // Shared validation traits and utilities
├── document.rs     // Document-level validation traits
├── field.rs        // Field-level validation traits
├── builder.rs      // ValidationBuilder implementation
├── users.rs        // User document validation
├── votes.rs        // Vote document validation
├── tags.rs         // Tag document validation
└── reputations.rs  // Reputation document validation
```

## Core Validation Traits
```rust
// common.rs
pub trait Validator {
    fn validate(&self) -> Result<(), String>;
}

pub trait UniqueValidator {
    async fn validate_unique(&self) -> Result<(), String>;
}

pub trait DocumentValidator: Validator + UniqueValidator {
    fn validate_document(&self) -> Result<(), String>;
}

// document.rs
pub trait UserDocumentValidator: DocumentValidator {
    fn validate_username(&self) -> Result<(), String>;
    fn validate_display_name(&self) -> Result<(), String>;
}

pub trait TagDocumentValidator: DocumentValidator {
    fn validate_name(&self) -> Result<(), String>;
    fn validate_description(&self) -> Result<(), String>;
    fn validate_time_periods(&self) -> Result<(), String>;
}

pub trait VoteDocumentValidator: DocumentValidator {
    /// Validates that the vote author matches the context user's key/principal
    /// Note: This validation is disabled in playground mode
    fn validate_author(&self) -> Result<(), String>;
    
    /// Validates that both the key and target documents exist
    fn validate_document_references(&self) -> Result<(), String>;
    
    /// Validates that the vote weight is within the tag's allowed ranges
    /// Typically -1 or 1, with no decimals or zeros
    /// Some tags may allow different ranges, but zero votes are never allowed
    fn validate_vote_weight(&self) -> Result<(), String>;
}

// Similar traits for votes and reputations
```

## Common Validation Functions
```rust
// field.rs
pub mod field_validators {
    pub fn validate_non_empty(value: &str, field_name: &str) -> Result<(), String> { ... }
    pub fn validate_length(value: &str, min: usize, max: usize, field_name: &str) -> Result<(), String> { ... }
    pub fn validate_characters(value: &str, allowed_chars: &[char], field_name: &str) -> Result<(), String> { ... }
    pub fn validate_numeric_range(value: f64, min: f64, max: f64, field_name: &str) -> Result<(), String> { ... }
}
```

## Document Validation Implementation
```rust
// users.rs
use super::common::{Validator, UniqueValidator, DocumentValidator};
use super::document::UserDocumentValidator;

pub struct UserValidator {
    user_data: UserData,
    context: AssertSetDocContext,
}

impl UserValidator {
    pub fn new(user_data: UserData, context: AssertSetDocContext) -> Self {
        Self { user_data, context }
    }
}

impl Validator for UserValidator {
    fn validate(&self) -> Result<(), String> {
        self.validate_document()?;
        Ok(())
    }
}

impl UniqueValidator for UserValidator {
    async fn validate_unique(&self) -> Result<(), String> {
        // Check normalized username uniqueness
        // Use description_helpers.rs for proper document description format
    }
}

impl DocumentValidator for UserValidator {
    fn validate_document(&self) -> Result<(), String> {
        self.validate_username()?;
        self.validate_display_name()?;
        Ok(())
    }
}

impl UserDocumentValidator for UserValidator {
    fn validate_username(&self) -> Result<(), String> {
        ValidationBuilder::new(&self.user_data.username, "Username")
            .not_empty()
            .length(3, 30)
            .alphanumeric_with(&['_', '-'])
            .normalize()
            .validate()
    }

    fn validate_display_name(&self) -> Result<(), String> {
        ValidationBuilder::new(&self.user_data.display_name, "Display name")
            .not_empty()
            .length(1, 50)
            .validate()
    }
}
```

## Validation Builder Pattern
```rust
// builder.rs
pub struct ValidationBuilder<T> {
    value: T,
    field_name: String,
    rules: Vec<Box<dyn Fn(&T) -> Result<(), String>>>,
}

impl<T> ValidationBuilder<T> {
    pub fn new(value: T, field_name: &str) -> Self { ... }
    
    // Chainable validation rules
    pub fn not_empty(mut self) -> Self { ... }
    pub fn length(mut self, min: usize, max: usize) -> Self { ... }
    pub fn alphanumeric_with(mut self, extra_chars: &'static [char]) -> Self { ... }
    pub fn normalize(mut self) -> Self { ... }
    pub fn unique_in_collection(mut self, collection: &str) -> Self { ... }
    pub fn validate(&self) -> Result<(), String> { ... }
}
```

## Integration with lib.rs
```rust
use crate::validations::{
    UserValidator, VoteValidator, TagValidator, ReputationValidator,
    Validator, UniqueValidator
};

#[assert_set_doc(collections = ["users", "votes", "tags", "reputations"])]
async fn assert_set_doc(context: AssertSetDocContext) -> Result<(), String> {
    match context.data.collection.as_str() {
        "users" => {
            let user_data = decode_doc_data(&context.data.data.proposed.data)?;
            let validator = UserValidator::new(user_data, context);
            validator.validate()?;
            validator.validate_unique().await?;
            Ok(())
        },
        // Similar pattern for other collections
        _ => Err(format!("Unknown collection: {}", context.data.collection))
    }
}
```

## Benefits
1. Clear separation of concerns
2. Reusable validation logic
3. Type-safe validation through traits
4. Easy to add new validation types
5. Testable in isolation
6. Consistent validation patterns
7. Readable and maintainable code
8. Support for both sync and async validation
9. Flexible validation composition

## Implementation Steps
1. Create validation directory structure
2. Implement core validation traits
3. Create common validation utilities
4. Implement ValidationBuilder
5. Create document-specific validators
6. Add comprehensive tests
7. Migrate existing validation code
8. Update documentation

## Dependencies
- Juno's document store functionality
- Description helpers for document formatting
- Existing validation functions (for reference)
- Test utilities for validation testing

## Advanced Features to Consider
1. Validation Caching
   - Cache expensive validation results
   - Implement cache invalidation strategy
   - Consider playground vs production caching

2. Custom Error Types
```rust
pub enum ValidationError {
    Empty { field: String },
    Length { field: String, min: usize, max: usize, current: usize },
    InvalidCharacters { field: String, allowed: String },
    NotUnique { field: String, value: String },
    // ... other error types
}
```

3. Validation Context
```rust
pub struct ValidationContext {
    is_playground: bool,
    caller: Principal,
    collection: String,
    // ... other context fields
}
```

4. Rule Composition
```rust
pub enum ValidationRule<T> {
    And(Vec<Box<dyn Fn(&T) -> Result<(), ValidationError>>>),
    Or(Vec<Box<dyn Fn(&T) -> Result<(), ValidationError>>>),
    Not(Box<dyn Fn(&T) -> Result<(), ValidationError>>),
}
```

## Migration Strategy
1. Create new validation structure
2. Implement new validators alongside existing code
3. Test new implementations thoroughly
4. Gradually migrate each collection
5. Remove old validation code
6. Update documentation

## Testing Guidelines
1. Unit test each validation rule
2. Test validation compositions
3. Test async validation operations
4. Test playground vs production modes
5. Test error messages and formatting
6. Test performance and caching
7. Test edge cases and boundary conditions

## Important Notes
1. Keep validation logic pure when possible
2. Use proper error types for better error handling
3. Consider validation performance
4. Document all validation rules clearly
5. Maintain consistency between playground/production modes
6. Consider future extensibility
7. Follow Rust best practices 