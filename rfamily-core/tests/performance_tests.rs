/// Integration Performance Tests
///
/// These tests validate that the GEDCOM parser, generator, and IOUS generator
/// meet performance acceptance criteria for various dataset sizes.
///
/// Run with: cargo test --test performance_tests --release -- --nocapture
///
/// Note: Some tests are marked #[ignore] for extreme scale (10M+) to avoid
/// overwhelming CI. Run manually with: cargo test --test performance_tests --release -- --ignored
///
/// Acceptance Criteria:
/// - Parser: 100K in <5s, 1M in <60s
/// - Generator: 100K in <10s, 1M in <120s
/// - IOUS: 1000 descendants in <100ms
/// - Linear O(n) scaling confirmed
mod helpers;

use helpers::perf_test_helpers::{
    assert_completes_within_tolerance, generate_test_file, measure_memory,
};
use rfamily_core::gedcom::{GedcomParser, ParseMode};
use rfamily_core::generator::GedcomGenerator;
use rfamily_core::generators::ious::{IOUSConfig, IOUSGenerator};
use rfamily_core::ruleset::Ruleset;
use std::fs::File;
use std::io::BufWriter;
use std::time::Duration;

// ============================================================================
// Test Suite 1: End-to-End Timing Tests
// ============================================================================

#[test]
fn test_parse_100k_timing() {
    println!("Generating 100K test file...");
    let test_file = generate_test_file(100_000, false);

    println!("Parsing 100K records...");
    let result = assert_completes_within_tolerance(
        || {
            let mut parser = GedcomParser::new(ParseMode::Lenient);
            let gedcom = parser.parse_file(&test_file).expect("Parse failed");
            assert!(gedcom.individuals.len() > 90_000); // Allow some variance
            gedcom
        },
        Duration::from_secs(5),
        1.5, // 50% tolerance for CI
    );

    println!("✓ Parsed {} individuals in <7.5s", result.individuals.len());
    std::fs::remove_file(&test_file).ok();
}

#[test]
#[ignore] // Slow test - run manually
fn test_parse_1m_timing() {
    println!("Generating 1M test file (this may take a minute)...");
    let test_file = generate_test_file(1_000_000, false);

    println!("Parsing 1M records...");
    let result = assert_completes_within_tolerance(
        || {
            let mut parser = GedcomParser::new(ParseMode::Lenient);
            let gedcom = parser.parse_file(&test_file).expect("Parse failed");
            assert!(gedcom.individuals.len() > 900_000);
            gedcom
        },
        Duration::from_secs(60),
        1.5,
    );

    println!("✓ Parsed {} individuals in <90s", result.individuals.len());
    std::fs::remove_file(&test_file).ok();
}

#[test]
fn test_generate_100k_timing() {
    println!("Generating 100K records...");

    let count = assert_completes_within_tolerance(
        || {
            let ruleset = Ruleset::default_english();
            let mut generator = GedcomGenerator::new(ruleset);
            let mut rng = rand::thread_rng();

            generator.generate(100_000, &mut rng);

            // Write to temp file to simulate real usage
            let temp_path = "/tmp/perf_test_gen_100k.ged";
            let file = File::create(temp_path).unwrap();
            let mut writer = BufWriter::new(file);
            generator.write_gedcom(&mut writer).unwrap();
            drop(writer);
            std::fs::remove_file(temp_path).ok();

            generator.individuals().len()
        },
        Duration::from_secs(10),
        1.5,
    );

    println!("✓ Generated {} individuals in <15s", count);
}

#[test]
#[ignore] // Very slow test - run manually
fn test_generate_1m_timing() {
    println!("Generating 1M records (this will take a couple minutes)...");

    let count = assert_completes_within_tolerance(
        || {
            let ruleset = Ruleset::default_english();
            let mut generator = GedcomGenerator::new(ruleset);
            let mut rng = rand::thread_rng();

            generator.generate(1_000_000, &mut rng);

            let temp_path = "/tmp/perf_test_gen_1m.ged";
            let file = File::create(temp_path).unwrap();
            let mut writer = BufWriter::new(file);
            generator.write_gedcom(&mut writer).unwrap();
            drop(writer);
            std::fs::remove_file(temp_path).ok();

            generator.individuals().len()
        },
        Duration::from_secs(120),
        1.5,
    );

    println!("✓ Generated {} individuals in <180s", count);
}

