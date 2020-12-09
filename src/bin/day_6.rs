extern crate my;

use my::input::InputReader;
use std::collections::BTreeSet;

struct GroupAnswers {
    answers: BTreeSet<char>,
}

fn main() {
    let reader = InputReader::new(6);

    let mut accum: u64 = 0;
    let mut current_group = GroupAnswers::new();
    for line in reader.lines() {
        // Empty line is the group delimiter, start tracking a new one
        if line.len() == 0 {
            accum += current_group.answers.len() as u64;
            current_group = GroupAnswers::new();
            continue;
        }
        for c in line.chars() {
            current_group.answers.insert(c);
        }
    }

    // Finally tally the last group
    accum += current_group.answers.len() as u64;

    println!("Magic sum: {}", accum);
}

impl GroupAnswers {
    fn new() -> Self {
        Self {
            answers: BTreeSet::new(),
        }
    }
}
