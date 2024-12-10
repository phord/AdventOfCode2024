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
