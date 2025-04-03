# Satellite Code Refactoring Plan

## Background

As the Reputator backend grows in complexity, there's a need to better organize the codebase with a more structured approach. Currently, some files like `reputation_calculations.rs` are becoming too large and complex, making maintenance and debugging challenging.

## Proposed Directory Structure

```
src/satellite/
├── src/
│   ├── lib.rs                 # Main entry point and exports
│   ├── validation/            # Input validation
│   │   ├── mod.rs             # Module exports
│   │   ├── user.rs            # User-related validations  
│   │   ├── tag.rs             # Tag-related validations
│   │   ├── vote.rs            # Vote-related validations
│   │   └── reputation.rs      # Reputation-related validations
│   ├── core/                  # Core business logic
│   │   ├── mod.rs             # Module exports
│   │   ├── reputation.rs      # Reputation calculation algorithms
│   │   └── tags.rs            # Tag management algorithms
│   └── utils/                 # Utility functions
│       ├── mod.rs             # Module exports
│       ├── structs.rs         # Data structures
│       ├── logging.rs         # Logging utilities
│       ├── time.rs            # Time-related utilities
│       ├── description.rs     # Document description helpers
│       ├── normalize.rs       # String normalization utilities
│       └── id_generator.rs    # ID generation utilities
```

## Implementation Plan

1. **Phase 1: Simple Utility Extractions**
   - Move standalone utility functions to dedicated files
   - Example: Extract `generate_random_doc_key` to `utils/id_generator.rs` ✅
   - Create additional utility modules as needed

2. **Phase 2: Core Logic Separation**
   - Split `reputation_calculations.rs` into logical components:
     - Base reputation algorithms -> `core/reputation.rs`
     - Vote weight calculations -> `core/reputation/vote_weight.rs`
     - Reputation update handlers -> `core/reputation/updates.rs`

3. **Phase 3: Validation Logic**
   - Extract all validation code into the validation directory
   - Separate by domain (user, tag, vote, reputation)

4. **Phase 4: Interface Refinement**
   - Update `lib.rs` to properly export the new module structure
   - Ensure backward compatibility for external consumers

## Data Sharing Strategies

Several approaches can be used to share data between functions in different modules:

1. **Parameter Passing**:
   - Explicit but can become unwieldy with many parameters
   - Example: `fn calculate(user: &str, tag: &str, votes: &[Vote]) -> Result<f64, String>`

2. **Shared Types/Structs**:
   - Define data structures in `structs.rs` and pass them between functions
   - Example: `fn calculate(context: &ReputationContext) -> Result<f64, String>`

3. **Context Objects**:
   ```rust
   // In structs.rs
   pub struct ReputationContext {
       pub user_key: String,
       pub tag_key: String,
       pub votes: Vec<Vote>,
       pub cached_calculations: HashMap<String, f64>,
   }
   
   // In reputation.rs
   pub fn calculate_vote_weight(context: &mut ReputationContext) -> Result<f64, String> {
       // Access user_key, tag_key, votes from context
       // Store results in context.cached_calculations
   }
   ```

4. **State Management**:
   - For more complex cases, implement a simple state management pattern
   - Use interior mutability with thread-safe containers if needed

## Considerations

- **Avoid Circular Dependencies**: Carefully arrange module hierarchy to prevent circular dependencies
- **Public vs Private**: Be intentional about which functions and types are exposed
- **Error Handling**: Maintain consistent error handling patterns across modules
- **Documentation**: Update documentation to reflect the new structure
- **Testing**: Ensure proper test coverage for all refactored components

## Benefits

- **Improved Maintainability**: Smaller, focused files are easier to understand and modify
- **Better Organization**: Clear separation of concerns and responsibilities
- **Easier Collaboration**: Multiple developers can work on different components
- **Enhanced Testability**: Isolated components are easier to test
- **Clearer Dependencies**: The dependency graph becomes more explicit and manageable

## Next Steps

1. Begin with small, incremental changes that don't affect functionality
2. Add comprehensive tests before making significant structural changes
3. Refactor gradually, one component at a time
4. Update documentation to reflect the new structure 