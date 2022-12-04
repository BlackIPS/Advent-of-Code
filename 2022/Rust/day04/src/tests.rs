#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn verify_part1() {
        let input_by_line: Vec<_> = INPUT.lines().collect();
        let result = process_part1(input_by_line);
        assert_eq!(result, 2);
    }

    #[test]
    fn verify_part2() {
        let input_by_line: Vec<_> = INPUT.lines().collect();
        let result = process_part2(input_by_line);
        assert_eq!(result, 4);
    }
}