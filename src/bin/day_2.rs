use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("inputs/day_2").expect("could not open input file");
    let reader = BufReader::new(file);

    let mut valid_password_count: u32 = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            let mut parts = line.split(": ");
            let rule_string = parts.next().expect("invalid line");
            let password = parts.next().expect("invalid line");

            let rule = Rule::from(rule_string);
            if rule.test(password) {
                valid_password_count += 1;
            }
        }
    }

    println!("Valid passwords: {}", valid_password_count);
}

struct Rule {
    character: char,
    min: u8,
    max: u8,
}

impl Rule {
    fn test(&self, password: &str) -> bool {
        let mut count: u32 = 0;
        for character in password.chars() {
            if character == self.character {
                count += 1;
            }
        }

        return count >= self.min as u32 && count <= self.max as u32;
    }
}

impl From<&str> for Rule {
    fn from(rule_string: &str) -> Rule {
        if !rule_string.is_ascii() {
            panic!("invalid rule");
        }

        let mut parts = rule_string.split_ascii_whitespace();

        let range_string = parts.next().expect("invalid rule");

        let character: &str = parts.next().expect("invalid rule");
        let character = character.chars().nth(0).expect("invalid rule");

        let mut bounds = range_string.split('-');
        let min: u8 = bounds.next().expect("invalid rule").parse().expect("invalid rule");
        let max: u8 = bounds.next().expect("invalid rule").parse().expect("invalid rule");

        return Rule {
            character,
            min,
            max,
        };
    }
}
