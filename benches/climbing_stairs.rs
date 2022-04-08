use criterion::{criterion_group, criterion_main, Criterion};
use leetcode::climbing_stairs::climb_stairs;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("climbing_stairs", |b| {
        b.iter(|| {
            assert_eq!(climb_stairs(2), 2);
            assert_eq!(climb_stairs(3), 3);
            assert_eq!(climb_stairs(4), 5);
            assert_eq!(climb_stairs(5), 8);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
