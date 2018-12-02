#[macro_use]
extern crate lazy_static;
use aoc18::hashbrown;
use hashbrown::HashMap;

const FILE: &str = include_str!("../../inputs/day02.txt");
lazy_static!(
    static ref PUZZLE: Vec<String> = FILE.lines().map(|w| w.trim().to_string()).collect();
);

fn main() {
    println!("{}", part_one(PUZZLE.as_slice()));
    println!("{}", part_two(PUZZLE.as_slice()));
}

fn part_one(input: &[String]) -> i32 {
    let mut out = HashMap::new();

    for words in input.iter() {
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

fn part_two(input: &[String]) -> String {
    for (i, word) in input.iter().enumerate() {
        for other in input.iter().skip(i + 1) {
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
        let boxes: Vec<String> = vec!["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"].iter().map(|w| w.to_string()).collect();
        assert_eq!(12, part_one(&boxes));
        assert_eq!(7350, part_one(PUZZLE.as_slice()));
    }

    #[test]
    fn part_2() {
        let boxes: Vec<String> = vec!["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"].iter().map(|w| w.to_string()).collect();
        assert_eq!(String::from("fgij"), part_two(&boxes));
        assert_eq!(String::from("wmlnjevbfodamyiqpucrhsukg"), part_two(PUZZLE.as_slice()));
    }
}
