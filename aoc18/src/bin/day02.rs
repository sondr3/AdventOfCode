#[macro_use]
extern crate lazy_static;
use aoc18::hashbrown;
use hashbrown::HashMap;

const FILE: &str = include_str!("../../inputs/day02.txt");
lazy_static!(
    static ref PUZZLE: Vec<&'static str> = FILE.lines().map(|w| w.trim()).collect();
);

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> i32 {
    let mut out = HashMap::new();

    for words in PUZZLE.iter() {
        let mut counts = HashMap::new();
        for c in words.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        let mut two = false;
        let mut three = false;
        for (_, count) in counts {
            if count == 2 && !two {
                *out.entry(count).or_insert(0) += 1;
                two = true;
            }
            if count == 3 && !three {
                *out.entry(count).or_insert(0) += 1;
                three = true;
            }
        }
    }

    out.get(&2).unwrap() * out.get(&3).unwrap()
}

fn part_two() -> String {
    for (i, word) in PUZZLE.iter().enumerate() {
        for other in PUZZLE.iter().skip(i + 1) {
            let mut diff = 0;
            let mut removed = 0;
            for (j, (x, y)) in word.chars().zip(other.chars()).enumerate() {
                if x != y {
                    diff += 1;
                    removed = j;
                }
            }

            if diff == 1 {
                let mut finished = other.to_string();
                finished.remove(removed);
                println!("{}", word);
                println!("{}", other);
                println!("{}", finished);
                return finished;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod day02 {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(7350, part_one());
    }

    #[test]
    fn part_2() {
        assert_eq!(String::from("wmlnjevbfodamyiqpucrhsukg"), part_two());
    }
}
