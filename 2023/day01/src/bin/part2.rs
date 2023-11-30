use day_01::part2::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../../input-part2.txt");

    let result = process(input)?;
    println!("{}", result);

    Ok(())
}