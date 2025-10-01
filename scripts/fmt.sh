#!/bin/bash
# Format Rust code using rustfmt

set -e

echo "🔧 Formatting Rust code..."
cargo fmt --all

echo "✅ Code formatting complete!"