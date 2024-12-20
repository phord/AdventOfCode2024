use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::astar;

use crate::{grid::{Grid, GroupMap}, point::Point};

type Answer = usize;
type Game = GroupMap;

#[aoc_generator(day20)]
fn parse(input: &str) -> Game {
    GroupMap::new(input)

}


fn print(grid: &Game, path: &[Point]) {
    for y in 0..grid.width {
        for x in 0..grid.height {
            let y = grid.height - y - 1;
            let pos = Point::new(x,y);
            if path.contains(&pos) {
                if grid.get(&pos) == '#' {
                    print!("\x1b[31m");
                } else {
                    // print in green
                    print!("\x1b[32m");
                }
                print!("O");
                print!("\x1b[0m");
            } else {
                print!("{}", grid.get(&pos));
            }
        }
        println!();
    }
}


fn path(origins: &HashMap<Point, Point>, pos: &Point) -> Vec<Point> {
    let mut path = Vec::new();
    let mut pos = pos;
    while let Some(prev) = origins.get(pos) {
        path.push(pos);
        pos = prev;
    }
    path.reverse();
    path.iter().map(|p| **p).collect()
}

// Return distance to predecessor point, if it exists in our path; else MAX
fn pred(origins: &HashMap<Point, Point>, pos: &Point, target: &Point, dist: usize) -> usize {
    let mut pos = pos;
    let mut count = 0;
    while let Some(prev) = origins.get(pos) {
        if count > dist {
            break;
        }
        if pos == target {
            return count;
        }
        count += 1;
        pos = prev;
    }
    usize::MAX
}

pub fn my_astar(grid: &Game, start: Point, end: Point, cheat: Point, len: usize, avoid: &Vec<Point>) -> Vec<Point> {

    let dirs = [Point::new(1, 0), Point::new(0, 1), Point::new(-1, 0), Point::new(0, -1)];
    let mut open: std::collections::BTreeSet<(usize,Point)> = std::collections::BTreeSet::new();
    let mut came_from = HashMap::new();

    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();

    g_score.insert(start, 0usize);
    f_score.insert(start, start.manhattan_distance(&end) as usize);
    open.insert((start.manhattan_distance(&end) as usize, start));

    while !open.is_empty() {
        let (_, cur) = open.pop_first().unwrap();

        if cur == end {
            // print(&grid, &path(&came_from, &end));
            return path(&came_from, &end);
        }

        let pred_pos = pred(&came_from, &cur, &cheat, len);
        let cheating = pred_pos < len;
        let was_cheating = pred_pos == len; // && grid.get(&cur) != '#';
        for pos in dirs.iter()
                .map(|d| cur + *d)
                .filter(|p| !was_cheating || !avoid.contains(p))
                .filter(|p| cheating || grid.get(p) != '#')
                .filter(|p| p.x >= 0 && p.x < grid.width  && p.y >= 0 && p.y < grid.height )
            {
                let score = g_score[&cur] + 1;
                if score < *g_score.get(&pos).unwrap_or(&usize::MAX) {
                    came_from.insert(pos, cur);
                    g_score.insert(pos, score);
                    f_score.insert(pos, score + pos.manhattan_distance(&end) as usize);
                    open.insert((score + pos.manhattan_distance(&end) as usize, pos));
            }
        }
    }
    vec![]
}

fn solve(grid: &Game, cheat: &Point, len: usize, avoid: &Vec<Point>) -> Vec<Point> {
    let start = grid.map.get(&'S').unwrap().iter().next().unwrap();
    let end = grid.map.get(&'E').unwrap().iter().next().unwrap();

    my_astar( grid, *start, *end, *cheat, len, avoid)
    // astar(start,
            // |p|
            //     p.neighbors_straight().iter()
            //                         .filter(|p| cheat.contains(p) || grid.get(p) != '#')
            //                         .map(|p| (p, 1))
            //                         .map(|(p, i)| (*p, i))
            //                         .collect::<Vec<_>>(),
            // |p| p.manhattan_distance(end),
            // |p| *p == *end) {
            //     // print(grid, &path);
            //      (path, cost as usize)
            // } else { (vec![], usize::MAX) }
}

#[aoc(day20, part1)]
fn part1(grid: &Game) -> Answer {
    let nullpoint = Point::new(-1, -1);

    let mut path = solve(grid, &nullpoint, 0, &vec![]);
    let start = grid.map.get(&'S').unwrap().iter().next().unwrap();
    path.insert(0, *start);
    let best = path.len();

    print(grid, &path);
    println!("Length: {}", best);

    let mut all_cheats = HashSet::new();
    for (i, pos) in path.iter().enumerate() {
        println!("{}/{}", i, best);
        if i > path.len()-3 {
            break;
        }
        all_cheats.extend(
            path[i+3..].iter()
                .enumerate()
                .filter(|(_, p)| p.manhattan_distance(pos) == 2)
                .map(|(j, p)| (j+1, *pos, *p))
        );
    }

    // for (d, pos, end) in all_cheats.iter().filter(|(d, _, _)| *d == 12) {
    //     println!("{}: {} -> {}", d, pos, end);
    // }
    let mut lengths = all_cheats.iter().map(|(d, _, _)| d).collect::<Vec<_>>();
    lengths.sort();
    println!("{:?}", lengths);
    all_cheats.iter().filter(|(d, _, _)| *d >= 100).count()
    // all_cheats.iter().filter(|(d, _, _)| *d == 4).count()
}

#[aoc(day20, part2)]
fn part2(grid: &Game) -> Answer {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 123);
    }
}

// 1651 is too high
// 1495 is too low
// 1321 is too low