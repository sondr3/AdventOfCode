use aoc18::hashbrown;
use hashbrown::HashMap;

const PUZZLE: &str = include_str!("../../inputs/day02.txt");

fn parse(input: &str) -> Vec<&str> {
    input.lines().map(|w| w.trim()).collect()
}

fn main() {
    println!("{}", part_one(&parse(PUZZLE)));
    println!("{}", part_two(&parse(PUZZLE)));
}

fn part_one(input: &[&str]) -> i32 {
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

fn part_two(input: &[&str]) -> String {
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
        let boxes = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
        assert_eq!(12, part_one(&parse(boxes)));
        assert_eq!(7350, part_one(&parse(PUZZLE)));
    }

    #[test]
    fn part_2() {
        let boxes = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
        assert_eq!("fgij", part_two(&parse(boxes)));
        assert_eq!("wmlnjevbfodamyiqpucrhsukg", &part_two(&parse(PUZZLE)));
    }
}
