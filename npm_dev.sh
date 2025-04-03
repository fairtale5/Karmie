#!/bin/bash
# Script to build and run the frontend development server

echo "Building and starting frontend development server..."

# Fix permissions for build directories
echo "Fixing permissions..."
./fix_permissions.sh

# Build the application
echo "Building the application..."
npm run build

# Start the development server
echo "Starting development server..."
npm run dev

echo "Frontend development server is running!"
echo "Make sure Juno dev environment is also running with juno_dev.sh" 