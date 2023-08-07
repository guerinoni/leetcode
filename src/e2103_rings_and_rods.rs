pub fn count_points(rings: String) -> i32 {
    let mut rods = [0; 10];

    for ring in rings.as_bytes().chunks(2) {
        let color = ring[0];
        let which_color = match color {
            b'R' => 0_usize,
            b'G' => 1_usize,
            b'B' => 2_usize,
            _ => panic!("Invalid color"),
        };

        let position = (ring[1] - b'0') as usize;
        rods[position] |= 1 << which_color;
    }

    rods.iter().filter(|&&colors| colors == 0b111).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_points() {
        assert_eq!(count_points("B0B6G0R6R0R6G9".to_string()), 1);
        assert_eq!(count_points("B0R0G0R9R0B0G0".to_string()), 1);
    }
}
