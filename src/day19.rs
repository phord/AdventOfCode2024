use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

type Towels = HashSet<String>;
type Game = (Towels, Vec<String>);
type Answer = usize;

#[aoc_generator(day19)]
fn parse(input: &str) -> Game {

    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let towels = towels.split(", ");
    let towels = HashSet::from_iter(towels.map(|p| p.to_string()));
    let patterns = patterns.split("\n").map(|p| p.to_string()).collect();

    (towels, patterns)

}

fn test_possible(towels: &Towels, pattern: &str) -> bool {
    if pattern.is_empty() {
        return true;
    }
    for c in towels.iter() {
        if pattern.starts_with(c) && test_possible(towels, &pattern[c.len()..]) {
            return true;
        }
    }
    false
}

fn count_possible(memo: &mut HashMap<String, usize>, towels: &Towels, pattern: &str) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    if memo.contains_key(pattern) {
        return memo[pattern];
    }

    let mut count = 0;
    for width in 0..9.min(pattern.len()) {
        if towels.contains(&pattern[0..=width]) {
            count += count_possible(memo, towels, &pattern[width+1..]);
        }
    }
    memo.insert(pattern.to_string(), count);
    count
}

#[aoc(day19, part1)]
fn part1(input: &Game) -> Answer {
    let (towels, patterns) = input;
    patterns.iter().map(|p| if test_possible(towels, p) {1} else {0}).sum()
}

#[aoc(day19, part2)]
fn part2(input: &Game) -> Answer {
    let (towels, patterns) = input;
    let mut memo = HashMap::new();
    patterns.iter().map(|p| { println!("{}", p); count_possible(&mut memo, towels, p)}).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 16);
    }
}