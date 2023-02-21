use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

fn recoursive_min_depth(root: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
    match root {
        Some(r) => {
            let r = r.borrow();
            match (&r.left, &r.right) {
                (Some(l), Some(r)) => std::cmp::min(
                    recoursive_min_depth(Some(l.clone()), depth + 1),
                    recoursive_min_depth(Some(r.clone()), depth + 1),
                ),
                _ => std::cmp::max(
                    recoursive_min_depth(r.left.clone(), depth + 1),
                    recoursive_min_depth(r.right.clone(), depth + 1),
                ),
            }
        }
        None => depth,
    }
}

pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    recoursive_min_depth(root, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_root() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));

        assert_eq!(min_depth(root), 1);
    }

    #[test]
    fn depth_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        })));

        assert_eq!(min_depth(root), 2);
    }
}
