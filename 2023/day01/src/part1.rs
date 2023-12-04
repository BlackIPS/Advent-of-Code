use nom::AsChar;
use crate::errors::AoCError;
use crate::{combine_digits, parse};

#[tracing::instrument]
pub fn process(input: &str) -> Result<String, AoCError> {
    let lines = parse(input);

    // Find first and last digit in each line and add them together
    let sum: u32 = lines.into_iter()
        .map(|line| extract_numbers_from_line(line).map_err(|_| AoCError::ExtractionError).unwrap())
        .map(|numbers| combine_digits(*numbers.first().unwrap(), *numbers.last().unwrap()))
        .sum();
        
    
    Ok(sum.to_string())
}

#[tracing::instrument]
fn extract_numbers_from_line(input: &str) -> Result<Vec<u32>, AoCError> {
    let chars: Vec<u32> = input.chars()
        .filter(|c| c.is_dec_digit() )
        .map(|c| c.to_digit(10).unwrap() )
        .collect();

    Ok(chars)
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn verify_process() -> Result<(),AoCError> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}