# Reputator

A decentralized reputation system built on the Internet Computer using Juno and SvelteKit.

## Architecture

### Juno Platform
Juno is our all-in-one backend platform that provides:
- **Satellite**: A smart contract on the Internet Computer that runs our backend code
- **Datastore**: A decentralized database for storing user data and reputation events
- **Storage**: File hosting for assets and user uploads
- **Authentication**: Built-in user authentication and authorization
- **Analytics**: Usage tracking and performance monitoring

### Application Structure
Our app is split into two main parts:

#### Frontend (`/src/routes`)
- Built with SvelteKit
- Deployed as static files through Juno Storage
- Communicates with the Satellite through Juno's client SDK
- Key features:
  - User interface and interactions
  - Real-time data updates
  - Client-side validation
  - API calls to Satellite functions

#### Backend (`/src/satellite`)
- Runs as a Juno Satellite on the Internet Computer
- Handles all business logic and data operations
- Key components:
  - Custom functions for reputation calculations
  - Database operations for user data
  - Event handling and validation
  - Security rules and access control

### Data Storage
We use Juno's Datastore for different types of data:
- **Users**: Profile information and reputation scores
- **Events**: Reputation events and interactions
- **Settings**: System configuration and rules
- **Analytics**: Usage data and metrics

### Custom Functions
Our Satellite includes custom functions for:
- Reputation calculation and updates
- User management and validation
- Event processing and recording
- Data aggregation and reporting

## Project Structure

### Source Code (`/src`)

#### Frontend (`/src/routes`)
- Main application pages and API endpoints
- Organized by route (e.g., `/admin`, `/users`)
- Each route has its own folder with:
  - `+page.svelte` - The page component
  - `+page.server.ts` - Server-side logic
  - `+server.ts` - API endpoints

#### Components (`/src/lib/components`)
- Reusable UI components
- Shared layouts and styles
- Common utilities and helpers

#### Types (`/src/lib/types.ts`)
- TypeScript interfaces and types
- Shared type definitions

#### Settings (`/src/lib/settings.ts`)
- Application configuration
- Environment variables
- Constants

### Satellite Code (`/src/satellite`)
- Backend code running on the Internet Computer
- Rust-based implementation

#### Source Files (`/src/satellite/src`)
- `lib.rs` - Main satellite code
- `utils/` - Helper functions and utilities

#### Configuration (`/src/satellite`)
- `Cargo.toml` - Rust dependencies and project settings
- `satellite.did` - Candid interface definitions
- `satellite_extension.did` - Extended interface definitions

### Documentation (`/docs`)

#### Core Documentation (`/docs/core`)
- Project-specific documentation
- Architecture decisions
- Implementation guides
- Key files:
  - `data-validation.md` - Data validation rules
  - `resources.md` - External resources
  - `skeleton_ui_integration.md` - UI setup guide

#### Implementation Docs (`/docs/implementation`)
- Detailed implementation guides
- Key files:
  - `reputation.md` - Reputation system design
  - `juno_integration.md` - Juno integration guide

#### Resources (`/docs/resources`)
- Reference documentation
- Key files:
  - `ic_and_juno_api_reference.md` - API documentation
  - `juno_index.md` - Juno quick reference

#### Juno Docs (`/docs/juno`)
- Juno-specific documentation
- Build features
- Integration guides
- Best practices

## Development

### Local Development
```bash
# Start frontend development server
npm run dev

# Start local Juno emulator (requires Docker)
juno dev start
```

### Production Preview
```bash
# Build the project
npm run build

# Preview production build
npm run preview
```

### Deployment
```bash
# Deploy to Juno Satellite
juno deploy
```

## Additional Documentation

- [Project Guidelines](.cursorrules) - AI assistant rules and project standards
- [Core Documentation](/docs/core/README.md) - Detailed project documentation
- [Development Guide](/docs/core/development/development.md) - Development setup and workflow
- [API Reference](/docs/resources/ic_and_juno_api_reference.md) - Complete API documentation
- [Juno Integration](/docs/implementation/juno_integration.md) - Juno integration details
