pub mod errors;
mod part1;
mod part2;


#[tracing::instrument]
pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
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