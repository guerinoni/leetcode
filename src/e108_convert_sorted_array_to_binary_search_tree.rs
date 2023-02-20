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

fn recoursive_create_tree(
    nums: &[i32],
    lower_bound: usize,
    higher_bound: usize,
) -> Option<Rc<RefCell<TreeNode>>> {
    match lower_bound >= higher_bound {
        true => None,
        false => {
            let middle = lower_bound + (higher_bound - lower_bound) / 2;
            Some(Rc::new(RefCell::new(TreeNode {
                val: nums[middle],
                left: recoursive_create_tree(nums, lower_bound, middle),
                right: recoursive_create_tree(nums, middle + 1, higher_bound),
            })))
        }
    }
}

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    recoursive_create_tree(&nums, 0, nums.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_case_1() {
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

        let output = sorted_array_to_bst(vec![-10, -3, 0, 5, 9]);
        assert_eq!(expected, output);
    }

    #[test]
    fn check_case_2() {
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: None,
        })));

        let output = sorted_array_to_bst(vec![1, 3]);
        assert_eq!(expected, output);
    }

    #[test]
    fn check_case_3() {
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));

        let output = sorted_array_to_bst(vec![3]);
        assert_eq!(expected, output);
    }

    #[test]
    fn check_case_4() {
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

        let output = sorted_array_to_bst(vec![-20, -10, -3, 0, 1, 5, 9]);
        assert_eq!(expected, output);
    }
}
