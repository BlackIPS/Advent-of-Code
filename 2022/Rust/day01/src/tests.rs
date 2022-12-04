#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn verify_part1() {
        let result = get_max_calories(&INPUT.to_string());
        assert_eq!(result, 24000);
    }

    #[test]
    fn verify_part2() {
        let result = get_top3_calories_summed(&INPUT.to_string());
        assert_eq!(result, 45000);
    }
}