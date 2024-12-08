use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::point::Point;

pub type Map = HashMap<char, HashSet<Point>>;

/// A Grid is a cartesian map of cells with some input contents.
pub struct Grid {
    pub map: Map,
    pub height: i32,
    pub width: i32,
}

impl Grid {
    pub fn new(input: &str) -> Grid {
        Grid {
            map: Grid::grid_map(input),
            height: input.lines().count() as i32,
            width: input.lines().next().unwrap().trim().len() as i32,
        }
    }

    fn grid_map(input: &str) -> Map {
        input
            .lines()
            .map(|l| l.trim())
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate()
                    .map(move |(x, c)| (c, Point::new(x as i32, y as i32)))
            })
            .into_grouping_map()
            .collect()
    }

    pub fn valid(&self, p: &Point) -> bool {
        p.x >= 0 && p.x < self.width && p.y >= 0 && p.y < self.height
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn sample1() {
        let game = Grid::new(SAMPLE);
        assert_eq!(game.map.len(), 3);
        assert_eq!(game.height, 12);
        assert_eq!(game.width, 12);
        assert_eq!(game.map[&'A'].len(), 3);
        assert_eq!(game.map[&'0'].len(), 4);
        assert_eq!(game.map[&'.'].len(), 144 - 7);
    }
}