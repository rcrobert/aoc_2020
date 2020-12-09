use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type InputType = i32;
const SHITHOLE_YEAR: InputType = 2020;

fn main() {
    let mut file = File::open("inputs/day_1").expect("could not open input file");

    let inputs = collect_inputs(&mut file);
    do_magic(&inputs);
}

fn collect_inputs(file: &mut File) -> Vec<InputType> {
    let mut reader = BufReader::new(file);

    let mut inputs = Vec::<InputType>::new();

    loop {
        let mut buf = String::new();
        match reader.read_line(&mut buf) {
            Ok(x) if x != 0 => {
                if let Ok(val) = buf.trim_end().parse::<InputType>() {
                    inputs.push(val);
                } else {
                    panic!("failed to parse line {}", buf);
                }
            }

            // Finished or failed reading
            Ok(_) => break,
            Err(_) => panic!("failed to read line"),
        }
    }

    return inputs;
}

fn do_magic(values: &Vec<InputType>) {
    for i in 0..values.len() {
        for j in i + 1..values.len() {
            for k in j + 1..values.len() {
                let candidate: Vec<InputType> = vec![values[i], values[j], values[k]];
                let sum = candidate.iter().sum::<InputType>();
                if sum == SHITHOLE_YEAR {
                    // Found it
                    println!("Result: {}", candidate.iter().product::<InputType>());
                    return;
                }
            }
        }
    }
    panic!("could not find a {} summing pair", SHITHOLE_YEAR);
}
