const PUZZLE: &str = include_str!("../inputs/day01.txt");

pub fn run() -> Result<(), Box<std::error::Error>> {
    let file: Vec<i32> = PUZZLE.lines().map(|c| c.trim().parse().unwrap()).collect();

    let mut finished = Vec::new();
    let mut max = 0;
    for line in file {
        if line >= max {
            max = line;
            finished.push(line);
        }
    }

    let sum: i32 = finished.iter().sum();

    println!("{}", sum);

    Ok(())
}
