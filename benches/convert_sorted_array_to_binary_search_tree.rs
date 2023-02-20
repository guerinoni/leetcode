use std::{cell::RefCell, rc::Rc};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::{
    e108_convert_sorted_array_to_binary_search_tree::sorted_array_to_bst,
    e108_convert_sorted_array_to_binary_search_tree::TreeNode,
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("convert_sorted_array_to_binary_search_tree", |b| {
        b.iter(|| {
            let expected = Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: -10,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
            })));

            let output = sorted_array_to_bst(black_box(vec![-10, -3, 0, 5, 9]));
            assert_eq!(expected, output);

            let expected = Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: None,
            })));

            let output = sorted_array_to_bst(black_box(vec![1, 3]));
            assert_eq!(expected, output);

            let expected = Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -10,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: -20,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: -3,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 9,
                        left: None,
                        right: None,
                    }))),
                }))),
            })));

            let output = sorted_array_to_bst(black_box(vec![-20, -10, -3, 0, 1, 5, 9]));
            assert_eq!(expected, output);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
