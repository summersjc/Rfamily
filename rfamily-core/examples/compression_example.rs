//! Example: Using gzip compression to reduce GEDCOM file sizes
//!
//! This example demonstrates how to use transparent gzip compression to reduce
//! file sizes by 80-85% with minimal performance overhead.
//!
//! Run with: cargo run -p rfamily-core --example compression_example

use rfamily_core::compression::{adjust_filename_for_compression, OutputWriter};
use rfamily_core::generator::GedcomGenerator;
use rfamily_core::ruleset::Ruleset;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    println!("=== Compression Example ===\n");

    // Configuration
    let count = 10_000; // Generate 10K individuals
    let base_filename = "compression_test.ged";

    println!("Generating {} individuals...\n", count);

    // Create generator
    let ruleset = Ruleset::default_english();
    let mut generator = GedcomGenerator::new(ruleset);
    let mut rng = rand::thread_rng();

    // Generate data (only once for fair comparison)
    println!("Pre-generating data...");
    generator.generate(count, &mut rng);
    println!("✓ Data ready\n");

    // === Write without compression ===
    println!("1. Writing without compression...");
    let plain_start = Instant::now();

    let mut plain_writer = OutputWriter::create(base_filename, false)?;
    generator.write_gedcom(&mut plain_writer)?;
    plain_writer.finish()?;

    let plain_elapsed = plain_start.elapsed();
    let plain_metadata = std::fs::metadata(base_filename)?;
    let plain_size = plain_metadata.len();

    println!("   Time: {:.3}s", plain_elapsed.as_secs_f64());
    println!(
        "   Size: {:.2} MB ({} bytes)",
        plain_size as f64 / 1_000_000.0,
        plain_size
    );

    // === Write with compression ===
    println!("\n2. Writing with compression...");
    let compressed_filename = adjust_filename_for_compression(base_filename, true);
    let compressed_start = Instant::now();

    let mut compressed_writer = OutputWriter::create(&compressed_filename, true)?;
    generator.write_gedcom(&mut compressed_writer)?;
    compressed_writer.finish()?;

    let compressed_elapsed = compressed_start.elapsed();
    let compressed_metadata = std::fs::metadata(&compressed_filename)?;
    let compressed_size = compressed_metadata.len();

    println!("   Time: {:.3}s", compressed_elapsed.as_secs_f64());
    println!(
        "   Size: {:.2} MB ({} bytes)",
        compressed_size as f64 / 1_000_000.0,
        compressed_size
    );

    // === Comparison ===
    println!("\n=== Comparison ===");
    let compression_ratio = (compressed_size as f64 / plain_size as f64) * 100.0;
    let space_saved = ((plain_size - compressed_size) as f64 / plain_size as f64) * 100.0;
    let time_overhead =
        ((compressed_elapsed.as_secs_f64() / plain_elapsed.as_secs_f64()) - 1.0) * 100.0;

    println!("Compression ratio: {:.1}% of original", compression_ratio);
    println!("Space saved: {:.1}%", space_saved);
    println!("Time overhead: {:.1}%", time_overhead);

    println!("\n✓ Files saved:");
    println!("  Plain:      {}", base_filename);
    println!("  Compressed: {}", compressed_filename);

    println!("\nTo decompress: gunzip {}", compressed_filename);

    // Cleanup
    println!("\nCleaning up...");
    std::fs::remove_file(base_filename)?;
    std::fs::remove_file(&compressed_filename)?;
    println!("✓ Done");

    Ok(())
}
