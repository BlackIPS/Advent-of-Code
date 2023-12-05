use tracing::debug;
use crate::errors::AoCError;
use crate::{Game, parse};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String, AoCError> {
    let lines = parse(input);

    let sum: usize = lines.into_iter()
        .map(|line| {
            let game = Game::parse(line).unwrap();

            debug!(line, min = game.minimum_cubes_necessary().power_of());
            
            game.minimum_cubes_necessary().power_of()
        })
        .sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn verify_process() -> Result<(),AoCError> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}