use crate::point::Point;
use num::Integer;

pub struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Line {
        assert!(p1 != p2);
        Line {p1, p2 }
    }

    // Returns (run, rise), normalized to positive run, reduced to smallest integer values
    pub fn slope(&self) -> (i32, i32) {
        let p = self.p2 - self.p1;

        let gcd = p.x.gcd(&p.y);
        let (dx, dy) = (p.x / gcd, p.y / gcd);

        if dx < 0 || (dx == 0 && dy < 0) {
            (-dx, -dy)
        } else {
            (dx, dy)
        }
    }

    pub fn intersect(&self, other: &Line) -> Option<Point> {
        // Find line intersection
        let d1 = self.p2 - self.p1;
        let (dx1, dy1) = (d1.x, d1.y);
        let d2 = other.p2 - other.p1;
        let (dx2, dy2) = (d2.x, d2.y);
        let det = dx1 * dy2 - dx2 * dy1;

        if det == 0 {
            return None;
        }

        let det1 = (self.p1.x * self.p2.y - self.p1.y * self.p2.x) * dx2 - (other.p1.x * other.p2.y - other.p1.y * other.p2.x) * dx1;
        let det2 = (self.p1.x * self.p2.y - self.p1.y * self.p2.x) * dy2 - (other.p1.x * other.p2.y - other.p1.y * other.p2.x) * dy1;

        let x = det1 / det;
        let y = det2 / det;
        Some(Point::new(x, y))
    }

    pub fn f(&self, x: i32) -> i32 {
        let (run, rise) = self.slope();
        if run == 0 {
            // Vertical line
            panic!("Vertical line");
            // Should return None?
            return self.p1.y;
        }

        let b = self.p1.y - self.p1.x * rise / run;
        x * rise / run + b
    }

    /// Is the line above the given point
    pub fn above(&self, o: &Point) -> bool {
        todo!("above");
    }

    /// Is the line below the given point
    pub fn below(&self, o: &Point) -> bool {
        todo!("below");
    }

    /// Is the point on the line?
    pub fn on(&self, o: &Point) -> bool {
        let other = Line::new(self.p1, *o);

        // If the point is on the line, it will have the same slope with the start point as the line
        *o == self.p1 || other.slope() == self.slope()
    }

    pub fn rotate_right(&self) -> Line {
        todo!("rotate_right");
    }

    pub fn rotate_left(&self) -> Line {
        todo!("rotate_left");
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::Grid;

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
}