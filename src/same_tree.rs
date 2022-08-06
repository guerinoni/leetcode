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

// NOTE: first try.
// fn recoursive_equality(p: &Rc<RefCell<TreeNode>>, q: &Rc<RefCell<TreeNode>>) -> bool {
//     if p.borrow().left.is_none() && q.borrow().left.is_some()
//         || p.borrow().left.is_some() && q.borrow().left.is_none()
//         || p.borrow().right.is_some() && q.borrow().right.is_none()
//         || p.borrow().right.is_none() && q.borrow().right.is_some()
//     {
//         return false;
//     }

//     if let Some(ref p_left) = p.borrow().left {
//         if let Some(ref q_left) = q.borrow().left {
//             if !recoursive_equality(p_left, q_left) {
//                 return false;
//             }
//         }
//     }

//     if p.borrow().val != q.borrow().val {
//         return false;
//     }

//     if let Some(ref p_right) = p.borrow().right {
//         if let Some(ref q_right) = q.borrow().right {
//             if !recoursive_equality(p_right, q_right) {
//                 return false;
//             }
//         }
//     }

//     true
// }

// pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
//     if p.is_none() && q.is_none() {
//         return true;
//     }

//     if p.is_none() && q.is_some() || p.is_some() && q.is_none() {
//         return false;
//     }

//     recoursive_equality(&p.unwrap(), &q.unwrap())
// }

// NOTE: more idiomatic rust solution with pattern matching.
pub fn recoursive_equality(
    p: &Option<Rc<RefCell<TreeNode>>>,
    q: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (p, q) {
        (Some(p), Some(q)) => {
            let p = p.borrow();
            let q = q.borrow();
            recoursive_equality(&p.left, &q.left)
                && p.val == q.val
                && recoursive_equality(&p.right, &q.right)
        }
        (None, None) => true,
        _ => false,
    }
}

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    recoursive_equality(&p, &q)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_none() {
        assert!(is_same_tree(None, None))
    }

    #[test]
    fn check_none_vs_1_node() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));

        assert!(!is_same_tree(None, root))
    }

    #[test]
    fn check_one_node() {
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
    }

    #[test]
    fn check_height_2() {
        let left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left,
            right: None,
        })));

        assert!(is_same_tree(root.clone(), root));

        let left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let root1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left,
            right: None,
        })));

        let left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let root2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left,
            right: None,
        })));

        assert!(!is_same_tree(root1, root2));
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
    }
}
