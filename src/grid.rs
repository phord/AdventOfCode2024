use std::{collections::{HashMap, HashSet}, ops::Index};
use itertools::Itertools;
use crate::point::Point;

// TODO: let cells hold more than just char;  Grid<Node>, for example.

type Carte = Vec<Vec<char>>;
type CarteMap = HashMap<Point, char>;
pub type Map = HashMap<char, HashSet<Point>>;

/// A Grid is a cartesian map of cells with some input contents.
pub struct Grid {
    /// A map of all cell values to the Points containing that value; always constrained to the input size
    pub map: Map,

    /// A cartesian map of cells, for when you need it
    pub carte: Carte,

    // Size of the input grid
    pub height: i32,
    pub width: i32,

}

const EMPTY_VALUE: char = ' ';

/// A Grid that allows empty cells and cells anywhere
pub struct InfiniteGrid {
    pub carte: CarteMap,
}

impl InfiniteGrid {
    pub fn new(input: &str) -> InfiniteGrid {
        InfiniteGrid::new_from(&Grid::new(input))
    }

    pub fn new_from(grid: &Grid) -> InfiniteGrid {
        InfiniteGrid {
            carte: grid.map.iter().flat_map(|(c, points)| points.iter().map(move |p| (*p, *c))).collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.carte.len()
    }

    pub fn is_empty(&self) -> bool {
        self.carte.is_empty()
    }

    pub fn top(&self) -> i32 {
        self.carte.keys().map(|p| p.y).max().unwrap_or_default()
    }

    pub fn bottom(&self) -> i32 {
        self.carte.keys().map(|p| p.y).min().unwrap_or_default()
    }

    pub fn left(&self) -> i32 {
        self.carte.keys().map(|p| p.x).min().unwrap_or_default()
    }

    pub fn right(&self) -> i32 {
        self.carte.keys().map(|p| p.x).max().unwrap_or_default()
    }

    pub fn height(&self) -> i32 {
        self.top() - self.bottom() + 1
    }

    pub fn width(&self) -> i32 {
        self.right() - self.left() + 1
    }

    pub fn rotate_right_around(&self, center: &Point) -> InfiniteGrid {
        InfiniteGrid {
            carte: self.carte.iter().map(|(p, c)| (p.rotate_right_around(center), *c)).collect(),
        }
    }

    pub fn rotate_left_around(&self, center: &Point) -> InfiniteGrid {
        InfiniteGrid {
            carte: self.carte.iter().map(|(p, c)| (p.rotate_left_around(center), *c)).collect(),
        }
    }

    pub fn rotate_right(&self) -> InfiniteGrid {
        let width = self.right();
        let origin = Point::new(0, 0);
        self.rotate_right_around(&origin).translate(&Point::new(0, - width + 1))
    }

    pub fn rotate_left(&self) -> InfiniteGrid {
        let height = self.height();
        let origin = Point::new(0, 0);
        self.rotate_left_around(&origin).translate(&Point::new(- height + 1, 0))
    }

    /// Shift the cells in the grid to new origin @ delta
    pub fn translate(&self, delta: &Point) -> InfiniteGrid {
        InfiniteGrid {
            carte: self.carte.iter().map(|(p, c)| (*p - *delta, *c)).collect(),
        }
    }
}



impl core::fmt::Display for InfiniteGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert!(self.top() >= self.bottom());
        assert!(self.right() >= self.left());
        for y in (self.bottom()..=self.top()).rev() {
            for x in self.left()..=self.right() {
                write!(f, "{}", self[&Point::new(x, y)])?;
            }
            if y > self.bottom() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Index<&Point> for InfiniteGrid {
    type Output = char;
    fn index(&self, p: &Point) -> &Self::Output {
        if let Some(c) = self.carte.get(p) {
            c
        } else {
            &EMPTY_VALUE
        }
    }
}

impl Grid {

    pub fn new(input: &str) -> Grid {
        Grid {
            map: Grid::grid_map(input),
            carte: input.lines().rev().map(|l| l.trim().chars().collect()).collect(),
            height: input.lines().count() as i32,
            width: input.lines().next().unwrap().trim().len() as i32,
        }
    }

    pub fn new_from(ig: &InfiniteGrid) -> Grid {
        Grid::new(format!("{}", ig).as_str())
    }

    fn grid_map(input: &str) -> Map {
        let height = input.lines().count();
        input
            .lines()
            .map(|l| l.trim())
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate()
                    .map(move |(x, c)| (c, Point::new(x as i32, (height - y - 1) as i32)))
            })
            .into_grouping_map()
            .collect()
    }

