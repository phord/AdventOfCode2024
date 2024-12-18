use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::astar;
use std::collections::{HashMap, HashSet};
use crate::point::Point;

type Grid = Vec<Point>;
#[aoc_generator(day18)]
fn parse(input: &str) -> Grid {
    input.split("\n")
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .map(|(x, y)| Point::new(x, y))
        .collect()
}

fn path(origins: &HashMap<Point, Point>, pos: &Point) -> Vec<Point> {
    let mut path = Vec::new();
    let mut pos = pos;
    while let Some(prev) = origins.get(&pos) {
        path.push(pos);
        pos = prev;
    }
    path.reverse();
    path.iter().map(|p| *pos).collect()
}

fn print(input: &[Point], path: &[Point], size: i32) {
    for y in 0..size {
        for x in 0..size {
            let pos = Point::new(x,y);
            if input.contains(&pos) {
                print!("#");
            } else if path.contains(&pos) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn my_astar(input: &[Point], size: i32, start: Point, end: Point) -> usize {

    let grid: HashSet<Point> = HashSet::from_iter(input.iter().cloned());

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
            // print(input, &path(&came_from, &end), size);
            return path(&came_from, &end).len();
        }
        for pos in dirs.iter()
                .map(|d| cur + *d)
                .filter(|p| !grid.contains(p))
                .filter(|p| p.x >= 0 && p.x < size  && p.y >= 0 && p.y < size )
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
    usize::MAX
}

// unexpectedly, my A* function is 8 x faster than the pathfinding crate
fn solve(bytes: &[Point], size: i32) -> usize {
    let start = Point::new(0,0);
    let end = Point::new(size-1,size-1);

    // if let Some((_, cost)) =  astar(&start,
    //         |p| p.neighbors_straight().iter()
    //                                 .filter(|p| !bytes.contains(p))
    //                                 .filter(|p| p.x >= 0 && p.x < size  && p.y >= 0 && p.y < size )
    //                                 .map(|p| (*p, 1))
    //                                 .collect::<Vec<_>>(),
    //         |p| p.manhattan_distance(&end),
    //          |p| *p == end) { cost as usize } else { usize::MAX }
    my_astar(bytes, size, start, end)
}

#[aoc(day18, part1)]
fn part1(input: &Grid) -> usize {
    let size = 71;
    let bytes = 1024;
    solve(&input[..bytes], size)
}

fn solve2(input: &Grid, first: usize, size: i32) -> Point {
    let indexes = (first+1..input.len()).collect::<Vec<_>>();
    let found = first + indexes.partition_point(|i| solve(&input[..*i], size) < usize::MAX-1);
    if found < input.len() {
        return input[found];
    }
    unreachable!()
}

#[aoc(day18, part2)]
fn part2(input: &Grid) -> Point {
    let size = 71;
    let bytes = 1024;
    solve2(input, bytes, size)
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn part1_example() {
        assert_eq!(solve(&parse(SAMPLE)[..12], 7), 22);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve2(&parse(SAMPLE), 12, 7), Point::new(6,1));
    }
}