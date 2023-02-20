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

fn recoursive_path_sum(
    root: Option<Rc<RefCell<TreeNode>>>,
    target_sum: i32,
    current_sum: i32,
) -> bool {
    match root {
        Some(r) => {
            let current_sum = current_sum + r.as_ref().borrow().val;
            match (&r.as_ref().borrow().left, &r.as_ref().borrow().right) {
                (Some(l), Some(r)) => {
                    let left = recoursive_path_sum(Some(l.clone()), target_sum, current_sum);
                    let right = recoursive_path_sum(Some(r.clone()), target_sum, current_sum);
                    left || right
                }
                (Some(l), None) => recoursive_path_sum(Some(l.clone()), target_sum, current_sum),
                (None, Some(r)) => recoursive_path_sum(Some(r.clone()), target_sum, current_sum),
                _ => target_sum == current_sum,
            }
        }
        None => target_sum == current_sum,
    }
}

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    match root {
        Some(r) => recoursive_path_sum(Some(r), target_sum, 0),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root() {
        let r = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })));

        assert!(has_path_sum(r, 2));
    }

    #[test]
    fn case_2_leaves() {
        let r = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        })));

        assert!(has_path_sum(r.clone(), 4));
        assert!(!has_path_sum(r, 5));
    }

    #[test]
    fn case_hard() {
        let r = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 11,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 13,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));

        assert!(has_path_sum(r.clone(), 22));
    }
}
