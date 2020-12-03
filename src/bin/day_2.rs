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
    positions: Vec<usize>,
}

impl Rule {
    fn test(&self, password: &str) -> bool {
        let mut count: u32 = 0;
        for (index, character) in password.char_indices() {
            let corporate_position = index + 1;
            if self.positions.contains(&corporate_position) {
                if character == self.character {
                    count += 1;
                }
            }
        }

        return count == 1;
    }
}

impl From<&str> for Rule {
    fn from(rule_string: &str) -> Rule {
        if !rule_string.is_ascii() {
            panic!("invalid rule");
        }

        let mut parts = rule_string.split_ascii_whitespace();

        let positions_string = parts.next().expect("invalid rule");

        let character: &str = parts.next().expect("invalid rule");
        let character = character.chars().nth(0).expect("invalid rule");

        let positions = positions_string.split('-');
        let positions = positions.map(|v| v.parse().expect("invalid rule")).collect::<Vec<usize>>();

        return Rule {
            character,
            positions,
        };
    }
}
