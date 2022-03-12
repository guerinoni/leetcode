use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::two_sum::two_sum;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("two_sum", |b| {
        b.iter(|| {
            two_sum(black_box(vec![2, 7, 11, 15]), 9);
            two_sum(black_box(vec![3, 2, 4]), 6);
            two_sum(black_box(vec![3, 3]), 6);
            two_sum(
                black_box(vec![
                    28, 1, 24, 2, 7, 18, 10, 27, 11, 9, 8, 3, 12, 5, 29, 30,
                ]),
                59,
            );
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
