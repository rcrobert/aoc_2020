use std::fs::File;
use std::io;

pub use std::io::BufRead;

type BufReader = io::BufReader<File>;

pub struct InputReader {
    reader: BufReader,
}

impl InputReader {
    pub fn new(day: u8) -> Self {
        let file = File::open(Self::input_file(day)).expect("could not open input file");
        let reader = BufReader::new(file);
        Self {
            reader,
        }
    }

    pub fn lines(self) -> Lines {
        Lines {
            inner: self.reader.lines()
        }
    }

    fn input_file(day: u8) -> String {
        format!("inputs/day_{}", day)
    }
}

pub struct Lines {
    inner: io::Lines<BufReader>,
}

impl Iterator for Lines {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        // Unwrap the inner result and just panic
        if let Some(result) = self.inner.next() {
            if let Ok(result) = result {
                return Some(result);
            } else {
                panic!("failed to read line");
            }
        } else {
            return None;
        }
    }
}
