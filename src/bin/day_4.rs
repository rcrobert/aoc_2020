use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("inputs/day_4").expect("could not open input file");
    let reader = BufReader::new(file);

    let mut line_no = 1;
    let mut num_valid_passports = 0;
    let mut builder = PassportBuilder::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            // Blank line, end of passport
            if line.len() == 0 {
                let passport = builder.bind();
                if PassportValidator::check(&passport) {
                    num_valid_passports += 1;
                }

                builder = PassportBuilder::new();
            }
            // Consume all the passport fields on this line
            else {
                assert!(line.is_ascii());

                for kv_pair in line.split_whitespace() {
                    let mut kv_pair = kv_pair.split(':').map(String::from);
                    let key = kv_pair.next().expect("missing key");
                    let value = kv_pair.next().expect("missing value");
                    match builder.add_field(key, value) {
                        Ok(_) => (),
                        Err(e) => panic!("Error on line {}: {}", line_no, e),
                    }
                }
            }
        } else {
            panic!("failed to read line");
        }

        line_no += 1;
    }

    // Finished reading, check the final passport
    let passport = builder.bind();
    if PassportValidator::check(&passport) {
        num_valid_passports += 1;
    }

    println!("Found {} valid passports", num_valid_passports);
}

const VALID_FIELDS: [&'static str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

// "cid" is not required.
const REQUIRED_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

type Passport = HashMap<String, String>;

struct PassportBuilder {
    passport: Passport,
}

impl PassportBuilder {
    fn add_field(&mut self, key: String, value: String) -> io::Result<()> {
        // Validate input key is a known field name
        if !VALID_FIELDS.iter().any(|f| *f == key) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unknown passport field {}", key),
            ));
        }

        // Validate input key is a new field
        if self.passport.keys().any(|k| *k == key) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("duplicate passport field {}", key),
            ));
        }

        self.passport.insert(key, value);
        return Ok(());
    }

    fn bind(self) -> Passport {
        self.passport
    }

    fn new() -> Self {
        Self {
            passport: HashMap::new(),
        }
    }
}

struct PassportValidator {}

impl PassportValidator {
    fn check(passport: &Passport) -> bool {
        return REQUIRED_FIELDS.iter().all(|f| passport.contains_key(*f));
    }
}
