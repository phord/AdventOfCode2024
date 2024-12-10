use std::collections::HashSet;
use itertools::Itertools;
use crate::{grid::Grid, point::Point};



#[aoc_generator(day10)]
fn input_generator(input: &str) -> Grid {
    Grid::new(input)
}


fn adjacent(p1: &Point, p2: &Point) -> bool {
    let delta = *p1 - *p2;
    delta.x.abs() == 1 && delta.y.abs() == 0 || delta.x.abs() == 0 && delta.y.abs() == 1
}

fn dfs(head: &Point, lvl: char, grid: &Grid) -> (usize, usize) {
    let mut summits = HashSet::new();
    let mut stack = vec![(lvl, head)];
    let mut count = 0;
    while let Some((lvl, p)) = stack.pop() {
        let next = (lvl as u8 + 1) as char;
        for p2 in grid.map[&next].iter() {
            if adjacent(p, p2) {
                if next == '9' {
                    summits.insert(*p2);
                    count += 1;
                } else {
                    stack.push((next, p2));
                }
            }
        }
    }
    (summits.len(), count)
}


fn solve(grid: &Grid) -> (usize, usize) {
    let heads = &grid.map[&'0'];
    let mut total = 0;
    let mut total2 = 0;
    for h in heads.iter() {
        let (part1, part2) = dfs(h, '0', grid);
        total += part1;
        total2 += part2;
    }
    (total, total2)
}



#[aoc(day10, part1)]
fn part1(grid: &Grid) -> usize {
    solve(grid).0
}

#[aoc(day10, part2)]
fn part2(grid: &Grid) -> usize {
    solve(grid).1
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE2: &str = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";

    const SAMPLE4: &str = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";

    const SAMPLE36: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    const SAMPLE3: &str = ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....";

    const SAMPLE13: &str = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";

const SAMPLE227: &str = "012345
123456
234567
345678
4.6789
56789.";

const SAMPLE81: &str = SAMPLE36;

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLE2)), 2);
        assert_eq!(part1(&input_generator(SAMPLE4)), 4);
        assert_eq!(part1(&input_generator(SAMPLE36)), 36);
        // assert_eq!(part2(&input_generator(SAMPLE)), 1928);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(SAMPLE3)), 3);
        assert_eq!(part2(&input_generator(SAMPLE13)), 13);
        assert_eq!(part2(&input_generator(SAMPLE227)), 227);
        assert_eq!(part2(&input_generator(SAMPLE81)), 81);
    }


}
