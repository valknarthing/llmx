#!/bin/bash
# rename-crates.sh - Systematically rename all Rust crates from codex to llmx

set -e

echo "=== Phase 2: Rust Workspace Transformation ==="
echo ""

cd "$(dirname "$0")/../llmx-rs"

echo "Step 1: Updating Cargo.toml files..."

# Update workspace dependencies
sed -i 's/codex-\([a-z-]*\) = { path/llmx-\1 = { path/g' Cargo.toml

# Update all individual Cargo.toml files
find . -name "Cargo.toml" -type f | while read -r file; do
    echo "  Processing: $file"

    # Update package name
    sed -i 's/name = "codex-/name = "llmx-/g' "$file"

    # Update dependency declarations
    sed -i 's/codex-\([a-z-]*\) = /llmx-\1 = /g' "$file"
    sed -i 's/codex-\([a-z-]*\)"/llmx-\1"/g' "$file"

    # Update path references
    sed -i 's/"codex-/"llmx-/g' "$file"
done

echo ""
echo "Step 2: Updating Rust source files (use statements)..."

# Update all use statements
find . -name "*.rs" -type f | while read -r file; do
    # Update use statements
    sed -i 's/use codex_/use llmx_/g' "$file"

    # Update extern crate
    sed -i 's/extern crate codex_/extern crate llmx_/g' "$file"

    # Update crate:: references
    sed -i 's/codex_\([a-z_]*\)::/llmx_\1::/g' "$file"
done

echo ""
echo "Step 3: Updating binary names in CLI..."

# Update binary name in cli/Cargo.toml
sed -i 's/name = "codex"/name = "llmx"/g' cli/Cargo.toml

echo ""
echo "=== Phase 2 Complete! ==="
echo "Summary:"
echo "  - Updated all Cargo.toml package names"
echo "  - Updated all Rust import statements"
echo "  - Updated binary name to 'llmx'"
