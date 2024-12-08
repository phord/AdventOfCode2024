use std::collections::HashSet;
use itertools::Itertools;
use crate::grid::Grid;


#[aoc_generator(day8)]
fn input_generator(input: &str) -> Grid {
    Grid::new(input)
}

fn solve(game: &Grid, start:i32, end: i32) -> usize {
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
fn part1(game: &Grid) -> usize {
    solve(game, 1, 2)
}

#[aoc(day8, part2)]
fn part2(game: &Grid) -> usize {
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
