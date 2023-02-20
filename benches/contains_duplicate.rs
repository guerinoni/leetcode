use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::e217_contains_duplicate::contains_duplicate;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("contains_duplicate", |b| {
        b.iter(|| {
            assert!(!contains_duplicate(black_box(vec![1, 2, 3, 4])));
            assert!(contains_duplicate(black_box(vec![1, 2, 3, 1])));
            assert!(contains_duplicate(black_box(vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 9
            ])));
            assert!(contains_duplicate(black_box(vec![
                10, 11, 9, 8, 7, 6, 5, 4, 3, 2, 2, 1
            ])));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
