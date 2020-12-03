
#[macro_use]
extern crate lazy_static;

extern crate regex;

use std::io::{BufRead, BufReader};
use std::fs::File;

use regex::Regex;

lazy_static! {
    static ref PASSWORD_REGEX : Regex = Regex::new("(\\d+)-(\\d+) (.): (.*)").unwrap();
}

fn split_password_entry(entry: &str) -> (usize,usize,char,String) {
    let matches = PASSWORD_REGEX.captures(&entry).unwrap();

    let min = matches[1].parse::<usize>().unwrap();
    let max = matches[2].parse::<usize>().unwrap();

    let required_char = matches[3].chars().next().unwrap();

    (min, max, required_char, matches[4].to_owned())
}

fn check_policy_1(entry: &str) -> bool {
    let (min, max, required, password) = split_password_entry(entry);

    let mut char_count = 0;
    for c in password.chars() {
        if c == required {
            char_count += 1;
        }
    }
    (char_count >= min) && (char_count <= max)
}

fn check_policy_2(entry: &str) -> bool {
    let (pos1, pos2, required, password) = split_password_entry(entry);

    let mut match_count = 0;
    for (index, c) in password.char_indices() {
        let one_based_index = index+1;
        if (one_based_index == pos1) || (one_based_index == pos2) {
            if required == c {
                match_count += 1;
            }
        }
    }

    match_count == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_1() {
        assert_eq!(check_policy_1("1-3 a: abcde"), true);
        assert_eq!(check_policy_1("1-3 b: cdefg"), false);
        assert_eq!(check_policy_1("2-9 c: ccccccccc"), true);
        assert_eq!(check_policy_1("10-20 .: ....333....asf...g"), true);
        assert_eq!(check_policy_1("0-0 b: aaaaaaaaa"), true);
        assert_eq!(check_policy_1("2-4 ü: ßßßuuüasð--Üüaaa"), true);
    }

    #[test]
    fn policy_2() {
        assert_eq!(check_policy_2("1-3 a: abcde"), true);
        assert_eq!(check_policy_2("1-3 b: cdefg"), false);
        assert_eq!(check_policy_2("2-9 c: ccccccccc"), false);
    }
}

fn main() {

    let mut pw_count = 0;
    let mut policy1_valid_count = 0;
    let mut policy2_valid_count = 0;

    let f = File::open("day2_input.txt").expect("Failed to open input file");
    let reader = BufReader::new(f);
    for line_result in reader.lines() {
        let line = line_result.expect("Failed to read line");
        pw_count += 1;
        if check_policy_1(&line) {
            policy1_valid_count += 1;
        }
        if check_policy_2(&line) {
            policy2_valid_count += 1;
        }
    }

    println!("Out of {} passwords, {} are valid according to policy 1, {} are valid according to policy 2", pw_count, policy1_valid_count, policy2_valid_count);
}
