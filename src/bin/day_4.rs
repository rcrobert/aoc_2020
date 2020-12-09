use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Range;

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
                if PassportValidator::new(&passport).check() {
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
    if PassportValidator::new(&passport).check() {
        num_valid_passports += 1;
    }

    println!("Found {} valid passports", num_valid_passports);
}

const VALID_FIELDS: [&'static str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

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

struct PassportValidator<'a> {
    passport: &'a Passport,
}

impl<'a> PassportValidator<'a> {
    fn new(passport: &'a Passport) -> Self {
        Self { passport }
    }

    fn check(&self) -> bool {
        let checks = [
            Self::check_required_fields,
            Self::check_byr,
            Self::check_iyr,
            Self::check_eyr,
            Self::check_hgt,
            Self::check_hcl,
            Self::check_ecl,
            Self::check_pid,
        ];

        return checks.iter().all(|c| c(self));
    }

    // "cid" is not required.
    const REQUIRED_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    fn check_required_fields(&self) -> bool {
        Self::REQUIRED_FIELDS
            .iter()
            .all(|f| self.passport.contains_key(*f))
    }

    const BYR_RANGE: Range<u32> = 1920..2003;

    fn check_byr(&self) -> bool {
        return self.check_in_range("byr", &Self::BYR_RANGE);
    }

    const IYR_RANGE: Range<u32> = 2010..2021;

    fn check_iyr(&self) -> bool {
        return self.check_in_range("iyr", &Self::IYR_RANGE);
    }

    const EYR_RANGE: Range<u32> = 2020..2031;

    fn check_eyr(&self) -> bool {
        return self.check_in_range("eyr", &Self::EYR_RANGE);
    }

    fn check_in_range(&self, key: &str, range: &Range<u32>) -> bool {
        let value = self.passport.get(key).expect("missing required field");
        let value = value
            .parse()
            .expect(format!("could not parse {}", key).as_str());
        return range.contains(&value);
    }

    const HGT_CM_RANGE: Range<u32> = 150..194;
    const HGT_IN_RANGE: Range<u32> = 59..77;

    fn check_hgt(&self) -> bool {
        let value = self.passport.get("hgt").expect("missing required field");
        if let Some(value) = value.strip_suffix("cm") {
            let value = value.parse().expect("could not parse hgt");
            return Self::HGT_CM_RANGE.contains(&value);
        } else if let Some(value) = value.strip_suffix("in") {
            let value = value.parse().expect("could not parse hgt");
            return Self::HGT_IN_RANGE.contains(&value);
        } else {
            return false;
        }
    }

    const HCL_VALID_CHARS: &'static str = "0123456789abcdef";
    const HCL_VALID_LEN: u8 = 6;

    fn check_hcl(&self) -> bool {
        let value = self.passport.get("hcl").expect("missing required field");
        let mut chars = value.chars();

        // Check for the leading '#'
        // it is also required to consume this for the next test
        if let Some(c) = chars.next() {
            if c != '#' {
                return false;
            }
        } else {
            return false;
        }

        // Ensure remaining chars are legal hex
        if !chars.all(|c| Self::HCL_VALID_CHARS.contains(c)) {
            return false;
        }

        // Refresh iterator to count
        let mut chars = value.chars();
        // Consume HCL_VALID_LEN chars, nth() is zero indexed but there is also a leading
        // '#' so they cancel out in length.
        if let None = chars.nth(Self::HCL_VALID_LEN as usize) {
            return false;
        }

        // There should be no remaining chars
        if let Some(_) = chars.next() {
            return false;
        }
        return true;
    }

    const ECL_VALID_ENTRIES: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    fn check_ecl(&self) -> bool {
        let value = self.passport.get("ecl").expect("missing required field");
        return Self::ECL_VALID_ENTRIES.iter().any(|entry| *entry == value);
    }

