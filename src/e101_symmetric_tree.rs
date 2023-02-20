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

fn recoursive_is_symmetric(
    left: &Option<Rc<RefCell<TreeNode>>>,
    right: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (left, right) {
        (Some(l), Some(r)) => {
            let l = l.borrow();
            let r = r.borrow();

            l.val == r.val
                && recoursive_is_symmetric(&l.left, &r.right)
                && recoursive_is_symmetric(&l.right, &r.left)
        }
        (None, None) => true,
        _ => false,
    }
}

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        Some(r) => recoursive_is_symmetric(&r.borrow().left, &r.borrow().right),
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_none() {
        assert!(is_symmetric(None));
    }

    #[test]
    fn check_height_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        assert!(is_symmetric(root));
    }

    #[test]
    fn check_case_1() {
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

        assert!(!is_symmetric(root));
    }

    #[test]
    fn check_case_2() {
        // create
        //            1
        //   2               2
        //3     None     3       None
        let last = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let second_level = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: last,
            right: None,
        })));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: second_level.clone(),
            right: second_level,
        })));

        assert!(!is_symmetric(root));
    }

    #[test]
    fn check_case_3() {
        // create
        //            1
        //     2               2
        //None     3       3       None
        let last = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let second_level = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: last.clone(),
            right: None,
        })));

        let second_level_mirrored = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: last,
        })));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: second_level_mirrored,
            right: second_level,
        })));

        assert!(is_symmetric(root));
    }

    #[test]
    fn check_case_4() {
        // create
        //            1
        //     2               2
        //None     3       3       None
        let last = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let second_level = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: last.clone(),
            right: None,
        })));

        let second_level_mirrored = Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: last,
        })));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: second_level_mirrored,
            right: second_level,
        })));

        assert!(!is_symmetric(root));
    }
}
