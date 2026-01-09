//! Example: Combining streaming generation with compression
//!
//! This example demonstrates how to combine streaming generation and compression
//! for optimal performance when generating large GEDCOM files.
//!
//! Run with: cargo run -p rfamily-core --example combined_features --release

use rfamily_core::compression::{adjust_filename_for_compression, OutputWriter};
use rfamily_core::generator::GedcomGenerator;
use rfamily_core::ruleset::Ruleset;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    println!("=== Combined Features: Streaming + Compression ===\n");

    // Configuration for large-scale generation
    let count = 500_000; // 500K individuals
    let base_filename = "large_dataset.ged";

    println!("Configuration:");
    println!("  Count: {} individuals", count);
    println!("  Streaming: Enabled (10K batches)");
    println!("  Compression: Enabled (gzip)");
    println!("  Expected memory: ~100MB (constant)");
    println!();

    // Adjust filename for compression
    let output_path = adjust_filename_for_compression(base_filename, true);
    println!("Output: {}", output_path);
    println!();

    // Create generator
    let ruleset = Ruleset::default_english();
    let mut generator = GedcomGenerator::new(ruleset);

    // Create compressed output writer
    let mut writer = OutputWriter::create(&output_path, true)?;

    // Track progress
    let start = Instant::now();
    let mut last_update = Instant::now();
    let mut last_count = 0;

    println!("Starting generation...");

    // Generate with streaming mode and compression
    generator.generate_streaming(count, &mut writer, |current| {
        // Update progress every 2 seconds
        if last_update.elapsed().as_secs() >= 2 || current == count {
            let elapsed = start.elapsed().as_secs_f64();
            let rate = current as f64 / elapsed;
            let eta = ((count - current) as f64 / rate) as u64;
            let batch_rate = (current - last_count) as f64 / last_update.elapsed().as_secs_f64();

            println!(
                "  {:7} / {} ({:5.1}%) | {:8.0} rec/s | batch: {:8.0} rec/s | ETA: {:3}s",
                current,
                count,
                (current as f64 / count as f64) * 100.0,
                rate,
                batch_rate,
                eta
            );

            last_update = Instant::now();
            last_count = current;
        }
    })?;

    // Important: Finalize the compressed stream
    writer.finish()?;

    let elapsed = start.elapsed();

    println!("\n✓ Generation complete!");
    println!("\nPerformance:");
    println!("  Time: {:.2}s", elapsed.as_secs_f64());
    println!(
        "  Rate: {:.0} records/sec",
        count as f64 / elapsed.as_secs_f64()
    );

    // Get compressed file size
    let metadata = std::fs::metadata(&output_path)?;
    let compressed_size = metadata.len();

    // Estimate uncompressed size (rough: ~100 bytes per record)
    let estimated_uncompressed = count * 100;

    println!("\nFile size:");
    println!(
        "  Compressed: {:.2} MB",
        compressed_size as f64 / 1_000_000.0
    );
    println!(
        "  Estimated uncompressed: {:.2} MB",
        estimated_uncompressed as f64 / 1_000_000.0
    );
    println!(
        "  Compression ratio: {:.1}%",
        (compressed_size as f64 / estimated_uncompressed as f64) * 100.0
    );

    println!("\nBenefits of this approach:");
    println!("  ✓ Constant memory usage (~100MB regardless of count)");
    println!("  ✓ 80-85% smaller file size");
    println!("  ✓ Parallel generation within batches (3-4x speedup)");
    println!("  ✓ Real-time progress updates");
    println!("  ✓ Can scale to 10M+ records");

    println!("\nTo decompress: gunzip {}", output_path);

    // Cleanup
    println!("\nCleaning up...");
    std::fs::remove_file(&output_path)?;
    println!("✓ Done");

    Ok(())
}