    fn check_pid(&self) -> bool {
        let value = self.passport.get("pid").expect("missing required field");

        if value.len() != 9 {
            return false;
        }

        return match value.parse::<u32>() {
            Ok(_) => true,
            Err(_) => false,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Maker {
        passport: Passport,
    }

    impl Maker {
        fn new() -> Self {
            Maker {
                passport: Passport::new(),
            }
        }

        fn with(mut self, key: &str, value: &str) -> Self {
            self.passport.insert(String::from(key), String::from(value));
            self
        }

        fn done(self) -> Passport {
            self.passport
        }
    }

    mod passport_validator {
        use super::*;

        #[test]
        fn test_check_byr_boundaries() {
            let p = Maker::new().with("byr", "1920").done();
            assert!(PassportValidator::new(&p).check_byr());
            let p = Maker::new().with("byr", "2002").done();
            assert!(PassportValidator::new(&p).check_byr());
        }

        #[test]
        fn test_check_byr_out_of_bounds() {
            let p = Maker::new().with("byr", "2003").done();
            assert!(!PassportValidator::new(&p).check_byr());
            let p = Maker::new().with("byr", "1919").done();
            assert!(!PassportValidator::new(&p).check_byr());
        }

        #[test]
        fn test_check_hgt_boundaries() {
            // cm
            let p = Maker::new().with("hgt", "150cm").done();
            assert!(PassportValidator::new(&p).check_hgt());
            let p = Maker::new().with("hgt", "193cm").done();
            assert!(PassportValidator::new(&p).check_hgt());

            // cm
            let p = Maker::new().with("hgt", "59in").done();
            assert!(PassportValidator::new(&p).check_hgt());
            let p = Maker::new().with("hgt", "76in").done();
            assert!(PassportValidator::new(&p).check_hgt());
        }

        #[test]
        fn test_check_hgt_out_of_bounds() {
            // cm
            let p = Maker::new().with("hgt", "149cm").done();
            assert!(!PassportValidator::new(&p).check_hgt());
            let p = Maker::new().with("hgt", "194cm").done();
            assert!(!PassportValidator::new(&p).check_hgt());

            // cm
            let p = Maker::new().with("hgt", "58in").done();
            assert!(!PassportValidator::new(&p).check_hgt());
            let p = Maker::new().with("hgt", "77in").done();
            assert!(!PassportValidator::new(&p).check_hgt());
        }

        #[test]
        fn test_check_hcl_accepts_all_hex_characters() {
            let p = Maker::new().with("hcl", "#abcdef").done();
            assert!(PassportValidator::new(&p).check_hcl());
            let p = Maker::new().with("hcl", "#012345").done();
            assert!(PassportValidator::new(&p).check_hcl());
            let p = Maker::new().with("hcl", "#6789ab").done();
            assert!(PassportValidator::new(&p).check_hcl());
        }

        #[test]
        fn test_check_hcl_rejects_non_hex_characters() {
            let p = Maker::new().with("hcl", "#xve--@").done();
            assert!(!PassportValidator::new(&p).check_hcl());
        }

        #[test]
        fn test_check_hcl_requires_hash_prefix() {
            let p = Maker::new().with("hcl", "abcdef0").done();
            assert!(!PassportValidator::new(&p).check_hcl());

            let p = Maker::new().with("hcl", "a#cdef0").done();
            assert!(!PassportValidator::new(&p).check_hcl());
        }

        #[test]
        fn test_check_hcl_requires_6_characters() {
            let p = Maker::new().with("hcl", "#0123").done();
            assert!(!PassportValidator::new(&p).check_hcl());

            let p = Maker::new().with("hcl", "#abcdef0").done();
            assert!(!PassportValidator::new(&p).check_hcl());
        }

        #[test]
        fn test_check_ecl_accepts_all_valid_colors() {
            const VALID_ECL: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            for color in VALID_ECL.iter() {
                let p = Maker::new().with("ecl", color).done();
                assert!(PassportValidator::new(&p).check_ecl());
            }
        }

        #[test]
        fn test_check_ecl_rejects_other_colors() {
            let p = Maker::new().with("ecl", "red").done();
            assert!(!PassportValidator::new(&p).check_ecl());
        }

        #[test]
        fn test_check_pid_accepts_any_9_digit_number() {
            let p = Maker::new().with("pid", "915789426").done();
            assert!(PassportValidator::new(&p).check_pid());

            let p = Maker::new().with("pid", "000789426").done();
            assert!(PassportValidator::new(&p).check_pid());
        }
    }
}
