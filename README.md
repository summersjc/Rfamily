# Rfamily

[![Rust CI](https://github.com/summersjc/Rfamily/actions/workflows/rust.yml/badge.svg)](https://github.com/summersjc/Rfamily/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![Test Coverage](https://img.shields.io/badge/coverage-81.5%25-brightgreen.svg)](https://github.com/summersjc/Rfamily)
[![Tests](https://img.shields.io/badge/tests-143_passing-brightgreen.svg)](https://github.com/summersjc/Rfamily)

A high-performance Rust tool for generating GEDCOM files with millions of people records using customizable rulesets.

**Available as both a CLI tool and web application with REST API!** 🚀

## Installation

See [INSTALL.md](INSTALL.md) for detailed installation instructions for all platforms.

**Quick install:** Download the binary for your platform from the [latest release](https://github.com/summersjc/Rfamily/releases/latest).

## Features

### CLI Tool
- **52 Language Presets**: Built-in support for European (including English USA & UK), Asian, Middle Eastern, Pacific, African, and Latin American languages with culturally appropriate names and locations
- **Ruleset-Based Generation**: Define custom rules for names, dates, locations, relationships, and LDS ordinances
- **Fast Generation**: Optimized for creating millions of records efficiently
- **Family Relationships**: Generate realistic multi-generational families with marriages, divorces, and children
- **Unicode Support**: Full UTF-8 support for non-Latin scripts (Arabic, Chinese, Japanese, Korean, etc.)
- **LDS Ordinances**: Optional support for baptism, endowment, sealing, and other LDS temple ordinances
- **Streaming Output**: Writes directly to file without loading everything into memory
- **Progress Tracking**: Real-time progress bar with ETA
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

On a typical modern machine, this tool can generate:

- 100,000 records with families in ~5-10 seconds
- 1 million records in ~30-60 seconds
- 10 million records in ~5-10 minutes

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
