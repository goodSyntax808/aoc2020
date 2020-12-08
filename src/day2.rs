use crate::read_nums;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Range;

fn is_valid_password(range: Range<usize>, allowed_char: char, password: &str) -> bool {
    let char_count = password.chars().filter(|&c| c == allowed_char).count();
    let is_valid = range.contains(&char_count);
    is_valid
}

fn count_valid_passwords() -> usize {
    let mut file = File::open("resources/day2.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut count = 0;
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let mut range_iter = iter.next().unwrap().split('-');
        let range = Range {
            start: range_iter.next().unwrap().parse::<usize>().unwrap(),
            end: range_iter.next().unwrap().parse::<usize>().unwrap() + 1,
        };
        let allowed_char = iter.next().unwrap().chars().next().unwrap();
        let password = iter.next().unwrap();
        if is_valid_password(range, allowed_char, password) {
            count += 1;
        }
    }

    count
}

fn is_valid_password2(range: Range<usize>, allowed_char: char, password: &str) -> bool {
    (password.chars().nth(range.start).unwrap() == allowed_char)
        ^ (password.chars().nth(range.end).unwrap() == allowed_char)
}

fn count_valid_passwords2() -> usize {
    let mut file = File::open("resources/day2.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut count = 0;
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let mut range_iter = iter.next().unwrap().split('-');
        let range = Range {
            start: range_iter.next().unwrap().parse::<usize>().unwrap() - 1,
            end: range_iter.next().unwrap().parse::<usize>().unwrap() - 1,
        };
        let allowed_char = iter.next().unwrap().chars().next().unwrap();
        let password = iter.next().unwrap();
        if is_valid_password2(range, allowed_char, password) {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_task1() {
        let x = count_valid_passwords();
        dbg!(x);
        assert_eq!(true, true);
    }

    #[test]
    fn run_task2() {
        let day_2_pt_2_answer = count_valid_passwords2();
        dbg!(day_2_pt_2_answer);
        assert_eq!(true, true);
    }
}
