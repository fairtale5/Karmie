# Development Guide

## Getting Started

### Essential Commands

#### Deploying
```bash
# For frontend changes:
npm run build # Builds the project frontend for production
juno deploy # Deploys your frontend application (static assets, HTML, CSS, JS files) to your satellite

# For backend/Rust changes:
juno dev build  # Compiles your Rust code to WASM
juno upgrade -t s -s ./target/deploy/satellite.wasm.gz  # Upgrades+deploys the satellite
```

#### Local Development
```bash
# Start local development server with hot reload
npm run dev

# Build frontend for production
npm run build

# Build and start Juno local development emulator (emulator is local blockchain + local II + local satellite)
juno dev build
juno dev start
```



### Source Code
The project is available at: https://github.com/fairtale5/Reputator

### Prerequisites
- Node.js and npm
- Rust and Cargo
- Juno CLI (`npm install -g @junobuild/cli`)
- Docker (for local development)
- Git

### Fresh Setup Guide

#### PART 1: Complete Cleanup
1. **Remove Node.js related files and folders**:
```bash
rm -rf node_modules
rm -rf .svelte-kit
rm -rf build
rm package-lock.json
```

2. **Remove Rust/Cargo build artifacts**:
```bash
rm -rf target
rm -rf src/satellite/target
```

3. **Remove Docker containers, images, and volumes**:
```bash
docker-compose down --rmi all --volumes
docker system prune -f --volumes  # This removes all unused containers, networks, images, AND volumes
```

4. **Remove any Juno local development artifacts**:
```bash
rm -rf .juno
```

#### PART 2: Fresh Rebuild
1. **Install Node.js dependencies**:
```bash
npm ci  # Uses package-lock.json for exact dependency versions
```

2. **Build the SvelteKit application**:
```bash
npm run build
```

3. **Build the satellite WASM module**:
```bash
cd src/satellite
rustup target add wasm32-unknown-unknown  # One-time setup for WebAssembly support
cargo build --target wasm32-unknown-unknown
cd ../..
```
Now to check if wasm32-unknown-unknown is installed:
```bash
rustup target list | grep wasm32-unknown-unknown
```

4. **Start Juno development environment**:
```bash
juno dev build
juno dev start
```

### Project Structure
```
├── src/                    # Frontend source code
│   ├── routes/            # SvelteKit routes
│   ├── lib/               # Shared components & utilities
│   └── satellite/         # Backend Rust code
│       ├── src/
│       │   └── lib.rs     # Main Rust implementation
│       └── Cargo.toml     # Rust dependencies
├── static/                # Static assets
├── docs/                  # Project documentation
│   ├── resources/         # Reference documentation
│   ├── core/             # Core project docs
│   └── implementation/   # Implementation guides
├── build/                # Production build output
├── juno.config.ts        # Main Juno configuration
├── juno.dev.config.ts    # Development-specific config
├── package.json          # Node.js dependencies
├── package-lock.json     # Locked Node.js dependencies
└── Cargo.toml           # Root Rust configuration
```

### Important Files
- `src/satellite/src/lib.rs`: Main backend implementation
- `src/satellite/Cargo.toml`: Backend dependencies and configuration
- `juno.config.ts`: Main Juno configuration
- `juno.dev.config.ts`: Development-specific Juno settings
- `package.json`: Frontend dependencies and scripts
- `docs/README.md`: Main project documentation

## Quick Start Commands

### Local Development (Testing)
```bash
# 1. Start local blockchain emulator
juno dev start

# 2. Start development server
npm run dev

# Access: http://localhost:5173
# Login: Use test anchor 10000-99999
```

### Production Deployment (On-Chain)
```bash
# 1. Stop local environment
juno dev stop

# 2. Build and deploy
npm run build
juno deploy

# Note: Make sure to update satellite ID in juno.config.ts first!
```

## Detailed Setup Guide

### Configuration Switch

