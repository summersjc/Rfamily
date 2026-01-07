use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rfamily_core::generator::GedcomGenerator;
use rfamily_core::ruleset::Ruleset;
use std::fs::File;
use std::io::BufWriter;
use std::time::Duration;

/// Benchmark: Generate small GEDCOM (1K individuals, no families)
fn bench_generate_simple_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("generator_simple");
    group.throughput(Throughput::Elements(1000));
    group.measurement_time(Duration::from_secs(10));

    group.bench_function(BenchmarkId::new("generate", "1K_simple"), |b| {
        b.iter(|| {
            let ruleset = Ruleset::default_english();
            let mut generator = GedcomGenerator::new(ruleset);
            let mut rng = rand::thread_rng();

            generator.generate(black_box(1000), &mut rng);
            black_box(&generator);
        });
    });

    group.finish();
}

/// Benchmark: Generate medium GEDCOM (10K individuals, no families)
fn bench_generate_simple_medium(c: &mut Criterion) {
    let mut group = c.benchmark_group("generator_simple");
    group.throughput(Throughput::Elements(10000));
    group.measurement_time(Duration::from_secs(20));
    group.sample_size(10);

    group.bench_function(BenchmarkId::new("generate", "10K_simple"), |b| {
        b.iter(|| {
            let ruleset = Ruleset::default_english();
            let mut generator = GedcomGenerator::new(ruleset);
            let mut rng = rand::thread_rng();

            generator.generate(black_box(10000), &mut rng);
            black_box(&generator);
        });
    });

    group.finish();
}

/// Benchmark: Generate large GEDCOM (100K individuals, no families)
fn bench_generate_simple_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("generator_simple");
    group.throughput(Throughput::Elements(100000));
    group.measurement_time(Duration::from_secs(30));
    group.sample_size(10);

    group.bench_function(BenchmarkId::new("generate", "100K_simple"), |b| {
        b.iter(|| {
            let ruleset = Ruleset::default_english();
            let mut generator = GedcomGenerator::new(ruleset);
            let mut rng = rand::thread_rng();

            generator.generate(black_box(100000), &mut rng);
            black_box(&generator);
        });
    });

    group.finish();
}

/// Benchmark: Generate with families (1K individuals, 3 generations)
fn bench_generate_families_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("generator_families");
    group.throughput(Throughput::Elements(1000));
    group.measurement_time(Duration::from_secs(15));

    group.bench_function(BenchmarkId::new("generate", "1K_families"), |b| {
        b.iter(|| {
            let mut ruleset = Ruleset::default_english();
            ruleset.relationships.generate_families = true;
            ruleset.relationships.generations = 3;

            let mut generator = GedcomGenerator::new(ruleset);
            let mut rng = rand::thread_rng();

            generator.generate(black_box(1000), &mut rng);
            black_box(&generator);
        });
    });

    group.finish();
}

/// Benchmark: Generate with families (10K individuals, 3 generations)
fn bench_generate_families_medium(c: &mut Criterion) {
    let mut group = c.benchmark_group("generator_families");
    group.throughput(Throughput::Elements(10000));
    group.measurement_time(Duration::from_secs(30));
    group.sample_size(10);

    group.bench_function(BenchmarkId::new("generate", "10K_families"), |b| {
        b.iter(|| {
            let mut ruleset = Ruleset::default_english();
            ruleset.relationships.generate_families = true;
            ruleset.relationships.generations = 3;

            let mut generator = GedcomGenerator::new(ruleset);
            let mut rng = rand::thread_rng();

            generator.generate(black_box(10000), &mut rng);
            black_box(&generator);
        });
    });

    group.finish();
}

/// Benchmark: GEDCOM file writing (1K individuals)
fn bench_write_gedcom_small(c: &mut Criterion) {
    // Pre-generate the data
    let ruleset = Ruleset::default_english();
    let mut generator = GedcomGenerator::new(ruleset);
    let mut rng = rand::thread_rng();
    generator.generate(1000, &mut rng);

    c.bench_function("write_gedcom_1K", |b| {
        b.iter(|| {
            let temp_path = "/tmp/bench_write_test.ged";
            let file = File::create(temp_path).unwrap();
            let mut writer = BufWriter::new(file);
            generator.write_gedcom(&mut writer).unwrap();
            drop(writer);
            let _ = std::fs::remove_file(temp_path);
        });
    });
}

/// Benchmark: GEDCOM file writing (10K individuals)
fn bench_write_gedcom_medium(c: &mut Criterion) {
    println!("Generating 10K individuals for write benchmark...");
    let ruleset = Ruleset::default_english();
    let mut generator = GedcomGenerator::new(ruleset);
    let mut rng = rand::thread_rng();
    generator.generate(10000, &mut rng);

    let mut group = c.benchmark_group("write");
    group.throughput(Throughput::Elements(10000));
    group.measurement_time(Duration::from_secs(15));

    group.bench_function(BenchmarkId::new("write_gedcom", "10K"), |b| {
        b.iter(|| {
            let temp_path = "/tmp/bench_write_test_10k.ged";
            let file = File::create(temp_path).unwrap();
            let mut writer = BufWriter::new(file);
            generator.write_gedcom(&mut writer).unwrap();
            drop(writer);
            let _ = std::fs::remove_file(temp_path);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_generate_simple_small,
    bench_generate_simple_medium,
    bench_generate_simple_large,
    bench_generate_families_small,
    bench_generate_families_medium,
    bench_write_gedcom_small,
    bench_write_gedcom_medium
);
criterion_main!(benches);
