#!/bin/bash
# CI script to verify generated artifacts are up to date

set -e

echo "Running generate..."
pixi run generate

echo "Running info..."
pixi run info

echo "Checking for uncommitted changes..."
if [ -n "$(git status --porcelain)" ]; then
    echo "ERROR: Generated files are out of date. Please run 'pixi run generate' and 'pixi run info' and commit the changes."
    git status
    git diff
    exit 1
fi

echo "All generated files are up to date."
