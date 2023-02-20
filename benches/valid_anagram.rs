use criterion::{criterion_group, criterion_main, Criterion};
use leetcode::e242_valid_anagram::is_anagram;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("valid_anagram", |b| {
        b.iter(|| {
            is_anagram("anagram".to_owned(), "nagaram".to_owned());
            is_anagram("rat".to_owned(), "car".to_owned());
            is_anagram("a".to_owned(), "n".to_owned());
            is_anagram("a".to_owned(), "a".to_owned());
            is_anagram("rat".to_owned(), "car".to_owned());
            is_anagram(
                "dfsfaskgfjsdkglhvnoncerwpeuobvjapcneioacNEAWOECN[RUOABAV JSRBFPSBFARWUAFWZ"
                    .to_owned(),
                "dfsfaskgfjsdkglhvnoncerwpeuobvjapcneioacNEAWOECN[RUOABAV JSRBFPSBFARWUAFWn"
                    .to_owned(),
            );
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
