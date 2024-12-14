use std::{collections::HashSet, ops::{Add, Mul, Sub}};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl core::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
// Measurements
impl Point {
    pub fn is_adjacent_straight(&self, other: &Point) -> bool {
        let delta = *self - *other;
        delta.x.abs() == 1 && delta.y.abs() == 0 || delta.x.abs() == 0 && delta.y.abs() == 1
    }
    pub fn is_adjacent_diagonal(&self, other: &Point) -> bool {
        let delta = *self - *other;
        delta.x.abs() == 1 && delta.y.abs() == 1
    }

    pub fn is_neighbor(&self, other: &Point) -> bool {
        self.is_adjacent_straight(other) || self.is_adjacent_diagonal(other)
    }

    pub fn neighbors(&self) -> HashSet<Point> {
        (-1..1)
            .flat_map(|x| (-1..1).map(move |y| Point::new(self.x + x, self.y + y)))
            .filter(|p| *p != *self)
            .collect()
    }

    pub fn neighbors_straight(&self) -> HashSet<Point> {
        [Point::new(1, 0), Point::new(0, 1), Point::new(-1, 0), Point::new(0, -1)]
            .iter()
            .map(|dp| *self + *dp)
            .collect()
    }

    pub fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

// Transformations
impl Point {
    pub fn wrap_to_grid(&self, width: usize, height: usize) -> Point {
        Point::new(self.x.rem_euclid(width as i32), self.y.rem_euclid(height as i32))
    }

    pub fn rotate_right_around(&self, center: &Point) -> Point {
        (*self - *center).rotate_right() + *center
    }

    pub fn rotate_left_around(&self, center: &Point) -> Point {
        (*self - *center).rotate_left() + *center
    }

    pub fn rotate_right(&self) -> Point {
        Point::new(self.y, -self.x)
    }

    pub fn rotate_left(&self) -> Point {
        Point::new(-self.y, self.x)
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for Point {
    type Output = Point;
    fn mul(self, other: i32) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_basic() {
        assert_eq!(Point::new(0, 0).rotate_right(), Point::new(0, 0));
        assert_eq!(Point::new(0, 0).rotate_left(), Point::new(0, 0));
    }

    #[test]
    fn rotate_around_basic() {
        let point = Point::new(5, 7);
        let center = Point::new(0, 0);
        assert_eq!(point.rotate_right_around(&center), point.rotate_right());
        assert_eq!(point.rotate_left_around(&center), point.rotate_left());
    }

    #[test]
    fn rotate_left_right() {
        let point = Point::new(5, 7);
        let target1 = Point::new(7, -5);
        let target2 = Point::new(-5, -7);
        let target3 = Point::new(-7, 5);
        let target4 = point;

        let point = point.rotate_right();
        assert_eq!(point, target1);
        let point = point.rotate_right();
        assert_eq!(point, target2);
        let point = point.rotate_right();
        assert_eq!(point, target3);
        let point = point.rotate_right();
        assert_eq!(point, target4);

        let point = point.rotate_left();
        assert_eq!(point, target3);
        let point = point.rotate_left();
        assert_eq!(point, target2);
        let point = point.rotate_left();
        assert_eq!(point, target1);
        let point = point.rotate_left();
        assert_eq!(point, target4);
    }

    #[test]
    fn rotate_left_right_around() {
        todo!();
    }

}