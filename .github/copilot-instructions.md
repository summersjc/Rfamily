# Rfamily - Rust Project

## Project Overview

Command-line Rust application for generating GEDCOM files with millions of people records efficiently. Supports 51 language presets with culturally appropriate names and locations.

## Completed Setup

✅ Project scaffolded with Cargo
✅ Dependencies configured (clap, indicatif, chrono, rand, rand_distr, serde, serde_json)
✅ GEDCOM generator implementation complete with ruleset system
✅ 51 language presets with embedded JSON files
✅ Preset registry system with include_str! for single-binary distribution
✅ New CLI with --preset and --list-presets flags
✅ Backward compatibility with deprecated language flags
✅ Project compiles successfully with zero warnings
✅ Documentation updated (README.md)
✅ Unit tests passing (91 tests: main.rs=14, generator.rs=43, ruleset.rs=13, preset_registry.rs=21)
✅ Integration tests passing (44 tests: integration_tests.rs=16, error_scenario_tests.rs=28)
✅ Total test coverage: 135 tests - all passing ✅
✅ Tested with sample data in multiple languages

## Project Structure

- `src/main.rs` - Main application with CLI and preset integration
- `src/generator.rs` - GEDCOM generation logic with family relationships
- `src/ruleset.rs` - Ruleset configuration structures and legacy preset functions
- `src/preset_registry.rs` - PresetRegistry for loading embedded JSON presets
- `src/lib.rs` - Library interface for testing
- `presets/*.json` - 51 language preset JSON files (embedded at compile time)
- `tests/integration_tests.rs` - 16 integration tests for full CLI workflows
- `tests/error_scenario_tests.rs` - 28 error scenario and edge case tests
- `generate_presets.py` - Python script for generating language preset files
- `Cargo.toml` - Project configuration and dependencies
- `README.md` - Usage documentation
- `example-ruleset.json` - Sample ruleset configuration

## Key Features

- **51 Language Presets**: Albanian, Arabic, Armenian, Bulgarian, Cebuano, Chinese (Traditional), Croatian, Czech, Danish, Dutch, English, Estonian, Farsi, Fijian, Finnish, French, German, Greek, Guarani, Haitian Creole, Hungarian, Icelandic, Italian, Japanese, Khmer, Korean, Latvian, LDS, Lithuanian, Macedonian, Malagasy, Malay, Mongolian, Norwegian, Polish, Portuguese, Romanian, Russian, Samoan, Serbian, Slovak, Slovenian, Spanish, Swahili, Swedish, Tagalog, Thai, Tongan, Turkish, Ukrainian, Vietnamese
- **Preset System**: JSON-based configuration with embedded presets for single-binary distribution
- **Unicode Support**: Full UTF-8 support for non-Latin scripts (Arabic, Chinese, Japanese, Korean, etc.)
- **Family Relationships**: Marriages, divorces, children, multi-generational families
- **LDS Ordinances**: Baptism, confirmation, endowment, sealing support
- **Multiple Cultural Presets**: Culturally appropriate names, locations, and demographics
- **Streaming File Writes**: Memory-efficient for millions of records
- **Progress Tracking**: indicatif progress bars
- **CLI**: clap-based argument parsing with --preset, --list-presets, --generate-ruleset
- **Realistic Demographics**: Statistical distributions for realistic data

## Preset Registry Architecture

- `PresetRegistry` struct loads all JSON files at compile time using `include_str!`
- Single self-contained binary with no external file dependencies
- Presets organized by region: European, Asian, Middle Eastern, Pacific, African, Caribbean/Latin American
- Legacy `default_*()` functions in ruleset.rs kept for backward compatibility in tests

## CLI Commands

```bash
# List all available presets
rfamily --list-presets

# Generate with specific preset
rfamily --preset japanese 100000 output.ged

# Generate with custom ruleset
rfamily --ruleset custom.json 100000 output.ged

# Generate example ruleset
rfamily --generate-ruleset example.json
```
