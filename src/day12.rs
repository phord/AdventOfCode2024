use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{grid::Grid, point::Point};


#[aoc_generator(day12)]
fn input_generator(input: &str) -> Grid {
    Grid::new(input)
}

type Region = HashSet<Point>;

// Iterate all contiguous regions of each plant type
fn split_regions(rgn: &Region) -> Vec<Region> {
    let mut cells = rgn.clone();
    let mut regions: Vec<Region> = Vec::new();
    while !cells.is_empty() {
        let mut rgn = HashSet::<Point>::from_iter(cells.iter().take(1).copied());
        loop {
            let territory:HashSet<_> = rgn.iter()
                .flat_map(|p| p.neighbors_straight())
                .filter(|p| cells.contains(p))
                .collect();

            // println!("{}:  ter={:?}", t, territory);
            // println!("    rgn={:?}", rgn);
            // println!("    nei={:?}", rgn.iter().next().unwrap().neighbors());

            let before = rgn.len();
            rgn.extend(territory);
            if before == rgn.len() {
                break;
            }
        }
        cells.retain(|p| !rgn.contains(p));
        regions.push(rgn);
    }
    regions
}

// Iterate all contiguous regions of each plant type
fn regions(grid: &Grid) -> HashMap<char, Vec<Region>> {
    let types = grid.map.keys().copied().collect::<Vec<_>>();
    types.iter().map(|t| {
        (*t, split_regions(&grid.map[t]))
    }).collect()
}


fn perimeter(rgn: &Region) -> usize {
    rgn.iter()
        .flat_map(|p| p.neighbors_straight())
        .filter(|p| !rgn.contains(p))
        .count()
}
fn sides(rgn: &Region) -> usize {
    let dirs = [Point::new(0,-1), Point::new(1,0), Point::new(0,1), Point::new(-1,0)];

    // Find all neighbor cells that are not in the region, paired with the direction of their assocation to our cells
    // Note this will find some cells four times.  For example,
    //  +---+---+---+
    //  | A | A | A |
    //  +---+-^-+---+
    //  | A < O > A |
    //  +---+-v-+---+
    //  | A | A | A |
    //  +---+---+---+
    //
    // For this region of A cells, the O cell will have four associations with A, one for each side.
    // And the Xs here complete the set of neighbors we will find.
    //
    //  +---+---+---+---+---+
    //  |   | X | X | X |   |
    //  +---+-v-+-v-+-v-+---+
    //  | X > A | A | A < X |
    //  +---+---+-^-+---+---+
    //  | X > A < O > A < X |
    //  +---+---+-v-+---+---+
    //  | X > A | A | A < X |
    //  +---+-^-+-^-+-^-+---+
    //  |   | X | X | X |   |
    //  +---+---+---+---+---+
    let mut foreign_neighbors: HashSet<(Point, Point)> = rgn.iter()
        .flat_map(|p| dirs.iter().map(|d| (*d, *d + *p)))
        .filter(|(_, p)| !rgn.contains(p))
        .collect();
    foreign_neighbors.retain(|(_, p)| !rgn.contains(p));

    // Now we must collect the contiguous sets of foreign neighbors which have the same direction-assocation with our set.

    let neighbor_sets: HashMap<Point, Region> = foreign_neighbors.iter()
        .copied()
        .into_grouping_map()
        .collect();

    // Find all the contiguous edges from the sets of similar foreign neighbors
    neighbor_sets.values()
        .map(|p| split_regions(p).len())
        .sum()
}

fn area(rgn: &Region) -> usize {
    rgn.len()
}

fn price(rgn: &Region) -> usize {
    area(rgn) * perimeter(rgn)
}

fn solve1(grid: &Grid) -> usize {
    regions(grid).iter()
        .flat_map(|(_, regions)| {
            regions.iter().map(move |rgn| {
                price(rgn)})
        }).sum::<usize>()
}


fn solve2(grid: &Grid) -> usize {
    regions(grid).iter()
        .flat_map(|(_, regions)| {
            regions.iter().map(move |rgn| {
                area(rgn) * sides(rgn)})
        }).sum::<usize>()
}

#[aoc(day12, part1)]
fn part1(grid: &Grid) -> usize {
    solve1(grid)
}

#[aoc(day12, part2)]
fn part2(grid: &Grid) -> usize {
    solve2(grid)
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "AAAA
BBCD
BBCC
EEEC";

    const SAMPLE2: &str =
"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

const SAMPLE3: &str =
"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn sample1() {
        let grid = input_generator(SAMPLE);
        assert_eq!(part1(&grid), 140);
        // assert_eq!(part2(&input_generator(SAMPLE)), 1928);
    }

    #[test]
    fn sample2() {
        let grid = input_generator(SAMPLE2);
        assert_eq!(part1(&grid), 772);
        // assert_eq!(part2(&input_generator(SAMPLE)), 1928);
    }

    #[test]
    fn sample3() {
        let grid = input_generator(SAMPLE3);
        assert_eq!(part1(&grid), 1930);
        // assert_eq!(part2(&input_generator(SAMPLE)), 1928);
    }

    #[test]
    fn sample21() {
        let grid = input_generator(SAMPLE);
        assert_eq!(part2(&grid), 80);
        // assert_eq!(part2(&input_generator(SAMPLE)), 1928);
    }

    #[test]
    fn sample22() {
        let grid = input_generator(SAMPLE2);
        assert_eq!(part2(&grid), 436);
        // assert_eq!(part2(&input_generator(SAMPLE)), 1928);
    }

    const SAMPLE23: &str =
"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    #[test]
    fn sample23() {
        let grid = input_generator(SAMPLE23);
        assert_eq!(part2(&grid), 236);
        // assert_eq!(part2(&input_generator(SAMPLE)), 1928);
    }


}