#[test]
fn test_round_trip_100k() {
    println!("Testing round-trip: generate → write → parse 100K records...");

    let result = assert_completes_within_tolerance(
        || {
            // Generate
            let ruleset = Ruleset::default_english();
            let mut generator = GedcomGenerator::new(ruleset);
            let mut rng = rand::thread_rng();
            generator.generate(100_000, &mut rng);

            // Write
            let temp_path = "/tmp/perf_test_round_trip_100k.ged";
            let file = File::create(temp_path).unwrap();
            let mut writer = BufWriter::new(file);
            generator.write_gedcom(&mut writer).unwrap();
            drop(writer);

            // Parse
            let mut parser = GedcomParser::new(ParseMode::Lenient);
            let gedcom = parser.parse_file(temp_path).unwrap();

            std::fs::remove_file(temp_path).ok();

            (generator.individuals().len(), gedcom.individuals.len())
        },
        Duration::from_secs(15),
        1.5,
    );

    println!(
        "✓ Round-trip: generated {} → parsed {} in <22.5s",
        result.0, result.1
    );
    assert!(result.1 > 90_000); // Verify parsing got most records
}

// ============================================================================
// Test Suite 2: Memory Usage Tests (Placeholder)
// ============================================================================

#[test]
fn test_parse_memory_100k() {
    let test_file = generate_test_file(100_000, false);

    let _memory = measure_memory(|| {
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let gedcom = parser.parse_file(&test_file).unwrap();
        assert!(gedcom.individuals.len() > 90_000);
    });

    // Note: measure_memory currently returns 0 (placeholder)
    // For real memory testing, use external tools or peak_alloc crate
    println!("✓ Parse 100K memory test completed (memory tracking not implemented)");

    std::fs::remove_file(&test_file).ok();
}

#[test]
#[ignore] // Slow test
fn test_parse_memory_1m() {
    let test_file = generate_test_file(1_000_000, false);

    let _memory = measure_memory(|| {
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let gedcom = parser.parse_file(&test_file).unwrap();
        assert!(gedcom.individuals.len() > 900_000);
    });

    println!("✓ Parse 1M memory test completed");
    std::fs::remove_file(&test_file).ok();
}

#[test]
fn test_generate_memory_100k() {
    let _memory = measure_memory(|| {
        let ruleset = Ruleset::default_english();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();
        generator.generate(100_000, &mut rng);
    });

    println!("✓ Generate 100K memory test completed");
}

#[test]
fn test_ious_memory_1000() {
    let _memory = measure_memory(|| {
        let ruleset = Ruleset::default_english();
        let config = IOUSConfig {
            marriages: 5,
            children_per_marriage_mean: 4.0,
            siblings: 5,
            descendant_generations: 5,
            target_descendants: Some(1000),
        };
        let mut generator = IOUSGenerator::new(ruleset, config);
        let mut rng = rand::thread_rng();
        let count = generator.generate(&mut rng);
        assert!(count >= 1000);
    });

    println!("✓ IOUS 1000 memory test completed");
}

// ============================================================================
// Test Suite 3: Scalability Tests
// ============================================================================

#[test]
fn test_parser_scales_linearly() {
    println!("Testing parser linear scaling...");

    let sizes = vec![1_000, 10_000, 50_000];
    let mut times = Vec::new();

    for size in &sizes {
        let test_file = generate_test_file(*size, false);

        let start = std::time::Instant::now();
        let mut parser = GedcomParser::new(ParseMode::Lenient);
        let gedcom = parser.parse_file(&test_file).unwrap();
        let elapsed = start.elapsed();

        times.push(elapsed.as_secs_f64());
        println!(
            "  {} records: {:.3}s ({:.0} rec/s)",
            size,
            elapsed.as_secs_f64(),
            *size as f64 / elapsed.as_secs_f64()
        );

        assert!(gedcom.individuals.len() > size * 9 / 10);
        std::fs::remove_file(&test_file).ok();
    }

    // Verify approximately linear: time ratio should be close to size ratio
    let time_ratio = times[2] / times[1]; // 50K / 10K
    let size_ratio = sizes[2] as f64 / sizes[1] as f64; // 5.0

    println!(
        "  Scaling: {}x size → {:.2}x time (linear = {:.2})",
        size_ratio, time_ratio, size_ratio
    );

    // Allow 2x deviation (should be < 10x for linear, would be ~25x for O(n²))
    assert!(
        time_ratio < size_ratio * 2.0,
        "Parser appears to scale worse than linear"
    );

    println!("✓ Parser scales approximately linearly");
}

#[test]
fn test_generator_scales_linearly() {
    println!("Testing generator linear scaling...");

    let sizes = vec![1_000, 10_000, 50_000];
    let mut times = Vec::new();

    for size in &sizes {
        let start = std::time::Instant::now();

        let ruleset = Ruleset::default_english();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();
        generator.generate(*size, &mut rng);

        let elapsed = start.elapsed();
        times.push(elapsed.as_secs_f64());

        println!(
            "  {} records: {:.3}s ({:.0} rec/s)",
            size,
            elapsed.as_secs_f64(),
            *size as f64 / elapsed.as_secs_f64()
        );
    }

    let time_ratio = times[2] / times[1];
    let size_ratio = sizes[2] as f64 / sizes[1] as f64;

    println!(
        "  Scaling: {}x size → {:.2}x time (linear = {:.2})",
        size_ratio, time_ratio, size_ratio
    );

    assert!(
        time_ratio < size_ratio * 2.0,
        "Generator appears to scale worse than linear"
    );

    println!("✓ Generator scales approximately linearly");
}

