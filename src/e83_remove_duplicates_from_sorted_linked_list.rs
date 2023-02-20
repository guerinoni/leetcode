// NOTE: first try.
// use crate::merge_two_sorted_list::create_list_node;
// pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     if head.is_none() {
//         return None;
//     }

//     let mut last_num = head.as_ref().unwrap().val;
//     let mut pointer = head.unwrap().next;
//     let mut values = vec![last_num];
//     while let Some(v) = pointer {
//         if last_num != v.val {
//             last_num = v.val;
//             values.push(last_num);
//         }
//         pointer = v.next;
//     }

//     create_list_node(values, 0)
// }

use crate::e21_merge_two_sorted_list::ListNode;
// NOTE: replace elements on same input.
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    head.as_ref()?;

    let mut hh = head;
    let mut prev = hh.as_mut().unwrap();

    while let Some(current) = prev.next.as_mut() {
        if current.val != prev.val {
            prev = prev.next.as_mut().unwrap();
            continue;
        }

        // prev.next = prev.next.next (or lets say current.next) a bit tricky in rust :)
        let next = std::mem::replace(&mut current.next, None);
        let _ = std::mem::replace(&mut prev.next, next);
    }

    hh
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::e21_merge_two_sorted_list::{compare, create_list_node};

    #[test]
    fn check() {
        let input = create_list_node(vec![1, 1, 2], 0);
        let output = delete_duplicates(input);
        let expected = create_list_node(vec![1, 2], 0);
        assert!(compare(output, expected));

        let input = create_list_node(vec![1, 1, 2, 3, 3], 0);
        let output = delete_duplicates(input);
        let expected = create_list_node(vec![1, 2, 3], 0);
        assert!(compare(output, expected));

        let input = create_list_node(vec![1, 1, 1, 2, 2, 3, 3, 4, 5, 6, 7, 99, 99, 99, 99], 0);
        let output = delete_duplicates(input);
        let expected = create_list_node(vec![1, 2, 3, 4, 5, 6, 7, 99], 0);
        assert!(compare(output, expected));
    }
}
