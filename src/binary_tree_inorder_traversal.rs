use std::cell::RefCell;
use std::rc::Rc;

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

fn recursive_inorder_traversal(root: &Rc<RefCell<TreeNode>>) -> Vec<i32> {
    let mut result = vec![];

    if let Some(ref left) = root.borrow().left {
        let v = recursive_inorder_traversal(left);
        result.extend(v);
    }

    result.push(root.borrow().val);

    if let Some(ref right) = root.borrow().right {
        let v = recursive_inorder_traversal(right);
        result.extend(v);
    }

    result
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }

    recursive_inorder_traversal(&root.unwrap())
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    #[test]
    fn check_simple() {
        assert_eq!(inorder_traversal(None), vec![]);

        let tree = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(inorder_traversal(tree), vec![1]);
    }

    #[test]
    fn check_height_3() {
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

        let root = TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(second_level))),
        };

        assert_eq!(
            inorder_traversal(Some(Rc::new(RefCell::new(root)))),
            vec![1, 3, 2]
        );
    }
}
