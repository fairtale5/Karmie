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

## WSL Development Environment

### Fixing WSL Permissions

If you encounter permission issues in WSL (common when files appear as "read-only" or have incorrect permissions), follow these steps:

1. **Configure WSL Mount Options**
   Create or edit `/etc/wsl.conf`:
   ```ini
   [automount]
   enabled = true
   options = "metadata,umask=22,fmask=11"
   ```

   This configuration:
   - Enables metadata for persisting WSL file permissions
   - Sets umask to mask out group/others write bits (files: 0644, directories: 0755)
   - Sets fmask to mask out group/others execute bits for files

2. **Fix Default Permissions**
   Add to your `~/.profile`:
   ```bash
   # Fix WSL umask if not set properly
   if [[ "$(umask)" = "0000" ]]; then
     umask 0022
   fi
   ```

3. **Apply Changes**
   ```bash
   # Restart WSL to apply changes
   wsl --shutdown
   # Then restart your terminal
   ```

These settings ensure:
- Files have permissions 0644 (rw-r--r--)
- Directories have permissions 0755 (rwxr-xr-x)
- New files maintain correct permissions
- Git and other tools work correctly with file permissions

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

### `/docs/core`
Core project documentation:

#### `/docs/core/architecture`
Architecture and design decisions:
- `reputation-system.md` - Technical architecture of the reputation system
- `technical_spec.md` - Technical specifications and constraints
- `skeleton_ui_integration.md` - Skeleton UI v2 setup and component patterns

#### `/docs/core/development`
Development guidelines and patterns:
- `testing.md` - Testing strategy and implementation
- `ui.md` - UI/UX guidelines and principles
- `data-validation.md` - Data validation patterns and security measures

### `/docs/resources`
Core reference documentation:
- `ic_and_juno_api_reference.md` - Complete API reference for Internet Computer and Juno
- `juno_index.md` - Quick reference guide for Juno integration, setup, and development

### `/docs/juno`
Official Juno documentation and guides, with key sections:

#### Build Features (`/docs/juno/docs/build/`)
- **Functions** (`/docs/juno/docs/build/functions/`): 
  - Event-driven functions (`development.md#hooks`)
  - Lifecycle hooks (`development.md#on_init`)
  - Assertions (`development.md#assertions`)
  - Logic execution (`development.md#implementation`)
  - Initialization (`development.md#on_init`)
  - Collection handlers (`development.md#on_set_doc`)
  - Logging (`logs.md`) - Native and custom logging capabilities

- **Storage** (`/docs/juno/docs/build/storage/`): 
  - File upload (`development.md#upload-asset`)
  - Protected assets (`development.md#protected-asset`)
  - Collections (`collections.md`)
  - Filtering (`development.md#list-assets`)
  - Metadata (`development.md#description`)
  - Access control (`development.md#security`)

- **Analytics** (`/docs/juno/docs/build/analytics/`): 
  - Page tracking (`development.md#page-views`)
  - Event tracking (`development.md#track-custom-events`)
  - Web Vitals (`development.md#performance-metrics-with-web-vitals`)
  - Performance (`development.md#key-metrics`)
  - User analysis (`development.md#custom-events`)
  - Privacy (`development.md#data-collection`)

- **Components** (`/docs/juno/docs/components/`):
  - Core utilities (`core.mdx`)
  - Bash helpers (`bash.mdx`)
  - Subnet tools (`subnets.md`)

#### SvelteKit Integration (`/docs/juno/docs/guides/sveltekit.mdx`)
Essential integration points:
- Project initialization
- Static site generation setup
- Local development configuration
- Production deployment
- Data management patterns
- Authentication flows
- Component integration

#### Development Guides
- **Local Development** (`/docs/juno/docs/guides/local-development.md`):
  - Setup workflow (`#setup`)
  - Environment config (`#configuration`)
  - Testing (`#testing`)
  - Debugging (`#debugging`)

- **Deployment** (`/docs/juno/docs/guides/manual-deployment.mdx`):
  - Build process (`#build`)
  - Configuration (`#config`)
  - Verification (`#verify`)
  - Monitoring (`#monitor`)

- **Component Patterns** (`/docs/juno/docs/guides/components/`):
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

2. **Cross-References** (`