# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-01-28

### Added

- Initial release of Rfamily GEDCOM generator
- Support for 51 language presets with culturally appropriate names and locations
- JSON-based preset system with compile-time embedding for single-binary distribution
- CLI interface with `--preset`, `--list-presets`, `--generate-ruleset`, and `--version` flags
- Realistic demographic distributions for birth years, death years, and marriage patterns
- LDS ordinances support (baptism, confirmation, endowment, sealing)
- Full UTF-8 support for non-Latin scripts (Arabic, Chinese, Japanese, Korean, etc.)
- Memory-efficient streaming file writes for millions of records
- Progress tracking with indicatif progress bars
- Comprehensive test suite (135 tests: 91 unit + 44 integration)
- MIT License

### Language Presets

European: Albanian, Bulgarian, Croatian, Czech, Danish, Dutch, English, Estonian, Finnish, French, German, Greek, Hungarian, Icelandic, Italian, Latvian, Lithuanian, Macedonian, Norwegian, Polish, Portuguese, Romanian, Russian, Serbian, Slovak, Slovenian, Spanish, Swedish, Turkish, Ukrainian

Asian: Chinese (Traditional), Japanese, Khmer, Korean, Mongolian, Thai, Vietnamese

Middle Eastern: Arabic, Armenian, Farsi

Pacific: Cebuano, Fijian, Malagasy, Malay, Samoan, Tagalog, Tongan

African: Swahili

Caribbean/Latin American: Guarani, Haitian Creole, Portuguese (Brazil)

Special: LDS (Latter-day Saints)
