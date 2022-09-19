#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut h = head;
    let mut ptr = &mut h;

    loop {
        match ptr {
            Some(node) if node.val == val => {
                *ptr = node.next.take();
            }
            Some(node) => ptr = &mut node.next,
            None => break,
        }
    }

    h
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

        let output = remove_elements(root, 2);
        assert_eq!(output, Some(Box::new(ListNode { val: 1, next: None })));
    }

    #[test]
    fn check_1_2_2() {
        let root = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 2, next: None })),
            })),
        }));

        let output = remove_elements(root, 2);
        assert_eq!(output, Some(Box::new(ListNode { val: 1, next: None })));
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

        let output = remove_elements(root, 3);
        assert_eq!(
            output,
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None })),
            }))
        );
    }

    #[test]
    fn check_1_2_3_delete_first() {
        let root = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        let output = remove_elements(root, 1);
        assert_eq!(
            output,
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            }))
        );
    }
}
