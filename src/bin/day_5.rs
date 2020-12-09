extern crate my;

use my::input::InputReader;
use std::char::ParseCharError;
use std::cmp;
use std::ops;
use std::ops::{Bound, RangeBounds};
use std::str::FromStr;

const PLANE_ROWS: u16 = 128;
const PLANE_COLS: u16 = 8;

type PlaneIndex = u16;
type Range = ops::Range<PlaneIndex>;

type BoardingPass = String;

#[derive(Debug)]
struct Seat {
    boarding_pass: String,
    row: PlaneIndex,
    column: PlaneIndex,
}

fn main() {
    let reader = InputReader::new(5);

    let mut max_seat_id: u64 = 0;
    for (i, line) in reader.lines().enumerate() {
        if let Ok(seat) = line.parse::<Seat>() {
            max_seat_id = cmp::max(max_seat_id, seat.get_id());
        } else {
            panic!("Error on line {}: {}", i + 1, line);
        }
    }
    println!("Highest seat id: {}", max_seat_id);
}

impl Seat {
    fn new(boarding_pass: String, row: PlaneIndex, column: PlaneIndex) -> Self {
        Self {
            boarding_pass,
            row,
            column,
        }
    }

    fn get_id(&self) -> u64 {
        ((self.row * 8) + self.column) as u64
    }
}

impl FromStr for Seat {
    type Err = my::Error;

    fn from_str(s: &str) -> my::Result<Self> {
        let row = Self::parse_row(s)?;
        let column = Self::parse_column(s)?;
        return Ok(Self::new(String::from(s), row, column));
    }
}

// FromStr helpers
impl Seat {
    fn parse_row(s: &str) -> my::Result<PlaneIndex> {
        return reduce(s.chars().take(7), 0..PLANE_ROWS, 'F', 'B');
    }

    fn parse_column(s: &str) -> my::Result<PlaneIndex> {
        if let Some(s) = s.get(7..10) {
            return reduce(s.chars(), 0..PLANE_COLS, 'L', 'R');
        } else {
            return Err(my::Error::new());
        }
    }
}

fn unwrap_range(range: Range) -> my::Result<PlaneIndex> {
    if range.start_bound() == Bound::Unbounded {
        return Err(my::Error::new());
    } else if range.end_bound() == Bound::Unbounded {
        return Err(my::Error::new());
    }

    if range.start >= range.end || range.end - range.start != 1 {
        return Err(my::Error::new());
    }

    return Ok(range.start);
}

fn reduce<I>(
    it: I,
    mut range: Range,
    lower_specifier: char,
    upper_specifier: char,
) -> my::Result<PlaneIndex>
where
    I: Iterator<Item = char>,
{
    range = it.fold(range, |r, elem| {
        let result = decide(elem, &r, lower_specifier, upper_specifier);
        return result;
    });
    return unwrap_range(range);
}

fn decide(specifier: char, range: &Range, lower_specifier: char, upper_specifier: char) -> Range {
    let mid = range.len() / 2;
    let mid = range.start + mid as PlaneIndex;
    match specifier {
        c if c == lower_specifier => range.start..mid,
        c if c == upper_specifier => mid..range.end,
        _ => panic!("unknown airline specifier {}", specifier),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_inputs() {
        assert_eq!(make_seat("BFFFBBFRRR").row, 70);
        assert_eq!(make_seat("BFFFBBFRRR").column, 7);
        assert_eq!(make_seat("BFFFBBFRRR").get_id(), 567);

        assert_eq!(make_seat("FFFBBBFRRR").row, 14);
        assert_eq!(make_seat("FFFBBBFRRR").column, 7);
        assert_eq!(make_seat("FFFBBBFRRR").get_id(), 119);

        assert_eq!(make_seat("BBFFBBFRLL").row, 102);
        assert_eq!(make_seat("BBFFBBFRLL").column, 4);
        assert_eq!(make_seat("BBFFBBFRLL").get_id(), 820);
    }

    fn make_seat(s: &'static str) -> Seat {
        s.parse::<Seat>().expect("failed to parse")
    }
}
