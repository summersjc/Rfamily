/// Example: Generate an IOUS (Individual of Unusual Size)
///
/// This example demonstrates how to use the IOUS generator to create highly
/// connected individuals with multiple marriages and extensive descendants.
///
/// Usage:
///   cargo run --example generate_ious
use rfamily_core::generators::ious::{IOUSConfig, IOUSGenerator};
use rfamily_core::ruleset::Ruleset;
use std::fs::File;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== IOUS Generator Example ===\n");

    // Load a language preset ruleset
    let ruleset = Ruleset::default_english();
    println!("Using English language preset");

    // Configure IOUS generation
    let config = IOUSConfig {
        marriages: 3,                    // Number of marriages
        children_per_marriage_mean: 4.0, // Mean children per marriage (Poisson distribution)
        siblings: 5,                     // Number of siblings for the IOUS
        descendant_generations: 4,       // Generations of descendants
        target_descendants: Some(200),   // Optional limit on total individuals
    };

    println!("\nConfiguration:");
    println!("  Marriages: {}", config.marriages);
    println!(
        "  Children per marriage (mean): {}",
        config.children_per_marriage_mean
    );
    println!("  Siblings: {}", config.siblings);
    println!(
        "  Descendant generations: {}",
        config.descendant_generations
    );
    println!(
        "  Target descendants: {}",
        config
            .target_descendants
            .map_or("unlimited".to_string(), |n| n.to_string())
    );

    // Create IOUS generator
    let mut generator = IOUSGenerator::new(ruleset, config);
    let mut rng = rand::thread_rng();

    // Generate the IOUS family tree
    println!("\nGenerating IOUS family tree...");
    let count = generator.generate(&mut rng);

    println!("✓ Generated {} individuals", count);

    // Get the underlying generator to write GEDCOM
    let gedcom_generator = generator.into_generator();

    // Write to GEDCOM file
    let output_path = "ious_example.ged";
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    println!("\nWriting to {}...", output_path);
    gedcom_generator.write_gedcom(&mut writer)?;

    println!("✓ GEDCOM file written successfully!");
    println!("\nYou can now:");
    println!("  1. Open {} in genealogy software", output_path);
    println!(
        "  2. Parse it with: cargo run --example parse_gedcom -- {}",
        output_path
    );

    Ok(())
}
