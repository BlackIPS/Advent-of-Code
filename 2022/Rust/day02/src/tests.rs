#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn verify_part1() {
        let input_by_line: Vec<_> = INPUT.lines().map(|value| value.split_once(" ").unwrap_or(("","")) ).collect();
        let result = calculate_total_points(input_by_line);
        assert_eq!(result, 15);
    }

    #[test]
    fn verify_part2() {
        let input_by_line: Vec<_> = INPUT.lines().map(|value| value.split_once(" ").unwrap_or(("","")) ).collect();
        let result = calculate_ultra_secret(input_by_line);
        assert_eq!(result, 12);
    }
}