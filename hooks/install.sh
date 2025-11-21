#!/bin/sh
#
# Setup script to install Git hooks

HOOK_DIR=".git/hooks"
HOOKS_SOURCE="hooks"

echo "Installing Git hooks..."

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "Error: Not a git repository"
    exit 1
fi

# Install pre-commit hook
if [ -f "$HOOKS_SOURCE/pre-commit" ]; then
    cp "$HOOKS_SOURCE/pre-commit" "$HOOK_DIR/pre-commit"
    chmod +x "$HOOK_DIR/pre-commit"
    echo "✅ Installed pre-commit hook"
else
    echo "❌ pre-commit hook not found in $HOOKS_SOURCE/"
    exit 1
fi

echo "✅ Git hooks installed successfully!"
echo ""
echo "The pre-commit hook will run:"
echo "  - cargo fmt --check (formatting)"
echo "  - cargo clippy (linting)"
echo "  - cargo test (tests)"
echo ""
echo "To bypass the hook, use: git commit --no-verify"
