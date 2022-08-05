use std::cell::RefCell;
use std::rc::Rc;

use criterion::{criterion_group, criterion_main, Criterion};
use leetcode::binary_tree_inorder_traversal::{inorder_traversal, TreeNode};

// NOTE: bench here is a bit fake because there is a creation of tree also :)
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("binary_tree_inorder_traversal", |b| {
        b.iter(|| {
            assert_eq!(inorder_traversal(None), Vec::<i32>::new());

            let tree = Some(Rc::new(RefCell::new(TreeNode::new(1))));
            assert_eq!(inorder_traversal(tree), vec![1]);

            let last = Some(Rc::new(RefCell::new(TreeNode::new(3))));
            let second_level = TreeNode {
                val: 2,
                left: last,
                right: None,
            };

            let root = TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(second_level))),
            };

            assert_eq!(
                inorder_traversal(Some(Rc::new(RefCell::new(root)))),
                vec![1, 3, 2]
            );
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
