#!/bin/bash
# Generate rio cogeo info markdown files for all TIFFs in the repo

REPO_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_DIR"

for tif in $(find . -name "*.tif" -type f); do
	md="${tif%.tif}_info.md"
	echo '```' >"$md"
	# Make File: path relative and strip trailing whitespace
	rio cogeo info "$tif" | sed "s|^File: $REPO_DIR/|File: |" | sed 's/[[:space:]]*$//' >>"$md"
	echo '```' >>"$md"
	echo "✓ Generated: $(basename "$md")"
done
