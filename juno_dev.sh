#!/bin/bash
# Script to build and start the Juno development environment

echo "Building and starting Juno development environment..."

# Fix permissions for build directories
echo "Fixing permissions..."
./fix_permissions.sh

# Build Juno for development
echo "Building Juno for development..."
juno dev build

# If the build was successful, start the development environment
if [ $? -eq 0 ]; then
  # Start Juno development environment - exec replaces the current process
  echo "Starting Juno development environment..."
  exec juno dev start
else
  echo "Juno build failed. Please fix the errors before continuing."
  exit 1
fi

echo "Juno development environment is running!"
echo "Run 'npm run dev' in another terminal to start the frontend."
echo "Access the app at: http://localhost:5173"
echo "Use test anchor 10000-99999 for local authentication." 