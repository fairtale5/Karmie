# Satellite Code Refactoring Plan

## Background

As the Reputator backend grows in complexity, there's a need to better organize the codebase with a more structured approach. Currently, some files like `reputation_calculations.rs` are becoming too large and complex, making maintenance and debugging challenging.

## Current Directory Structure

```
src/satellite/src/
├── lib.rs
├── assert_set_doc/
│   ├── assert_doc_user.rs
│   ├── assert_doc_tag.rs
│   ├── assert_doc_vote.rs
│   ├── assert_doc_reputation.rs
│   └── mod.rs
├── validation/
│   ├── validate_handle.rs
│   ├── validate_tag_date.rs
│   ├── display_name.rs
│   ├── description.rs
│   ├── ulid_timestamp_validate.rs
│   └── mod.rs
├── processors/
│   ├── document_keys.rs
│   ├── ulid_generator.rs
│   ├── ulid_timestamp_extract.rs
│   ├── ulid_type.rs
│   ├── mod.rs
├── utils/
│   ├── reputation_calculations.rs
│   ├── tag_calculations.rs
│   ├── query_helpers.rs
│   ├── structs.rs
│   ├── logger.rs
│   ├── time.rs
│   ├── normalize.rs
│   └── mod.rs
```

## Proposed Directory Structure (post-refactor)

```
src/satellite/src/
├── lib.rs
├── structs/
│   ├── struct_user.rs         # (from structs.rs)
│   ├── struct_tag.rs          # (from structs.rs)
│   ├── struct_vote.rs         # (from structs.rs)
│   ├── struct_reputation.rs   # (from structs.rs)
│   └── mod.rs
├── core/
│   ├── reputation_calculations.rs  # (from utils/)
│   ├── tag_calculations.rs         # (from utils/)
│   └── mod.rs
├── assert_set_doc/
│   ├── assert_doc_user.rs
│   ├── assert_doc_tag.rs
│   ├── assert_doc_vote.rs
│   ├── assert_doc_reputation.rs
│   └── mod.rs
├── validation/
│   ├── validate_handle.rs
│   ├── validate_tag_date.rs
│   ├── display_name.rs
│   ├── description.rs
│   ├── ulid_timestamp_validate.rs
│   └── mod.rs
├── processors/
│   ├── document_keys.rs
│   ├── ulid_generator.rs
│   ├── ulid_timestamp_extract.rs
│   ├── ulid_type.rs
│   ├── document_queries.rs   # (was query_helpers.rs)
│   └── mod.rs
└── utils/
    ├── logger.rs
    ├── time.rs
    ├── normalize.rs
    └── mod.rs
```

## Migration Plan

- **Move `structs.rs`**: Split into `structs/struct_user.rs`, `structs/struct_tag.rs`, `structs/struct_vote.rs`, `structs/struct_reputation.rs`, and add `structs/mod.rs`.
- **Move `reputation_calculations.rs` and `tag_calculations.rs`**: From `utils/` to `core/`.
- **Rename and move `query_helpers.rs`**: Rename to `document_queries.rs` and move to `processors/`.
- **Keep all other files** in their current or analogous locations as shown above.
- **Update all imports** in the codebase to reflect new paths and file names.
- **No files are lost**; all are accounted for in the new structure.

## Rationale
- Structs are now domain-specific and separated for clarity.
- Core business logic is centralized in `core/`.
- Document processing helpers are grouped in `processors/`.
- Utilities remain in `utils/`.
- Naming is consistent (e.g., `document_keys.rs`, `document_queries.rs`, `struct_user.rs`).

## Next Steps
1. Create the new directory structure and move files as above.
2. Update all module imports and references.
3. Run tests and validate the build.
4. Update this document if further changes are made.

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