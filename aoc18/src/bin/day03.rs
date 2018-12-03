#[macro_use]
extern crate lazy_static;

use aoc18::regex::Regex;
use hashbrown::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

const PUZZLE: &str = include_str!("../../inputs/day03.txt");

#[derive(Debug)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl FromStr for Claim {
    type Err = ParseIntError;

    fn from_str(claim: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        }
        let matches = RE.captures(claim).unwrap();

        Ok(Claim {
            id: matches[1].parse().unwrap(),
            x: matches[2].parse().unwrap(),
            y: matches[3].parse().unwrap(),
            width: matches[4].parse().unwrap(),
            height: matches[5].parse().unwrap(),
        })
    }
}

fn parse(input: &str) -> Vec<Claim> {
    input
        .lines()
        .map(Claim::from_str)
        .map(Result::unwrap)
        .collect()
}

fn main() {
    println!("{}", part_one(&parse(PUZZLE)));
    println!("{}", part_two(&parse(PUZZLE)));
}

fn part_one(input: &[Claim]) -> usize {
    let mut grid = [[0u8; 1000]; 1000];
    for claim in input {
        for x in &mut grid[claim.x..claim.x + claim.width] {
            for y in &mut x[claim.y..claim.y + claim.height] {
                *y += 1;
            }
        }
    }

    grid.iter()
        .flat_map(|c| c.iter())
        .filter(|c| **c >= 2)
        .count()
}

fn part_two(input: &[Claim]) -> usize {
    let mut grid = vec![vec![0; 1000]; 1000];
    let mut claims: HashSet<usize> = input.iter().map(|c| c.id).collect();
    for claim in input {
        for x in &mut grid[claim.x..claim.x + claim.width] {
            for y in &mut x[claim.y..claim.y + claim.height] {
                if *y == 0 {
                    *y = claim.id;
                } else {
                    claims.remove(&claim.id);
                    claims.remove(y);
                }
            }
        }
    }

    claims.into_iter().next().unwrap()
}

#[cfg(test)]
mod day03 {
    use super::*;

    #[test]
    fn test_regex() {
        let squares = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"];
        let re = Regex::new(r"(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        for square in squares {
            assert!(re.is_match(square));
        }
    }

    #[test]
    fn part_1() {
        let squares = parse("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2");
        assert_eq!(4, part_one(&squares));
        assert_eq!(105231, part_one(&parse(PUZZLE)));
    }

    #[test]
    fn part_2() {
        let squares = parse("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2");
        assert_eq!(3, part_two(&squares));
    }
}
