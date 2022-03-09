use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::roman_to_integer::roman_to_int;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("roman_to_integer", |b| {
        b.iter(|| {
            assert_eq!(roman_to_int(black_box("III".to_string())), 3);
            assert_eq!(roman_to_int(black_box("IV".to_string())), 4);
            assert_eq!(roman_to_int(black_box("LVIII".to_string())), 58);
            assert_eq!(roman_to_int(black_box("MCMXCIV".to_string())), 1994);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
