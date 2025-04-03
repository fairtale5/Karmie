#!/bin/bash
# Script to force clean the project and remove all build artifacts

echo "Force cleaning Reputator project..."

# Remove Node.js related files and folders
echo "Removing Node.js artifacts..."
rm -rf node_modules
rm -rf .svelte-kit
rm -rf build
# Comment out the package-lock.json removal to prevent reinstallation issues
# rm -f package-lock.json

# Remove Rust/Cargo build artifacts
echo "Removing Rust/Cargo artifacts..."
cargo clean
rm -rf src/satellite/target

# Remove Docker containers, images, and volumes
echo "Removing Docker artifacts..."
docker-compose down --rmi all --volumes || true
docker system prune -f --volumes || true

# Remove any Juno local development artifacts
echo "Removing Juno artifacts..."
rm -rf .juno || true

# Fix permissions
echo "Fixing permissions..."
./fix_permissions.sh

echo "Force clean complete! Run setup_environment.sh to rebuild." 