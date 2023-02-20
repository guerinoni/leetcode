use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn recursive_preorder_traversal(root: &Rc<RefCell<TreeNode>>) -> Vec<i32> {
    let mut v = vec![];
    v.push(root.borrow().val);

    if let Some(ref left) = root.borrow().left {
        v.extend(recursive_preorder_traversal(left));
    }

    if let Some(ref right) = root.borrow().right {
        v.extend(recursive_preorder_traversal(right));
    }

    v
}

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    match root {
        Some(ref r) => recursive_preorder_traversal(r),
        None => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let tree = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(preorder_traversal(tree), vec![1]);

        // create
        //          1
        //  None        2
        //          3       None
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
        })));
        assert_eq!(preorder_traversal(tree), vec![1, 2, 3]);
    }
}
