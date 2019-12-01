use aoc18::logos;

use logos::Logos;
use std::collections::HashMap;

const PUZZLE: &str = include_str!("../../inputs/day04.txt");

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[end]
    End,
    #[error]
    Error,
    #[token = "["]
    StartDate,
    #[token = "]"]
    EndDate,
    #[regex = "[0-9][0-9][0-9][0-9]"]
    Year,
    #[regex = "[0-2][0-9]"]
    Time,
    #[token = "-"]
    DateSeparator,
    #[token = ":"]
    TimeSeparator,
    #[token = "Guard"]
    Guard,
    #[regex = "#[0-9]+"]
    GuardID,
    #[token = "begins shift"]
    ShiftBegin,
    #[token = "falls asleep"]
    ShiftAsleep,
    #[token = "wakes up"]
    ShiftAwake,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
enum Action {
    BeginsShift(usize),
    FallAsleep,
    WakeUp,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Time {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
struct Entry {
    time: Time,
    event: Action,
}

fn parse(input: &str) -> Vec<Entry> {
    let mut entries = Vec::new();

    for action in input.lines() {
        let mut lexer = Token::lexer(action);
        lexer.advance();

        let time = {
            let year = lexer.slice().parse().unwrap();
            lexer.advance();
            lexer.advance();
            let month = lexer.slice().parse().unwrap();
            lexer.advance();
            lexer.advance();
            let day = lexer.slice().parse().unwrap();
            lexer.advance();
            let hour = lexer.slice().parse().unwrap();
            lexer.advance();
            lexer.advance();
            let minute = lexer.slice().parse().unwrap();

            Time {
                year,
                month,
                day,
                hour,
                minute,
            }
        };
        lexer.advance();
        lexer.advance();

        let event = match lexer.token {
            Token::Guard => {
                lexer.advance();
                Action::BeginsShift(lexer.slice().replace("#", "").parse().unwrap())
            }
            Token::ShiftAsleep => Action::FallAsleep,
            Token::ShiftAwake => Action::WakeUp,
            _ => unreachable!(),
        };

        entries.push(Entry { time, event });
    }

    entries.sort_by_key(|k| k.time);
    entries
}

fn main() {
    println!("{}", part_one(PUZZLE));
    println!("{}", part_two(PUZZLE));
}

fn part_one(input: &str) -> usize {
    let entries = parse(input);
    let mut map: HashMap<usize, (_, _)> = HashMap::new();
    let mut current_id = 0;
    let mut sleep = 0;

    for entry in entries {
        match entry.event {
            Action::BeginsShift(id) => {
                current_id = id;
                map.entry(id).or_insert((0, [0; 60]));
            }
            Action::FallAsleep => sleep = entry.time.minute,
            Action::WakeUp => {
                let current_guard = map.get_mut(&current_id).unwrap();
                for minute in &mut current_guard.1[sleep as usize..entry.time.minute as usize] {
                    *minute += 1;
                }
                current_guard.0 += entry.time.minute - sleep;
            }
        }
    }

    let entry = map.iter().max_by_key(|k| k.1 .0).unwrap();
    let id = entry.0;

    let min = entry.1 .1.iter().enumerate().max_by_key(|k| k.1).unwrap().0;

    id * min
}

fn part_two(input: &str) -> usize {
    let entries = parse(input);
    let mut map = HashMap::new();
    let mut current_id = 0;
    let mut sleep = 0;

    for entry in entries {
        match entry.event {
            Action::BeginsShift(id) => {
                current_id = id;
                map.entry(id).or_insert([0; 60]);
            }
            Action::FallAsleep => sleep = entry.time.minute,
            Action::WakeUp => {
                let current_guard = map.get_mut(&current_id).unwrap();
                for minute in &mut current_guard[sleep as usize..entry.time.minute as usize] {
                    *minute += 1;
                }
            }
        }
    }

    let entry = map
        .into_iter()
        .map(|(a, b)| {
            (
                a,
                b.iter().cloned().enumerate().max_by_key(|v| v.1).unwrap(),
            )
        })
        .max_by_key(|v| v.1 .1)
        .unwrap();
    let id = entry.0;
    let min = entry.1 .0;

    id * min
}

#[cfg(test)]
mod day04 {
    use super::*;

    #[test]
    fn part_1() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
        assert_eq!(240, part_one(input));
        assert_eq!(119835, part_one(PUZZLE));
    }

    #[test]
    fn part_2() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
        assert_eq!(4455, part_two(input));
        assert_eq!(12725, part_two(PUZZLE));
    }

    #[test]
    fn test_lexer() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up";
        let entries = parse(input);
        println!("{:?}", entries);
        let date = Time {
            year: 1518,
            month: 11,
            day: 01,
            hour: 00,
            minute: 00,
        };
        let entry = Entry {
            time: date,
            event: Action::BeginsShift(10),
        };
        assert_eq!(entries[0], entry);

