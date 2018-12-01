use utils::parse;
use hashbrown::HashSet;

pub fn run() -> Result<i32, Box<std::error::Error>> {
    let out = parse("inputs/day01.txt")?;
    let mut frequencies = HashSet::new();
    let mut current_frequency = 0;
    let mut curr_pos = 0;
    // Answer to part one
    // let sum: i32 = out.iter().sum();

    frequencies.insert(0);
    loop {
        current_frequency += out[curr_pos % out.len()];
        curr_pos += 1;
        if frequencies.contains(&current_frequency) {
            return Ok(current_frequency);
        }
        frequencies.insert(current_frequency);
    }
}