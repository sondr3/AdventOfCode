#[macro_use]
extern crate lazy_static;

use aoc18::hashbrown;
use hashbrown::HashSet;

const FILE: &str = include_str!("../../inputs/day01.txt");
lazy_static!(
    static ref PUZZLE: Vec<i32> = FILE.lines().map(|c| c.trim().parse().unwrap()).collect();
);

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> i32 {
    PUZZLE.iter().sum()
}

fn part_two() -> i32 {
    let mut frequencies = HashSet::new();
    let frequency = PUZZLE
        .iter()
        .cycle()
        .scan(0, |frequency, &curr| {
            *frequency += curr;
            Some(*frequency)
        }).find(|frequency| !frequencies.insert(*frequency))
        .unwrap();

    frequency
}

#[cfg(test)]
mod day01 {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(518, part_one());
    }

    #[test]
    fn part_2() {
        assert_eq!(72889, part_two());
    }
}