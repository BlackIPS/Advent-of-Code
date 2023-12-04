pub mod errors;

pub mod part1;
pub mod part2;


#[tracing::instrument]
pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

#[tracing::instrument]
pub fn combine_digits(first: u32, last: u32) -> u32 {
    (10 * first) + last
}


#[cfg(debug_assertions)]
pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(tracing::Level::DEBUG)
        .init();
}


#[cfg(not(debug_assertions))]
pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(tracing::Level::INFO)
        .init();
}