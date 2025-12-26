mod generator;
mod preset_registry;
mod ruleset;

use clap::Parser;
use generator::GedcomGenerator;
use indicatif::{ProgressBar, ProgressStyle};
use preset_registry::PresetRegistry;
use ruleset::Ruleset;
use std::fs::File;
use std::io::BufWriter;

fn determine_preset_name(args: &Args) -> Option<String> {
    // Check new --preset flag first
    if let Some(ref preset) = args.preset {
        return Some(preset.clone());
    }

    // Fall back to deprecated flags for backward compatibility
    if args.lds {
        eprintln!("Warning: --lds is deprecated. Use --preset lds instead.");
        return Some("lds".to_string());
    }
    if args.icelandic {
        eprintln!("Warning: --icelandic is deprecated. Use --preset icelandic instead.");
        return Some("icelandic".to_string());
    }
    if args.spanish {
        eprintln!("Warning: --spanish is deprecated. Use --preset spanish instead.");
        return Some("spanish".to_string());
    }
    if args.french {
        eprintln!("Warning: --french is deprecated. Use --preset french instead.");
        return Some("french".to_string());
    }
    if args.italian {
        eprintln!("Warning: --italian is deprecated. Use --preset italian instead.");
        return Some("italian".to_string());
    }

    None
}

#[derive(Parser, Debug)]
#[command(name = "rfamily")]
#[command(version)]
#[command(about = "Generate GEDCOM files with millions of people", long_about = None)]
struct Args {
    /// Number of individuals to generate
    #[arg(short, long, default_value = "100000")]
    count: usize,

    /// Output file path
    #[arg(short, long, default_value = "output.ged")]
    output: String,

    /// Preset to use (e.g., english, spanish, french). Use --list-presets to see all
    #[arg(short, long)]
    preset: Option<String>,

    /// List all available presets
    #[arg(long)]
    list_presets: bool,

    /// Ruleset configuration file (JSON)
    #[arg(short, long)]
    ruleset: Option<String>,

    /// Generate example ruleset file
    #[arg(long)]
    generate_ruleset: Option<String>,

    /// [DEPRECATED] Use --preset lds instead
    #[arg(long, hide = true)]
    lds: bool,

    /// [DEPRECATED] Use --preset icelandic instead
    #[arg(long, hide = true)]
    icelandic: bool,

    /// [DEPRECATED] Use --preset spanish instead
    #[arg(long, hide = true)]
    spanish: bool,

    /// [DEPRECATED] Use --preset french instead
    #[arg(long, hide = true)]
    french: bool,

    /// [DEPRECATED] Use --preset italian instead
    #[arg(long, hide = true)]
    italian: bool,

    /// Override number of generations for family trees (1-10)
    #[arg(short = 'g', long)]
    generations: Option<usize>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let registry = PresetRegistry::new();

    // Handle list presets
    if args.list_presets {
        println!("Available presets:");
        for preset in registry.list() {
            println!("  - {}", preset);
        }
        println!("\nUse with: rfamily --preset <name> <count> <output>");
        return Ok(());
    }

    // Handle ruleset generation
    if let Some(ref path) = args.generate_ruleset {
        let preset_name = determine_preset_name(&args);
        let ruleset = if let Some(name) = preset_name {
            registry.load(&name).expect("Failed to load preset")
        } else {
            registry
                .load("english")
                .expect("Failed to load default preset")
        };

        ruleset.save_to_file(path).expect("Failed to save ruleset");
        println!("Generated ruleset file: {}", path);
        return Ok(());
    }

