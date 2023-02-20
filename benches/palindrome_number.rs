use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::e9_palindrome_number::is_palindrome;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("palindrome_number", |b| {
        b.iter(|| {
            assert!(is_palindrome(black_box(121)));
            assert!(is_palindrome(black_box(9)));
            assert!(!is_palindrome(black_box(12)));
            assert!(!is_palindrome(black_box(-121)));
            assert!(!is_palindrome(black_box(12345678)));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