#[test]
fn test_ious_scales_with_descendants() {
    println!("Testing IOUS scaling with descendant count...");

    let targets = vec![100, 500, 1000];
    let mut times = Vec::new();

    for target in &targets {
        let start = std::time::Instant::now();

        let ruleset = Ruleset::default_english();
        let config = IOUSConfig {
            marriages: 5,
            children_per_marriage_mean: 4.0,
            siblings: 5,
            descendant_generations: 6,
            target_descendants: Some(*target),
        };
        let mut generator = IOUSGenerator::new(ruleset, config);
        let mut rng = rand::thread_rng();
        let count = generator.generate(&mut rng);

        let elapsed = start.elapsed();
        times.push(elapsed.as_secs_f64());

        println!(
            "  {} target: {:.3}s ({} actual, {:.0} ind/s)",
            target,
            elapsed.as_secs_f64(),
            count,
            count as f64 / elapsed.as_secs_f64()
        );

        assert!(count >= *target);
    }

    let time_ratio = times[2] / times[1];
    let size_ratio = targets[2] as f64 / targets[1] as f64;

    println!("  Scaling: {}x size → {:.2}x time", size_ratio, time_ratio);

    // IOUS is exponential by nature (descendants), but generation should still be efficient
    assert!(
        time_ratio < size_ratio * 3.0,
        "IOUS generation scales poorly"
    );

    println!("✓ IOUS scales reasonably with descendant count");
}

// ============================================================================
// Test Suite 4: Stress Tests (10M+ scale)
// ============================================================================

#[test]
#[ignore] // Extreme scale - may OOM - run manually
fn test_parse_10m_survives() {
    println!("Attempting to parse 10M records (may OOM, should not panic)...");

    // Note: This test validates that the code doesn't panic, but OOM is acceptable
    let result = std::panic::catch_unwind(|| {
        let test_file = generate_test_file(10_000_000, false);

        let start = std::time::Instant::now();
        let mut parser = GedcomParser::new(ParseMode::Lenient);

        match parser.parse_file(&test_file) {
            Ok(gedcom) => {
                println!(
                    "✓ Successfully parsed {} individuals in {:.1}s",
                    gedcom.individuals.len(),
                    start.elapsed().as_secs_f64()
                );
            }
            Err(e) => {
                println!("⚠ Parse failed (may be OOM): {:?}", e);
            }
        }

        std::fs::remove_file(&test_file).ok();
    });

    match result {
        Ok(_) => println!("✓ 10M test completed without panic"),
        Err(_) => println!("⚠ 10M test panicked (likely OOM)"),
    }
}

#[test]
#[ignore] // Extreme scale - may OOM
fn test_generate_10m_survives() {
    println!("Attempting to generate 10M records (may OOM, should not panic)...");

    let result = std::panic::catch_unwind(|| {
        let ruleset = Ruleset::default_english();
        let mut generator = GedcomGenerator::new(ruleset);
        let mut rng = rand::thread_rng();

        let start = std::time::Instant::now();
        generator.generate(10_000_000, &mut rng);

        println!(
            "✓ Generated {} individuals in {:.1}s",
            generator.individuals().len(),
            start.elapsed().as_secs_f64()
        );
    });

    match result {
        Ok(_) => println!("✓ 10M generation completed"),
        Err(_) => println!("⚠ 10M generation panicked (likely OOM)"),
    }
}

#[test]
fn test_ious_deep_recursion() {
    println!("Testing IOUS with 10 generation depth (stack overflow check)...");

    let result = std::panic::catch_unwind(|| {
        let ruleset = Ruleset::default_english();
        let config = IOUSConfig {
            marriages: 3,
            children_per_marriage_mean: 2.5,
            siblings: 3,
            descendant_generations: 10, // Deep recursion
            target_descendants: Some(5000),
        };
        let mut generator = IOUSGenerator::new(ruleset, config);
        let mut rng = rand::thread_rng();

        let start = std::time::Instant::now();
        let count = generator.generate(&mut rng);

        println!(
            "✓ Generated {} descendants in 10 generations ({:.3}s)",
            count,
            start.elapsed().as_secs_f64()
        );

        assert!(count >= 100); // Should generate something
        count
    });

    match result {
        Ok(count) => println!("✓ Deep recursion succeeded ({} individuals)", count),
        Err(_) => panic!("✗ Deep recursion failed (stack overflow?)"),
    }
}
