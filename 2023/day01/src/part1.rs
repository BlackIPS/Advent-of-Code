use crate::errors::AoCError;

#[tracing::instrument]
pub fn process(_input: &str) -> Result<String, AoCError> {
    todo!("day 01 - part 1");
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn verify_process() -> Result<(),AoCError> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", crate::part2::process(input)?);
        Ok(())
    }
}