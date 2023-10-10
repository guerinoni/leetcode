pub fn day_of_year(date: String) -> i32 {
    let mut parts = date.split('-');
    let year = parts.next().unwrap().parse::<i32>().unwrap();
    let month = parts.next().unwrap().parse::<i32>().unwrap();
    let day = parts.next().unwrap().parse::<i32>().unwrap();

    let months = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut days = 0;

    for m in 1..month {
        if m == 2 {
            days += if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                29
            } else {
                28
            }
        } else {
            days += months[(m - 1) as usize];
        }
    }

    days + day
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(day_of_year("2019-01-09".to_string()), 9);
        assert_eq!(day_of_year("2019-02-10".to_string()), 41);
        assert_eq!(day_of_year("2003-03-01".to_string()), 60);
        assert_eq!(day_of_year("2004-03-01".to_string()), 61);
    }
}
