#!/usr/bin/env bash

# Pre-commit hook script for versionwatch
# This script runs formatting, linting, and security checks

set -e

echo "🔍 Running pre-commit checks..."

# Check code formatting
echo "📝 Checking code formatting..."
if ! cargo fmt --all -- --check; then
    echo "❌ Code formatting check failed. Run 'cargo fmt --all' to fix."
    exit 1
fi

# Run clippy for linting
echo "🔍 Running clippy..."
if ! cargo clippy --all-targets --all-features -- -D warnings; then
    echo "❌ Clippy check failed. Fix the warnings above."
    exit 1
fi

# Run security audit (allow known warnings)
echo "🔒 Running security audit..."
if ! cargo audit --ignore RUSTSEC-2023-0071 --ignore RUSTSEC-2024-0436; then
    echo "❌ Security audit failed with critical vulnerabilities."
    exit 1
else
    echo "✅ Security audit passed (known warnings ignored)."
fi

echo "✅ All pre-commit checks passed!" 