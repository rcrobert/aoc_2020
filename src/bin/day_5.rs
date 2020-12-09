extern crate my;

use my::input::InputReader;
use std::cmp;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::ops;
use std::ops::Bound::{Excluded, Unbounded};
use std::ops::RangeBounds;
use std::str::FromStr;

const PLANE_ROWS: u16 = 128;
const PLANE_COLS: u16 = 8;

type PlaneIndex = u16;
type SeatId = u64;
type Set = BTreeSet<SeatIdRange>;
type Range = ops::Range<PlaneIndex>;

#[derive(Debug)]
struct Seat {
    boarding_pass: String,
    row: PlaneIndex,
    column: PlaneIndex,
}

fn main() {
    let reader = InputReader::new(5);

    // Build contiguous seat ranges
    let mut seat_id_ranges = Set::new();
    for (i, line) in reader.lines().enumerate() {
        if let Ok(seat) = line.parse::<Seat>() {
            let seat_id = seat.get_id();
            range_insert(&mut seat_id_ranges, seat_id);
        } else {
            panic!("Error on line {}: {}", i + 1, line);
        }
    }

    // In the end, there are just two contiguous ranges because this is a fully booked plane, your
    // seat is between them.
    // 
    // Assert there are exactly two and their distance is 1
    {
        assert!(seat_id_ranges.len() == 2);
        let mut ranges = seat_id_ranges.iter();
        assert_eq!(range_distance(ranges.next().unwrap(), ranges.next().unwrap()), 1);
    }

    let your_seat_id = seat_id_ranges.iter().next().unwrap().end;
    println!("My seat is {}", your_seat_id);
}

impl Seat {
    fn new(boarding_pass: String, row: PlaneIndex, column: PlaneIndex) -> Self {
        Self {
            boarding_pass,
            row,
            column,
        }
    }

    fn get_id(&self) -> SeatId {
        ((self.row * 8) + self.column) as SeatId
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
    if range.start_bound() == Unbounded {
        return Err(my::Error::new());
    } else if range.end_bound() == Unbounded {
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

fn range_insert(s: &mut Set, id: SeatId) {
    let id_as_range = SeatIdRange::from(id);

    // Some(SeatIdRange) if we should grow this range tail
    let before_range = match s.range((Unbounded, Excluded(&id_as_range))).next_back() {
        Some(&range) => {
            if range.end == id {
                Some(range)
            } else {
                None
            }
        }
        None => None,
    };

    // Some(SeatIdRange) if we should grow this range head
    let after_range = match s.range((Excluded(&id_as_range), Unbounded)).next() {
        Some(&range) => {
            if range.start == id + 1 {
                Some(range)
            } else {
                None
            }
        }
        None => None,
    };

    // Build the new range to insert, remove mergeable ranges as needed
    let mut range_accum = id_as_range;

    if let Some(before_range) = before_range {
        if range_can_merge(&id_as_range, &before_range) {
            range_accum = range_merge(&range_accum, &before_range);
            s.remove(&before_range);
        }
    }

    if let Some(after_range) = after_range {
        if range_can_merge(&id_as_range, &after_range) {
            range_accum = range_merge(&range_accum, &after_range);
            s.remove(&after_range);
        }
    }

    s.insert(range_accum);
}

fn range_can_merge(l: &SeatIdRange, r: &SeatIdRange) -> bool {
    (l.start <= r.start && l.end >= r.start) || (l.start <= r.end && l.end >= r.end)
}

fn range_merge(l: &SeatIdRange, r: &SeatIdRange) -> SeatIdRange {
    SeatIdRange::new(cmp::min(l.start, r.start), cmp::max(l.end, r.end))
}

fn range_distance(l: &SeatIdRange, r: &SeatIdRange) -> usize {
    if range_can_merge(l, r) {
        0
    } else if l < r {
        (r.start - l.end) as usize
    } else if r < l {
        (l.start - r.end) as usize
    } else {
        panic!("equal ranges not covered by merge");
    }
}

/// Range variant that provides a total ordering on range starts.
///
/// SeatIdRange is always [start, end)
#[derive(Copy, Clone, Eq, Debug)]
struct SeatIdRange {
    start: SeatId,
    end: SeatId,
}

impl SeatIdRange {
    fn new(start: SeatId, end: SeatId) -> Self {
        Self { start, end }
    }
}

impl From<SeatId> for SeatIdRange {
    fn from(id: SeatId) -> Self {
        Self::new(id, id + 1)
    }
}

impl Ord for SeatIdRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for SeatIdRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SeatIdRange {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
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

    mod seat_id_range {
        use super::*;

        #[test]
        fn test_empty_insert() {
            let mut s = Set::new();
            let id = 10;
            range_insert(&mut s, id);

            let id_range = SeatIdRange::from(id);
            assert_eq!(s.get(&id_range), Some(&id_range));
        }

        #[test]
        fn test_disjoint_insert() {
            let mut s = Set::new();
            let id = 10;
            let other_id = 15;
            range_insert(&mut s, id);
            range_insert(&mut s, other_id);

            let mut contents = s.iter();
            assert_eq!(contents.next(), Some(&SeatIdRange::from(id)));
            assert_eq!(contents.next(), Some(&SeatIdRange::from(other_id)));
            assert_eq!(contents.next(), None);
        }

        #[test]
        fn test_merging_lower_bound() {
            let mut s = Set::new();
            let id = 10;
            let other_id = 9;
            range_insert(&mut s, id);
            range_insert(&mut s, other_id);

            let mut contents = s.iter();
            assert_eq!(contents.next(), Some(&SeatIdRange::new(9, 11)));
            assert_eq!(contents.next(), None);
        }

        #[test]
        fn test_merging_upper_bound() {
            let mut s = Set::new();
            let id = 10;
            let other_id = 11;
            range_insert(&mut s, id);
            range_insert(&mut s, other_id);

            let mut contents = s.iter();
            assert_eq!(contents.next(), Some(&SeatIdRange::new(10, 12)));
            assert_eq!(contents.next(), None);
        }

        #[test]
        fn test_merging_upper_and_lower_bound() {
            let mut s = Set::new();
            let id = 10;
            let other_id = 12;
            let joining_id = 11;
            range_insert(&mut s, id);
            range_insert(&mut s, other_id);
            range_insert(&mut s, joining_id);

            let mut contents = s.iter();
            assert_eq!(contents.next(), Some(&SeatIdRange::new(10, 13)));
            assert_eq!(contents.next(), None);
        }

        #[test]
        fn test_merging_equal_ranges() {
            let mut s = Set::new();
            let id = 10;
            let other_id = 10;
            range_insert(&mut s, id);
            range_insert(&mut s, other_id);

            let mut contents = s.iter();
            assert_eq!(contents.next(), Some(&SeatIdRange::new(10, 11)));
            assert_eq!(contents.next(), None);
        }

        #[test]
        fn test_merge_boundaries() {
            let mut s = Set::new();
            let id = 10;
            let other_id = 12;
            range_insert(&mut s, id);
            range_insert(&mut s, other_id);

            let mut contents = s.iter();
            assert_eq!(contents.next(), Some(&SeatIdRange::from(id)));
            assert_eq!(contents.next(), Some(&SeatIdRange::from(other_id)));
            assert_eq!(contents.next(), None);
        }
    }
}
