use day_01::part1::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../input-part1.txt");

    let result = process(input)?;
    println!("{}", result);
    
    Ok(())
}