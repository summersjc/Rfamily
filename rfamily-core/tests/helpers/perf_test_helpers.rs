/// Performance testing helper utilities
///
/// This module provides utilities for integration performance tests including:
/// - Test file generation
/// - Memory usage measurement
/// - Timing assertions
use rfamily_core::generator::GedcomGenerator;
use rfamily_core::ruleset::Ruleset;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Generate a test GEDCOM file with specified number of individuals
///
/// # Arguments
/// * `individuals` - Number of individuals to generate
/// * `with_families` - If true, generate multi-generational families (slower)
///
/// # Returns
/// PathBuf to the generated temporary file
///
/// # Example
/// ```ignore
/// let test_file = generate_test_file(1000, false);
/// // Use test_file for parsing benchmarks
/// std::fs::remove_file(test_file).unwrap();
/// ```
pub fn generate_test_file(individuals: usize, with_families: bool) -> PathBuf {
    let mut ruleset = Ruleset::default_english();

    if with_families {
        ruleset.relationships.generate_families = true;
        ruleset.relationships.generations = 3;
    }

    let mut generator = GedcomGenerator::new(ruleset);
    let mut rng = rand::thread_rng();

    generator.generate(individuals, &mut rng);

    // Use process ID, thread ID, and random number to ensure unique filenames
    use std::thread;
    let temp_path = format!(
        "/tmp/perf_test_{}_{}_{}_{:?}.ged",
        individuals,
        if with_families { "families" } else { "simple" },
        std::process::id(),
        thread::current().id()
    );

    let file = File::create(&temp_path).expect("Failed to create test file");
    let mut writer = BufWriter::new(file);
    generator
        .write_gedcom(&mut writer)
        .expect("Failed to write GEDCOM");
    writer.flush().expect("Failed to flush writer");
    drop(writer);

    PathBuf::from(temp_path)
}

/// Measure peak memory usage of a closure (approximation)
///
/// Note: This is a simplified memory measurement. For production use,
/// consider using tools like `peak_alloc` crate or OS-level profiling.
///
/// # Arguments
/// * `f` - Closure to execute and measure
///
/// # Returns
/// Estimated memory usage in bytes (currently returns 0 as placeholder)
///
/// # Example
/// ```ignore
/// let memory = measure_memory(|| {
///     let data = vec![0u8; 1_000_000];
///     black_box(data);
/// });
/// assert!(memory > 1_000_000);
/// ```
#[allow(unused_variables)]
pub fn measure_memory<F: FnOnce()>(f: F) -> usize {
    // Simple implementation: execute the function
    // For real memory measurement, use:
    // - peak_alloc crate
    // - Custom allocator with tracking
    // - OS-level profiling (via external tools)

    f();

    // Placeholder return - would need custom allocator or OS queries
    // to get actual memory usage
    0
}

/// Assert that a closure completes within a specified duration
///
/// # Arguments
/// * `f` - Closure to execute
/// * `max_duration` - Maximum allowed execution time
///
/// # Panics
/// Panics if execution takes longer than `max_duration`
///
/// # Example
/// ```ignore
/// assert_completes_within(|| {
///     // Fast operation
///     let sum: u64 = (0..1000).sum();
///     black_box(sum);
/// }, Duration::from_millis(10));
/// ```
pub fn assert_completes_within<F: FnOnce() -> R, R>(f: F, max_duration: Duration) -> R {
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();

    assert!(
        elapsed <= max_duration,
        "Operation took {:?}, expected <= {:?} ({}% over limit)",
        elapsed,
        max_duration,
        ((elapsed.as_secs_f64() / max_duration.as_secs_f64() - 1.0) * 100.0) as i32
    );

    result
}

/// Assert that a closure completes within a duration with percentile tolerance
///
/// This is more forgiving than `assert_completes_within` and accounts for
/// CI environment variability by using a multiplier on the max duration.
///
/// # Arguments
/// * `f` - Closure to execute
/// * `max_duration` - Target duration (will be multiplied by tolerance)
/// * `tolerance_multiplier` - Multiplier for CI variability (e.g., 1.5 = 50% margin)
///
/// # Panics
/// Panics if execution takes longer than `max_duration * tolerance_multiplier`
///
/// # Example
/// ```ignore
/// // Allow 50% margin for CI variability
/// assert_completes_within_tolerance(|| {
///     generate_100k_records();
/// }, Duration::from_secs(10), 1.5);
/// ```
pub fn assert_completes_within_tolerance<F: FnOnce() -> R, R>(
    f: F,
    max_duration: Duration,
    tolerance_multiplier: f64,
) -> R {
    let adjusted_max = Duration::from_secs_f64(max_duration.as_secs_f64() * tolerance_multiplier);

    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();

    assert!(
        elapsed <= adjusted_max,
        "Operation took {:?}, expected <= {:?} (target: {:?} + {}% tolerance)",
        elapsed,
        adjusted_max,
        max_duration,
        ((tolerance_multiplier - 1.0) * 100.0) as i32
    );

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_generate_test_file_simple() {
        let path = generate_test_file(10, false);
        assert!(path.exists());

        // Verify file has content
        let metadata = std::fs::metadata(&path).unwrap();
        assert!(metadata.len() > 0);

        // Cleanup
        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_generate_test_file_with_families() {
        let path = generate_test_file(10, true);
        assert!(path.exists());

        // File with families should be larger
        let metadata = std::fs::metadata(&path).unwrap();
        assert!(metadata.len() > 500); // Very small families still produce some FAM records

        // Cleanup
        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_assert_completes_within_success() {
        // Should pass - operation is fast
        assert_completes_within(
            || {
                let sum: u64 = (0..1000).sum();
                let _ = sum;
            },
            Duration::from_millis(100),
        );
    }

    #[test]
    #[should_panic(expected = "Operation took")]
    fn test_assert_completes_within_failure() {
        // Should fail - operation takes 100ms but limit is 10ms
        assert_completes_within(
            || {
                thread::sleep(Duration::from_millis(100));
            },
            Duration::from_millis(10),
        );
    }

    #[test]
    fn test_assert_completes_within_tolerance_success() {
        // Should pass with tolerance
        assert_completes_within_tolerance(
            || {
                thread::sleep(Duration::from_millis(50));
            },
            Duration::from_millis(40),
            1.5,
        ); // 40ms * 1.5 = 60ms limit
    }

    #[test]
    #[should_panic(expected = "Operation took")]
    fn test_assert_completes_within_tolerance_failure() {
        // Should fail even with tolerance
        assert_completes_within_tolerance(
            || {
                thread::sleep(Duration::from_millis(100));
            },
            Duration::from_millis(40),
            1.5,
        ); // 40ms * 1.5 = 60ms limit < 100ms
    }

    #[test]
    fn test_measure_memory_placeholder() {
        // Currently returns 0 (placeholder implementation)
        let memory = measure_memory(|| {
            let _data = vec![0u8; 1000];
        });

        // Just verify it doesn't crash
        assert_eq!(memory, 0); // Placeholder returns 0
    }
}
