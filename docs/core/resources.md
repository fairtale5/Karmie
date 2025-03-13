# Development Resources

## Core Documentation

### Juno SDK & Serverless
- [Juno Documentation](https://docs.juno.build/)
  - Primary reference for Juno's serverless features
  - Collection management
  - Authentication integration
  - Deployment workflows
- [Juno GitHub](https://github.com/buildwithjuno)
  - Example implementations
  - Source code reference
  - Community contributions

### ICP Integration

#### Canister Development
- [Rust Agent Documentation](https://docs.rs/ic-agent/latest/ic_agent/)
  - Low-level canister interactions
  - Agent configuration
  - Identity management
- [IC Interface Specification](https://internetcomputer.org/docs/current/references/ic-interface-spec)
  - Canister interface standards
  - Type system details
  - Protocol specifications

#### Authentication
- [Internet Identity Guide](https://internetcomputer.org/docs/current/developer-docs/integrations/internet-identity/)
  - II integration steps
  - Authentication flows
  - Security considerations
- [Authentication in Rust](https://docs.rs/ic-agent/latest/ic_agent/identity/index.html)
  - Identity types
  - Signature verification
  - Principal management

#### System Integration
- [IC System API](https://internetcomputer.org/docs/current/references/ic-system-api)
  - Memory management
  - Cycles handling
  - Inter-canister calls
  - System interfaces

## How We Use These Resources

### In Our Project
1. **Juno SDK**: Primary for collection management and basic authentication
   - Used in: `src/lib/collections.ts`
   - Used in: `src/lib/auth.ts`

2. **Rust Agent**: For custom canister interactions
   - Used in: `src/backend/reputation.rs`
   - Used in: `src/backend/voting.rs`

3. **IC System API**: For advanced features
   - Memory management in state handling
   - Cycle management for sustainable operation
   - Inter-canister communication for future integrations

### Key Integration Points
1. **Collections & State**
   ```typescript
   // Example using Juno collections with custom backend
   import { Collection } from '@junobuild/core';
   const votes = new Collection({ collection: "votes" });
   ```

2. **Authentication Flow**
   ```typescript
   // Combining II with our reputation system
   import { authSubscribe } from '@junobuild/core';
   authSubscribe((user) => {
     if (user) {
       // Load user's reputation data
     }
   });
   ```

3. **Custom Canister Integration**
   ```rust
   // Using IC System API with our reputation logic
   #[update]
   async fn update_reputation() -> Result<(), String> {
       // Memory management
       STATE.with(|state| {
           // State updates
       })
   }
   ```

## Best Practices
1. Always refer to official docs for up-to-date API changes
2. Check GitHub issues for known problems/solutions
3. Use the IC System API documentation for optimization
4. Follow Internet Identity best practices for security

## Updates
This document should be updated when:
- New major versions of Juno are released
- IC interface specifications change
- New integration patterns are discovered
- Project requirements evolve 