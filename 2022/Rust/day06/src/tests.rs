#[cfg(test)]
mod tests {
    use crate::*;
    
    const TESTCASES_PART1: [(&str, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)
    ];

    #[test]
    fn verify_part1() {
        for (input, expectation) in TESTCASES_PART1 {
            assert_eq!(expectation, process_part1(input))
        }
    }

    const TESTCASES_PART2: [(&str, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)
    ];

    #[test]
    fn verify_part2() {
        for (input, expectation) in TESTCASES_PART2 {
            assert_eq!(expectation, process_part2(input))
        }
    }
}