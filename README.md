# Karmie

A decentralized reputation system built on the Internet Computer using Juno and SvelteKit.

**Intro/Overview**

I wanted a reputation system that is:
- Truly decentralized
- Bot-resistant
- Doesn't require KYC

### How It Works
Reputation is earned by being voted on by users who already have a reputation. The higher the reputation of the user voting on you, the greater the effect—both positive and negative.

This means that bots, bad actors, and newcomers all start with a reputation of **0**. They can vote on others, but their votes won't have an effect until they reach the **minimum threshold** (which is customizable).

Additionally, if the community downvotes a user, they lose their privileges, and all users they voted on also lose the reputation they gained from them. This makes it easy to identify and neutralize bad actors by downvoting them, undoing any damage they caused.

### Bootstrapping New Communities
The challenge was how to bootstrap new communities, as early on, no one has enough reputation to vote on others. To address this, I implemented a **"reward for voting"** system, which works in two phases:
- **Early on**: Everyone receives rewards while the community is still small. At this stage, anyone can join and earn voting rewards.
- **Later on**: Only trusted users who have already gained reputation receive rewards. This prevents bad actors from farming reputation through votes.

### Technical Details
- Runs in its own **canister**
- Backend powered by **Juno (juno.build)**
- Custom functions written in **Rust**
- Any ICP app can access its API remotely and integrate it into their own apps

### Graph Visualization
The system includes interactive graph visualization to show vote relationships between users:

#### Smart Edge Rendering
To keep graphs clean and readable, we use smart edge consolidation:
- **Bidirectional Same Sentiment**: Single double-ended arrow (straight for positive, curved for negative)
- **Bidirectional Conflicting**: Two separate arrows with different curvatures
- **Unidirectional**: Single directional arrow

#### Visual Encoding
- **Green edges** = positive votes (+1)
- **Red edges** = negative votes (-1) 
- **Edge thickness** = vote count/weight
- **Node size** = user reputation
- **Edge labels** = vote counts and direction indicators (+/- with counts)

*Note: Advanced curved edges for negative votes and double-ended arrows for mutual relationships are planned for future implementation.*

#### Implementation
- **Frontend**: Sigma.js with ForceAtlas2 layout algorithm for natural positioning
- **Backend**: Rust functions that intelligently process votes into graph data
- **Integration**: Real-time loading in tag pages and dashboard views

This approach reduces visual clutter (max 2 edges between users instead of 4) while preserving all relationship information.

