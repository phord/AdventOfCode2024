use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::{grid::GroupMap, point::Point};

type Answer = usize;
type Game = GroupMap;

#[aoc_generator(day20)]
fn parse(input: &str) -> Game {
    GroupMap::new(input)

}

fn simple_path(grid: &Game, start: Point, end: Point) -> Vec<Point> {
    let mut cur = start;
    let mut path = Vec::new();
    let mut prev = start;
    loop {
        path.push(cur);
        if cur == end {
            return path;
        }
        let inext:Vec<_> = cur.neighbors_straight().iter()
                .cloned()
                .filter(|p| grid.get(p) != '#')
                .filter(|p| p.x >= 0 && p.x < grid.width  && p.y >= 0 && p.y < grid.height )
                .filter(|p| *p != prev)
                .collect();
        assert_eq!(inext.len(), 1);
        prev = cur;
        cur = *inext.first().unwrap();
    }
}

fn solve(grid: &Game, cheat_distance: usize, gain: usize) -> usize {
    let start = grid.map.get(&'S').unwrap().iter().next().unwrap();
    let end = grid.map.get(&'E').unwrap().iter().next().unwrap();
    let path = simple_path(grid, *start, *end);

    let mut all_cheats = HashSet::new();
    for (i, pos) in path.iter().enumerate() {
        if i > path.len() - cheat_distance - 1 {
            break;
        }
        all_cheats.extend(
            path[i + cheat_distance..].iter()
                .enumerate()
                .filter(|(_, p)| p.manhattan_distance(pos) as usize <= cheat_distance)
                .filter(|(j, p)| *j + cheat_distance - p.manhattan_distance(pos) as usize >= gain)
                .map(|(_, p)| (*pos, *p))
        );
    }

    all_cheats.len()
}

#[aoc(day20, part1)]
fn part1(grid: &Game) -> Answer {
    solve(grid, 2, 100)
}

#[aoc(day20, part2)]
fn part2(grid: &Game) -> Answer {
    solve(grid, 20, 100)
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
        assert_eq!(solve(&parse(SAMPLE), 2, 8), 14);
        assert_eq!(solve(&parse(SAMPLE), 2, 3), 30);
        assert_eq!(solve(&parse(SAMPLE), 2, 1), 44);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve(&parse(SAMPLE), 20, 73), 7);
        assert_eq!(solve(&parse(SAMPLE), 20, 50), 285);
    }
}

// 1651 is too high
// 1495 is too low
// 1321 is too low

// Part2
// 1072786 is too high