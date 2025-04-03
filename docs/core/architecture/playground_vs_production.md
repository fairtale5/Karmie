# Playground vs Production Mode

This document outlines the key differences between playground (development) and production modes in the Reputator system.

## Core Differences

### Document Ownership
- **Playground**: Single Juno user creates all documents, ownership tracked via description field
- **Production**: Each user creates their own documents, ownership tracked via Juno's Principal ID

### Principal ID Handling
- **Playground**: Principal IDs stored in description field for simulation
- **Production**: Uses Juno's native Principal ID system

### User Document Limits
- **Playground**: Multiple documents per identity allowed (for testing)
- **Production**: Strict one document per identity limit

## Affected Files and Settings

### 1. `src/satellite/src/lib.rs`
```rust
// Playground Mode:
- One-document-per-identity check disabled
- Uses description field for ownership lookup
- Allows multiple documents per identity

// Production Mode:
- Strict one-document-per-identity enforcement
- Uses Juno's native owner field
- Enforces single document per identity
```

### 2. `src/satellite/src/utils/description_helpers.rs`
```rust
// Playground Mode:
- Includes owner in description field
- Format: [owner:{key}],[field:{value}]
- Owner is document key

// Production Mode:
- Owner from Juno Principal
- Format: [field:{value}]
- Owner is Principal ID
```

### 3. `src/satellite/src/utils/structs.rs`
```rust
// Playground Mode:
- UserData includes key field
- Tag/Vote/Reputation include owner in data

// Production Mode:
- UserData uses Principal ID
- Owner handled by Juno
```

### 4. Database Schema (`database.md`)
```typescript
// Playground Mode:
- Description field includes ownership
- Format: [owner:{key}],[field:{value}]

// Production Mode:
- Owner field from Juno
- Format: [field:{value}]
```

## Switching Between Modes

### Configuration
Add a global configuration flag:
```rust
pub const IS_PLAYGROUND: bool = true; // Set to false for production
```

### Validation Rules
- Playground: Relaxed validation for testing
- Production: Strict validation and security

### Security Implications
1. **Playground**
   - Less strict security for testing
   - Simulated ownership
   - Multiple documents per identity

2. **Production**
   - Strict security enforcement
   - Real Principal ID ownership
   - One document per identity
   - Proper access control

## Migration Considerations

When moving from playground to production:
1. Update all document descriptions to remove owner field
2. Migrate ownership to Juno Principal IDs
3. Enable strict validation rules
4. Clean up duplicate documents
5. Update frontend to use proper Principal ID handling

## Testing Guidelines

1. **Playground Testing**
   - Test with multiple documents per identity
   - Test ownership simulation
   - Test description field handling

2. **Production Testing**
   - Test Principal ID enforcement
   - Test one-document-per-identity rule
   - Test proper access control

## Important Notes

1. Never mix playground and production data
2. Always clearly indicate current mode
3. Use separate deployments for each mode
4. Document all mode-specific code with clear comments 