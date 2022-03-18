use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::implement_strstr::str_str;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("implement_strstr", |b| {
        b.iter(|| {
            assert_eq!(str_str(black_box("".to_string()), "".to_string()), 0);
            assert_eq!(
                str_str(black_box("aaaaa".to_string()), "bba".to_string()),
                -1
            );
            assert_eq!(str_str(black_box("hello".to_string()), "ll".to_string()), 2);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
