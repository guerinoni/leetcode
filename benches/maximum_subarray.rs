use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::e53_maximum_subarray::max_sub_array;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("maximum_subarray", |b| {
        b.iter(|| {
            assert_eq!(max_sub_array(vec![]), 0);
            assert_eq!(max_sub_array(black_box(vec![1])), 1);
            assert_eq!(
                max_sub_array(black_box(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])),
                6
            );
            assert_eq!(max_sub_array(black_box(vec![5, 4, -1, 7, 8])), 23);
            assert_eq!(max_sub_array(black_box(vec![0, 0, 0, 0, 0, 0, 0, 0])), 0);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
