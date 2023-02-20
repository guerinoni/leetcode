use criterion::{criterion_group, criterion_main, Criterion};
use leetcode::e88_merge_sorted_array::merge;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("merge_sorted_array", |b| {
        b.iter(|| {
            let mut nums1 = vec![1, 2, 3, 0, 0, 0];
            let mut nums2 = vec![2, 5, 6];
            let expected = vec![1, 2, 2, 3, 5, 6];
            merge(&mut nums1, 3, &mut nums2, 3);
            assert_eq!(expected, nums1);

            let mut nums1 = vec![1];
            let mut nums2 = vec![];
            let expected = vec![1];
            merge(&mut nums1, 1, &mut nums2, 0);
            assert_eq!(expected, nums1);

            let mut nums1 = vec![0];
            let mut nums2 = vec![1];
            let expected = vec![1];
            merge(&mut nums1, 0, &mut nums2, 1);
            assert_eq!(expected, nums1);

            let mut nums1 = vec![4, 5, 6, 0, 0, 0];
            let mut nums2 = vec![1, 2, 3];
            let expected = vec![1, 2, 3, 4, 5, 6];
            merge(&mut nums1, 3, &mut nums2, 3);
            assert_eq!(expected, nums1);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
