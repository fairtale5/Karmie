# Juno Documentation Index

This document serves as a central reference point for all Juno-related documentation in our project. It provides quick access to essential information about Juno's features, setup, and development workflow.

:::warning
IMPORTANT: Follow the installation steps in order. DO NOT install `@junobuild/core` directly - it will be installed automatically during project initialization.
:::

:::note
This index is maintained in our repository's `/docs/core` directory. When Juno releases new documentation, you may need to update the paths in this index to match the new structure in `/docs/juno`.
:::

## Quick Start

### Prerequisites
1. Docker installed and running
2. Node.js installed
3. Git installed

### Development Environment Setup
1. Install Docker (required for local development)
2. Install Juno CLI:
   ```bash
   npm install -g @junobuild/cli
   ```
   Purpose: Installs the Juno command-line interface
   Why: Required for local development and project management
   Risks: May require sudo/admin privileges
   Outcome: Enables Juno CLI commands

3. Start local development environment:
   ```bash
   juno dev start
   ```
   Purpose: Starts the local Juno development environment
   Why: Required for local development and testing
   Risks: Port conflicts (default: 5987)
   Outcome: Local Juno environment running

4. Stop local development environment:
   ```bash
   juno dev stop
   ```
   Purpose: Stops the local Juno development environment
   Why: Required to free up system resources
   Risks: None
   Outcome: Local Juno environment stopped

### Project Initialization
```bash
npm create juno@latest
```
Purpose: Creates a new Juno project with latest template
Why: Sets up project structure and dependencies
Risks: May conflict with existing project files
Outcome: New Juno project structure created

:::warning
Common Mistake: Installing `@junobuild/core` directly
- ❌ DO NOT: `npm install @junobuild/core`
- ✅ DO: Use `npm create juno@latest` which will install all required dependencies
:::

## Core Features

### Datastore
- [Development Guide](/juno/docs/build/datastore/development.md)
  - Document management
  - Collections
  - Data validation
  - Batch operations

### Authentication
- [Development Guide](/juno/docs/build/authentication/development.md)
  - Sign-in/Sign-out
  - User session management
  - Internet Identity integration
  - NFID provider support

### Storage
- [Development Guide](/juno/docs/build/storage/development.md)
  - File upload/download
  - Asset management
  - Protected assets
  - Collection organization

### Functions (Rust)
- [Development Guide](/juno/docs/build/functions/development.md)
  - Event hooks
  - Custom logic
  - Data validation
  - Batch operations

### Analytics
- [Development Guide](/juno/docs/build/analytics/development.md)
  - Page view tracking
  - Custom events
  - Web Vitals metrics
  - Performance monitoring

## Development Workflow

### Local Development
- [Local Development Guide](/juno/docs/guides/local-development.md)
  - Docker setup
  - Environment configuration
  - Hot reloading
  - Testing

### Deployment
- [Manual Deployment Guide](/juno/docs/guides/manual-deployment.mdx)
  - Build process
  - Deployment steps
  - Verification
  - Monitoring

### SvelteKit Integration
- [SvelteKit Guide](/juno/docs/guides/sveltekit.mdx)
  - Project setup
  - Static site generation
  - Data management
  - Authentication flow

## Architecture & Infrastructure

### Technical Architecture
- [Architecture Overview](/juno/docs/white-paper/architecture.md)
  - Core components
  - Smart contracts
  - Infrastructure
  - Security model

### Memory Management
- [Memory Guidelines](/juno/docs/miscellaneous/memory.md)
  - Heap memory (1GB max)
  - Stable memory (400GB max)
  - Usage patterns
  - Performance optimization

## Best Practices

### Configuration
- [Configuration Guide](/juno/docs/miscellaneous/configuration.mdx)
  - Project setup
  - Environment variables
  - Resource management
  - Security settings

### Security
- [Best Practices](/juno/docs/miscellaneous/best-practices.md)
  - Authentication
  - Data validation
  - Access control
  - Error handling

## Troubleshooting

### Common Issues
- [FAQ](/juno/docs/faq.md)
  - Support channels
  - Cost information
  - Upgrade verification
  - Platform limitations

### Workarounds
- [Common Workarounds](/juno/docs/miscellaneous/workarounds.md)
  - Satellite management
  - Identity sharing
  - Access control
  - Transition planning

## API Reference

For detailed API documentation, see our [IC and Juno API Reference](/core/ic_and_juno_api_reference.md).

## Important Notes

1. **Version Control**
   - Always upgrade sequentially
   - Avoid skipping versions
   - Check changelog for breaking changes

2. **Resource Limits**
   - Heap Memory: 1GB max
   - Stable Memory: 400GB max
   - Document size: 1MB max
   - Batch size: 100 documents max

3. **Development Environment**
   - Local Satellite ID: `jx5yt-yyaaa-aaaal-abzbq-cai`
   - Default ports: 5987 (Satellite), 5999 (Admin)
   - Hot reloading supported

4. **Security Considerations**
   - Always validate data
   - Implement proper access control
   - Use secure authentication flows
   - Follow best practices for sensitive data

## Updates and Maintenance

When Juno releases new documentation:
1. Update paths in this index to match new structure
2. Review and update any deprecated features
3. Check for new best practices
4. Update local development setup if needed

## Support

- [Discord Community](https://discord.gg/wHZ57Z2RAG)
- [GitHub Issues](https://github.com/junobuild/juno)
- [Documentation](https://docs.juno.build) 