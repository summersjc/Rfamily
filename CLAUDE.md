# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rfamily is a high-performance Rust CLI tool that generates and parses GEDCOM (genealogy) files with millions of people records. It supports 51 language presets with culturally appropriate names, locations, and demographics. The tool includes a full GEDCOM 5.5.1 parser and an IOUS (Individual of Unusual Size) generator for creating highly connected family trees. Distributed as a single self-contained binary (~1.5 MB) with all presets embedded at compile time.

## Essential Commands

### Building
```bash
# Debug build
cargo build

# Release build (optimized, required for performance)
cargo build --release

# Binary location: target/release/rfamily
```

### Testing
```bash
# Run all tests (200 total: 112 unit + 52 integration + 3 parser + 22 performance + 11 benches)
cargo test

# Run specific test modules
cargo test --test integration_tests
cargo test --test error_scenario_tests
cargo test --test parser_integration_test

# Run tests in a specific source file
cargo test --lib generator
cargo test --lib gedcom
cargo test --lib generators
```

### Code Quality
```bash
# Format code (required before commit)
cargo fmt

# Check formatting without modifying
cargo fmt --check

# Lint with clippy (must pass with no warnings)
cargo clippy -- -D warnings
```

### Running the Tool
```bash
# During development (use --release for realistic performance)
cargo run --release -- --list-presets
cargo run --release -- --preset japanese -c 100000 -o output.ged

# After building
./target/release/rfamily --preset english -c 50000 -o family.ged
```

### Git Hooks
```bash
# Install pre-commit hooks (runs fmt, clippy, tests)
./hooks/install.sh
```

## Architecture

### Module Structure

**src/main.rs**
- CLI entry point using clap for argument parsing
- Handles preset selection (new `--preset` flag + deprecated legacy flags)
- Coordinates between PresetRegistry, Ruleset, and GedcomGenerator
- Progress bar management using indicatif

**src/generator.rs**
- Core GEDCOM generation engine
- `GedcomGenerator` struct manages individuals and families
- Three generation modes:
  - **Parallel**: Generate independent individuals using rayon (3-4x faster)
  - **Streaming**: Batch-based generation with constant memory (10K batches)
  - **Family mode**: Generate multi-generational families with realistic relationships
- **Parallel generation**: Uses rayon for multi-core CPU utilization, AtomicUsize for thread-safe IDs
- **Streaming mode**: generate_streaming() method - O(BATCH_SIZE) memory instead of O(n)
- Flexible write interface: Accepts any Write trait for compression support
- Handles birth/death dates, marriages, divorces, children, LDS ordinances

**src/compression.rs** (NEW)
- Transparent compression module for gzip support
- `OutputWriter` enum handles both plain and compressed output
- Implements Write trait for seamless integration
- Automatic .gz extension handling
- Proper gzip stream finalization
- 80-85% file size reduction with minimal performance impact

**src/ruleset.rs**
- Configuration structures defining generation rules
- Six main rule categories:
  - `NameRules`: Given names, surnames, naming formats (Western, Eastern, Patronymic, Icelandic)
  - `DateRules`: Birth years, marriage ages, life expectancy
  - `LocationRules`: Countries, cities, languages with probability weights
  - `DemographicRules`: Sex ratios, twin/triplet rates
  - `RelationshipRules`: Marriage/divorce rates, children distribution, generations
  - `OrdinanceRules`: LDS temple ordinances (baptism, endowment, sealing)
- Deserializes from JSON using serde
- Contains legacy `default_*()` functions for backward compatibility in tests

**src/preset_registry.rs**
- `PresetRegistry` struct loads all 51 JSON presets at compile time
- Uses `include_str!` macro to embed JSON files in binary
- Organized by region: European (30), Asian (7), Middle Eastern (3), Pacific (6), African (1), Caribbean/Latin American (3), Special (LDS)
- Returns parsed `Ruleset` objects
- Enables single-binary distribution with no external file dependencies

