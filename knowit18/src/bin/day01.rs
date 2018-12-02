const PUZZLE: &str = include_str!("../../inputs/day01.txt");

fn main() {
    println!("{}", run());
}

fn run() -> i32 {
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

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_one() {
        assert_eq!(run(), 12920419);
    }
}
