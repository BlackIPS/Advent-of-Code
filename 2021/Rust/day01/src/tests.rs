#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn verify_part1() {
        let result = process_part1(INPUT);
        assert_eq!(result, "7");
    }

    #[test]
    fn verify_part2() {
        let result = process_part2(INPUT);
        assert_eq!(result, "5");
    }
}
