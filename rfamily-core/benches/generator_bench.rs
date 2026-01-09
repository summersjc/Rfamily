use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rfamily_core::compression::OutputWriter;
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

/// Benchmark: Streaming generation (10K individuals)
fn bench_streaming_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("streaming");
    group.throughput(Throughput::Elements(10000));
    group.measurement_time(Duration::from_secs(20));
    group.sample_size(10);

    group.bench_function(BenchmarkId::new("streaming", "10K"), |b| {
        b.iter(|| {
            let ruleset = Ruleset::default_english();
            let mut generator = GedcomGenerator::new(ruleset);

            let temp_path = "/tmp/bench_streaming.ged";
            let file = File::create(temp_path).unwrap();
            let mut writer = BufWriter::new(file);

            generator
                .generate_streaming(black_box(10000), &mut writer, |_| {})
                .unwrap();
            drop(writer);
            let _ = std::fs::remove_file(temp_path);
        });
    });

    group.finish();
}

/// Benchmark: Compression overhead (10K individuals)
fn bench_compression_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("compression");
    group.throughput(Throughput::Elements(10000));
    group.measurement_time(Duration::from_secs(20));
    group.sample_size(10);

    // Pre-generate data
    let ruleset = Ruleset::default_english();
    let mut generator = GedcomGenerator::new(ruleset);
    let mut rng = rand::thread_rng();
    generator.generate(10000, &mut rng);

    // Benchmark plain write
    group.bench_function(BenchmarkId::new("write", "plain"), |b| {
        b.iter(|| {
            let temp_path = "/tmp/bench_plain.ged";
            let mut writer = OutputWriter::create(temp_path, false).unwrap();
            generator.write_gedcom(&mut writer).unwrap();
            writer.finish().unwrap();
            let _ = std::fs::remove_file(temp_path);
        });
    });

    // Benchmark compressed write
    group.bench_function(BenchmarkId::new("write", "compressed"), |b| {
        b.iter(|| {
            let temp_path = "/tmp/bench_compressed.ged.gz";
            let mut writer = OutputWriter::create(temp_path, true).unwrap();
            generator.write_gedcom(&mut writer).unwrap();
            writer.finish().unwrap();
            let _ = std::fs::remove_file(temp_path);
        });
    });

    group.finish();
}

/// Benchmark: Parallel vs Sequential generation speedup
fn bench_parallel_speedup(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_speedup");
    group.throughput(Throughput::Elements(50000));
    group.measurement_time(Duration::from_secs(30));
    group.sample_size(10);

    // Note: The generate() method automatically uses parallel generation for simple mode
    // This benchmark shows the actual performance with parallel enabled by default
    group.bench_function(BenchmarkId::new("generate", "50K_parallel"), |b| {
        b.iter(|| {
            let mut ruleset = Ruleset::default_english();
            ruleset.relationships.generate_families = false; // Simple mode for parallelization
            let mut generator = GedcomGenerator::new(ruleset);
            let mut rng = rand::thread_rng();

            generator.generate(black_box(50000), &mut rng);
            black_box(&generator);
        });
    });

    group.finish();
}

/// Benchmark: Streaming vs Traditional memory usage (100K individuals)
fn bench_streaming_vs_traditional(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_modes");
    group.throughput(Throughput::Elements(100000));
    group.measurement_time(Duration::from_secs(40));
    group.sample_size(10);

    // Traditional: generate all, then write
    group.bench_function(BenchmarkId::new("mode", "traditional"), |b| {
        b.iter(|| {
            let mut ruleset = Ruleset::default_english();
            ruleset.relationships.generate_families = false;
            let mut generator = GedcomGenerator::new(ruleset);
            let mut rng = rand::thread_rng();

            generator.generate(black_box(100000), &mut rng);

            let temp_path = "/tmp/bench_traditional.ged";
            let file = File::create(temp_path).unwrap();
            let mut writer = BufWriter::new(file);
            generator.write_gedcom(&mut writer).unwrap();
            drop(writer);
            let _ = std::fs::remove_file(temp_path);
        });
    });

    // Streaming: generate and write in batches
    group.bench_function(BenchmarkId::new("mode", "streaming"), |b| {
        b.iter(|| {
            let mut ruleset = Ruleset::default_english();
            ruleset.relationships.generate_families = false;
            let mut generator = GedcomGenerator::new(ruleset);

            let temp_path = "/tmp/bench_streaming.ged";
            let file = File::create(temp_path).unwrap();
            let mut writer = BufWriter::new(file);

            generator
                .generate_streaming(black_box(100000), &mut writer, |_| {})
                .unwrap();
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
    bench_write_gedcom_medium,
    bench_streaming_generation,
    bench_compression_overhead,
    bench_parallel_speedup,
    bench_streaming_vs_traditional
);
criterion_main!(benches);
