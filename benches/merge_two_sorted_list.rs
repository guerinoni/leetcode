use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::e21_merge_two_sorted_list::{create_list_node, merge_two_lists};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("merge_two_sorted_list", |b| {
        b.iter(|| {
            assert_eq!(merge_two_lists(None, None), None);

            let list = create_list_node(vec![1, 2, 4], 0);
            assert_eq!(merge_two_lists(black_box(list.clone()), None), list.clone());
            assert_eq!(merge_two_lists(None, black_box(list.clone())), list);

            let list_1 = create_list_node(vec![1, 2, 4], 0);
            let list_2 = create_list_node(vec![1, 3, 4], 0);
            let expected = create_list_node(vec![1, 1, 2, 3, 4, 4], 0);
            assert_eq!(
                merge_two_lists(black_box(list_1), black_box(list_2)),
                expected
            );
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
