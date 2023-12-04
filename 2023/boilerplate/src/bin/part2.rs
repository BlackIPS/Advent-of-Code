use day_XX::init_tracing;
use day_XX::part2::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();
    
    let input = include_str!("../../input-part2.txt");

    let result = process(input)?;
    println!("{}", result);
    
    Ok(())
}