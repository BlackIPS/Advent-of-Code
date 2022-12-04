use std::fs;

mod tests;

fn main() {
    let content = fs::read_to_string("./input/input.txt").expect("could not load input file");

    println!("Step 1 result: {}", process_part1(content.as_str()));
    println!("Step 2 result: {}", process_part2(content.as_str()));
}

fn process_part1(input: &str) -> String {
    let mut horizontal = 0i32;
    let mut depth = 0i32;

    let command_list: Vec<_> = input
        .lines()
        .map(|value| value.split_once(" ").unwrap() )
        .collect();
    for command in command_list {
        horizontal += match command.0 {
            "forward" => command.1.parse::<i32>().unwrap(),
            _ => 0
        };

        depth += match command.0 {
            "down" => command.1.parse::<i32>().unwrap(),
            "up" => command.1.parse::<i32>().unwrap() * -1,
            _ => 0
        }
    }


    (horizontal * depth).to_string()
}

fn process_part2(input: &str) -> String {
    let mut horizontal = 0i32;
    let mut depth = 0i32;
    let mut aim = 0i32;

    let command_list: Vec<_> = input
        .lines()
        .map(|value| value.split_once(" ").unwrap() )
        .collect();
    for command in command_list {
        let noun = command.0;
        let verb = command.1.parse::<i32>().unwrap();

        aim += match noun {
            "down" => verb,
            "up" => verb * -1,
            _ => 0
        };

        horizontal += match noun {
            "forward" => verb,
            _ => 0
        };

        depth += match noun {
            "forward" => verb * aim,
            _ => 0
        }
    }


    (horizontal * depth).to_string()
}