The `juno.config.ts` file needs to be updated when switching between local and on-chain:

```typescript
import { defineConfig } from '@junobuild/config';

export default defineConfig({
  satellite: {
    // For local development (using emulator)
    id: 'jx5yt-yyaaa-aaaal-abzbq-cai',

    // For on-chain deployment (uncomment when deploying)
    // id: 'rigfr-siaaa-aaaal-ab4fa-cai',
    
    source: 'build'
  }
}); 
```

### Environment Modes

This project can run in two modes:
1. **Local Development** (using Juno emulator)
   - Uses Docker to simulate blockchain locally
   - Provides test authentication
   - Great for rapid development
   
2. **On-Chain Deployment** (on the Internet Computer)
   - Real blockchain deployment
   - Real Internet Identity authentication
   - Production environment

## Environment Setup

### Running Locally

1. Start the Juno emulator:
   ```bash
   juno dev start
   ```
   This will:
   - Start local Internet Identity service
   - Deploy local test satellite
   - Set up local blockchain environment

2. Make sure `juno.config.ts` uses the local ID:
   ```typescript
   id: 'jx5yt-yyaaa-aaaal-abzbq-cai'
   ```

3. Start the development server:
   ```bash
   npm run dev
   ```

4. Access the app:
   - Frontend: http://localhost:5173
   - Use test anchor 10000-99999 for local authentication

### Deploying On-Chain

1. Stop any running local emulator:
   ```bash
   juno dev stop
   ```

2. Update `juno.config.ts` to use your production satellite ID:
   ```typescript
   id: 'rigfr-siaaa-aaaal-ab4fa-cai'
   ```

3. Build and deploy:
   ```bash
   npm run build
   juno deploy
   ```

## Quick Commands Reference

### Core Development Commands
```bash
# Start local development server with hot reload
npm run dev

# Build the project for production
npm run build

# Preview production build locally
npm run preview

# Deploy to Juno satellite
juno deploy
```

### Local Development with Docker
```bash
# Start the local development emulator
juno dev start

# Stop the emulator
juno dev stop

# Clear emulator data
juno dev clear
```

### Juno CLI Commands
```bash
# Initialize Juno in an existing project
juno init

# Login to Juno
juno login

# Check Juno version
juno --version

# Configure satellite
juno config

# Take a snapshot of your data
juno snapshot

# Clear deployed files
juno clear

# Upgrade satellite
juno upgrade
```

## Command Explanations

### Development Commands
- `npm run dev`: Starts development server at localhost:5173 with hot module reloading (and connected to the local emulator)
- `npm run build`: Creates production build in ./build directory
- `npm run preview`: Serves production build locally for testing (and connected to live satellite)

### Deployment Commands
- `juno deploy`: Deploys your frontend application (static assets, HTML, CSS, JS files) to your satellite
  - Use after `npm run build`
  - Only uploads new or changed files
  - For regular application updates and deployments

- `juno upgrade -t s -s ./target/deploy/satellite.wasm.gz`: Upgrades the satellite's WebAssembly module
  - Use after modifying Rust code (serverless functions, hooks, etc.)
  - Creates automatic backup before upgrading
  - The `-t s` specifies upgrading a satellite
  - The `-s` flag specifies the path to your compiled WASM file
  - Requires stable internet connection
  - Always upgrade sequentially through versions



### Docker Commands
- `juno dev start`: Launches local development environment with Docker
  - Simulates IC environment locally
  - Provides local testing of authentication
  - Enables testing collections without deploying
- `juno dev stop`: Stops the local development environment
- `juno dev clear`: Resets local development data (useful when testing)

### Configuration Commands
- `juno init`: Creates juno.config.ts file in your project
- `juno login`: Authenticates your terminal for deployments
- `juno config`: Updates satellite configuration
- `juno upgrade`: Upgrades your satellite to latest version

## Development Tips

