use bst_bench::Bst;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

fn bench_100k_random_lookup_hits(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(23254452323);
    let mut keys: Vec<i64> = (0..100_000).map(|_| rng.gen::<i64>()).collect();

    let mut bst_set = Bst::default();
    for &x in keys.iter() {
        bst_set.insert(x);
    }

    keys.shuffle(&mut rng);

    let mut group = c.benchmark_group("100k_random_lookup_hits");

    group.bench_function("contains_if_else", |b| {
        b.iter(|| {
            black_box({
                let mut r = false;
                for &x in keys.iter() {
                    r ^= bst_set.contains_if_else(x);
                }
                r
            })
        })
    });

    group.bench_function("contains_match", |b| {
        b.iter(|| {
            black_box({
                let mut r = false;
                for &x in keys.iter() {
                    r ^= bst_set.contains_match(x);
                }
                r
            })
        })
    });
}

fn bench_100k_random_lookup_misses(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(5430426233246);

    let mut bst_set = Bst::default();
    for _ in 0..100_000 {
        bst_set.insert(rng.gen::<i64>() & !1);
    }

    let lookups: Vec<i64> = (0..100_000).map(|_| rng.gen::<i64>() | 1).collect();

    let mut group = c.benchmark_group("100k_random_lookup_misses");

    group.bench_function("contains_if_else", |b| {
        b.iter(|| {
            black_box({
                let mut r = false;
                for &x in lookups.iter() {
                    r ^= bst_set.contains_if_else(x);
                }
                r
            })
        })
    });

    group.bench_function("contains_match", |b| {
        b.iter(|| {
            black_box({
                let mut r = false;
                for &x in lookups.iter() {
                    r ^= bst_set.contains_match(x);
                }
                r
            })
        })
    });
}

criterion_group!(
    benches,
    bench_100k_random_lookup_hits,
    bench_100k_random_lookup_misses,
);

criterion_main!(benches);