    // Load ruleset
    let mut ruleset = if let Some(ref path) = args.ruleset {
        Ruleset::load_from_file(path).expect("Failed to load ruleset file")
    } else {
        let preset_name = determine_preset_name(&args);
        if let Some(name) = preset_name {
            registry.load(&name).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            })
        } else {
            registry
                .load("english")
                .expect("Failed to load default preset")
        }
    };

    // Override generations if specified
    if let Some(generations) = args.generations {
        if generations < 1 || generations > 10 {
            eprintln!("Error: generations must be between 1 and 10 (got {})", generations);
            std::process::exit(1);
        }
        ruleset.relationships.generations = generations;
        println!("Overriding generations to: {}", generations);
    }

    println!("Rfamily v0.2.0");
    println!("Generating {} individuals to {}", args.count, args.output);

    let mut rng = rand::thread_rng();
    let mut generator = GedcomGenerator::new(ruleset);

    // Progress bar setup
    let pb = ProgressBar::new(args.count as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("##-"),
    );

    // Generate data
    generator.generate(args.count, &mut rng);
    pb.finish_with_message("Generation complete");

    // Write to file
    let file = File::create(&args.output)?;
    let mut writer = BufWriter::new(file);
    generator.write_gedcom(&mut writer)?;

    println!(
        "Successfully generated GEDCOM file with {} individuals",
        args.count
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_preset_name_with_preset_flag() {
        let args = Args {
            count: 100,
            output: "test.ged".to_string(),
            preset: Some("japanese".to_string()),
            list_presets: false,
            ruleset: None,
            generate_ruleset: None,
            lds: false,
            icelandic: false,
            spanish: false,
            french: false,
            italian: false,
            generations: None,
        };

        assert_eq!(determine_preset_name(&args), Some("japanese".to_string()));
    }

    #[test]
    fn test_determine_preset_name_with_lds_flag() {
        let args = Args {
            count: 100,
            output: "test.ged".to_string(),
            preset: None,
            list_presets: false,
            ruleset: None,
            generate_ruleset: None,
            lds: true,
            icelandic: false,
            spanish: false,
            french: false,
            italian: false,
            generations: None,
        };

        assert_eq!(determine_preset_name(&args), Some("lds".to_string()));
    }

    #[test]
    fn test_determine_preset_name_with_icelandic_flag() {
        let args = Args {
            count: 100,
            output: "test.ged".to_string(),
            preset: None,
            list_presets: false,
            ruleset: None,
            generate_ruleset: None,
            lds: false,
            icelandic: true,
            spanish: false,
            french: false,
            italian: false,
            generations: None,
        };

        assert_eq!(determine_preset_name(&args), Some("icelandic".to_string()));
    }

    #[test]
    fn test_determine_preset_name_with_spanish_flag() {
        let args = Args {
            count: 100,
            output: "test.ged".to_string(),
            preset: None,
            list_presets: false,
            ruleset: None,
            generate_ruleset: None,
            lds: false,
            icelandic: false,
            spanish: true,
            french: false,
            italian: false,
            generations: None,
        };

        assert_eq!(determine_preset_name(&args), Some("spanish".to_string()));
    }

    #[test]
    fn test_determine_preset_name_with_french_flag() {
        let args = Args {
            count: 100,
            output: "test.ged".to_string(),
            preset: None,
            list_presets: false,
            ruleset: None,
            generate_ruleset: None,
            lds: false,
            icelandic: false,
            spanish: false,
            french: true,
            italian: false,
            generations: None,
        };

        assert_eq!(determine_preset_name(&args), Some("french".to_string()));
    }

    #[test]
    fn test_determine_preset_name_with_italian_flag() {
        let args = Args {
            count: 100,
            output: "test.ged".to_string(),
            preset: None,
            list_presets: false,
            ruleset: None,
            generate_ruleset: None,
            lds: false,
            icelandic: false,
            spanish: false,
            french: false,
            italian: true,
            generations: None,
        };

        assert_eq!(determine_preset_name(&args), Some("italian".to_string()));
    }

    #[test]
    fn test_determine_preset_name_no_flags() {
        let args = Args {
            count: 100,
            output: "test.ged".to_string(),
            preset: None,
            list_presets: false,
            ruleset: None,
            generate_ruleset: None,
            lds: false,
            icelandic: false,
            spanish: false,
            french: false,
            italian: false,
            generations: None,
        };

        assert_eq!(determine_preset_name(&args), None);
    }

    #[test]
    fn test_determine_preset_name_preset_overrides_deprecated() {
        // Preset flag should take precedence over deprecated flags
        let args = Args {
            count: 100,
            output: "test.ged".to_string(),
            preset: Some("german".to_string()),
            list_presets: false,
            ruleset: None,
            generate_ruleset: None,
            lds: true, // This should be ignored
            icelandic: false,
            spanish: false,
            french: false,
            italian: false,
            generations: None,
        };

        assert_eq!(determine_preset_name(&args), Some("german".to_string()));
    }

    #[test]
    fn test_generations_override() {
        let args = Args::parse_from(&[
            "rfamily",
            "--preset",
            "english",
            "--count",
            "1000",
            "--generations",
            "6",
        ]);

        assert_eq!(args.generations, Some(6));
        assert_eq!(args.count, 1000);
        assert_eq!(args.preset, Some("english".to_string()));
    }

    #[test]
    fn test_args_default_values() {
        // Test that clap default values work
        let args = Args::parse_from(&["rfamily"]);

        assert_eq!(args.count, 100000);
        assert_eq!(args.output, "output.ged");
        assert_eq!(args.preset, None);
        assert_eq!(args.list_presets, false);
        assert_eq!(args.ruleset, None);
        assert_eq!(args.generate_ruleset, None);
    }

    #[test]
    fn test_args_with_preset() {
        let args = Args::parse_from(&[
            "rfamily",
            "--preset",
            "korean",
            "--count",
            "5000",
            "--output",
            "korea.ged",
        ]);

        assert_eq!(args.preset, Some("korean".to_string()));
        assert_eq!(args.count, 5000);
        assert_eq!(args.output, "korea.ged");
    }

    #[test]
    fn test_args_with_list_presets() {
        let args = Args::parse_from(&["rfamily", "--list-presets"]);

        assert_eq!(args.list_presets, true);
    }

    #[test]
    fn test_args_with_ruleset_file() {
        let args = Args::parse_from(&["rfamily", "--ruleset", "custom.json", "--count", "1000"]);

        assert_eq!(args.ruleset, Some("custom.json".to_string()));
        assert_eq!(args.count, 1000);
    }

    #[test]
    fn test_args_with_generate_ruleset() {
        let args = Args::parse_from(&["rfamily", "--generate-ruleset", "example.json"]);

        assert_eq!(args.generate_ruleset, Some("example.json".to_string()));
    }

    #[test]
    fn test_args_short_flags() {
        let args =
            Args::parse_from(&["rfamily", "-c", "50000", "-o", "family.ged", "-p", "polish"]);

        assert_eq!(args.count, 50000);
        assert_eq!(args.output, "family.ged");
        assert_eq!(args.preset, Some("polish".to_string()));
    }
}