### Rust and WebAssembly for Custom Backend Features
When developing custom backend features for your Juno application (such as validation hooks, data transformation functions, or custom API endpoints), you'll be writing Rust code that needs to be compiled to WebAssembly. This is because Juno satellites run on the Internet Computer blockchain, which executes WebAssembly modules.

1. **One-Time Rust Setup**
   ```bash
   # Run this once on your development machine (from any directory)
   # This adds WebAssembly compilation support to your Rust installation
   rustup target add wasm32-unknown-unknown
   ```

2. **When to Build Rust Code**
   You need to build your Rust code whenever you:
   - Create or modify hooks (like `on_set_doc`, `assert_set_doc`)
   - Add custom validation logic
   - Create new API endpoints
   - Change any Rust code in the `src/satellite` directory

3. **Building Your Rust Code**
   ```bash
   # Recommended Method:
   # Run from your project root directory
   juno dev build
   ```
   The `juno dev build` command handles everything for you:
   - Compiles your Rust code to WebAssembly
   - Ensures proper target configuration
   - Automatically deploys to your local emulator
   - Provides helpful error messages
   
   When building custom Rust code for the first time, you'll be prompted to install the candid-extractor:
   ```bash
   The candid-extractor tool is required to generate the API ("did file"). Would you like to install it? › (yes/no)
   ```
   You should select 'yes'. This tool is needed to:
   - Generate interface definitions for your custom Rust code
   - Allow the Internet Computer to understand your custom functions
   - Enable frontend-backend communication with your custom code
   - Only needs to be installed once per machine

   ```bash
   # Alternative Method (only if needed):
   # Manual compilation from the src/satellite directory
   cd src/satellite
   cargo build --target wasm32-unknown-unknown
   ```
   The manual cargo command is shown for understanding but isn't necessary for normal development.

4. **Development Workflow**
   a. Write/modify Rust code in `src/satellite/src/lib.rs`
   b. Run `juno dev build` from your project root
   c. If changes aren't reflected, restart the emulator:
      ```bash
      juno dev stop
      juno dev start
      ```

5. **Common Issues and Solutions**
   - **Proc-macro errors**: Check that required features are enabled in `src/satellite/Cargo.toml`
   - **Build failures**: Make sure you're building with the WebAssembly target
   - **Changes not reflecting**: Remember to rebuild and restart the emulator
   - **Missing dependencies**: Ensure all required crates are listed in `Cargo.toml`

The WebAssembly compilation is specifically needed for the backend Rust code that runs on the Internet Computer. Your frontend code (JavaScript/TypeScript/Svelte) doesn't require this compilation step.

### Local Development
1. Always run `juno dev start` before `npm run dev`
2. Use the emulator for testing authentication and collections
3. Changes to Rust code require restarting the emulator

### Deployment
1. Always build (`npm run build`) before deploying
2. Test the build locally with `npm run preview`
3. Make sure you're logged in (`juno login`) before deploying
4. Verify your satellite ID in `juno.config.ts`

### Troubleshooting
1. If emulator fails:
   ```bash
   juno dev stop
   juno dev clear
   juno dev start
   ```

2. If deployment fails:
   - Check your internet connection
   - Verify you're logged in
   - Ensure build is successful
   - Check satellite ID is correct

### Environment Variables
- Development: `.env`
- Production: Set in Juno console
- Local emulator: Uses development variables

### Important Paths
```
├── src/
│   ├── routes/         # SvelteKit routes
│   ├── lib/           # Shared components & utilities
│   └── backend/       # Rust endpoints
├── static/           # Static assets
├── build/           # Production build
└── juno.config.ts   # Juno configuration
```

