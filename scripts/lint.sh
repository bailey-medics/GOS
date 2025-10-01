#!/bin/bash
# Run Clippy linter on Rust code

set -e

echo "🔍 Running Clippy linter..."
cargo clippy --all-targets --all-features -- -D warnings

echo "✅ Linting complete!"