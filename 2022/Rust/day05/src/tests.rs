#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn verify_part1() {
        let result = process_part1(INPUT.to_string());
        assert_eq!(result, "CMZ".to_string());
    }

    #[test]
    fn verify_part2() {
        let result = process_part2(INPUT.to_string());
        assert_eq!(result, "MCD".to_string());
    }
}