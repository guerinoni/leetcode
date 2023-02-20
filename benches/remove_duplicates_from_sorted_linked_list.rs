use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::e21_merge_two_sorted_list::{compare, create_list_node};
use leetcode::e83_remove_duplicates_from_sorted_linked_list::delete_duplicates;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("remove_duplicates_from_sorted_linked_list", |b| {
        b.iter(|| {
            let input = create_list_node(vec![1, 1, 2], 0);
            let output = delete_duplicates(black_box(input));
            let expected = create_list_node(vec![1, 2], 0);
            assert!(compare(output, expected));

            let input = create_list_node(vec![1, 1, 2, 3, 3], 0);
            let output = delete_duplicates(black_box(input));
            let expected = create_list_node(vec![1, 2, 3], 0);
            assert!(compare(output, expected));

            let input = create_list_node(vec![1, 1, 1, 2, 2, 3, 3, 4, 5, 6, 7, 99, 99, 99, 99], 0);
            let output = delete_duplicates(input);
            let expected = create_list_node(vec![1, 2, 3, 4, 5, 6, 7, 99], 0);
            assert!(compare(output, expected));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
