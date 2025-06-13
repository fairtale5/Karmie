# Karmie

[![Built with SvelteKit](https://img.shields.io/badge/SvelteKit-FF3E00?style=for-the-badge&logo=svelte&logoColor=white)](https://kit.svelte.dev)
[![Powered by Juno](https://img.shields.io/badge/Juno-000000?style=for-the-badge&logo=internetcomputer&logoColor=white)](https://juno.build)
[![Internet Computer](https://img.shields.io/badge/Internet_Computer-29ABE2?style=for-the-badge&logo=internetcomputer&logoColor=white)](https://internetcomputer.org)

## Table of Contents
- [Overview](#overview)
- [Who This Is For](#who-this-is-for)
- [How It Works](#how-it-works)
- [Architecture](#architecture)
- [Database](#database)
- [Development](#development)
- [Documentation](#documentation)

## Overview

Karmie is a decentralized reputation system that helps communities build trust and identify valuable contributors. It's designed to be bot-resistant, transparent, and flexible enough to work across any platform or community.

### Why Karmie?
- **Truly Decentralized**: Runs on the Internet Computer, no central authority
- **Bot-Resistant**: Only trusted users can influence reputation
- **Privacy-Focused**: No KYC required, just Internet Identity
- **Flexible**: Create custom reputation tags for any use case
- **Transparent**: All votes and calculations are on-chain
- **Self-Protecting**: Bad actors are automatically neutralized

## Who This Is For

Karmie is perfect for any platform that needs to distinguish real users from bots and reward genuine contributions. Here are some key use cases:

### 🛒 E-commerce Marketplaces
- Create bot-proof marketplaces where buyer and seller reputations actually matter
- Track seller reliability and buyer behavior
- Build trust in peer-to-peer trading platforms
- Rate freelancers, contractors, and service providers

### 🪂 Token Distribution & Airdrops
- Ensure rewards reach real humans, not bots
- Filter airdrop recipients based on #genuine reputation
- Distribute community rewards based on contribution history
- Create incentive programs that reward real engagement

### 🗣️ Web3 Social Media
- Build platforms where expertise determines influence, not follower count
- Give domain experts more weight in their areas of expertise
- Create higher quality discussions and content curation
- Enable topic-based influence (e.g., developers in #programming discussions)

### 🎮 Gaming Communities
- Track player skills, teamwork, and community behavior
- Build #skill reputation through gameplay
- Monitor #sportsmanship and #teamwork
- Carry trust between different games

### 💼 Professional Networks
- Build verifiable expertise without traditional credentials
- Enable peer validation of technical abilities
- Find teammates based on reputation history
- Give experts more influence in their domains

### 🏛️ Community Governance
- Create democratic decision-making with weighted expertise
- Weight votes by relevant expertise
- Enable trusted members to help govern content
- Allocate resources based on contribution history

## How It Works

### Core Reputation Mechanics
- Users gain reputation in two ways:
  1. By being voted on by trusted users
  2. By voting on others (earning voting rewards)
- Each user's reputation is split across all their votes
- Reputation tags have a minimum threshold of trusted users
- Once a tag reaches this threshold of trusted users:
  - Only trusted users' votes count
  - Only trusted users receive voting rewards
- This creates a self-protecting system where:
  - Bad actors must earn trust from existing trusted users
  - New users must prove themselves before gaining influence
  - The community can easily identify and neutralize bad actors

### Tag System
- Anyone can create reputation tags (like Twitter hashtags)
- Tags can be:
  - App-specific (e.g., #myapp)
  - Topic-based (e.g., #BTC, #startup)
- Each tag has customizable rules:
  - Decay rules for vote influence over time
  - Minimum threshold for trusted users
  - Minimum trusted users before stopping voting rewards
  - Automatic re-enabling of rewards if community shrinks

### Bootstrapping New Communities
The system handles the "cold start" problem through a two-phase reward system:
1. **Early Phase**: Everyone receives voting rewards while the community is small
2. **Mature Phase**: Only trusted users receive rewards, preventing reputation farming

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

### Project Structure

Our app is split into two main parts:

#### Frontend (`/src`)
- Built with SvelteKit
- Deployed as static files through Juno Storage
- Communicates with the Satellite through Juno's client SDK
- Key features:
  - User interface and interactions
  - Real-time data updates
  - Client-side validation
  - API calls to Satellite functions

```
src/
├── app.css                    # Global styles
├── app.html                   # HTML template
├── app.d.ts                   # TypeScript declarations
├── declarations/              # Type declarations
├── routes/                    # SvelteKit routes
│   ├── +layout.svelte         # Root layout
│   ├── +layout.ts             # Layout logic
│   ├── +page.svelte           # Home page
│   ├── admin/                 # Admin pages
│   ├── dashboard/             # Dashboard pages
│   ├── graph/                 # Graph visualization pages
│   ├── new/                   # New item creation
│   │   ├── tag/               # Tag creation form
│   │   └── user/              # User profile creation
│   ├── tag/                   # Tag views and management
│   └── u/                     # User profiles
└── lib/                       # Core frontend library
    ├── auth/                  # Authentication
    ├── components/            # UI components
    │   ├── common/            # Shared components
    │   ├── content/           # Content display components
    │   ├── dashboard/         # Dashboard components
    │   ├── graph/             # Graph visualization
    │   ├── layout/            # Layout components
    │   ├── modals/            # Modal dialogs
    │   ├── onboarding/        # User onboarding
    │   ├── profile/           # User profile
    │   └── tags/              # Tag-related components
    ├── data/                  # Static data and types
    ├── docs-crud/             # Document operations
    ├── keys/                  # Key management
    ├── skeletonui/            # UI framework
    ├── stores/                # State management
    ├── utils/                 # Helper functions
    ├── juno.ts                # Juno client setup
    ├── login.ts               # Login utilities
    ├── settings.ts            # App configuration
    └── types.ts               # TypeScript types
```

### Backend (`/src/satellite`)
Runs as a Juno Satellite on the Internet Computer, handling all business logic, data operations, and security.

```
src/satellite/
├── Cargo.toml                            # Rust dependencies
├── satellite.did                         # Candid interface
├── satellite_extension.did               # Extended interface
└── src/                                  # Source code
    ├── lib.rs                            # Main entry point
    ├── core/                             # Core business logic
    │   ├── reputation_calculations.rs    # Reputation score calculations
    │   └── tag_calculations.rs           # Tag-specific calculations
    ├── assert_set_doc/                   # Document validation
    │   ├── assert_doc_user.rs            # User document validation
    │   ├── assert_doc_tag.rs             # Tag document validation
    │   ├── assert_doc_vote.rs            # Vote document validation
    │   └── assert_doc_reputation.rs      # Reputation document validation
    ├── validation/                       # Field validation
    │   ├── validate_handle.rs            # Username/tag handle validation
    │   ├── validate_tag_date.rs          # Tag time period validation
    │   ├── display_name.rs               # Display name validation
    │   ├── description.rs                # Description validation
    │   └── ulid_timestamp_validate.rs    # ULID timestamp validation
    ├── processors/                       # Data processing
    │   ├── document_keys.rs              # Document key generation/validation
    │   ├── document_queries.rs           # Document querying utilities
    │   ├── ulid_generator.rs             # ULID generation
    │   ├── ulid_timestamp_extract.rs     # ULID timestamp extraction
    │   ├── ulid_type.rs                  # ULID type definitions
    │   ├── username_availability.rs      # Username availability checks
    │   └── graph_processors.rs           # Graph data processing
    └── utils/                            # Utilities
        ├── structs.rs                    # Data structure definitions
        ├── logger.rs                     # Logging utilities
        ├── time.rs                       # Time-related utilities
        └── normalize.rs                  # String normalization
```

### Documentation (`/docs`)
Project documentation, architecture guides, and implementation details.

```
docs/
├── README.md                 # Main documentation
├── core/                     # Core documentation
│   ├── architecture/         # System architecture
│   │   ├── database.md       # Database schema
│   │   └── todo/             # Development tasks
│   ├── data-validation.md    # Validation patterns
│   └── resources.md          # External resources
├── implementation/           # Implementation guides
│   ├── reputation.md         # Reputation system
│   └── juno_integration.md   # Juno integration
└── juno/                     # Juno documentation
    └── docs/                 # Official Juno docs
```

## Database (Juno Datastore)

We use Juno's Datastore for data storage, which provides a document-based storage system with the following collections:

### Collections Overview
| Collection | Purpose | Access Level |
|------------|---------|--------------|
| Users | User profiles and authentication | Public read, private write |
| Tags | Reputation tag definitions | Public read, managed write |
| Votes | Vote records | Public read, private write |
| Reputations | Reputation scores | Public read, controller write |


### Users Collection
- Stores user profiles and authentication data
- Key fields: username, display_name, user_ulid
- Permissions: Public read, private write
- Used for: User management, authentication, profile data

### Tags Collection
- Stores reputation tag definitions and rules
- Key fields: tag_handle, description, time_periods, reputation_threshold
- Permissions: Public read, managed write
- Used for: Tag management, vote rules, reputation thresholds

### Votes Collection
- Records all votes cast by users
- Key fields: owner_ulid, tag_ulid, target_ulid, value, weight
- Permissions: Public read, private write
- Used for: Vote tracking, reputation calculations

### Reputations Collection
- Stores calculated reputation scores
- Key fields: owner_ulid, tag_ulid, reputation_total_effective
- Permissions: Public read, controller write
- Used for: Reputation tracking, voting power calculations

### Key Features
- All documents use ULID for unique identification
- Documents are versioned for concurrency control
- Timestamps are stored in nanoseconds
- Keys follow a consistent format for efficient querying
- Collections use appropriate access control levels

For detailed schema information and validation rules, see [Database Schema](docs/core/architecture/database.md).

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

## Documentation

### For Developers
- [API Reference](/docs/resources/ic_and_juno_api_reference.md) - Complete API documentation
- [Juno Integration](/docs/core/development/juno_integration.md) - Integration details
- [Development Guide](/docs/core/development/development.md) - Setup and workflow
- [Testing Guide](/docs/core/development/testing.md) - Testing strategies

### For Users
- [Project Overview](/docs/core/architecture/reputator-dApp-overview.md) - System overview
- [UI Guidelines](/docs/core/development/ui.md) - UI/UX standards
- [Resource Links](/docs/resources/resources.md) - External resources

### For Contributors
- [Project Rules](.cursorrules) - AI assistant rules and standards
- [Database Architecture](/docs/core/architecture/database.md) - Database design
- [Data Validation](/docs/core/architecture/data-validation-reputator.md) - Validation rules
- [Test Calculations](/docs/core/development/test-calculations.md) - Calculation examples

### Additional Resources
- [Juno Index](/docs/resources/juno_index.md) - Quick reference
- [Data Validation (Juno)](/docs/resources/data-validation-juno.md) - Juno patterns
- [Development Resources](/docs/resources/development.md) - Additional resources
- [Playground vs Production](/docs/core/architecture/playground_vs_production.md) - Environment guide

