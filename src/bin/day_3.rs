extern crate my;

use my::input::InputReader;

fn main() {
    let reader = InputReader::new(3);

    // Build the map
    let mut map_details: Vec<Vec<Landmark>> = Vec::new();
    for line in reader.lines() {
        let mut map_row: Vec<Landmark> = Vec::new();
        for character in line.chars() {
            match character {
                '.' => map_row.push(Landmark::FreshPow),
                '#' => map_row.push(Landmark::Tree),
                _ => panic!("invalid input file"),
            }
        }
        map_details.push(map_row);
    }

    let toboggans = vec![
        Toboggan::new(1, 1),
        Toboggan::new(1, 3),
        Toboggan::new(1, 5),
        Toboggan::new(1, 7),
        Toboggan::new(2, 1),
    ];

    // Follow the route

    let map = TobogganMap::new(map_details);
    let mut strange_collision_product = 0;
    for toboggan in toboggans.iter() {
        let num_collisions = count_tree_strikes(&map, &toboggan);
        strange_collision_product = match strange_collision_product {
            0 => num_collisions,
            _ => strange_collision_product * num_collisions,
        };
    }

    println!("Weird answer?: {}", strange_collision_product);
}

fn count_tree_strikes(map: &TobogganMap, toboggan: &Toboggan) -> usize {
    let mut num_collisions = 0;

    for location in toboggan.slide() {
        // Check if we have reached the bottom of the slope
        if location.y as usize >= map.height() {
            break;
        }

        match map.get(location) {
            Some(Landmark::Tree) => num_collisions += 1,
            Some(Landmark::FreshPow) => (),
            None => panic!("here be dragons! {:?} is uncharted territory", location),
        }
    }

    return num_collisions;
}

/// A location within a [TobogganMap].
///
/// [TobogganMap] is zeroed at the upper-left side of the slope so coordinates can be unsigned.
#[derive(Clone, Copy, Debug)]
struct Coordinate {
    x: u32,
    y: u32,
}

/// Points of interest within a [TobogganMap].
enum Landmark {
    /// Map location containing fresh powder, a dream for all tobogganers.
    FreshPow,
    /// Map location containing a dangerous tree.
    Tree,
}

/// Map for brave tobogganers to navigate themselves down a scary slope.
struct TobogganMap {
    // Row-major matrix of landmarks.
    details: Vec<Vec<Landmark>>,
    height: usize,
}

impl TobogganMap {
    fn new(details: Vec<Vec<Landmark>>) -> Self {
        let height = details.len();
        Self { details, height }
    }

    /// Returns a reference to the [Landmark] from a map location or None if the location is out of
    /// range.
    fn get(&self, coordinate: Coordinate) -> Option<&Landmark> {
        if let Some(slope_row) = self.details.get(coordinate.y as usize) {
            let width = slope_row.len();
            return Some(
                slope_row
                    .get(coordinate.x as usize % width)
                    .expect("invalid map construction"),
            );
        } else {
            None
        }
    }

    /// The height of the mountain slope that the map covers.
    fn height(&self) -> usize {
        self.height
    }
}

/// Toboggans follow very determinate paths defined by their construction.
#[derive(Clone, Copy, Debug)]
struct Toboggan {
    /// The number of map altitude levels this toboggan descends per unit time slice.
    descent_rate: u32,
    /// The number of map longitude levels this toboggan slides per unit time slice.
    slide_rate: u32,
}

impl Toboggan {
    fn new(descent_rate: u32, slide_rate: u32) -> Self {
        Self {
            descent_rate,
            slide_rate,
        }
    }

    fn slide<'a>(&'a self) -> TobogganPath<'a> {
        TobogganPath {
            parent: &self,
            next_pos: Coordinate { x: 0, y: 0 },
        }
    }
}

struct TobogganPath<'s> {
    parent: &'s Toboggan,
    next_pos: Coordinate,
}

impl<'s> Iterator for TobogganPath<'s> {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        let yield_pos = self.next_pos;
        self.next_pos.x += self.parent.slide_rate;
        self.next_pos.y += self.parent.descent_rate;
        return Some(yield_pos);
    }
}
