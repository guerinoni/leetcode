use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::remove_duplicates_from_sorted_array::remove_duplicates;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("remove_duplicates_from_sorted_array", |b| {
        b.iter(|| {
            assert_eq!(remove_duplicates(black_box(&mut vec![1, 1, 2])), 2);
            assert_eq!(
                remove_duplicates(black_box(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4])),
                5
            );

            assert_eq!(
                remove_duplicates(black_box(&mut vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0])),
                1
            );

            assert_eq!(
                remove_duplicates(black_box(&mut vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])),
                10
            );
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
