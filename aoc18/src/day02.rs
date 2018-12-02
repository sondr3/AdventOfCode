extern crate hashbrown;

use hashbrown::HashMap;

const PUZZLE: &str = include_str!("../inputs/day02.txt");

pub fn run() -> Result<(), Box<std::error::Error>> {
    let words: Vec<&str> = PUZZLE.lines().map(|w| w.trim()).collect();
    let mut out = HashMap::new();

    for words in &words {
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

    println!("{:?}", out);
    println!("Part 1: {:?}", out.get(&2).unwrap() * out.get(&3).unwrap());

    for (i, word) in words.iter().enumerate() {
        for other in words.iter().skip(i + 1) {
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
                return Ok(());
            }
        }
    }

    Ok(())
}
