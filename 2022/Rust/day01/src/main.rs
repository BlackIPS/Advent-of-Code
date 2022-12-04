use std::fs;

mod tests;

fn main() {
    let input = fs::read_to_string("./input/input.txt").expect("couldn't read input file");

    println!("Step 1 result: {}", get_max_calories(&input));
    println!("Step 2 result: {}", get_top3_calories_summed(&input));
}

fn get_max_calories(input: &String) -> u32 {
    let by_elves = input
        .split("\n\n")
        .map(|elf_raw| {
            elf_raw
                .lines()
                .map(|item| { item.parse::<u32>().unwrap() })
                .sum::<u32>()
        })
        .max()
        .unwrap();

    by_elves
}

fn get_top3_calories_summed(input: &String) -> u32 {
    let mut by_elves:Vec<u32> = input
        .split("\n\n")
        .map(|elf_raw| {
            elf_raw
                .lines()
                .map(|item| { item.parse::<u32>().unwrap() })
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    by_elves.sort_by(|a,b| b.cmp(a));
    let sum: u32 = by_elves.iter().take(3).sum();
//    dbg!(sum);

    sum
}
