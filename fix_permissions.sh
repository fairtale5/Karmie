#!/bin/bash
# Script to fix permissions in the project directory

echo "Fixing ownership and permissions..."
sudo chown -R $(whoami):$(whoami) .
chmod -R 755 .
chmod -R 777 build .svelte-kit node_modules target

echo "Done!"
