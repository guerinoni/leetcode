pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = img.len();
    let n = img[0].len();

    if m == 0 || n == 0 {
        return vec![];
    }

    let mut result = vec![vec![0; img[0].len()]; img.len()];

    let directions = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for y in 0..n {
        for x in 0..m {
            let mut counter = 1;
            let s = directions.iter().fold(img[x][y], |sum, item| {
                let (mut dx, mut dy) = item;
                dx += x as i32;
                dy += y as i32;
                if dx < 0 || dx > (m - 1) as i32 || dy < 0 || dy > (n - 1) as i32 {
                    sum
                } else {
                    counter += 1;
                    sum + img[(dx) as usize][(dy) as usize]
                }
            });
            result[x][y] = s / counter;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_661() {
        assert_eq!(
            image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
        );

        assert_eq!(
            image_smoother(vec![
                vec![100, 200, 100],
                vec![200, 50, 200],
                vec![100, 200, 100]
            ]),
            vec![
                vec![137, 141, 137],
                vec![141, 138, 141],
                vec![137, 141, 137]
            ]
        );
    }
}
