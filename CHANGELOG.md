# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-01-10

### Added

- **Parallel Generation**: Multi-core CPU utilization using rayon for 3-4x speedup (100K records in ~0.16s vs ~0.6s single-threaded)
- **Streaming Generation**: Memory-efficient batch-based generation (10K batches) with constant ~100MB memory footprint, enabling 10M+ record generation
- **Gzip Compression**: Transparent compression support achieving 80-85% file size reduction with minimal (<10%) performance overhead
- **Real-time Progress**: Enhanced progress reporting with records/sec throughput, ETA, and batch-level updates
- **Feature Flags**: Optional `parallel` and `compression` features (enabled by default) allowing minimal builds
- **Programmatic Examples**: Three new examples demonstrating streaming generation, compression, and combined features
- **Performance Benchmarks**: Criterion benchmarks for parallel generation, streaming, compression overhead, and scalability comparisons
- **Comprehensive Documentation**: Extensive API documentation with examples for all new public methods
- **200 Tests**: Expanded test suite covering streaming generation, compression/decompression, automatic thresholds, and feature combinations

### Performance

- Parallel generation: 3-4x faster on multi-core systems
- Streaming mode: 10x lower memory usage (O(BATCH_SIZE) vs O(n))
- Compression: 80-85% file size reduction
- Automatic streaming: Enabled automatically for 500K+ records
- Linear O(n) scaling verified for all generation modes

### Changed

- Updated CLI with `--compress` and `--streaming` flags in `generate` command
- Enhanced `generate_streaming()` method with real-time progress callbacks
- Improved documentation with performance-focused examples
- Updated benchmarks to measure new performance features

### Dependencies

- Added `rayon` 1.10 (optional, enabled by default) for parallel generation
- Added `flate2` 1.0 (optional, enabled by default) for gzip compression
- Removed unused `crossbeam` and `parking_lot` dependencies

### Documentation

- Updated README.md with detailed performance feature documentation
- Updated CLAUDE.md architecture documentation with new design patterns
- Added module-level documentation for compression support
- Added comprehensive doc comments to all new public APIs

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