**src/gedcom/** (NEW)
- **parser.rs**: Full GEDCOM 5.5.1 parser with two-pass parsing algorithm
  - First pass: Parses lines into structured GedcomLine objects
  - Second pass: Builds GedcomFile with individuals and families
  - Supports CONC/CONT continuation lines
  - Handles strict and lenient parsing modes
  - Collects warnings for unknown tags
- **types.rs**: Data structures for parsed GEDCOM data
  - GedcomFile, ParsedIndividual, ParsedFamily
  - Header, GedcomLine, ParseMode, GedcomVersion
- **error.rs**: Comprehensive error handling
  - ParseError with 10 error variants (InvalidLineFormat, InvalidLevel, MissingRequiredTag, InvalidXref, BrokenXref, InvalidDate, InvalidEncoding, IoError, Utf8Error, Other)
  - ParseWarning for non-fatal issues
- **mod.rs**: Module exports and re-exports

**src/generators/** (NEW)
- **ious.rs**: IOUS (Individual of Unusual Size) generator
  - Creates highly connected individuals with multiple marriages
  - Generates siblings for the central IOUS individual
  - Creates multi-generational descendant trees recursively
  - Configurable: marriages, children per marriage, siblings, generations, total descendants
  - Uses Poisson distribution for realistic family sizes
  - Respects target limits to control output size
- **mod.rs**: Module exports

**src/lib.rs**
- Library interface exposing public API for testing
- Re-exports generator, ruleset, preset_registry, gedcom, and generators modules

### Data Flow

1. CLI parses arguments → determines preset name or custom ruleset path
2. PresetRegistry loads embedded JSON → parses into Ruleset struct
3. GedcomGenerator receives Ruleset → generates Individual and Family structs
4. Generator writes GEDCOM records → streams directly to output file
5. Progress bar updates → shows completion percentage and ETA

### Key Design Patterns

**Embedded Resources**: All 51 language presets are embedded at compile time using `include_str!`, enabling single-binary distribution without external dependencies.

**Streaming Output**: GEDCOM records are written directly to a buffered file handle rather than building everything in memory, allowing generation of millions of records efficiently.

**Configurable Generation**: All aspects of generation (names, dates, locations, relationships) are controlled by the Ruleset, enabling easy addition of new language presets without code changes.

**Statistical Realism**: Uses `rand_distr` for normal and Poisson distributions to generate realistic demographic data (ages, children counts, etc.).

**Two-Pass Parsing**: GEDCOM parser uses a two-pass algorithm - first pass parses raw lines, second pass builds semantic structures. This allows for clean separation of syntax and semantics while handling continuation lines (CONC/CONT) efficiently.

**Error Recovery**: Parser supports both strict and lenient modes - strict mode fails fast on errors, lenient mode collects warnings and continues parsing, making it suitable for real-world GEDCOM files with quirks.

**Parallel Generation**: Uses rayon's data parallelism with thread-safe atomic counters (AtomicUsize) for ID generation and Arc-wrapped shared rulesets. Each thread has its own RNG for independent random number generation. Provides 3-4x speedup on multi-core systems with no code complexity for users.

**Streaming Generation**: Batch-based approach (10K batches) that generates, writes, and frees memory in a loop. Maintains O(BATCH_SIZE) memory usage instead of O(n), enabling generation of 10M+ records with constant ~100MB memory footprint. Progress callbacks provide real-time updates during generation.

**Transparent Compression**: OutputWriter enum implements the Write trait for both plain and compressed output. Gzip compression is completely transparent to the generation code - same write calls work for both modes. Achieves 80-85% file size reduction with minimal (<10%) performance overhead.

**Trait-Based Flexibility**: All write methods accept generic `Write` trait instead of concrete types, enabling seamless support for plain files, compressed files, and future extensions (network streams, in-memory buffers, etc.) without changing core generation code.

## Adding New Language Presets

1. Create JSON file in `presets/` directory following existing format
2. Add to `PresetRegistry::new()` in src/preset_registry.rs:
   ```rust
   map.insert("newlang", include_str!("../presets/newlang-preset.json"));
   ```
3. Add integration test in tests/integration_tests.rs
4. Update README.md language list
5. Run full test suite: `cargo test`

## Testing Strategy

**Unit Tests** (112 tests):
- Located in `#[cfg(test)]` modules within each source file
- Test individual functions and edge cases
- **Generator tests** (87): name generation, date calculations, GEDCOM formatting, parallel generation correctness
- **GEDCOM Parser tests** (25): line parsing, family records, CONC/CONT, error handling, strict/lenient modes
- **IOUS Generator tests** (13): sibling generation, multiple marriages, descendant recursion, reference integrity
- **Compression tests** (3): plain writer, compressed writer, filename adjustment

**Integration Tests** (47 tests):
- `tests/integration_tests.rs` (19): CLI workflows with various presets, IOUS command testing
- `tests/error_scenario_tests.rs` (28): Error handling and edge cases
- Test full end-to-end generation with file output validation

**Parser Integration Tests** (3 tests):
- `tests/parser_integration_test.rs`: Round-trip generate→parse→verify, multi-generation GEDCOM parsing

**Performance Tests** (22 tests + 17 benchmarks):
- **Criterion.rs Benchmarks** (`benches/`):
  - `parser_bench.rs`: Parser performance at 1K, 10K, 100K scales
  - `generator_bench.rs`: Generator performance (simple & families) + GEDCOM writing
  - `ious_bench.rs`: IOUS generation (minimal to xlarge), marriage/generation scaling
- **Integration Performance Tests** (`tests/performance_tests.rs`):
  - **Timing tests** (5): 100K/1M parsing, 100K/1M generation, round-trip validation
  - **Memory tests** (4): Memory usage validation at scale (placeholder for future implementation)
  - **Scalability tests** (3): Linear O(n) scaling verification for parser, generator, IOUS
  - **Stress tests** (3): 10M+ record handling, deep recursion validation (marked `#[ignore]`)
- **Test Helpers** (`tests/helpers/perf_test_helpers.rs`):
  - `generate_test_file()`: On-demand GEDCOM test file generation
  - `assert_completes_within()`: Timing assertions with tolerance for CI variability
  - `measure_memory()`: Memory tracking utilities (placeholder)

**Acceptance Criteria**:
- ✅ Parser: 100K in <5s, 1M in <60s, linear O(n) scaling
- ✅ Generator: 100K in <10s, 1M in <120s, linear O(n) scaling
- ✅ IOUS: 1000 descendants in <100ms, 10 generations without stack overflow
- ✅ No panics on 10M+ records (OOM is acceptable)

**CI/CD**: GitHub Actions workflow (`.github/workflows/rust.yml`) runs tests, fmt, and clippy on every push.

## Project-Specific Notes

**Backward Compatibility**: Deprecated CLI flags (`--lds`, `--icelandic`, etc.) are maintained with deprecation warnings to avoid breaking existing scripts.

**GEDCOM Format**: Generates GEDCOM 5.5.1 standard with UTF-8 encoding for full Unicode support (Arabic, Chinese, Japanese, Korean scripts).

**Performance Target**: Should generate 100K records in ~0.16 seconds (parallel mode) or ~0.6 seconds (single-threaded) in release mode on modern hardware.

**Binary Size**: Final binary is ~1.5 MB including all 51 presets (204 KB preset data).

**No Runtime Dependencies**: The compiled binary has no external dependencies and can be distributed standalone.

## CI/CD and Release

**GitHub Actions Workflows**:
- `rust.yml`: Runs on every push - builds, tests, fmt, clippy
- `release.yml`: Creates GitHub releases with binaries for macOS (ARM64/Intel), Linux, Windows

**Pre-commit Checks**: Git hooks in `hooks/` directory enforce fmt, clippy, and tests before commit.

## Common Development Patterns

**When modifying generation logic**: Update generator.rs and add unit tests in the same file.

**When adding configuration options**: Extend ruleset.rs structures and update example-ruleset.json.

**When testing presets**: Use `cargo run --release` (debug mode is 10-50x slower).

**When debugging GEDCOM output**: Generate small files (e.g., `-c 100`) and inspect manually.
