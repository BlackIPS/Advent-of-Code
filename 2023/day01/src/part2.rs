use tracing::{debug, info, span, trace};
use crate::errors::AoCError;
use crate::{combine_digits, parse};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String, AoCError> {
    let patterns: Vec<(&str, u32)> = vec![
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ];

    let lines = parse(input);
    let mut sum = 0_u32;
    
    for line in lines {
        sum = sum + find_by_pattern(line, &patterns);
    }
    
    Ok(sum.to_string())
}



#[tracing::instrument(skip(all_patterns))]
fn find_by_pattern(input: &str, all_patterns: &Vec<(&str, u32)>) -> u32 {
    let mut first_value = 0_u32;
    let mut last_value = 0_u32;
    let mut first_offset: Option<usize> = None;
    let mut last_offset: Option<usize> = None;
    
    for (pattern, value) in all_patterns {
        span!(tracing::Level::TRACE, "start new search for pattern");
        trace!(pattern);
        
        
        if let Some(offset) = input.find(pattern) {
            if first_offset.is_none() || offset < first_offset.unwrap() {
                debug!(value, offset, "found new first value");
                first_value = *value;
                first_offset = Some(offset);
            }
        }
        
        
        if let Some(offset) = input.rfind(pattern) {
            if last_offset.is_none() || offset > last_offset.unwrap() {
                debug!(value, offset, "found new last value");
                last_value = *value;
                last_offset = Some(offset);
            }
        }
    }

    info!(first_value, last_value, "found solution");
    combine_digits(first_value, last_value)
}


#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn verify_process() -> Result<(),AoCError> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}