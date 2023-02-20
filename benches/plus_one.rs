use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::e66_plus_one::plus_one;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("plus_one", |b| {
        b.iter(|| {
            assert_eq!(plus_one(black_box(vec![1, 2, 3])), vec![1, 2, 4]);
            assert_eq!(plus_one(black_box(vec![1, 2, 9])), vec![1, 3, 0]);
            assert_eq!(plus_one(black_box(vec![4, 3, 2, 1])), vec![4, 3, 2, 2]);
            assert_eq!(plus_one(black_box(vec![9])), vec![1, 0]);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
