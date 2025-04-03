#!/bin/bash
# Script to set up the environment for the first time

echo "Setting up environment for Reputator project..."

# Fix permissions first
echo "Fixing permissions..."
./fix_permissions.sh

# Install Node.js dependencies
echo "Installing Node.js dependencies..."
npm ci

# Add WebAssembly target for Rust (this is a one-time setup per machine, not per project)
# This line is only needed the first time you set up any Juno project on this machine
echo "Setting up Rust WebAssembly target (one-time setup)..."
rustup target add wasm32-unknown-unknown

# Build the satellite using Juno 
# This handles all the Rust compilation and WebAssembly generation
echo "Building satellite with Juno..."
juno dev build

# If the satellite build failed, exit
if [ $? -ne 0 ]; then
  echo "Satellite build failed. Please fix the errors before continuing."
  exit 1
fi

# Build the SvelteKit application
echo "Building SvelteKit application..."
npm run build

# Check if build was successful
if [ $? -eq 0 ]; then
  echo "Environment setup complete!"
  echo ""
  echo "To start development, run:"
  echo "1. ./juno_dev.sh - to start the local blockchain emulator"
  echo "2. ./npm_dev.sh - to start the frontend development server"
else
  echo "SvelteKit build failed. Please fix the errors and try again."
  exit 1
fi