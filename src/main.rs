extern crate aoc18;

use aoc18::day01;

fn main() -> Result<(), Box<std::error::Error>> {
    println!("{}", day01::run()?);
    Ok(())
}