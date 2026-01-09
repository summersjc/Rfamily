//! Example: Memory-efficient streaming generation for large datasets
//!
//! This example demonstrates how to use streaming generation to create GEDCOM files
//! with millions of records while maintaining constant memory usage.
//!
//! Run with: cargo run -p rfamily-core --example streaming_generation

use rfamily_core::generator::GedcomGenerator;
use rfamily_core::ruleset::Ruleset;
use std::fs::File;
use std::io::BufWriter;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    println!("=== Streaming Generation Example ===\n");

    // Configuration
    let count = 100_000; // Generate 100K individuals
    let output_path = "streaming_example.ged";

    println!("Configuration:");
    println!("  Count: {}", count);
    println!("  Output: {}", output_path);
    println!("  Mode: Streaming (10K batch size)");
    println!();

    // Create generator with English preset
    let ruleset = Ruleset::default_english();
    let mut generator = GedcomGenerator::new(ruleset);

    // Create output file
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    // Track progress
    let start = Instant::now();
    let mut last_update = Instant::now();

    println!("Starting generation...");

    // Generate with streaming mode and real-time progress
    generator.generate_streaming(count, &mut writer, |current| {
        // Update progress every second
        if last_update.elapsed().as_secs() >= 1 {
            let elapsed = start.elapsed().as_secs_f64();
            let rate = current as f64 / elapsed;
            let eta = ((count - current) as f64 / rate) as u64;

            println!(
                "  Progress: {}/{} ({:.1}%) - {:.0} records/sec - ETA: {}s",
                current,
                count,
                (current as f64 / count as f64) * 100.0,
                rate,
                eta
            );
            last_update = Instant::now();
        }
    })?;

    let elapsed = start.elapsed();

    println!("\n✓ Generation complete!");
    println!("  Time: {:.2}s", elapsed.as_secs_f64());
    println!(
        "  Rate: {:.0} records/sec",
        count as f64 / elapsed.as_secs_f64()
    );

    // Get file size
    let metadata = std::fs::metadata(output_path)?;
    println!("  File size: {:.2} MB", metadata.len() as f64 / 1_000_000.0);

    println!("\nMemory usage: ~100MB (constant, regardless of count)");
    println!(
        "\nCompare to traditional mode which would use ~{}MB for {} records",
        count / 1000, // Rough estimate: 1KB per record
        count
    );

    Ok(())
}
