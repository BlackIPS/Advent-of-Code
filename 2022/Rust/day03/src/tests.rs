#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn verify_part1() {
        let input_by_line: Vec<_> = INPUT.lines().collect();
        let result = process_part1(input_by_line);
        assert_eq!(result, 157);
    }

    #[test]
    fn verify_part2() {
        let input_by_line: Vec<_> = INPUT.lines().collect();
        let result = process_part2(input_by_line);
        assert_eq!(result, 70);
    }
}