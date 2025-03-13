# Reputator Documentation

This directory contains comprehensive documentation for the Reputator project, organized into distinct sections for clarity and ease of access. This guide is designed to help both users and AI assistants navigate the documentation effectively.

## Installation Guide

### Prerequisites
1. Docker installed and running
2. Node.js installed
3. Git installed

### Step-by-Step Installation

1. **Install Juno CLI**
   ```bash
   npm install -g @junobuild/cli
   ```
   Purpose: Installs the Juno command-line interface
   Why: Required for local development and project management
   Risks: May require sudo/admin privileges
   Outcome: Enables Juno CLI commands

2. **Start Local Development Environment**
   ```bash
   juno dev start
   ```
   Purpose: Starts the local Juno development environment
   Why: Required for local development and testing
   Risks: Port conflicts (default: 5987)
   Outcome: Local Juno environment running

3. **Initialize Project**
   ```bash
   npm create juno@latest
   ```
   Purpose: Creates a new Juno project with latest template
   Why: Sets up project structure and dependencies
   Risks: May conflict with existing project files
   Outcome: New Juno project structure created

4. **Install Dependencies**
   ```bash
   npm install
   ```
   Purpose: Installs project dependencies
   Why: Required for project functionality
   Risks: Version conflicts
   Outcome: All dependencies installed

