use day_01::part2::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(tracing::Level::DEBUG)
        // .pretty()
        .init();
    
    let input = include_str!("../../input-part2.txt");

    let result = process(input)?;
    println!("{}", result);

    Ok(())
}