## Useful Links
- [Juno Console](https://console.juno.build)
- [Internet Identity](https://identity.ic0.app)
- [IC Dashboard](https://dashboard.internetcomputer.org)

## Common Operations

### Working with Collections
```typescript
// Initialize
const collection = new Collection({ collection: "name" });

// Create
await collection.insert({ data });

// Read
await collection.get({ key });

// List
await collection.list({
  filter: {}, 
  order: { desc: true }
});

// Delete
await collection.remove({ key });
```

### Authentication Flow
```typescript
// Subscribe to auth state
authSubscribe((user) => {
  if (user === null) {
    // Signed out
  } else {
    // Signed in
  }
});

// Sign in/out
await signIn();
await signOut();
```

### Build & Deploy Cycle
1. Stop development server
2. Build project: `npm run build`
3. Test build: `npm run preview`
4. Deploy: `juno deploy`
5. Verify in console

## Maintenance

### Regular Tasks
- Take snapshots before major changes
- Clear emulator data periodically
- Update dependencies regularly
- Check for Juno updates

### Before Production
- Test all authentication flows
- Verify collection permissions
- Check build size
- Test on multiple devices 

## Git Commands Reference

### Basic Git Workflow
```bash
# Check status of your changes
git status

# Stage all changes
git add .

# Stage specific files
git add file1.ts file2.ts

# Commit changes with a simple message
git commit -m "Your commit message"

# Commit with a detailed message (opens text editor)
git commit

# Example of a detailed commit message format:
Title: Brief summary of changes

- Bullet point of major change 1
- Bullet point of major change 2

Technical details:
- Specific implementation detail 1
- Specific implementation detail 2

# Push changes to remote
git push origin branch-name
```

### Git Configuration
```bash
# Set your Git identity
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"

# Set identity for single repository (remove --global)
git config user.name "Your Name"
git config user.email "your.email@example.com"
```

## Helper Scripts

The project includes several shell scripts to streamline common development tasks and fix permission issues that can occur when working with WSL.

### Available Scripts

```bash
# Fix permissions in the project directory
./fix_permissions.sh

# Complete cleanup of the project (remove build artifacts)
./force_clean.sh

# Build and start the Juno development environment
./juno_dev.sh

# Build and start the frontend development server
./npm_dev.sh

# Set up the environment for the first time
./setup_environment.sh
```

### Script Details

1. **fix_permissions.sh**
   - Fixes ownership and permissions for the entire project
   - Sets basic permissions (755) for most files
   - Sets full permissions (777) for build directories (build, .svelte-kit, node_modules, target)
   - Run this whenever you encounter permission issues

2. **force_clean.sh**
   - Performs a complete cleanup of the project
   - Removes Node.js artifacts (node_modules, .svelte-kit, build)
   - Cleans Rust/Cargo build artifacts
   - Removes Docker containers and volumes
   - Removes Juno local development artifacts
   - Fixes permissions

3. **juno_dev.sh**
   - Builds and starts the Juno development environment
   - Fixes permissions first
   - Runs `juno dev build` to compile Rust code to WebAssembly
   - Runs `juno dev start` to start the local environment

4. **npm_dev.sh**
   - Builds and runs the frontend development server
   - Fixes permissions first
   - Builds the application with `npm run build`
   - Starts the development server with `npm run dev`

5. **setup_environment.sh**
   - Sets up the development environment from scratch
   - Fixes permissions
   - Installs Node.js dependencies with `npm ci`
   - Sets up Rust WebAssembly target (one-time setup)
   - Builds the Juno satellite
   - Builds the SvelteKit application

### Common Workflows

```bash
# First-time setup
./setup_environment.sh

# Daily development
./juno_dev.sh
# (in another terminal)
./npm_dev.sh

# When things get messy
./force_clean.sh
./setup_environment.sh

# When permissions issues occur
./fix_permissions.sh
```

The scripts automatically handle common issues with permissions in WSL environments and provide a consistent workflow for all developers. 

## Candid URL Format

To access the Candid interface for your project, use the following URL format:

```
https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.ic0.app/?id=[the project's ID]
```

Replace `[the project's ID]` with your actual project ID to access the Candid interface. 