// NOTE: first try.
// pub fn get_row(row_index: i32) -> Vec<i32> {
//     if row_index == 0 {
//         return vec![1];
//     }

//     let mut res = vec![1, 1];
//     let mut counter = 1;
//     while counter < row_index {
//         let mut new_vec = vec![match res.first() {
//             Some(v) => *v,
//             None => break,
//         }];

//         new_vec.extend(res.windows(2).map(|v| v[0] + v[1]));

//         new_vec.push(match res.last() {
//             Some(v) => *v,
//             None => break,
//         });

//         res = new_vec;
//         counter += 1;
//     }

//     res
// }

// NOTE: dynamic programming solution
pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut result = vec![0; row_index as usize + 1];
    result[0] = 1;
    for i in 1..result.len() {
        for j in (1..=i).rev() {
            result[j] = result[j] + result[j - 1];
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let output = get_row(0);
        let expected = vec![1];
        assert_eq!(output, expected);

        let output = get_row(1);
        let expected = vec![1, 1];
        assert_eq!(output, expected);

        let output = get_row(2);
        let expected = vec![1, 2, 1];
        assert_eq!(output, expected);

        let output = get_row(3);
        let expected = vec![1, 3, 3, 1];
        assert_eq!(output, expected);

        let output = get_row(4);
        let expected = vec![1, 4, 6, 4, 1];
        assert_eq!(output, expected);
    }
}
