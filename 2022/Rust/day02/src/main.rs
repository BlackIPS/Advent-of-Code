use std::fs;

mod tests;

const POINTS_DRAW: u32 = 3;
const POINTS_WIN: u32  = 6;

const POINTS_ROCK: u32    = 1;
const POINTS_PAPER: u32   = 2;
const POINTS_SCISSOR: u32 = 3;


fn main() {
    let input = fs::read_to_string("./input/input.txt").expect("could not load input file");
    let gamesteps: Vec<_> = input
        .lines()
        .map(|value| value.split_once(" ").unwrap_or(("","")) )
        .collect();

    println!("Step 1 result: {}", calculate_total_points(gamesteps.clone()));
    println!("Step 2 result: {}", calculate_ultra_secret(gamesteps.clone()));
}

fn calculate_total_points(gamesteps: Vec<(&str, &str)>) -> u32 {
    let mut scores: Vec<u32> = vec![];
    for step in gamesteps {
        let mut roundpoints = 0u32;

        roundpoints += match step.1 {
            "X" => POINTS_ROCK,
            "Y" => POINTS_PAPER,
            "Z" => POINTS_SCISSOR,
            _ => 0
        };

        roundpoints += match step.0 {
            "A" if step.1 == "X" => POINTS_DRAW,
            "A" if step.1 == "Y" => POINTS_WIN,
            "B" if step.1 == "Y" => POINTS_DRAW,
            "B" if step.1 == "Z" => POINTS_WIN,
            "C" if step.1 == "Z" => POINTS_DRAW,
            "C" if step.1 == "X" => POINTS_WIN,
            _ => 0
        };

        scores.push(roundpoints);
    }

    scores.iter().sum::<u32>()
}

fn calculate_ultra_secret(gamesteps: Vec<(&str, &str)>) -> u32 {
    let mut scores: Vec<u32> = vec![];
    for step in gamesteps {
        let mut roundpoints = 0u32;

        roundpoints += match step.0 {
            "A" if step.1 == "X" => POINTS_SCISSOR,
            "A" if step.1 == "Y" => POINTS_DRAW + POINTS_ROCK,
            "A" if step.1 == "Z" => POINTS_WIN + POINTS_PAPER,
            "B" if step.1 == "X" => POINTS_ROCK,
            "B" if step.1 == "Y" => POINTS_DRAW + POINTS_PAPER,
            "B" if step.1 == "Z" => POINTS_WIN + POINTS_SCISSOR,
            "C" if step.1 == "X" => POINTS_PAPER,
            "C" if step.1 == "Y" => POINTS_DRAW + POINTS_SCISSOR,
            "C" if step.1 == "Z" => POINTS_WIN + POINTS_ROCK,
            _ => 0
        };

        scores.push(roundpoints)
    }

    scores.iter().sum::<u32>()
}
