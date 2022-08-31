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

fn recoursive_postorder_traversal(node: &Rc<RefCell<TreeNode>>) -> Vec<i32> {
    let mut v = vec![];

    if let Some(ref l) = node.borrow().left {
        v.extend(recoursive_postorder_traversal(l));
    }

    if let Some(ref r) = node.borrow().right {
        v.extend(recoursive_postorder_traversal(r));
    }

    v.push(node.borrow().val);

    v
}

pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    match root {
        Some(ref r) => recoursive_postorder_traversal(r),
        None => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_simple() {
        assert_eq!(postorder_traversal(None), vec![]);

        let tree = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(postorder_traversal(tree), vec![1]);
    }

    #[test]
    fn check_height_2() {
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
            postorder_traversal(Some(Rc::new(RefCell::new(root)))),
            vec![3, 2, 1]
        );
    }
}
