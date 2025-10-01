#!/bin/bash
# Run all quality checks: format, lint, test

set -e

echo "🚀 Running all quality checks..."

echo "📝 1. Formatting check..."
cargo fmt --all -- --check

echo "🔍 2. Linting..."
cargo clippy --all-targets --all-features -- -D warnings

echo "🔧 3. Compilation check..."
cargo check --all-targets --all-features

echo "🧪 4. Running tests..."
cargo test --all-features

echo "✅ All checks passed!"