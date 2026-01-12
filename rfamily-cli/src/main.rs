use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use rfamily_core::compression::{adjust_filename_for_compression, OutputWriter};
use rfamily_core::generator::GedcomGenerator;
use rfamily_core::generators::ious::{IOUSConfig, IOUSGenerator};
use rfamily_core::preset_registry::PresetRegistry;
use rfamily_core::ruleset::Ruleset;
use std::fs::File;
use std::io::BufWriter;

#[derive(Parser, Debug)]
#[command(name = "rfamily")]
#[command(version)]
#[command(about = "Generate GEDCOM files with millions of people", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    // Legacy flags for backward compatibility (when no subcommand is used)
    /// Number of individuals to generate
    #[arg(short, long, global = true)]
    count: Option<usize>,

    /// Output file path
    #[arg(short, long, global = true)]
    output: Option<String>,

    /// Preset to use (e.g., english, spanish, french). Use --list-presets to see all
    #[arg(short, long, global = true)]
    preset: Option<String>,

    /// List all available presets
    #[arg(long, global = true)]
    list_presets: bool,

    /// Ruleset configuration file (JSON)
    #[arg(short, long, global = true)]
    ruleset: Option<String>,

    /// Generate example ruleset file
    #[arg(long, global = true)]
    generate_ruleset: Option<String>,

    /// Override number of generations for family trees (1-10)
    #[arg(short = 'g', long, global = true)]
    generations: Option<usize>,

    // Deprecated flags
    #[arg(long, hide = true, global = true)]
    lds: bool,
    #[arg(long, hide = true, global = true)]
    icelandic: bool,
    #[arg(long, hide = true, global = true)]
    spanish: bool,
    #[arg(long, hide = true, global = true)]
    french: bool,
    #[arg(long, hide = true, global = true)]
    italian: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate standard GEDCOM file (default)
    Generate {
        /// Number of individuals to generate
        #[arg(short, long, default_value = "100000")]
        count: usize,

        /// Output file path
        #[arg(short, long, default_value = "output.ged")]
        output: String,

        /// Preset to use
        #[arg(short, long)]
        preset: Option<String>,

        /// Ruleset configuration file (JSON)
        #[arg(short, long)]
        ruleset: Option<String>,

        /// Override number of generations (1-10)
        #[arg(short = 'g', long)]
        generations: Option<usize>,

        /// Compress output with gzip (adds .gz extension)
        #[arg(long)]
        compress: bool,

        /// Use streaming generation (memory-efficient for 1M+ records)
        #[arg(long)]
        streaming: bool,
    },

    /// Generate IOUS (Individual of Unusual Size) tree
    GenerateIous {
        /// Output file path
        #[arg(short, long, default_value = "ious.ged")]
        output: String,

        /// Preset to use
        #[arg(short, long)]
        preset: Option<String>,

        /// Ruleset configuration file (JSON)
        #[arg(short, long)]
        ruleset: Option<String>,

        /// Number of marriages for the IOUS individual
        #[arg(long, default_value = "3")]
        marriages: usize,

        /// Mean number of children per marriage
        #[arg(long, default_value = "4.0")]
        children_per_marriage: f64,

        /// Number of siblings for the IOUS
        #[arg(long, default_value = "5")]
        siblings: usize,

        /// Number of generations of descendants
        #[arg(long, default_value = "5")]
        descendant_gens: usize,

        /// Target total number of descendants (approximate)
        #[arg(long)]
        total_descendants: Option<usize>,
    },
}

fn determine_preset_name_legacy(cli: &Cli) -> Option<String> {
    // Check new --preset flag first
    if let Some(ref preset) = cli.preset {
        return Some(preset.clone());
    }

    // Fall back to deprecated flags
    if cli.lds {
        eprintln!("Warning: --lds is deprecated. Use --preset lds instead.");
        return Some("lds".to_string());
    }
    if cli.icelandic {
        eprintln!("Warning: --icelandic is deprecated. Use --preset icelandic instead.");
        return Some("icelandic".to_string());
    }
    if cli.spanish {
        eprintln!("Warning: --spanish is deprecated. Use --preset spanish instead.");
        return Some("spanish".to_string());
    }
    if cli.french {
        eprintln!("Warning: --french is deprecated. Use --preset french instead.");
        return Some("french".to_string());
    }
    if cli.italian {
        eprintln!("Warning: --italian is deprecated. Use --preset italian instead.");
        return Some("italian".to_string());
    }

    None
}

fn load_ruleset(
    registry: &PresetRegistry,
    ruleset_path: &Option<String>,
    preset_name: Option<String>,
) -> Ruleset {
    if let Some(ref path) = ruleset_path {
        Ruleset::load_from_file(path).expect("Failed to load ruleset file")
    } else if let Some(name) = preset_name {
        registry.load(&name).unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        })
    } else {
        registry
            .load("english")
            .expect("Failed to load default preset")
    }
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let registry = PresetRegistry::new();

    // Handle list presets
    if cli.list_presets {
        println!("Available presets:");
        for preset in registry.list() {
            println!("  - {}", preset);
        }
        println!("\nUse with: rfamily --preset <name> --count <count> --output <file>");
        println!("Or: rfamily generate-ious --preset <name> --output <file>");
        return Ok(());
    }

    // Handle ruleset generation
    if let Some(ref path) = cli.generate_ruleset {
        let preset_name = determine_preset_name_legacy(&cli);
        let ruleset = load_ruleset(&registry, &cli.ruleset, preset_name);
        ruleset.save_to_file(path).expect("Failed to save ruleset");
        println!("Generated ruleset file: {}", path);
        return Ok(());
    }

    match cli.command {
        Some(Commands::Generate {
            count,
            output,
            preset,
            ruleset,
            generations,
            compress,
            streaming,
        }) => handle_generate(
            registry,
            count,
            output,
            preset,
            ruleset,
            generations,
            compress,
            streaming,
        ),
        Some(Commands::GenerateIous {
            output,
            preset,
            ruleset,
            marriages,
            children_per_marriage,
            siblings,
            descendant_gens,
            total_descendants,
        }) => handle_generate_ious(
            registry,
            output,
            preset,
            ruleset,
            marriages,
            children_per_marriage,
            siblings,
            descendant_gens,
            total_descendants,
        ),
        None => {
            // Backward compatibility: no subcommand means default generate behavior
            let count = cli.count.unwrap_or(100000);
            let output = cli
                .output
                .clone()
                .unwrap_or_else(|| "output.ged".to_string());
            let preset = determine_preset_name_legacy(&cli);
            let ruleset = cli.ruleset.clone();
            let generations = cli.generations;
            handle_generate(
                registry,
                count,
                output,
                preset,
                ruleset,
                generations,
                false,
                false,
            )
        }
    }
}

