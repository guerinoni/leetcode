use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::e58_length_last_word::length_of_last_word;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("length_last_word", |b| {
        b.iter(|| {
            assert_eq!(length_of_last_word(black_box("Hello World".to_string())), 5);
            assert_eq!(
                length_of_last_word(black_box("   fly me   to   the moon  ").to_string()),
                4
            );
            assert_eq!(
                length_of_last_word(black_box("luffy is still joyboy".to_string())),
                6
            );
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
