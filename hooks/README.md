# Git Hooks

This directory contains Git hooks that help maintain code quality.

## Available Hooks

### pre-commit
Runs before each commit to check:
- Code formatting (`cargo fmt --check`)
- Linting (`cargo clippy -- -D warnings`)
- Tests (`cargo test`)

## Installation

Run the installation script from the project root:

```bash
./hooks/install.sh
```

Or manually copy the hook:

```bash
cp hooks/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

## Bypassing Hooks

If you need to commit without running the hooks (not recommended):

```bash
git commit --no-verify
```

## Uninstalling

To remove the hooks:

```bash
rm .git/hooks/pre-commit
```
