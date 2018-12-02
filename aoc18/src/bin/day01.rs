#[macro_use]
extern crate lazy_static;

use aoc18::hashbrown;
use hashbrown::HashSet;

const FILE: &str = include_str!("../../inputs/day01.txt");
lazy_static!(
    static ref PUZZLE: Vec<i32> = FILE.lines().map(|c| c.trim().parse().unwrap()).collect();
);

fn main() {
    println!("{}", part_one(&PUZZLE));
    println!("{}", part_two(&PUZZLE));
}

fn part_one(input: &[i32]) -> i32 {
    input.iter().sum()
}

fn part_two(input: &[i32]) -> i32 {
    let mut frequencies = HashSet::new();
    let frequency = input
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
        assert_eq!(3, part_one(&vec![1, -2, 3, 1]));
        assert_eq!(3, part_one(&vec![1, 1, 1]));
        assert_eq!(0, part_one(&vec![1, 1, -2]));
        assert_eq!(-6, part_one(&vec![-1, -2, -3]));
        assert_eq!(518, part_one(&PUZZLE));
    }

    #[test]
    fn part_2() {
        assert_eq!(2, part_two(&vec![1, -2, 3, 1]));
        assert_eq!(10, part_two(&vec![3, 3, 4, -2, -4]));
        assert_eq!(5, part_two(&vec![-6, 3, 8, 5, -6]));
        assert_eq!(14, part_two(&vec![7, 7, -2, -7, -4]));
        assert_eq!(72889, part_two(&PUZZLE));
    }
}