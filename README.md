# Rfamily

[![Release](https://img.shields.io/github/v/release/summersjc/Rfamily?color=blue)](https://github.com/summersjc/Rfamily/releases/latest)
[![Rust CI](https://github.com/summersjc/Rfamily/actions/workflows/rust.yml/badge.svg)](https://github.com/summersjc/Rfamily/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![Test Coverage](https://img.shields.io/badge/coverage-85%25-brightgreen.svg)](https://github.com/summersjc/Rfamily)
[![Tests](https://img.shields.io/badge/tests-200_passing-brightgreen.svg)](https://github.com/summersjc/Rfamily)

A high-performance Rust tool for generating GEDCOM files with millions of people records using customizable rulesets.

**Available as both a CLI tool and web application with REST API!** 🚀

## Installation

See [INSTALL.md](INSTALL.md) for detailed installation instructions for all platforms.

**Quick install:** Download the binary for your platform from the [latest release](https://github.com/summersjc/Rfamily/releases/latest).

## Features

### CLI Tool
- **52 Language Presets**: Built-in support for European (including English USA & UK), Asian, Middle Eastern, Pacific, African, and Latin American languages with culturally appropriate names and locations
- **Ruleset-Based Generation**: Define custom rules for names, dates, locations, relationships, and LDS ordinances
- **Blazing Fast Generation**: Multi-core parallel generation (3-4x faster) - 100K records in ~0.16s
- **Memory Efficient**: Streaming mode for 10M+ records with constant memory usage
- **Compression Support**: Gzip compression for 80-85% file size reduction
- **Family Relationships**: Generate realistic multi-generational families with marriages, divorces, and children
- **GEDCOM Parser**: Parse and validate existing GEDCOM 5.5.1 files with strict/lenient modes
- **IOUS Generator**: Create "Individuals of Unusual Size" - highly connected people with multiple marriages and extensive descendants
- **Unicode Support**: Full UTF-8 support for non-Latin scripts (Arabic, Chinese, Japanese, Korean, etc.)
- **LDS Ordinances**: Optional support for baptism, endowment, sealing, and other LDS temple ordinances
- **Real-time Progress**: Progress bar with throughput rate (records/sec) and ETA
- **Highly Configurable**: Customize every aspect through JSON ruleset files
- **Single Binary**: All 52 presets embedded - no external files needed

### Web Application (NEW!)
- **REST API**: Full-featured REST API with 6 endpoints for preset management and GEDCOM generation
- **Swagger Documentation**: Interactive API documentation at `/api/docs`
- **Web Interface**: User-friendly web UI for generating GEDCOM files
- **Preview Mode**: Generate small samples (10-100 records) for testing
- **Batch Generation**: Create files with up to 10M individuals
- **Real-time Statistics**: View generation metrics (individuals, families, time)

See [README_WEB.md](README_WEB.md) for web application documentation.

## Quick Start

See [INSTALL.md](INSTALL.md) for detailed installation instructions.

**Download:** Get the binary for your platform from the [latest release](https://github.com/summersjc/Rfamily/releases/latest).

## Usage

```bash
git clone https://github.com/yourusername/Rfamily.git
cd Rfamily
./install.sh
```

The install script will:

1. Build the optimized release binary
2. Let you choose installation location (system-wide or user)
3. Optionally copy to your PATH for easy access

### Option 1: Build from Source

Clone this repository and build:

```bash
git clone https://github.com/yourusername/Rfamily.git
cd Rfamily
cargo build --release
```

The compiled binary will be in `target/release/rfamily`

### Option 2: Use Pre-compiled Binary

After building, the standalone binary can be copied anywhere and run independently:

```bash
# Copy binary to a directory in your PATH
cp target/release/rfamily /usr/local/bin/

# Or run directly from build directory
./target/release/rfamily --help
```

The binary is completely self-contained with all 52 language presets embedded at compile time.

**Binary Size:** ~1.5 MB (includes all presets and dependencies)

## Usage Examples

### Web Application

Start the web server:

```bash
# Using cargo
cargo run -p rfamily-web

# Or run the binary directly
./target/release/rfamily-web
```

Then visit:
- **Web Interface**: http://localhost:3000
- **API Documentation**: http://localhost:3000/api/docs

See [README_WEB.md](README_WEB.md) and [API_DOCUMENTATION.md](API_DOCUMENTATION.md) for complete web application documentation.

### CLI Usage

#### Using the binary

```bash
# List all available language presets
rfamily --list-presets

# Generate with a specific language preset
rfamily --preset japanese --count 100000 --output japan.ged

# Generate with custom count and output file
rfamily -p german -c 50000 -o germany.ged
```

### Using cargo run (for development)

```bash
# List all available language presets
cargo run --release -- --list-presets
```

### Generate with a specific language preset

```bash
# Generate with Japanese names and locations
cargo run --release -- --preset japanese --count 100000 --output japan.ged

# Generate with Arabic names (UTF-8 encoded)
cargo run --release -- --preset arabic --count 50000 --output arabic.ged

# Generate with German names
cargo run --release -- --preset german --count 75000 --output germany.ged
```

### Generate with default English ruleset

```bash
cargo run --release -- --preset english --count 100000 --output family.ged
# or simply:
cargo run --release -- --count 100000 --output family.ged
```

### Generate with LDS ordinances

```bash
cargo run --release -- --preset lds --count 50000 --output lds-family.ged
```

### Generate IOUS (Individual of Unusual Size)

Create highly connected individuals with multiple marriages and extensive descendants:

```bash
# Generate IOUS with default settings (3 marriages, 5 siblings, 5 generations)
rfamily generate-ious --preset english --output ious.ged

# Customize IOUS generation
rfamily generate-ious \
  --preset japanese \
  --output ious-japan.ged \
  --marriages 4 \
  --children-per-marriage 3.5 \
  --siblings 6 \
  --descendant-gens 4 \
  --total-descendants 500

# Minimal IOUS (1 marriage, no siblings, 2 generations)
rfamily generate-ious \
  --preset spanish \
  --output ious-minimal.ged \
  --marriages 1 \
  --children-per-marriage 2.0 \
  --siblings 0 \
  --descendant-gens 2
```

**IOUS Parameters:**
- `--marriages`: Number of marriages (1-10, default: 3)
- `--children-per-marriage`: Mean children per marriage (0-15, default: 4.0)
- `--siblings`: Number of siblings for IOUS (0-20, default: 5)
- `--descendant-gens`: Generations of descendants (1-10, default: 5)
- `--total-descendants`: Optional limit on total individuals

### Performance Optimization Features (NEW!)

Rfamily now includes advanced performance optimizations for massive-scale generation:

#### Parallel Generation (Automatic)

All simple-mode generation automatically uses multi-core parallelization:

```bash
# Automatically uses all CPU cores for 3-4x speedup
rfamily generate --preset english -c 100000 -o output.ged
```

**Performance:** 100K records in ~0.16s (vs 0.6s single-threaded) on 4-core CPU

#### Compression Support

Compress output files with gzip for 80-85% size reduction:

```bash
# Generate compressed file (auto-adds .gz extension)
rfamily generate --preset english -c 100000 -o output.ged --compress

# Result: output.ged.gz (10MB → 1.8MB)
```

**Benefits:**
- 80-85% smaller files
- Valid gzip format - decompress with `gunzip`
- Only ~10% slower generation

#### Streaming Mode (Memory-Efficient)

For very large datasets (1M+ records), use streaming mode for constant memory usage:

```bash
# Memory-efficient generation for large datasets
rfamily generate --preset english -c 1000000 -o output.ged --streaming

# Automatically enabled for 500K+ records
rfamily generate --preset english -c 600000 -o output.ged
```

**Benefits:**
- Constant memory usage (~100MB regardless of size)
- Can generate 10M+ records without running out of memory
- Real-time progress updates during generation
- 10x lower memory footprint

#### Combine All Features

```bash
# Large dataset with compression and streaming
rfamily generate --preset english -c 1000000 -o family.ged --compress --streaming
```

**Performance Summary:**
| Feature | Benefit | Example |
|---------|---------|---------|
| Parallel Generation | 3-4x faster | 100K in 0.16s (was 0.6s) |
| Compression | 80-85% smaller | 10MB → 1.8MB |
| Streaming | 10x less memory | Constant O(10K) vs O(n) |
| Progress Bar | Real-time updates | Shows records/sec |

### Parse GEDCOM Files

Use the library API to parse existing GEDCOM files:

```rust
use rfamily_core::gedcom::{GedcomParser, ParseMode};

// Parse in lenient mode (accepts real-world GEDCOM quirks)
let mut parser = GedcomParser::new(ParseMode::Lenient);
let gedcom = parser.parse_file("family.ged")?;

println!("Parsed {} individuals", gedcom.individuals.len());
println!("Parsed {} families", gedcom.families.len());

// Access parsed data
for (xref, individual) in &gedcom.individuals {
    println!("{}: {}", xref, individual.name.as_ref().unwrap());
}

// Check for warnings
for warning in parser.warnings() {
    println!("Warning: {}", warning);
}
```

## Examples

Six working examples are provided in `rfamily-core/examples/`:

### GEDCOM Parsing and Generation

```bash
# Example 1: Parse an existing GEDCOM file
cargo run -p rfamily-core --example parse_gedcom -- path/to/file.ged

# Example 2: Generate an IOUS (Individual of Unusual Size)
cargo run -p rfamily-core --example generate_ious

# Example 3: Round-trip test (generate → parse → verify)
cargo run -p rfamily-core --example round_trip
```

### Performance Features

```bash
# Example 4: Streaming generation (memory-efficient)
cargo run -p rfamily-core --example streaming_generation --release

# Example 5: Compression example (size comparison)
cargo run -p rfamily-core --example compression_example --release

# Example 6: Combined features (streaming + compression)
cargo run -p rfamily-core --example combined_features --release
```

**Example Descriptions:**
- `parse_gedcom`: Parses GEDCOM files, shows individuals/families, validates references
- `generate_ious`: Creates a 200-person IOUS family tree with 3 marriages, 5 siblings, 4 generations
- `round_trip`: Generates 100 individuals, parses them back, verifies data integrity
- `streaming_generation`: Demonstrates memory-efficient batch generation with 100K records
- `compression_example`: Compares plain vs compressed output with performance metrics
- `combined_features`: Shows streaming + compression for large datasets (500K records)

## Available Language Presets

**European Languages** (30):
Albanian, Bulgarian, Croatian, Czech, Danish, Dutch, English, Estonian, Finnish, French, German, Greek, Hungarian, Icelandic, Italian, Latvian, Lithuanian, Macedonian, Norwegian, Polish, Portuguese, Romanian, Russian, Serbian, Slovak, Slovenian, Spanish, Swedish, Turkish, Ukrainian

**Asian Languages** (7):
Chinese (Traditional), Japanese, Korean, Khmer (Cambodian), Mongolian, Thai, Vietnamese

**Middle Eastern Languages** (3):
Arabic, Armenian, Farsi (Persian)

**Pacific Languages** (6):
Fijian, Malagasy (Madagascar), Malay, Samoan, Tongan, Tagalog (Filipino)

**African Languages** (1):
Swahili

**Caribbean & Latin American Languages** (3):
Haitian Creole, Guarani (Paraguayan), Cebuano (Filipino)

**Special Presets** (1):
LDS (Latter-day Saints with temple ordinances)

## Using Custom Rulesets

### 1. Generate an example ruleset file

```bash
cargo run --release -- --generate-ruleset my-ruleset.json
```

### 2. Edit the ruleset file to customize

- **Names**: Male/female given names, surnames, naming conventions (Western, Eastern, Patronymic, Icelandic)
- **Dates**: Birth year ranges, marriage ages, life expectancy, parent age ranges
- **Locations**: Countries, cities, languages with probability weights
- **Demographics**: Sex ratio, twin/triplet rates
- **Relationships**: Marriage probability, divorce rates, children distribution, multi-generational families
- **Ordinances**: LDS temple ordinance settings (baptism, endowment, sealing, etc.)

### 3. Generate GEDCOM using your custom ruleset

```bash
cargo run --release -- --ruleset my-ruleset.json --count 200000 --output custom.ged
```

## Command-line Options

```text
Options:
  -c, --count <COUNT>              Number of individuals to generate [default: 100000]
  -o, --output <OUTPUT>            Output file path [default: output.ged]
  -p, --preset <PRESET>            Language preset to use (see --list-presets)
      --list-presets               List all available language presets
  -r, --ruleset <RULESET>          Custom ruleset configuration file (JSON)
      --generate-ruleset <FILE>    Generate example ruleset file
  -h, --help                       Print help
  -V, --version                    Print version
```

**Deprecated options** (still supported for backward compatibility):

- `--lds`, `--icelandic`, `--spanish`, `--french`, `--italian` - Use `--preset <name>` instead

## Ruleset Configuration Examples

See the generated `example-ruleset.json` file for complete configuration options. Key sections include:

**Names**, **Dates**, **Locations**, **Demographics**, **Relationships**, and **Ordinances**.

Refer to the full documentation in the source code for detailed parameter descriptions.

## Performance

Rfamily is optimized for high-performance GEDCOM generation and parsing with comprehensive benchmarking.

### Benchmark Results

Tested on modern hardware (Apple Silicon / Intel x86_64):

**Parser Performance:**
- 1,000 records: ~2.2ms (~462K records/sec)
- 10,000 records: ~21ms (~481K records/sec)
- 100,000 records: ~370ms (~270K records/sec)
- **Scaling**: Linear O(n) - confirmed via scalability tests

**Generator Performance** (with parallel generation):
- 1,000 records: ~0.2ms (~5M records/sec)
- 10,000 records: ~1.5ms (~6.7M records/sec)
- 100,000 records: ~16ms (~6.3M records/sec)
- **Scaling**: Linear O(n) - confirmed via scalability tests
- **Speedup**: 3-4x faster on multi-core systems vs single-threaded

**IOUS Generator Performance:**
- 100 descendants: <10ms
- 1,000 descendants: <100ms
- 5,000 descendants: <1 second
- Supports up to 10 generations deep without stack overflow

**Acceptance Criteria Met:**
- ✅ Parse 100K records in <5 seconds (actual: 0.37s)
- ✅ Generate 100K records in <10 seconds (actual: 0.11s)
- ✅ IOUS 1000 descendants in <100ms
- ✅ Linear O(n) scaling confirmed for parser and generator
- ✅ Memory efficient streaming for large datasets

### Running Benchmarks

```bash
# Run all Criterion.rs benchmarks
cargo bench -p rfamily-core

# Run specific benchmark suite
cargo bench -p rfamily-core --bench parser_bench
cargo bench -p rfamily-core --bench generator_bench
cargo bench -p rfamily-core --bench ious_bench

# Run integration performance tests
cargo test -p rfamily-core --test performance_tests --release -- --nocapture

# Run ignored stress tests (10M+ scale)
cargo test -p rfamily-core --test performance_tests --release -- --ignored --nocapture
```

Actual performance depends on your CPU, disk I/O speed, and complexity of family relationships.

## Generated GEDCOM Features

The generated GEDCOM file includes:

- Standard GEDCOM 5.5.1 header
- Individual records (INDI) with:
  - Full names (given name and surname)
  - Sex (based on demographic rules)
  - Birth date and place
  - Death date and place (optional)
  - Language
  - Family relationships (parents and spouses)
  - LDS ordinances (optional)
- Family records (FAM) with:
  - Husband and wife references
  - Children references
  - Marriage date and place
  - Divorce date (if applicable)
- Proper GEDCOM trailer

## Example Output

```gedcom
0 HEAD
1 SOUR Rfamily
2 VERS 0.2.0
1 GEDC
2 VERS 5.5.1
0 @I1@ INDI
1 NAME James /Smith/
2 GIVN James
2 SURN Smith
1 SEX M
1 BIRT
2 DATE 15 MAR 1985
...
0 TRLR
```

## Use Cases

- **Genealogy Software Testing**: Generate realistic test data for genealogy applications in 51 different languages
- **Performance Testing**: Test how software handles large GEDCOM files with millions of records
- **Data Analysis**: Create datasets for studying genealogical patterns across different cultures
- **LDS Family History**: Generate data with temple ordinances for testing FamilySearch integrations
- **Cultural Studies**: Generate families following specific cultural naming conventions and demographics
- **Internationalization Testing**: Test genealogy software with Unicode names and non-Latin scripts
- **Database Population**: Quickly populate databases with realistic multi-generational family data

## Technical Details

- **Format**: GEDCOM 5.5.1 standard
- **Encoding**: UTF-8 with full Unicode support
- **Language**: Rust for optimal performance and memory safety
- **Architecture**: Streaming output for minimal memory footprint
- **Distribution**: Single self-contained binary with all 51 presets embedded
- **Binary Size**: ~1.5 MB (includes all presets and dependencies)
- **Platform**: macOS (Apple Silicon/Intel), Linux, Windows (via cross-compilation)

## Distribution

The compiled binary is completely standalone and can be distributed without any dependencies:

**Included in Binary:**

- ✅ All 51 language presets (embedded at compile time)
- ✅ Complete GEDCOM generation engine
- ✅ UTF-8 support for all character sets
- ✅ No external files required
- ✅ No runtime dependencies

**To distribute:**

1. Build the release binary: `cargo build --release`
2. The binary is located at `target/release/rfamily` (~1.5 MB)
3. Copy to any location - it works standalone
4. Optional: Copy to PATH for system-wide access: `cp target/release/rfamily /usr/local/bin/`

**Binary Information:**

- macOS: Mach-O 64-bit executable (Apple Silicon: arm64, Intel: x86_64)
- All 51 language presets embedded (204 KB preset data)
- Self-contained - no external files or dependencies needed

**Cross-compilation for other platforms:**

```bash
# For Linux from macOS
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu

# For Windows from macOS  
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

## Future Enhancements

Potential features to add:

- More language presets (Hindi, Tamil, Telugu, Urdu, etc.)
- More sophisticated relationship modeling
- Historical accuracy improvements
- DNA/genetic relationship modeling
- Import/merge with existing GEDCOM files
- Custom name frequency distributions
- Migration patterns across locations

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