### Important Notes
- DO NOT install `@junobuild/core` directly - it will be installed by the project initialization
- Always follow the installation steps in order
- If you encounter issues, check the [Troubleshooting Guide](/docs/core/juno_index.md#troubleshooting)
- For detailed setup instructions, see [Juno Index](/docs/core/juno_index.md)

## Documentation Guidelines

### Code Documentation Principles

1. **Preserve Reference Documentation**
   - Never delete example code blocks unless the entire feature changes
   - Keep existing comments that explain functionality
   - Maintain commented-out code that serves as reference
   - Document all available hooks, functions, and features

2. **Code Organization**
   - Group related functionality with clear section headers
   - Use consistent spacing between sections
   - Keep imports organized and documented
   - Maintain a clear module structure

3. **Comments and Documentation**
   - Every function should have a doc comment explaining:
     - Purpose
     - Parameters
     - Return values
     - Example usage
   - Use section comments to group related code
   - Include links to relevant documentation
   - Explain complex logic with inline comments

4. **Feature Documentation**
   - Document all available features
   - Include usage examples
   - Explain configuration options
   - Provide common use cases

5. **Code Changes**
   - Don't delete reference code - comment it out instead
   - Explain why code is commented out
   - Add migration notes when features change
   - Update examples to match new functionality

### Example Documentation Style

```rust
/*!
 * Module documentation
 * 
 * Provides a detailed overview of the module's purpose
 * and its main components.
 * 
 * # Features
 * - Feature 1: Description
 * - Feature 2: Description
 * 
 * # Examples
 * ```rust
 * // Example code
 * ```
 */

// Section header with clear purpose
// --------------------------------

/// Function documentation with:
/// - Purpose
/// - Parameters
/// - Return values
/// - Examples
pub fn example_function() {
    // Implementation
}

// Commented out reference code
// ---------------------------
/*
// Keep this as reference for feature X
#[feature_x]
fn old_implementation() {
    // Old code kept for reference
}
*/
```

## Document Structure Reference

### Juno Datastore Documents

Every document in the Juno datastore follows this structure:

```typescript
interface Document<T> {
    // Unique identifier for the document
    key: string;  // Recommended: Use nanoid()
    
    // The actual data payload (any JSON-serializable data)
    data: T;
    
    // Optional field for filtering/organization (max 1024 chars)
    description?: string;
    
    // Metadata automatically managed by Juno
    owner: Principal;        // Document owner's Principal ID
    created_at: bigint;     // Creation timestamp (nanoseconds)
    updated_at: bigint;     // Last update timestamp (nanoseconds)
    version: bigint;        // Document version for concurrency control
}
```

### Working with Documents

1. **Creating Documents**:
```typescript
import { setDoc } from "@junobuild/core";
import { nanoid } from "nanoid";

const myId = nanoid();

await setDoc({
    collection: "my_collection_key",
    doc: {
        key: myId,
        data: {
            hello: "world"
        }
    }
});
```

2. **Updating Documents**:
```typescript
import { setDoc } from "@junobuild/core";

await setDoc({
    collection: "my_collection_key",
    doc: {
        key: myId,
        data: {
            count: 123
        },
        version: 3n
    }
});
```

3. **Batch Operations**:
```typescript
import { setManyDocs } from "@junobuild/core";

const update1 = {
    collection: "my_collection",
    doc: {
        key: "my_document_key_1",
        data: {
            hello: "world"
        }
    }
};

const update2 = {
    collection: "my_other_collection",
    doc: {
        key: "my_document_key_2",
        data: {
            count: 123
        }
    }
};

const docs = await setManyDocs({ docs: [update1, update2] });
```

### Important Notes

1. **Document Structure**:
   - `key`: Unique identifier (use nanoid())
   - `data`: Any JSON-serializable payload
   - `description`: Optional field for filtering/search (max 1024 chars)
   - `owner`, `created_at`, `updated_at`, `version`: Managed by Juno

2. **Version Control**:
   - Required for updates
   - Prevents concurrent edit conflicts
   - Must match current version
   - Automatically increments

3. **Filtering Options**:
   - By key (regex)
   - By description (regex)
   - By timestamps (created_at, updated_at)
   - By owner
   - Pagination support
   - Custom ordering

4. **Best Practices**:
   - Use nanoid() for keys
   - Include version for updates
   - Use atomic batch operations when possible
   - Leverage description field for filtering
   - Handle version conflicts gracefully

## AI Assistant Guidelines

When providing assistance, ALWAYS follow these guidelines:

### Explanation Requirements

1. **Command Explanations**
   - Before running ANY command, explain:
     - What the command does
     - Why it's necessary
     - What could go wrong
     - What the expected outcome is
   - Example:
     ```bash
     npm install vite
     # Explanation:
     # - What: Installing the Vite build tool
     # - Why: Required by SvelteKit to run the development server
     # - Risks: May conflict with existing versions
     # - Outcome: Enables 'npm run dev' to work
     ```

2. **Architecture Understanding**
   - Explain how different parts work together:
     - Frontend (SvelteKit) vs Backend (Juno Satellite)
     - Development servers vs Production builds
     - Local vs Deployed environments
   - Example:
     ```
     Your project has two main parts:
     1. Frontend (SvelteKit + Vite)
        - Runs on: npm run dev (localhost:3000)
        - Purpose: User interface
     2. Backend (Juno Satellite)
        - Runs on: juno dev start (localhost:8000)
        - Purpose: Data storage and business logic
     ```

3. **Error Handling**
   - For any error, explain:
     - What caused it
     - Why it matters
     - How to fix it
     - How to prevent it
   - Example:
     ```
     Error: 'vite' not found
     - Cause: Missing development dependency
     - Impact: Can't start frontend server
     - Fix: Install vite package
     - Prevention: Ensure package.json includes it
     ```

4. **Development Workflow**
   - Always clarify:
     - Which server needs to be running
     - What order to start things in
     - How to verify everything is working
     - How to troubleshoot common issues
   - Example:
     ```
     Development Setup:
     1. Start Juno server first (handles auth & data)
     2. Then start frontend server (shows UI)
     3. Verify by checking both URLs
     4. Common issue: Port conflicts
     ```

### Best Practices

1. **Never assume knowledge**
   - Explain terms and concepts
   - Provide context for commands
   - Link to relevant documentation
   - Use analogies for complex topics

2. **Show the big picture**
   - Explain how pieces fit together
   - Describe the flow of data
   - Clarify dependencies
   - Highlight important relationships

3. **Juno Documentation (`/juno/docs/build/index.md`)**
   - Verify Juno feature compatibility in `/juno/docs/docs/build/index.md`
   - Check best practices in `/juno/docs/docs/miscellaneous/best-practices.md`
   - Review examples in `/juno/docs/docs/guides/sveltekit.mdx`

4. **Framework Compatibility**
   - Ensure SvelteKit compatibility (`/juno/docs/docs/guides/sveltekit.mdx`)
   - Verify ICP patterns (`/core/ic_and_juno_api_reference.md`)
   - Check framework limitations (`/juno/docs/docs/guides/sveltekit.mdx#static-site-generation`)

## Directory Structure

### `/core`
Core reference documentation and specifications:
- `data-validation.md` (`/core/data-validation.md`) - Data validation patterns and security measures
- `ic_and_juno_api_reference.md` (`/core/ic_and_juno_api_reference.md`) - Complete API reference
- `technical_spec.md` (`/core/technical_spec.md`) - Technical specification
- `resources.md` (`/core/resources.md`) - External resources and links
- `skeleton_ui_integration.md` (`/core/skeleton_ui_integration.md`) - Skeleton UI v2 setup, configuration, and component usage

Key aspects:
- Architecture decisions (`/core/technical_spec.md#architecture`)
- Security patterns (`/core/data-validation.md#security`)
- Technical constraints (`/core/technical_spec.md#constraints`)
- UI setup and component patterns (`/core/skeleton_ui_integration.md`)

### `/implementation`
Implementation-specific documentation:
- `reputation.md` (`/implementation/reputation.md`) - Reputation system guide
- `juno_integration.md` (`/implementation/juno_integration.md`) - Integration patterns

Important considerations:
- Business logic (`/implementation/reputation.md#core-logic`)
- Integration patterns (`/implementation/juno_integration.md#patterns`)
- Data structures (`/implementation/reputation.md#data-structures`)

### `/juno`
Official Juno documentation and guides, with key sections:

#### Build Features (`/juno/docs/build/`)
- **Functions** (`/juno/docs/build/functions/`): 
  - Event-driven functions (`development.md#hooks`)
  - Lifecycle hooks (`development.md#on_init`)
  - Assertions (`development.md#assertions`)
  - Logic execution (`development.md#implementation`)
  - Initialization (`development.md#on_init`)
  - Collection handlers (`development.md#on_set_doc`)

- **Storage** (`/juno/docs/build/storage/`): 
  - File upload (`development.md#upload-asset`)
  - Protected assets (`development.md#protected-asset`)
  - Collections (`collections.md`)
  - Filtering (`development.md#list-assets`)
  - Metadata (`development.md#description`)
  - Access control (`development.md#security`)

- **Analytics** (`/juno/docs/build/analytics/`): 
  - Page tracking (`development.md#page-views`)
  - Event tracking (`development.md#track-custom-events`)
  - Web Vitals (`development.md#performance-metrics-with-web-vitals`)
  - Performance (`development.md#key-metrics`)
  - User analysis (`development.md#custom-events`)
  - Privacy (`development.md#data-collection`)

- **Components** (`/juno/docs/components/`):
  - Core utilities (`core.mdx`)
  - Bash helpers (`bash.mdx`)
  - Subnet tools (`subnets.md`)

#### SvelteKit Integration (`/docs/guides/sveltekit.mdx`)
Essential integration points:
- Project initialization
- Static site generation setup
- Local development configuration
- Production deployment
- Data management patterns
- Authentication flows
- Component integration

#### Development Guides
- **Local Development** (`/juno/docs/guides/local-development.md`):
  - Setup workflow (`#setup`)
  - Environment config (`#configuration`)
  - Testing (`#testing`)
  - Debugging (`#debugging`)

- **Deployment** (`/juno/docs/guides/manual-deployment.mdx`):
  - Build process (`#build`)
  - Configuration (`#config`)
  - Verification (`#verify`)
  - Monitoring (`#monitor`)

- **Component Patterns** (`/juno/docs/guides/components/`):
  - Build patterns (`build.mdx`)
  - Choice patterns (`choice.mdx`)
  - CLI usage (`cli.mdx`)
  - Deployment patterns (`deploy.mdx`)

## Documentation Updates

When updating documentation:
1. Core API changes in `ic_and_juno_api_reference.md` (`/core/ic_and_juno_api_reference.md`)
2. Implementation changes in respective files (`/implementation/`)
3. External resource links in `resources.md` (`/core/resources.md`)
4. Cross-references validation (`/docs/README.md`)

## Best Practices (`/juno/docs/miscellaneous/best-practices.md`)

1. **Version Control** (`#version-control`):
   - Commit messages
   - Change tracking
   - Version tagging
   - History maintenance

2. **Cross-References** (`#documentation`):
   - Relative links
   - Path validation
   - Link maintenance
   - Reference checking

3. **Code Examples** (`#code-examples`):
   - Up-to-date code
   - Tested snippets
   - Clear comments
   - Error handling

4. **API Documentation** (`#api-documentation`):
   - Complete coverage
   - Example usage
   - Parameter details
   - Return values

5. **Formatting** (`#formatting`):
   - Markdown standards
   - Consistent style
   - Clear structure
   - Readable layout

6. **Security Headers** (`#content-security-policy-csp`): 
   - CSP implementation
   - HTTP headers
   - iframe protection
   - Transport security

## Memory Management Guidelines (`/juno/docs/miscellaneous/memory.md`)

1. **Memory Types**:
   - Heap Memory (1GB max)
     - Fast read/write operations
     - Best for small datasets
     - Requires serialization during upgrades
     - Used for app bundles and assets
   - Stable Memory (400GB max)
     - Larger storage capacity
     - Slower but more resilient
     - Persists through upgrades
     - Used for user data and analytics

2. **Usage Patterns** (`/juno/docs/miscellaneous/memory.md#recommendations`):
   - Choose heap for frequently accessed data
   - Use stable for large datasets
   - Consider upgrade implications
   - Monitor memory consumption

3. **Performance Optimization** (`/juno/docs/miscellaneous/memory.md#in-a-nutshell`):
   - Balance memory types
   - Implement efficient data structures
   - Monitor memory limits
   - Plan for scalability

4. **Upgrade Considerations** (`/juno/docs/miscellaneous/memory.md#default-usage`):
   - Handle serialization properly
   - Test memory migration
   - Monitor upgrade impact
   - Plan for data growth

## SvelteKit-Specific Guidelines (`/juno/docs/guides/sveltekit.mdx`)

1. **Static Generation** (`/juno/docs/guides/sveltekit.mdx#static-site-generation`):
   - Use `@sveltejs/adapter-static` for deployment
   - Configure prerendering in `+layout.js`
   - Avoid server-side rendering dependencies

2. **Data Management** (`/juno/docs/build/datastore/development.md`):
   - Use atomic operations for data consistency
   - Implement version control for updates
   - Handle concurrent modifications
   - Use batch operations when appropriate

3. **Authentication Flow** (`/juno/docs/build/authentication/development.md`):
   - Implement proper error handling
   - Use state subscription for user management
   - Configure session timeouts appropriately
   - Handle authentication interruptions gracefully

4. **Performance Considerations** (`/juno/docs/build/datastore/development.md#set-multiple-documents`):
   - Use batch operations for multiple documents
   - Implement proper filtering and pagination
   - Optimize data queries and updates
   - Handle state management efficiently

## Serverless Functions Guidelines (`/juno/docs/build/functions/development.md`)

1. **Event Hooks** (`/juno/docs/build/functions/development.md#hooks`):
   - Document lifecycle events (create, update, delete)
   - Asset management events
   - Batch operation handlers
   - Initialization and upgrade hooks

2. **Assertions and Validation** (`/juno/docs/build/functions/development.md#assertions`):
   - Pre-operation validation
   - Custom business logic checks
   - Security constraints
   - Data integrity rules

3. **Implementation Patterns** (`/juno/docs/build/functions/development.md#on_set_doc`):
   - Collection-scoped handlers
   - Error handling and logging
   - Asynchronous operations
   - State management

4. **Development Workflow** (`/juno/docs/build/functions/lifecycle.md`):
   - Local testing with emulator
   - Function deployment
   - Version management
   - Debugging and monitoring

## Storage Management Guidelines (`/juno/docs/build/storage/development.md`)

1. **Asset Organization** (`/juno/docs/build/storage/collections.md`):
   - Collection-based file structure
   - Filename and path conventions
   - Protected vs public assets
   - Token-based access control

2. **Upload Patterns** (`/juno/docs/build/storage/development.md#upload-asset`):
   - File type handling
   - Custom headers and encoding
   - Overwrite protection
   - Error handling

3. **Asset Retrieval** (`/juno/docs/build/storage/development.md#list-assets`):
   - Filtering and pagination
   - Timestamp-based queries
   - Owner-based filtering
   - Sorting and ordering

4. **Security Considerations** (`/juno/docs/build/storage/development.md#protected-asset`):
   - Token generation and management
   - Access control implementation
   - URL handling and encoding
   - Asset protection strategies

## Analytics Guidelines (`/juno/docs/build/analytics/development.md`)

1. **Page View Tracking** (`#page-views`):
   - Automatic setup (`#automatic-tracking`)
   - Navigation monitoring (`#navigation-events`)
   - Journey analysis (`#user-journey`)
   - Privacy settings (`#privacy`)

2. **Custom Events** (`#track-custom-events`):
   - Event naming (`#event-names`)
   - Metadata structure (`#metadata`)
   - Size limits (`#limitations`)
   - Validation rules (`#validation`)

3. **Performance Monitoring** (`#performance-metrics-with-web-vitals`):
   - Web Vitals setup (`#key-metrics`)
   - TTFB metrics (`#time-to-first-byte`)
   - Layout analysis (`#cumulative-layout-shift`)
   - Latency tracking (`#interaction-to-next-paint`)

4. **Configuration Options** (`/juno/docs/build/analytics/setup.mdx`):
   - Initialization (`#init`)
   - Opt-out settings (`#opting-out`)
   - Data rules (`#data-collection`)
   - Customization (`#configuration`)

## Configuration Guidelines (`/juno/docs/miscellaneous/configuration.mdx`)

1. **Project Setup** (`#satellite-configuration`):
   - File structure (`#config-file`)
   - Environment setup (`#environments`)
   - Deploy config (`#deployment`)
   - Resource setup (`#resources`)

2. **Satellite Configuration** (`#id-or-ids`):
   - ID setup (`#satellite-id`)
   - Source config (`#source`)
   - File handling (`#file-handling`)
   - Memory config (`#memory-limits`)

3. **Build Settings** (`#predeploy`):
   - Deploy hooks (`#hooks`)
   - Asset building (`#assets`)
   - Test setup (`#testing`)
   - Environment vars (`#environment-variables`)

4. **Resource Management** (`#maximum-memory-size`):
   - Memory setup (`#memory-allocation`)
   - Compute config (`#compute-resources`)
   - Storage setup (`#storage-limits`)
   - Auth settings (`#authentication`)

## Collaboration Guidelines (`/juno/docs/miscellaneous/workarounds.md`)

1. **Satellite Transfer** (`#transferring-a-satellite-to-another-account`):
   - Controller setup (`#add-the-new-controllers`)
   - Permission config (`#permissions`)
   - Account setup (`#account-setup`)
   - Access checks (`#verify-access`)

2. **Team Collaboration** (`#how-to-collaborate-on-the-same-project`):
   - Identity config (`#create-a-new-identity`)
   - Controller setup (`#controller-setup`)
   - Access rules (`#access-management`)
   - Permission types (`#permission-levels`)

3. **Security Considerations** (`/juno/docs/miscellaneous/best-practices.md#security`):
   - ID verification (`#identity-verification`)
   - Controller checks (`#controller-validation`)
   - Access removal (`#access-revocation`)
   - Audit setup (`#audit-logging`)

4. **Best Practices** (`#collaboration-best-practices`):
   - Communication (`#communication`)
   - Documentation (`#documentation`)
   - Planning (`#transition-planning`)
   - Backups (`#backup-procedures`)

## Common Workarounds (`/juno/docs/miscellaneous/workarounds.md`)

1. **Satellite Management** (`#satellite-management`):
   - Account transfer (`#transferring-a-satellite`)
   - Controller setup (`#controller-configuration`)
   - Mission setup (`#mission-control-setup`)
   - Detachment (`#detachment`)

2. **Identity Sharing** (`#identity-sharing`):
   - II setup (`#internet-identity-setup`)
   - Passkey config (`#passkey-management`)
   - Device setup (`#device-registration`)
   - Access sync (`#access-coordination`)

3. **Access Control** (`/juno/docs/miscellaneous/controllers.md`):
   - Permission setup (`#permission-setup`)
   - Controller config (`#controller-config`)
   - ID checks (`#identity-checks`)
   - Security rules (`#security-rules`)

4. **Transition Planning** (`#transition`):
   - Handover steps (`#handover`)
   - Access checks (`#access-checks`)
   - Backup plan (`#backup-plan`)
   - Doc updates (`#documentation-updates`)

## Dependency Configuration

### Important: Juno Feature Configuration

When configuring Juno dependencies in `Cargo.toml`, follow these guidelines:

1. For `junobuild-satellite`:
   ```toml
   junobuild-satellite = { 
       version = "0.0.22", 
       default-features = false, 
       features = ["on_set_doc", "assert_set_doc"] 
   }
   ```
   - Must disable default features with `default-features = false`
   - Explicitly specify only the features you need
   - Don't include features you're not using

2. For `junobuild-macros`:
   ```toml
   junobuild-macros = { version = "0.0.4" }
   ```
   - Do not specify any features
   - The macros are enabled through the satellite features

3. Other required dependencies:
   ```toml
   junobuild-shared = "0.0.24"
   junobuild-utils = "0.0.4"
   ```

This configuration ensures that:
- Only necessary features are included
- Macro dependencies are properly resolved
- No conflicts with feature flags

### Common Issues to Avoid

1. Don't enable features in `junobuild-macros` - they are controlled through `junobuild-satellite`
2. Always disable default features when specifying custom features
3. Only include the features you actually use in your code

## AI Assistant Notes (`/docs/README.md#ai-assistant-guidelines`)

When suggesting solutions:
1. Verify Juno/ICP compatibility (`/core/ic_and_juno_api_reference.md`)
2. Check SvelteKit patterns (`/juno/docs/guides/sveltekit.mdx`)
3. Follow validation patterns (`/core/data-validation.md`)
4. Consider blockchain impact (`/core/technical_spec.md#performance`)
5. Alternative approaches must:
   - Match SvelteKit/Juno/ICP (`/implementation/juno_integration.md`)
   - Improve patterns (`/core/technical_spec.md#patterns`)
   - Maintain security (`/core/data-validation.md#security`)
   - Follow documentation (`/docs/README.md#documentation-updates`)

## Documentation Structure

### Core Documentation
- `docs/core/ic_and_juno_api_reference.md` - Comprehensive API reference for Internet Computer and Juno
- `docs/core/juno_index.md` - Quick reference guide for Juno integration, setup, and development

### Juno Documentation
// ... existing code ... 