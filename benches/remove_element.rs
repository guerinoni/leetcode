use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::e27_remove_element::remove_element;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("remove_element", |b| {
        b.iter(|| {
            assert_eq!(remove_element(black_box(&mut vec![3, 2, 2, 3]), 3), 2);
            assert_eq!(
                remove_element(black_box(&mut vec![0, 1, 2, 2, 3, 0, 4, 2]), 2),
                5
            );
            assert_eq!(remove_element(black_box(&mut vec![]), 0), 0);
            assert_eq!(
                remove_element(black_box(&mut vec![0, 0, 0, 0, 0, 0, 0, 0]), 0),
                0
            );
            assert_eq!(
                remove_element(black_box(&mut vec![0, 0, 0, 0, 0, 0, 0, 0]), 1),
                8
            );
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
