use criterion::{criterion_group, criterion_main, Criterion};
use leetcode::reverse_bits::reverse_bits;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("reverse_bits", |b| {
        b.iter(|| {
            let output = reverse_bits(43261596);
            assert_eq!(964176192, output);

            let output = reverse_bits(4294967293);
            assert_eq!(3221225471, output);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
