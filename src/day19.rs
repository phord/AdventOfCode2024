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

fn count_possible(memo: &mut HashMap<usize, usize>, towels: &Towels, pattern: &str) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    if memo.contains_key(&pattern.len()) {
        return memo[&pattern.len()];
    }

    let mut count = 0;
    for width in 0..9.min(pattern.len()) {
        if towels.contains(&pattern[0..=width]) {
            count += count_possible(memo, towels, &pattern[width+1..]);
        }
    }
    memo.insert(pattern.len(), count);
    count
}

fn solve_recursive(input: &Game) -> usize {
    let (towels, patterns) = input;
    patterns.iter().map(|p| { let mut memo = HashMap::new(); count_possible(&mut memo, towels, p)}).sum()
}

// A dynamic programming solution that is O(len(P)*T)
fn dp(towels: &Towels, pattern: &str) -> usize {
    let mut memo = vec![0usize; pattern.len() + 1];
    memo[pattern.len()] = 1;
    for i in 0..pattern.len() {
        let j = pattern.len() - i - 1;
        for t in towels.iter().filter(|t| pattern[j..].starts_with(*t)) {
            memo[j] += memo[j + t.len()];
        }
    }

    memo[0]
}

// A dynamic programming solution that is O(len(P)*logT)
fn dp2(towels: &Towels, pattern: &str) -> usize {
    let mut memo = vec![0usize; pattern.len() + 1];
    memo[pattern.len()] = 1;
    for i in 0..pattern.len() {
        let j = pattern.len() - i - 1;
        for k in 1..9 {
            if j + k <= pattern.len() && memo[j+k] > 0 && towels.contains(&pattern[j..j+k]) {
                memo[j] += memo[j + k];
            }
        }
    }

    memo[0]
}

fn solve_dp(input: &Game) -> usize {
    let (towels, patterns) = input;

    patterns.iter().map(|p| dp2(towels, p)).sum()
}

#[aoc(day19, part1)]
fn part1(input: &Game) -> Answer {
    let (towels, patterns) = input;
    patterns.iter().map(|p| if test_possible(towels, p) {1} else {0}).sum()
}

#[aoc(day19, part2)]
fn part2(input: &Game) -> Answer {
    solve_dp(input)
    // solve_recursive(input)
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