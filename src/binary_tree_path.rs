use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let mut v: Vec<String> = vec![];
    go_to_leaf(&root, "".to_owned(), &mut v);
    v
}

fn go_to_leaf(node: &Option<Rc<RefCell<TreeNode>>>, s: String, v: &mut Vec<String>) {
    match node {
        None => {}
        Some(n) => {
            let val = n.borrow().val;
            let ss = if s.is_empty() {
                format!("{val}")
            } else {
                let mut x = s;
                x.push_str(&format!("->{val}"));
                x
            };

            match (&n.borrow().left, &n.borrow().right) {
                (None, None) => {
                    v.push(ss);
                }
                _ => {
                    go_to_leaf(&n.borrow().left, ss.clone(), v);
                    go_to_leaf(&n.borrow().right, ss.clone(), v);
                }
            };
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));

        let v = binary_tree_paths(root);
        assert_eq!(v, vec!["1->2->5".to_owned(), "1->3".to_owned()]);

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let v = binary_tree_paths(root);
        assert_eq!(v, vec!["3".to_owned()]);

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
            right: None,
        })));

        let v = binary_tree_paths(root);
        assert_eq!(v, vec!["1->2->5->3".to_owned()]);
    }
}
