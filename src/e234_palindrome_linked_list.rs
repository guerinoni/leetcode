#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn is_palindrome_recoursive(head: &Option<Box<ListNode>>, mut v: Vec<i32>) -> bool {
    match head {
        Some(h) => {
            v.push(h.val);
            is_palindrome_recoursive(&h.next, v)
        }
        None => {
            let mut l = 0;
            let mut h = v.len() - 1;
            while l < h {
                if v[l] != v[h] {
                    return false;
                }
                l += 1;
                h -= 1;
            }

            true
        }
    }
}

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let v = vec![];
    is_palindrome_recoursive(&head, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let root = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));

        assert!(!is_palindrome(root));

        let root = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 1, next: None })),
                })),
            })),
        }));

        assert!(is_palindrome(root));
    }
}
