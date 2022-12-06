use std::collections::HashSet;
use std::fs;

mod tests;

fn main() {
    let input = fs::read_to_string("./input/input.txt").expect("could not load input file");

    println!("Step 1 result: {}", process_part1(input.as_str()));
    println!("Step 2 result: {}", process_part2(input.as_str()));
}

const UNIQUE_CHARS_PART1: usize = 4;
const UNIQUE_CHARS_PART2: usize = 14;

fn process_part1(input: &str) -> usize {    
    for idx in 0..input.len()-UNIQUE_CHARS_PART1 {
        let solution = input.get(idx..idx+UNIQUE_CHARS_PART1).unwrap().chars().collect::<HashSet<char>>();
        
        if solution.len() == UNIQUE_CHARS_PART1 {
            return idx + UNIQUE_CHARS_PART1;
        }        
    }
    
    0
}


fn process_part2(input: &str) -> usize {
    for idx in 0..input.len()-UNIQUE_CHARS_PART2 {
        let solution = input.get(idx..idx+UNIQUE_CHARS_PART2).unwrap().chars().collect::<HashSet<char>>();

        if solution.len() == UNIQUE_CHARS_PART2 {
            return idx + UNIQUE_CHARS_PART2;
        }
    }

    0
}
