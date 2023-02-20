use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }

    pub fn new_with_leaf(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left,
            right,
        })))
    }
}

// NOTE: First loop it will add 2 nodes (list[1]) to the root and you have  X
//                                                                        /   \
//                                                                       X     X
// Second iteration uses the simple node at list[1] (no kids) and list[3] (tree above)
// Third iteration uses same logic but inverts the args, it uses list[3] and list[1]
// 4th iteration uses as left+right list[1] and list[5]
// ...
// It builds                  X
// After it builds       X
//                     /   \
//                    X     X
// Later with new node uses as left the first and as right the second + inverted left/right and you have
//                       X
//                     /   \
//                    X     X
//                        /   \
//                       X     X                 and the mirrored version
pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if n % 2 == 0 {
        // not a binary tree
        return vec![];
    }
    let mut list = vec![vec![], vec![TreeNode::new_with_leaf(None, None)]];

    for i in (3..=n).step_by(2) {
        let mut v = vec![];
        for j in (1..i - 1).step_by(2) {
            for a in list[j as usize].iter() {
                let y = i as usize - j as usize - 1;
                v.extend(
                    list[y]
                        .iter()
                        .map(|b| TreeNode::new_with_leaf(a.clone(), b.clone()))
                        .collect::<Vec<Option<Rc<RefCell<TreeNode>>>>>(),
                );
            }
        }

        list.extend(vec![vec![], v]);
    }

    list.last().unwrap().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert!(all_possible_fbt(2).is_empty());

        let output = all_possible_fbt(3);
        assert_eq!(
            output,
            vec![TreeNode::new_with_leaf(TreeNode::new(0), TreeNode::new(0)),]
        );

        let output = all_possible_fbt(1);
        assert_eq!(output, vec![TreeNode::new(0),]);

        let output = all_possible_fbt(7);
        assert_eq!(output.len(), 5);
        assert_eq!(
            output[0],
            TreeNode::new_with_leaf(
                TreeNode::new(0),
                TreeNode::new_with_leaf(
                    TreeNode::new(0),
                    TreeNode::new_with_leaf(TreeNode::new(0), TreeNode::new(0))
                )
            )
        );
        assert_eq!(
            output[1],
            TreeNode::new_with_leaf(
                TreeNode::new(0),
                TreeNode::new_with_leaf(
                    TreeNode::new_with_leaf(TreeNode::new(0), TreeNode::new(0)),
                    TreeNode::new(0)
                )
            )
        );
        assert_eq!(
            output[2],
            TreeNode::new_with_leaf(
                TreeNode::new_with_leaf(TreeNode::new(0), TreeNode::new(0)),
                TreeNode::new_with_leaf(TreeNode::new(0), TreeNode::new(0)),
            )
        );
    }
}
