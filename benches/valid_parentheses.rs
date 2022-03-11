use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::valid_parentheses::is_valid;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("valid_parentheses", |b| {
        b.iter(|| {
            assert_eq!(is_valid(black_box("()".to_string())), true);
            assert_eq!(is_valid(black_box("[)".to_string())), false);
            assert_eq!(is_valid(black_box("()[]{}".to_string())), true);
            assert_eq!(is_valid(black_box("".to_string())), true);
            assert_eq!(is_valid(black_box("{[()]}".to_string())), true);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
