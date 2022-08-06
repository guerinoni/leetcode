use std::cell::RefCell;
use std::rc::Rc;

use criterion::{criterion_group, criterion_main, Criterion};
use leetcode::same_tree::{is_same_tree, TreeNode};

// NOTE: bench here is a bit fake because there is a creation of tree also :)
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("same_tree", |b| {
        b.iter(|| {
            // create
            //          1
            //  None        2
            //          3       None
            let last = Some(Rc::new(RefCell::new(TreeNode::new(3))));
            let second_level = TreeNode {
                val: 2,
                left: last,
                right: None,
            };

            let root = Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(second_level))),
            })));

            assert!(is_same_tree(root.clone(), root));

            // create
            //          1
            //  None        2
            //          3       None
            let last = Some(Rc::new(RefCell::new(TreeNode::new(3))));
            let second_level = TreeNode {
                val: 2,
                left: last,
                right: None,
            };

            let root1 = Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(second_level))),
            })));

            // create
            //          1
            //  None        2
            //          3       0
            let last = Some(Rc::new(RefCell::new(TreeNode::new(3))));
            let diff = Some(Rc::new(RefCell::new(TreeNode::new(0))));
            let second_level = TreeNode {
                val: 2,
                left: last,
                right: diff,
            };
            let root2 = Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(second_level))),
            })));

            assert!(!is_same_tree(root1, root2));

            let root = Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            })));

            assert!(is_same_tree(root.clone(), root));

            let root1 = Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            })));

            let root2 = Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })));

            assert!(!is_same_tree(root1, root2));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
