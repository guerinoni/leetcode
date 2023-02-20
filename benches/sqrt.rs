use criterion::{criterion_group, criterion_main, Criterion};
use leetcode::e69_sqrt::my_sqrt;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sqrt", |b| {
        b.iter(|| {
            assert_eq!(my_sqrt(9), 3);
            assert_eq!(my_sqrt(8), 2);
            assert_eq!(my_sqrt(4), 2);
            assert_eq!(my_sqrt(85), 9);
            assert_eq!(my_sqrt(100), 10);
            assert_eq!(my_sqrt(1), 1);
            assert_eq!(my_sqrt(0), 0);
            assert_eq!(my_sqrt(2147483647), 46340);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
