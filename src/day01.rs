use hashbrown::HashSet;
use utils::parse;

pub fn run() -> Result<(), Box<std::error::Error>> {
    let out = parse("inputs/day01.txt")?;

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