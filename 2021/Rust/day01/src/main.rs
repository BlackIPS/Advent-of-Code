use std::fs;

mod tests;

fn main() {
    let content = fs::read_to_string("./input/input.txt").expect("could not load input file");

    println!("Step 1 result: {}", process_part1(content.as_str()));
    println!("Step 2 result: {}", process_part2(content.as_str()));
}

fn process_part1(input: &str) -> String {
    let depths: Vec<&str> = input.lines().collect();

    let mut last = 99999999u32;
    let mut increased = 0u32;
    for depth in depths {
        let current = depth.parse::<u32>().unwrap();

        if current > last { increased += 1; }
        last = current
    }

    increased.to_string()
}

fn process_part2(input: &str) -> String {
    let depths: Vec<u32> = input.lines().map(|value| value.parse::<u32>().unwrap()).collect();

    let mut grouped: Vec<u32> = vec![];
    for i in 0..(depths.len()-2) {
        grouped.push(depths[i] + depths[i+1] + depths[i+2]);
    }

    let mut last = 99999999u32;
    let mut increased = 0u32;
    for current in grouped {
        if current > last { increased += 1; }
        last = current
    }

    increased.to_string()
}