# Contributing to Rfamily

Thank you for your interest in contributing to Rfamily! This document provides guidelines for contributing to the project.

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue on GitHub with:
- A clear description of the problem
- Steps to reproduce the issue
- Expected vs. actual behavior
- Your operating system and Rust version (`rustc --version`)
- Sample command that demonstrates the issue

### Suggesting Enhancements

Enhancement suggestions are welcome! Please create an issue that includes:
- A clear description of the enhancement
- Why this enhancement would be useful
- Example use cases
- Any relevant implementation details you've considered

### Adding New Language Presets

To add a new language preset:

1. Create a JSON file in the `presets/` directory following the existing format
2. Include culturally appropriate:
   - Given names (male and female)
   - Surnames
   - Location names
   - Demographic distributions (birth years, death rates, marriage rates)
3. Add the preset to `src/preset_registry.rs` in the `PresetRegistry::new()` function
4. Add integration tests in `tests/integration_tests.rs`
5. Update the README.md with the new language

Example preset structure:
```json
{
  "language": "NewLanguage",
  "given_names_male": ["Name1", "Name2"],
  "given_names_female": ["Name1", "Name2"],
  "surnames": ["Surname1", "Surname2"],
  "locations": ["City1", "City2"],
  "birth_year_mean": 1940.0,
  "birth_year_std_dev": 25.0,
  "death_rate": 0.65,
  "average_death_age": 75.0,
  "marriage_rate": 0.70,
  "average_marriage_age": 25.0,
  "average_children": 2.5,
  "include_lds": false
}
```

### Pull Request Process

1. Fork the repository
2. Create a new branch for your feature (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run the test suite: `cargo test`
5. Run code formatting: `cargo fmt`
6. Run clippy: `cargo clippy -- -D warnings`
7. Commit your changes with a clear commit message
8. Push to your fork
9. Open a Pull Request with:
   - Description of changes
   - Why the changes are needed
   - Any relevant issue numbers

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- All code must pass `cargo clippy` without warnings
- Add tests for new functionality
- Maintain existing test coverage
- Write clear comments for complex logic
- Use descriptive variable and function names

### Testing Guidelines

- Write unit tests for new functions in the same file using `#[cfg(test)]`
- Add integration tests to `tests/` directory for CLI workflows
- Ensure all tests pass before submitting PR: `cargo test`
- Test with multiple language presets when applicable
- Include edge cases and error scenarios

### Documentation

- Update README.md if adding user-facing features
- Add doc comments (`///`) for public functions and structs
- Update CHANGELOG.md following [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format
- Include usage examples in documentation

## Development Setup

### Prerequisites

- Rust 1.70 or later (`rustup install stable`)
- Cargo (included with Rust)
- Git

### Building from Source

```bash
# Clone the repository
git clone https://github.com/summersjc/Rfamily.git
cd Rfamily

# Install Git hooks (recommended)
./hooks/install.sh

# Build debug version
cargo build

# Build release version
cargo build --release

# Run tests
cargo test

# Run the tool
cargo run -- --list-presets
cargo run -- --preset english 1000 output.ged
```

### Project Structure

```
Rfamily/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── generator.rs         # GEDCOM generation logic
│   ├── ruleset.rs           # Ruleset configuration
│   ├── preset_registry.rs   # Preset loading system
│   └── lib.rs               # Library interface
├── presets/                 # 51 language preset JSON files
├── tests/
│   ├── integration_tests.rs # CLI integration tests
│   └── error_scenario_tests.rs # Error handling tests
├── Cargo.toml               # Project configuration
└── README.md                # User documentation
```

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on constructive feedback
- Assume good intentions

## Questions?

Feel free to open an issue for questions about contributing or the project in general.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
