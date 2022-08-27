use criterion::{criterion_group, criterion_main, Criterion};
use leetcode::pascal_triangle_two::get_row;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("pascal_triangle_two", |b| {
        b.iter(|| {
            let output = get_row(0);
            let expected = vec![1];
            assert_eq!(output, expected);

            let output = get_row(1);
            let expected = vec![1, 1];
            assert_eq!(output, expected);

            let output = get_row(2);
            let expected = vec![1, 2, 1];
            assert_eq!(output, expected);

            let output = get_row(3);
            let expected = vec![1, 3, 3, 1];
            assert_eq!(output, expected);

            let output = get_row(4);
            let expected = vec![1, 4, 6, 4, 1];
            assert_eq!(output, expected);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