    pub fn valid(&self, p: &Point) -> bool {
        p.x >= 0 && p.x < self.width && p.y >= 0 && p.y < self.height
    }

    pub fn set(&mut self, p: &Point, c: char) {
        let old = self[p];
        self.map.get_mut(&old).unwrap().remove(p);
        self.map.entry(c).or_default().insert(*p);
        self.carte[p.y as usize][p.x as usize] = c;
    }
}

impl core::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, line) in self.carte.iter().enumerate().rev() {
            for c in line.iter() {
                write!(f, "{}", c)?;
            }
            if y > 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}


impl Index<&Point> for Grid {
    type Output = char;
    fn index(&self, p: &Point) -> &Self::Output {
        assert!(self.valid(p));
        &self.carte[p.y as usize][p.x as usize]
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



const TINY: &str = "abc
def
ghi";

const TINY_ROT_RIGHT1: &str = "gda
heb
ifc";

const TINY_ROT_RIGHT2: &str = "ihg
fed
cba";

const TINY_ROT_RIGHT3: &str = "cfi
beh
adg";


const ASYMETRIC: &str = "abcd\nefgh\nijkl";
const ASYMETRIC_ROT_RIGHT1: &str = "iea\njfb\nkgc\nlhd";
const ASYMETRIC_ROT_RIGHT2: &str = "lkji\nhgfe\ndcba";
const ASYMETRIC_ROT_RIGHT3: &str = "dhl\ncgk\nbfj\naei";

    #[test]
    fn grid_basic() {
        let game = Grid::new(SAMPLE);
        assert_eq!(game.map.len(), 3);
        assert_eq!(game.height, 12);
        assert_eq!(game.width, 12);
        assert_eq!(game.map[&'A'].len(), 3);
        assert_eq!(game.map[&'0'].len(), 4);
        assert_eq!(game.map[&'.'].len(), 144 - 7);
        for p in game.map[&'A'].iter() {
            assert_eq!(game[p], 'A');
        }
        for p in game.map[&'0'].iter() {
            assert_eq!(game[p], '0');
        }
        for p in game.map[&'.'].iter() {
            assert_eq!(game[p], '.');
        }
    }

    #[test]
    fn infinite_grid_basic() {
        let game = Grid::new(SAMPLE);
        let ig = InfiniteGrid::new_from(&game);
        assert_eq!(ig.len(), 144);
        assert_eq!(ig.top(), 11);
        assert_eq!(ig.bottom(), 0);
        assert_eq!(ig.left(), 0);
        assert_eq!(ig.right(), 11);
        assert_eq!(ig.height(), 12);
        assert_eq!(ig.width(), 12);

        assert_eq!(format!("{}", game), SAMPLE);
        assert_eq!(format!("{}", ig), SAMPLE);
    }

    #[test]
    fn infinite_grid_rotate() {
        let game = Grid::new(TINY);
        let ig = InfiniteGrid::new_from(&game);
        let ig_rot = ig.rotate_right();
        assert_eq!(format!("{}", ig_rot), TINY_ROT_RIGHT1);
        assert_eq!(ig_rot.bottom(), 0);
        assert_eq!(ig_rot.left(), 0);
        let ig_rot = ig_rot.rotate_right();
        assert_eq!(format!("{}", ig_rot), TINY_ROT_RIGHT2);
        assert_eq!(ig_rot.bottom(), 0);
        assert_eq!(ig_rot.left(), 0);
        let ig_rot = ig_rot.rotate_right();
        assert_eq!(format!("{}", ig_rot), TINY_ROT_RIGHT3);
        assert_eq!(ig_rot.bottom(), 0);
        assert_eq!(ig_rot.left(), 0);
        let ig_rot = ig_rot.rotate_right();
        assert_eq!(format!("{}", ig_rot), TINY);
        assert_eq!(ig_rot.bottom(), 0);
        assert_eq!(ig_rot.left(), 0);

        let ig_rot = ig.rotate_left();
        assert_eq!(format!("{}", ig_rot), TINY_ROT_RIGHT3);
        assert_eq!(ig_rot.bottom(), 0);
        assert_eq!(ig_rot.left(), 0);
        let ig_rot = ig_rot.rotate_left();
        assert_eq!(format!("{}", ig_rot), TINY_ROT_RIGHT2);
        assert_eq!(ig_rot.bottom(), 0);
        assert_eq!(ig_rot.left(), 0);
        let ig_rot = ig_rot.rotate_left();
        assert_eq!(format!("{}", ig_rot), TINY_ROT_RIGHT1);
        assert_eq!(ig_rot.bottom(), 0);
        assert_eq!(ig_rot.left(), 0);
        let ig_rot = ig_rot.rotate_left();
        assert_eq!(format!("{}", ig_rot), TINY);
        assert_eq!(ig_rot.bottom(), 0);
        assert_eq!(ig_rot.left(), 0);
    }

    #[test]
    fn infinite_grid_rotate_asymm() {
        let game = Grid::new(ASYMETRIC);
        let ig = InfiniteGrid::new_from(&game);

        assert_eq!(format!("{}", ig), ASYMETRIC);
        assert_eq!(format!("{}", game), ASYMETRIC);

        let ig_rot = ig.rotate_right();
        assert_eq!(format!("{}", ig_rot), ASYMETRIC_ROT_RIGHT1);
        let ig_rot = ig_rot.rotate_right();
        assert_eq!(format!("{}", ig_rot), ASYMETRIC_ROT_RIGHT2);
        let ig_rot = ig_rot.rotate_right();
        assert_eq!(format!("{}", ig_rot), ASYMETRIC_ROT_RIGHT3);
        let ig_rot = ig_rot.rotate_right();
        assert_eq!(format!("{}", ig_rot), ASYMETRIC);

        let ig_rot = ig.rotate_left();
        assert_eq!(format!("{}", ig_rot), ASYMETRIC_ROT_RIGHT3);
        let ig_rot = ig_rot.rotate_left();
        assert_eq!(format!("{}", ig_rot), ASYMETRIC_ROT_RIGHT2);
        let ig_rot = ig_rot.rotate_left();
        assert_eq!(format!("{}", ig_rot), ASYMETRIC_ROT_RIGHT1);
        let ig_rot = ig_rot.rotate_left();
        assert_eq!(format!("{}", ig_rot), ASYMETRIC);

    }

    const COORDS: &str = "This is row 5
    abcdefghijklm
    0123456543210
    Row 2: boring
        row 1   .
    This is row 0";
    //23456789012   <-- Grid is 13 wide, 6 high

    #[test]
    fn infinite_grid_coords() {
        let game = Grid::new(COORDS);
        let ig = InfiniteGrid::new_from(&game);

        for (p, c) in ig.carte.iter() {
            assert_eq!(ig[p], *c);
            assert_eq!(game[p], *c);
        }

        assert_eq!(game[&Point::new(12,5)], '5');
        assert_eq!(game[&Point::new(12,0)], '0');
        assert_eq!(game[&Point::new(0,4)], 'a');
        assert_eq!(game[&Point::new(6,3)], '6');

   }

}