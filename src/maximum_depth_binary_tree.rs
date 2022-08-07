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

fn recoursive_max_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(root) => {
            let n = root.borrow();
            std::cmp::max(
                recoursive_max_depth(&n.left),
                recoursive_max_depth(&n.right),
            ) + 1
        }
        None => 0,
    }
}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    recoursive_max_depth(&root)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_none() {
        assert_eq!(max_depth(None), 0);
    }

    #[test]
    fn check_height_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        assert_eq!(max_depth(root), 1);
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

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(second_level))),
        })));
        assert_eq!(max_depth(root), 3);
    }

    #[test]
    fn check_height_3_with_2_valid_path() {
        // create
        //          1
        //  None        2
        //          3       None
        let last = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let second_level = TreeNode {
            val: 2,
            left: last.clone(),
            right: last,
        };

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(second_level))),
        })));
        assert_eq!(max_depth(root), 3);
    }
}
