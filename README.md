# Reputator

A decentralized reputation system built on the Internet Computer using Juno and SvelteKit.

**Intro/Overview**

I wanted a reputation system that is:
- Truly decentralized
- Bot-resistant
- Doesn’t require KYC

### How It Works
Reputation is earned by being voted on by users who already have a reputation. The higher the reputation of the user voting on you, the greater the effect—both positive and negative.

This means that bots, bad actors, and newcomers all start with a reputation of **0**. They can vote on others, but their votes won’t have an effect until they reach the **minimum threshold** (which is customizable).

Additionally, if the community downvotes a user, they lose their privileges, and all users they voted on also lose the reputation they gained from them. This makes it easy to identify and neutralize bad actors by downvoting them, undoing any damage they caused.

### Bootstrapping New Communities
The challenge was how to bootstrap new communities, as early on, no one has enough reputation to vote on others. To address this, I implemented a **“reward for voting”** system, which works in two phases:
- **Early on**: Everyone receives rewards while the community is still small. At this stage, anyone can join and earn voting rewards.
- **Later on**: Only trusted users who have already gained reputation receive rewards. This prevents bad actors from farming reputation through votes.

### Technical Details
- Runs in its own **canister**
- Backend powered by **Juno (juno.build)**
- Custom functions written in **Rust**
- Any ICP app can access its API remotely and integrate it into their own apps

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
One of the biggest challenges is managing cascading updates. I’ve implemented caching mechanisms to optimize this. Reputation updates occur only when:
- A user is queried
- A user casts a vote
- A user receives a vote

This system is working well so far.

### Integration with Apps
You can define actions within your app that grant reputation to users. This means your app’s **canister user** will be voting on community members based on specific actions.

- The community can also vote back.
- You can automate parts of this process, e.g., after every transaction, both the user and the app receive votes.
- Admin users need to maintain enough reputation, so this helps keep them active.

You simply add hooks to the desired actions. For example, a social media app could embed votes into “like” buttons.

I’m also developing a **web interface** for users to interact with the system, but the primary goal is for apps to create their own integrations.

### Next Features
- **Improve caching** further so older votes aren’t always recalculated and can be fetched less frequently.
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

