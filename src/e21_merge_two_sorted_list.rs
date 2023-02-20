// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// NOTE: recursive solution
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(l1), Some(l2)) => {
            if l1.val > l2.val {
                Some(Box::new(ListNode {
                    val: l2.val,
                    next: merge_two_lists(Some(l1), l2.next),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: l1.val,
                    next: merge_two_lists(l1.next, Some(l2)),
                }))
            }
        }
    }
}

pub fn compare(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> bool {
    match (list1, list2) {
        (None, None) => true,
        (Some(_), None) | (None, Some(_)) => false,
        (Some(l1), Some(l2)) => {
            if l1.val != l2.val {
                return false;
            }

            compare(l1.next, l2.next)
        }
    }
}

pub fn create_list_node(values: Vec<i32>, idx: usize) -> Option<Box<ListNode>> {
    let val = *values.first().unwrap();
    let new_idx = idx + 1;
    let mut new_values = values;
    new_values.remove(0);
    if new_values.is_empty() {
        Some(Box::new(ListNode { val, next: None }))
    } else {
        Some(Box::new(ListNode {
            val,
            next: create_list_node(new_values, new_idx),
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_empty() {
        let output = merge_two_lists(None, None);
        let expected = None;
        assert!(compare(output, expected));
    }

    #[test]
    fn check_one_list_none() {
        let list = create_list_node(vec![1, 2, 4], 0);
        let output = merge_two_lists(list.clone(), None);
        let expected = list.clone();
        assert!(compare(output, expected));

        let output = merge_two_lists(None, list.clone());
        let expected = list;
        assert!(compare(output, expected));
    }

    #[test]
    fn check() {
        let list_1 = create_list_node(vec![1, 2, 4], 0);
        let list_2 = create_list_node(vec![1, 3, 4], 0);
        let output = merge_two_lists(list_1, list_2);
        let expected = create_list_node(vec![1, 1, 2, 3, 4, 4], 0);
        assert!(compare(output, expected));
    }
}
