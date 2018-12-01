use utils::parse;

pub fn run() -> Result<i32, Box<std::error::Error>> {
    let out = parse("src/inputs/day01.txt")?;
    let sum: i32 = out.iter().sum();

    Ok(sum)
}