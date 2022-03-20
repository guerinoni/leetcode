use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::search_insert_position::search_insert;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("search_insert_position", |b| {
        b.iter(|| {
            assert_eq!(search_insert(black_box(vec![1, 3, 5, 6]), 0), 0);
            assert_eq!(search_insert(black_box(vec![1, 3, 5, 6]), 5), 2);
            assert_eq!(search_insert(black_box(vec![1, 3, 5, 6]), 7), 4);
            assert_eq!(search_insert(black_box(vec![1, 3, 5, 6]), 2), 1);
            assert_eq!(search_insert(black_box(vec![1, 3]), 4), 2);
            assert_eq!(search_insert(black_box(vec![1, 3, 5]), 4), 2);
            assert_eq!(search_insert(black_box(vec![2, 7, 8, 9, 10]), 9), 3);
            assert_eq!(search_insert(black_box(vec![3, 5, 7, 9, 10]), 8), 3);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
