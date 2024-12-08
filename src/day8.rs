use std::collections::{HashMap, HashSet};
use itertools::Itertools;


type Map = HashMap<char, HashSet<Point>>;
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

use std::ops::{Add, Sub, Mul};
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

struct Game {
    map: Map,
    height: i32,
    width: i32,
}

impl Game {
    fn new(input: &str) -> Game {
        Game {
            map: Game::grid_map(input),
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

    fn valid(&self, p: &Point) -> bool {
        p.x >= 0 && p.x < self.width && p.y >= 0 && p.y < self.height
    }
}


#[aoc_generator(day8)]
fn input_generator(input: &str) -> Game {
    Game::new(input)
}

fn solve(game: &Game, start:i32, end: i32) -> usize {
    let mut antinodes = HashSet::new();

    game.map.iter()
        .filter(|(c, _)| *c != &'.')
        .for_each(|(_, nodes)| {
            for p in nodes.iter().combinations(2) {
                let delta = *p[0] - *p[1];
                let adds = (start..end)
                    .map(|i| *p[0] + delta * i)
                    .take_while(|p| game.valid(p));
                let subs = (start..end)
                    .map(|i| *p[1] - delta * i)
                    .take_while(|p| game.valid(p));
                antinodes.extend(adds.chain(subs));
            }
        });

    antinodes.len()
}

#[aoc(day8, part1)]
fn part1(game: &Game) -> usize {
    solve(game, 1, 2)
}

#[aoc(day8, part2)]
fn part2(game: &Game) -> usize {
    solve(game, 0, game.width.max(game.height))
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
        assert_eq!(part1(&input_generator(SAMPLE)), 14);
        assert_eq!(part2(&input_generator(SAMPLE)), 34);
    }
}
