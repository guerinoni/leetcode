use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::e14_longest_common_prefix::longest_common_prefix;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("longest_common_prefix", |b| {
        b.iter(|| {
            assert_eq!(longest_common_prefix(black_box(vec![])), "".to_string());
            assert_eq!(
                longest_common_prefix(black_box(vec!["ab".to_string(), "a".to_string()])),
                "a".to_string()
            );
            assert_eq!(
                longest_common_prefix(black_box(vec![
                    "flower".to_string(),
                    "flow".to_string(),
                    "flight".to_string()
                ])),
                "fl".to_string()
            );

            assert_eq!(
                longest_common_prefix(black_box(vec![
                    "asd".to_string(),
                    "asd".to_string(),
                    "asd".to_string()
                ])),
                "asd".to_string()
            );

            assert_eq!(
                longest_common_prefix(black_box(vec![
                    "dog".to_string(),
                    "racecar".to_string(),
                    "car".to_string()
                ])),
                "".to_string()
            );
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
