use std::collections::HashMap;
use std::fs;

mod tests;

fn main() {
    let input = fs::read_to_string("./input/input.txt").expect("could not load input file");
    let content: Vec<_> = input.lines().collect();

    println!("Step 1 result: {}", process_part1(content.clone()));
    println!("Step 2 result: {}", process_part2(content.clone()));
}

// 8401
fn process_part1(input: Vec<&str>) -> usize {
    let mut duplicates: Vec<&str> = vec![];
    for line in input {
        let (first_str, second_str) = line.split_at(line.len()/2);
        let first = first_str.split("").collect::<Vec<_>>();
        let second = second_str.split("").collect::<Vec<_>>();

        let mut differences = Vec::new();
        for char in first {
            if !char.is_empty() && second.contains(&char) { differences.push(char.clone()); }
        }
        differences.dedup();
        duplicates.append(&mut differences);
    }

    let priorities = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    duplicates
        .iter()
        .map(|c| priorities.find(c).unwrap())
        .sum::<usize>()
}


// 2641
fn process_part2(input: Vec<&str>) -> usize {
    let letter_score = ('a'..='z')
        .chain('A'..='Z')
        .into_iter()
        .enumerate()
        .map(|(idx,c)| (c, idx+1))
        .collect::<HashMap<char, usize>>();

    let chunks = input
        .chunks(3)
        .collect::<Vec<_>>();

    chunks
        .into_iter()
        .map(|chunk| {
            let common_char = chunk[0]
                .chars()
                .find(|a_char| {
                    chunk[1].contains(*a_char) && chunk[2].contains(*a_char)
                })
                .unwrap();

            letter_score.get(&common_char).unwrap()
        })
        .sum::<usize>()
}