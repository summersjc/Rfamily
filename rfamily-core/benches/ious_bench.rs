use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rfamily_core::generators::ious::{IOUSConfig, IOUSGenerator};
use rfamily_core::ruleset::Ruleset;
use std::time::Duration;

/// Benchmark: IOUS minimal (1 marriage, 1 generation, baseline)
fn bench_ious_minimal(c: &mut Criterion) {
    let mut group = c.benchmark_group("ious");
    group.measurement_time(Duration::from_secs(5));

    group.bench_function(BenchmarkId::new("generate", "minimal"), |b| {
        b.iter(|| {
            let ruleset = Ruleset::default_english();
            let config = IOUSConfig {
                marriages: 1,
                children_per_marriage_mean: 1.0,
                siblings: 0,
                descendant_generations: 1,
                target_descendants: None,
            };

            let mut generator = IOUSGenerator::new(ruleset, config);
            let mut rng = rand::thread_rng();

            let count = generator.generate(&mut rng);
            black_box(count);
        });
    });

    group.finish();
}

/// Benchmark: IOUS typical (3 marriages, 4 generations, 200 target)
fn bench_ious_typical(c: &mut Criterion) {
    let mut group = c.benchmark_group("ious");
    group.throughput(Throughput::Elements(200));
    group.measurement_time(Duration::from_secs(10));

    group.bench_function(BenchmarkId::new("generate", "typical"), |b| {
        b.iter(|| {
            let ruleset = Ruleset::default_english();
            let config = IOUSConfig {
                marriages: 3,
                children_per_marriage_mean: 4.0,
                siblings: 5,
                descendant_generations: 4,
                target_descendants: Some(200),
            };

            let mut generator = IOUSGenerator::new(ruleset, config);
            let mut rng = rand::thread_rng();

            let count = generator.generate(&mut rng);
            black_box(count);
        });
    });

    group.finish();
}

/// Benchmark: IOUS large (5 marriages, 5 generations, 1000 target)
fn bench_ious_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("ious");
    group.throughput(Throughput::Elements(1000));
    group.measurement_time(Duration::from_secs(15));
    group.sample_size(10);

    group.bench_function(BenchmarkId::new("generate", "large"), |b| {
        b.iter(|| {
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
            black_box(count);
        });
    });

    group.finish();
}

/// Benchmark: IOUS extra-large (5 marriages, 6 generations, 5000 target)
fn bench_ious_xlarge(c: &mut Criterion) {
    let mut group = c.benchmark_group("ious");
    group.throughput(Throughput::Elements(5000));
    group.measurement_time(Duration::from_secs(20));
    group.sample_size(10);

    group.bench_function(BenchmarkId::new("generate", "xlarge"), |b| {
        b.iter(|| {
            let ruleset = Ruleset::default_english();
            let config = IOUSConfig {
                marriages: 5,
                children_per_marriage_mean: 5.0,
                siblings: 10,
                descendant_generations: 6,
                target_descendants: Some(5000),
            };

            let mut generator = IOUSGenerator::new(ruleset, config);
            let mut rng = rand::thread_rng();

            let count = generator.generate(&mut rng);
            black_box(count);
        });
    });

    group.finish();
}

/// Benchmark: IOUS with different marriage counts
fn bench_ious_marriage_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("ious_marriages");
    group.measurement_time(Duration::from_secs(10));

    for marriages in [1, 3, 5, 7, 10].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(marriages),
            marriages,
            |b, &marriages| {
                b.iter(|| {
                    let ruleset = Ruleset::default_english();
                    let config = IOUSConfig {
                        marriages,
                        children_per_marriage_mean: 3.0,
                        siblings: 3,
                        descendant_generations: 3,
                        target_descendants: Some(500),
                    };

                    let mut generator = IOUSGenerator::new(ruleset, config);
                    let mut rng = rand::thread_rng();

                    let count = generator.generate(&mut rng);
                    black_box(count);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark: IOUS with different generation depths
fn bench_ious_generation_depth(c: &mut Criterion) {
    let mut group = c.benchmark_group("ious_generations");
    group.measurement_time(Duration::from_secs(10));

    for generations in [2, 4, 6, 8].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(generations),
            generations,
            |b, &generations| {
                b.iter(|| {
                    let ruleset = Ruleset::default_english();
                    let config = IOUSConfig {
                        marriages: 3,
                        children_per_marriage_mean: 2.5,
                        siblings: 3,
                        descendant_generations: generations,
                        target_descendants: Some(1000),
                    };

                    let mut generator = IOUSGenerator::new(ruleset, config);
                    let mut rng = rand::thread_rng();

                    let count = generator.generate(&mut rng);
                    black_box(count);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_ious_minimal,
    bench_ious_typical,
    bench_ious_large,
    bench_ious_xlarge,
    bench_ious_marriage_scaling,
    bench_ious_generation_depth
);
criterion_main!(benches);
