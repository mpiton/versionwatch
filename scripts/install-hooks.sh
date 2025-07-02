#!/usr/bin/env bash

# Script to install git hooks for versionwatch

set -e

echo "🔧 Installing git hooks..."

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "❌ Error: Not in a git repository root"
    exit 1
fi

# Create hooks directory if it doesn't exist
mkdir -p .git/hooks

# Copy pre-commit hook
cp scripts/pre-commit.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit

echo "✅ Git hooks installed successfully!"
echo "💡 The pre-commit hook will now run automatically before each commit."
echo "💡 To skip the hook temporarily, use: git commit --no-verify" 