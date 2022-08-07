// NOTE: version with split_whitespace API.
// pub fn length_of_last_word(s: String) -> i32 {
//     s.split_ascii_whitespace()
//         .last()
//         .map(|s| s.len() as i32)
//         .unwrap_or(0)
// }

// NOTE: iterator version with fold.
// use std::ops::ControlFlow;
// pub fn length_of_last_word(s: String) -> i32 {
//     let t = s.as_bytes().iter().rev().try_fold(0, |sum, &current| {
//         if sum != 0 && current.is_ascii_whitespace() {
//             return ControlFlow::Break(sum);
//         }

//         if sum == 0 && current.is_ascii_whitespace() {
//             return ControlFlow::Continue(sum);
//         }

//         if current.is_ascii_alphabetic() {
//             return ControlFlow::Continue(sum + 1);
//         }

//         ControlFlow::Continue(sum)
//     });

//     match t {
//         ControlFlow::Break(v) => v,
//         _ => 0,
//     }
// }

// NOTE: without consuming all string with an iterator.
pub fn length_of_last_word(s: String) -> i32 {
    let mut flag_first_char_found = false;
    let mut counter = 0;

    for c in s.as_bytes().iter().rev() {
        if flag_first_char_found && c.is_ascii_whitespace() {
            break;
        }

        if !c.is_ascii_whitespace() {
            flag_first_char_found = true;

            if flag_first_char_found {
                counter += 1;
            }
        }
    }

    counter
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(
            length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
        assert_eq!(length_of_last_word("luffy is still joyboy".to_string()), 6);
    }
}
