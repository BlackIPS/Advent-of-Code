#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn verify_part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "150");
    }

    #[test]
    fn verify_part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "900");
    }
}
