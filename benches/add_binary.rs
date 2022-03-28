use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::add_binary::add_binary;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("add_binary", |b| b.iter(|| {

        assert_eq!(
            add_binary(black_box("11".to_string()), black_box("1".to_string())),
            "100".to_string()
        );
        assert_eq!(
            add_binary(black_box("1010".to_string()), black_box("1011".to_string())),
            "10101".to_string()
        );

        let a = "10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101".to_string();
        let b = "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011".to_string();
        let expected = "110111101100010011000101110110100000011101000101011001000011011000001100011110011010010011000000000".to_string();
        assert_eq!(add_binary(black_box(a), black_box(b)), expected);

    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