        let date = Time {
            year: 1518,
            month: 11,
            day: 01,
            hour: 00,
            minute: 05,
        };
        let entry = Entry {
            time: date,
            event: Action::FallAsleep,
        };
        assert_eq!(entries[1], entry);

        let date = Time {
            year: 1518,
            month: 11,
            day: 01,
            hour: 00,
            minute: 25,
        };
        let entry = Entry {
            time: date,
            event: Action::WakeUp,
        };
        assert_eq!(entries[2], entry);
    }

    #[test]
    fn test_lexer_guard() {
        let mut lexer = Token::lexer("[1518-11-01 00:00] Guard #10 begins shift");
        assert_eq!(lexer.token, Token::StartDate);
        lexer.advance();
        assert_eq!(lexer.token, Token::Year);
        assert_eq!(lexer.slice(), "1518");
        lexer.advance();
        assert_eq!(lexer.token, Token::DateSeparator);
        lexer.advance();
        assert_eq!(lexer.token, Token::Time);
        assert_eq!(lexer.slice(), "11");
        lexer.advance();
        assert_eq!(lexer.token, Token::DateSeparator);
        lexer.advance();
        assert_eq!(lexer.token, Token::Time);
        assert_eq!(lexer.slice(), "01");
        lexer.advance();
        assert_eq!(lexer.token, Token::Time);
        assert_eq!(lexer.slice(), "00");
        lexer.advance();
        assert_eq!(lexer.token, Token::TimeSeparator);
        lexer.advance();
        assert_eq!(lexer.token, Token::Time);
        assert_eq!(lexer.slice(), "00");
        lexer.advance();
        assert_eq!(lexer.token, Token::EndDate);
        lexer.advance();
        assert_eq!(lexer.token, Token::Guard);
        lexer.advance();
        assert_eq!(lexer.token, Token::GuardID);
        assert_eq!(lexer.slice(), "#10");
        lexer.advance();
        assert_eq!(lexer.token, Token::ShiftBegin);
        lexer.advance();
        assert_eq!(lexer.token, Token::End);
    }

    #[test]
    fn test_lexer_asleep() {
        let mut lexer = Token::lexer("[1518-11-01 00:05] falls asleep");
        assert_eq!(lexer.token, Token::StartDate);
        lexer.advance();
        assert_eq!(lexer.token, Token::Year);
        assert_eq!(lexer.slice(), "1518");
        lexer.advance();
        assert_eq!(lexer.token, Token::DateSeparator);
        lexer.advance();
        assert_eq!(lexer.token, Token::Time);
        assert_eq!(lexer.slice(), "11");
        lexer.advance();
        assert_eq!(lexer.token, Token::DateSeparator);
        lexer.advance();
        assert_eq!(lexer.token, Token::Time);
        assert_eq!(lexer.slice(), "01");
        lexer.advance();
        assert_eq!(lexer.token, Token::Time);
        assert_eq!(lexer.slice(), "00");
        lexer.advance();
        assert_eq!(lexer.token, Token::TimeSeparator);
        lexer.advance();
        assert_eq!(lexer.token, Token::Time);
        assert_eq!(lexer.slice(), "05");
        lexer.advance();
        assert_eq!(lexer.token, Token::EndDate);
        lexer.advance();
        assert_eq!(lexer.token, Token::ShiftAsleep);
    }

    #[test]
    fn test_lexer_wakes() {
        let mut lexer = Token::lexer("[1518-11-01 00:25] wakes up");
        assert_eq!(lexer.token, Token::StartDate);
        lexer.advance();
        assert_eq!(lexer.token, Token::Year);
        assert_eq!(lexer.slice(), "1518");
        lexer.advance();
        assert_eq!(lexer.token, Token::DateSeparator);
        lexer.advance();
        assert_eq!(lexer.token, Token::Time);
        assert_eq!(lexer.slice(), "11");
        lexer.advance();
        assert_eq!(lexer.token, Token::DateSeparator);
        lexer.advance();
        assert_eq!(lexer.token, Token::Time);
        assert_eq!(lexer.slice(), "01");
        lexer.advance();
        assert_eq!(lexer.token, Token::Time);
        assert_eq!(lexer.slice(), "00");
        lexer.advance();
        assert_eq!(lexer.token, Token::TimeSeparator);
        lexer.advance();
        assert_eq!(lexer.token, Token::Time);
        assert_eq!(lexer.slice(), "25");
        lexer.advance();
        assert_eq!(lexer.token, Token::EndDate);
        lexer.advance();
        assert_eq!(lexer.token, Token::ShiftAwake);
    }
}
