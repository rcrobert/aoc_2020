extern crate my;

use my::input::InputReader;
use std::collections::BTreeSet;

type Answers = BTreeSet<char>;
struct GroupAnswers {
    answers: Vec<Answers>,
}

fn main() {
    let reader = InputReader::new(6);

    let mut accum: u64 = 0;
    let mut current_group = GroupAnswers::new();
    for line in reader.lines() {
        // Empty line is the group delimiter, start tracking a new one
        if line.len() == 0 {
            accum += tally_group(&current_group);
            current_group = GroupAnswers::new();
            continue;
        }
        let mut passenger_answers = BTreeSet::<char>::new();
        for c in line.chars() {
            passenger_answers.insert(c);
        }
        current_group.answers.push(passenger_answers);
    }

    // Finally tally the last group
    accum += tally_group(&current_group);

    println!("Magic sum: {}", accum);
}

/// Returns the number of questions every group member responded 'True' to.
fn tally_group(g: &GroupAnswers) -> u64 {
    if g.answers.len() == 0 {
        return 0;
    }

    let mut answers = g.answers.iter();
    let initial: Answers = answers.next().unwrap().clone();
    let common_answers = answers.fold(initial, |accum, member_answers| {
        accum
            .intersection(member_answers)
            .cloned()
            .collect()
    });

    return common_answers.len() as u64;
}

impl GroupAnswers {
    fn new() -> Self {
        Self {
            answers: Vec::new(),
        }
    }
}
