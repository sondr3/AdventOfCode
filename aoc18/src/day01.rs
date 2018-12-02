use hashbrown::HashSet;

const PUZZLE: &str = include_str!("../inputs/day01.txt");

pub fn run() -> Result<(), Box<std::error::Error>> {
    let out: Vec<i32> = PUZZLE.lines().map(|c| c.trim().parse().unwrap()).collect();

    let sum: i32 = out.iter().sum();
    println!("{:?}", sum);

    let mut frequencies = HashSet::new();
    let frequency = out
        .iter()
        .cycle()
        .scan(0, |frequency, &curr| {
            *frequency += curr;
            Some(*frequency)
        }).find(|frequency| !frequencies.insert(*frequency))
        .unwrap();

    println!("{:?}", frequency);

    Ok(())
}
