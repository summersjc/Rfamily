use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rfamily_core::gedcom::{GedcomParser, ParseMode};
use rfamily_core::generator::GedcomGenerator;
use rfamily_core::ruleset::Ruleset;
use std::fs::File;
use std::io::BufWriter;
use std::time::Duration;

/// Helper: Generate a test GEDCOM file with specified number of individuals
fn generate_test_gedcom(individuals: usize) -> String {
    let ruleset = Ruleset::default_english();
    let mut generator = GedcomGenerator::new(ruleset);
    let mut rng = rand::thread_rng();

    generator.generate(individuals, &mut rng);

    let temp_path = format!("/tmp/bench_test_{}.ged", individuals);
    let file = File::create(&temp_path).unwrap();
    let mut writer = BufWriter::new(file);
    generator.write_gedcom(&mut writer).unwrap();
    drop(writer);

    temp_path
}

/// Benchmark: Parse small GEDCOM files (1K individuals)
fn bench_parser_small(c: &mut Criterion) {
    let test_file = generate_test_gedcom(1000);

    let mut group = c.benchmark_group("parser");
    group.throughput(Throughput::Elements(1000));
    group.measurement_time(Duration::from_secs(10));

    group.bench_function(BenchmarkId::new("parse", "1K"), |b| {
        b.iter(|| {
            let mut parser = GedcomParser::new(ParseMode::Lenient);
            let gedcom = parser.parse_file(black_box(&test_file)).unwrap();
            black_box(gedcom);
        });
    });

    group.finish();
    let _ = std::fs::remove_file(&test_file);
}

/// Benchmark: Parse medium GEDCOM files (10K individuals)
fn bench_parser_medium(c: &mut Criterion) {
    let test_file = generate_test_gedcom(10000);

    let mut group = c.benchmark_group("parser");
    group.throughput(Throughput::Elements(10000));
    group.measurement_time(Duration::from_secs(20));
    group.sample_size(10); // Reduce samples for slower benchmarks

    group.bench_function(BenchmarkId::new("parse", "10K"), |b| {
        b.iter(|| {
            let mut parser = GedcomParser::new(ParseMode::Lenient);
            let gedcom = parser.parse_file(black_box(&test_file)).unwrap();
            black_box(gedcom);
        });
    });

    group.finish();
    let _ = std::fs::remove_file(&test_file);
}

/// Benchmark: Parse large GEDCOM files (100K individuals)
fn bench_parser_large(c: &mut Criterion) {
    println!("Generating 100K test file (this may take a minute)...");
    let test_file = generate_test_gedcom(100000);

    let mut group = c.benchmark_group("parser");
    group.throughput(Throughput::Elements(100000));
    group.measurement_time(Duration::from_secs(30));
    group.sample_size(10);

    group.bench_function(BenchmarkId::new("parse", "100K"), |b| {
        b.iter(|| {
            let mut parser = GedcomParser::new(ParseMode::Lenient);
            let gedcom = parser.parse_file(black_box(&test_file)).unwrap();
            black_box(gedcom);
        });
    });

    group.finish();
    let _ = std::fs::remove_file(&test_file);
}

/// Benchmark: Parse extra-large GEDCOM files (1M individuals)
/// Note: This is expensive and slow - only run when needed
#[allow(dead_code)]
fn bench_parser_xlarge(c: &mut Criterion) {
    println!("Generating 1M test file (this will take several minutes)...");
    let test_file = generate_test_gedcom(1_000_000);

    let mut group = c.benchmark_group("parser");
    group.throughput(Throughput::Elements(1_000_000));
    group.measurement_time(Duration::from_secs(60));
    group.sample_size(5);

    group.bench_function(BenchmarkId::new("parse", "1M"), |b| {
        b.iter(|| {
            let mut parser = GedcomParser::new(ParseMode::Lenient);
            let gedcom = parser.parse_file(black_box(&test_file)).unwrap();
            black_box(gedcom);
        });
    });

    group.finish();
    let _ = std::fs::remove_file(&test_file);
}

criterion_group!(
    benches,
    bench_parser_small,
    bench_parser_medium,
    bench_parser_large // bench_parser_xlarge, // Uncomment to test 1M scale
);
criterion_main!(benches);