fn handle_generate(
    registry: PresetRegistry,
    count: usize,
    output: String,
    preset: Option<String>,
    ruleset_path: Option<String>,
    generations: Option<usize>,
    compress: bool,
    streaming: bool,
) -> std::io::Result<()> {
    let mut ruleset = load_ruleset(&registry, &ruleset_path, preset);

    // Override generations if specified
    if let Some(gens) = generations {
        if !(1..=10).contains(&gens) {
            eprintln!("Error: generations must be between 1 and 10 (got {})", gens);
            std::process::exit(1);
        }
        ruleset.relationships.generations = gens;
        println!("Overriding generations to: {}", gens);
    }

    // Adjust filename for compression
    let final_output = adjust_filename_for_compression(&output, compress);

    println!("Rfamily v0.3.0 - Generate");
    println!("Generating {} individuals to {}", count, final_output);
    if compress {
        println!("Compression: enabled (gzip)");
    }
    if streaming {
        println!("Mode: streaming (memory-efficient)");
    }

    let mut generator = GedcomGenerator::new(ruleset);

    // Progress bar setup with real-time updates
    let pb = ProgressBar::new(count as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({per_sec}, eta {eta})")
            .unwrap()
            .progress_chars("█▓▒░-"),
    );

    // Choose generation mode
    if streaming || count >= 500_000 {
        // Use streaming mode for large datasets or when explicitly requested
        let mut writer = OutputWriter::create(&final_output, compress)?;

        // Streaming with real-time progress updates
        generator.generate_streaming(count, &mut writer, |current| {
            pb.set_position(current as u64);
        })?;

        writer.finish()?;
        pb.finish_with_message("✓ Generation complete");
    } else {
        // Traditional mode for smaller datasets
        let mut rng = rand::thread_rng();
        generator.generate(count, &mut rng);
        pb.set_position(count as u64);
        pb.finish_with_message("✓ Generation complete");

        // Write to file
        let mut writer = OutputWriter::create(&final_output, compress)?;
        generator.write_gedcom(&mut writer)?;
        writer.finish()?;
    }

    println!(
        "Successfully generated GEDCOM file with {} individuals",
        count
    );

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn handle_generate_ious(
    registry: PresetRegistry,
    output: String,
    preset: Option<String>,
    ruleset_path: Option<String>,
    marriages: usize,
    children_per_marriage: f64,
    siblings: usize,
    descendant_gens: usize,
    total_descendants: Option<usize>,
) -> std::io::Result<()> {
    let ruleset = load_ruleset(&registry, &ruleset_path, preset);

    // Validate parameters
    if !(1..=10).contains(&marriages) {
        eprintln!(
            "Error: marriages must be between 1 and 10 (got {})",
            marriages
        );
        std::process::exit(1);
    }
    if !(0.0..=15.0).contains(&children_per_marriage) {
        eprintln!(
            "Error: children_per_marriage must be between 0 and 15 (got {})",
            children_per_marriage
        );
        std::process::exit(1);
    }
    if siblings > 20 {
        eprintln!("Error: siblings must be <= 20 (got {})", siblings);
        std::process::exit(1);
    }
    if !(1..=10).contains(&descendant_gens) {
        eprintln!(
            "Error: descendant_gens must be between 1 and 10 (got {})",
            descendant_gens
        );
        std::process::exit(1);
    }

    println!("Rfamily v0.3.0 - Generate IOUS");
    println!("Configuration:");
    println!("  Marriages: {}", marriages);
    println!("  Children per marriage (mean): {}", children_per_marriage);
    println!("  Siblings: {}", siblings);
    println!("  Descendant generations: {}", descendant_gens);
    if let Some(target) = total_descendants {
        println!("  Target descendants: ~{}", target);
    }
    println!("  Output: {}", output);

    let config = IOUSConfig {
        marriages,
        children_per_marriage_mean: children_per_marriage,
        siblings,
        descendant_generations: descendant_gens,
        target_descendants: total_descendants,
    };

    let mut ious_generator = IOUSGenerator::new(ruleset, config);
    let mut rng = rand::thread_rng();

    println!("\nGenerating IOUS family tree...");
    let count = ious_generator.generate(&mut rng);
    println!("Generated {} individuals", count);

    // Write to file
    let file = File::create(&output)?;
    let mut writer = BufWriter::new(file);
    let generator = ious_generator.into_generator();
    generator.write_gedcom(&mut writer)?;

    println!("Successfully generated IOUS GEDCOM file: {}", output);

    Ok(())
}
