#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// NOTE: first approach.
// pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     let mut rf = &head;
//     let mut latest = None;
//     loop {
//         match rf {
//             Some(h) => {
//                 latest = match latest {
//                     Some(l) => Some(Box::new(ListNode {
//                         val: h.val,
//                         next: Some(l),
//                     })),
//                     None => Some(Box::new(ListNode {
//                         val: h.val,
//                         next: None,
//                     })),
//                 };
//                 rf = &h.next;
//             }
//             None => break,
//         }
//     }

//     latest
// }

// NOTE: No other allocation, just swap next :)
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head;
    while let Some(mut node) = curr {
        curr = node.next;
        node.next = prev;
        prev = Some(node);
    }

    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_1_2() {
        let root = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));

        let output = reverse_list(root);
        let expected = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        }));
        assert_eq!(expected, output);
    }

    #[test]
    fn check_1_2_3() {
        let root = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        let output = reverse_list(root);
        let expected = Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            })),
        }));
        assert_eq!(expected, output);
    }
}