### Custom Reputations
Anyone can create new **#reputations** (like Twitter hashtags). This means there can be reputations for:
- A specific app
- A topic (e.g., #BTC, #startup)

Each reputation has its own customizable rules, set by the creator. Currently, these include:
- **Decay rules**: Votes decay over time, using either default settings or custom configurations
  * Configurable time periods (e.g., 1 month, 3 months, 6 months)
  * Custom decay multipliers for each period, allowing for votes to loose power over time
- **Minimum threshold for trusted users**.
- **Minimum number of trusted users before stopping voting rewards**. If the community shrinks, this mechanism is re-enabled automatically.

### Handling Cascading Updates
One of the biggest challenges is managing cascading updates. I've implemented caching mechanisms to optimize this. Reputation updates occur only when:
- A user is queried
- A user casts a vote
- A user receives a vote

This system is working well so far.

### Integration with Apps
You can define actions within your app that grant reputation to users. This means your app's **canister user** will be voting on community members based on specific actions.

- The community can also vote back.
- You can automate parts of this process, e.g., after every transaction, both the user and the app receive votes.
- Admin users need to maintain enough reputation, so this helps keep them active.

You simply add hooks to the desired actions. For example, a social media app could embed votes into "like" buttons.

I'm also developing a **web interface** for users to interact with the system, but the primary goal is for apps to create their own integrations.

### Next Features
- **Improve caching** further so older votes aren't always recalculated and can be fetched less frequently.
- **Overarching reputations**: Apps can link their reputation systems together, allowing trusted users from one app to carry over trust to another. Each app remains independent but can participate in a **trusted circle** with custom rules for how influence is shared.





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

#### Frontend

##### Routes (`/src/routes`)
- Main application pages and user interface
- Organized by feature area:
  - `/` - Home page
  - `/profile` - User profile page and settings
  - `/admin` - Administrative functions 
  - `/onboarding` - User registration and setup
  - `/reputations` - Reputation management
- Each route typically contains:
  - `+page.svelte` - The page component with UI and interactions
  - `+layout.svelte` - Shared layout for route group (when applicable)
  - `+page.ts` or `+page.server.ts` - Data loading logic

##### Library (`/src/lib`)
- **Components** (`/src/lib/components/`)
  - `Header.svelte` - Main navigation component
  - `Footer.svelte` - Site footer component
  - `AvatarCropper.svelte` - Profile image handling
  - `SkeletonLoader.svelte` - Loading states
  - **Graph Components** (`/src/lib/components/graph/`)
    - `SigmaGraph.svelte` - Interactive graph visualization with Sigma.js
    - `graphData.ts` - Graph data structures and dummy data generation
  - **Tag Components** (`/src/lib/components/tags/`)
    - `TagGraphCard.svelte` - Tag-specific graph visualization card
  - Various UI elements and shared components
- **Types** (`/src/lib/types.ts`)
  - TypeScript interfaces for data models
  - Type definitions for state and props
- **Settings** (`/src/lib/settings.ts`)
  - Application configuration
  - Environment variables
  - Feature flags and constants
- **SkeletonUI** (`/src/lib/skeletonui/`)
  - Skeleton UI framework components
  - Theming and styling utilities
- **Juno Integration** (`/src/lib/juno.ts`)
  - Juno client SDK setup
  - Authentication helpers
- **Stores** (`/src/lib/stores/`)
  - Svelte stores for state management
  - Shared application state
- **Keys** (`/src/lib/keys/`)
  - Document key management
  - Key generation utilities
- **Utils** (`/src/lib/utils/`)
  - `graphApi.ts` - Graph data API functions and type definitions

##### App Configuration
- `app.html` - HTML template
- `app.css` - Global styles
- `app.d.ts` - TypeScript declarations

#### Backend (`/src/satellite`)

##### Configuration Files
- `Cargo.toml` - Rust dependencies and project settings
- `satellite.did` - Candid interface definitions for Internet Computer
- `satellite_extension.did` - Extended interface definitions

##### Source Files (`/src/satellite/src`)
- `lib.rs` - Main entry point and core functionality
  - Satellite initialization
  - API endpoints
  - Event handlers
  - Core business logic

###### Core Logic (`/src/satellite/src/core/`)
- Core business logic and domain-specific calculations
- `reputation_calculations.rs` - Reputation scoring algorithms
- `tag_calculations.rs` - Tag-based calculations and filtering

###### Processors (`/src/satellite/src/processors/`)
- Data processing and document operations
- `document_keys.rs` - Key generation and management for documents
- `document_queries.rs` - Database query helpers
- `graph_processors.rs` - Graph data generation for visualization
- `ulid_generator.rs` - ULID generation for unique identifiers
- `ulid_timestamp_extract.rs` - Timestamp extraction from ULIDs
- `ulid_type.rs` - ULID type definitions and utilities

###### Assertion Logic (`/src/satellite/src/assert_set_doc/`)
- Business rules and validations for document creation
- `assert_doc_user.rs` - User document validation
- `assert_doc_tag.rs` - Tag document validation
- `assert_doc_vote.rs` - Vote document validation
- `assert_doc_reputation.rs` - Reputation document validation

###### Validation (`/src/satellite/src/validation/`)
- Data validation for individual fields
- `validate_handle.rs` - Username validation
- `display_name.rs` - User display name validation
- `description.rs` - Text description validation
- `validate_tag_date.rs` - Date validation for tags
- `ulid_timestamp_validate.rs` - ULID timestamp validation

###### Utilities (`/src/satellite/src/utils/`)
- Shared helper functions
- `structs.rs` - Data structures shared across the satellite
- `logger.rs` - Logging utilities
- `time.rs` - Time-related helper functions
- `normalize.rs` - String normalization utilities

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

### Core Documentation
- [Project Overview](/docs/core/architecture/reputator-dApp-overview.md) - High-level system overview
- [Database Architecture](/docs/core/architecture/database.md) - Database design and standards
- [Data Validation](/docs/core/architecture/data-validation-reputator.md) - Validation rules and implementation
- [Playground vs Production](/docs/core/architecture/playground_vs_production.md) - Environment differences

### Development Guides
- [Development Guide](/docs/core/development/development.md) - Development setup and workflow
- [Testing Guide](/docs/core/development/testing.md) - Testing strategies and implementation
- [Juno Integration](/docs/core/development/juno_integration.md) - Juno integration details
- [Test Calculations](/docs/core/development/test-calculations.md) - Reputation calculation examples
- [Description Migration](/docs/core/development/description_migration_track_temp.md) - Field format standards

### Resources and References
- [API Reference](/docs/resources/ic_and_juno_api_reference.md) - Complete API documentation
- [Juno Index](/docs/resources/juno_index.md) - Quick reference for Juno
- [Data Validation (Juno)](/docs/resources/data-validation-juno.md) - Juno validation patterns
- [Development Resources](/docs/resources/development.md) - Additional development resources
- [Resource Links](/docs/resources/resources.md) - External resources and tools

### Project Guidelines
- [Project Rules](.cursorrules) - AI assistant rules and project standards
- [UI Guidelines](/docs/core/development/ui.md) - UI/UX standards and patterns

