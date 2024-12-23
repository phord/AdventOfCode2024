use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<usize> {
    input.split("\n").map(|x| x.parse().unwrap()).collect()
}

fn prng(x: usize) -> usize {
    let a = x * 64;
    let x = x ^ a;
    let x = x % 16777216;

    let a = x / 32;
    let x = x ^ a;
    let x = x % 16777216;

    let a = x * 2048;
    let x = x ^ a;
    let x = x % 16777216;
    x
}

fn monkey_map(seed: usize) -> HashMap<[i32; 4], i32> {
    let mut x = seed as i32;

    let mut map = HashMap::new();
    let mut seq = [0i32; 4];
    let mut p = x % 10;
    for i in 0..4 {
        x = prng(x as usize) as i32;
        seq[i] = x%10 - p;
        p = x % 10;
    }
    map.insert(seq, p);
    for _ in 4..2000 {
        x = prng(x as usize) as i32;
        seq.rotate_left(1);
        seq[3] = x%10 - p;
        p = x % 10;
        if !map.contains_key(&seq) {
            map.insert(seq, p);
        }
    }
    map
}

fn print_seqs(input: &Vec<usize>) -> usize {
    for i in 0..input.len() {
        let mut x = input[i] as i32;
        let mut p = x;
        print!("{}: ", p);
        for j in 0..2000 {
            p = x ;
            x = prng(x as usize) as i32;
            print!("{}({}) ", x%10, x%10-p%10);
        }
        println!();
    }
    0
}

#[aoc(day22, part1)]
fn part1(input: &Vec<usize>) -> usize {
    let mut monkeys = input.clone();
    for i in 0..2000 {
        monkeys = monkeys.iter().map(|m| prng(*m)).collect();
    }
    monkeys.iter().sum()
}

#[aoc(day22, part2)]
fn part2(input: &Vec<usize>) -> usize {
    let mut map = HashMap::new();
    for m in input.iter() {
        let monk_map = monkey_map(*m);
        for (k,v) in monk_map.iter().map(|(k,v)| (*k, *v as i64)) {
            *map.entry(k).or_insert(0) += v;
        }
    }
    *map.values().max().unwrap() as usize
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[usize;4] = &[1,
                                10,
                                100,
                                2024];

    const SAMPLE2: &[usize;4] = &[1,
                                2,
                                3,
                                2024];
    #[test]
    fn part1_example0() {
        assert_eq!(part1(&SAMPLE.to_vec()), 37327623);
    }

    #[test]
    fn part1_example1() {
        assert_eq!(print_seqs(&SAMPLE2.to_vec()), 1);
    }

    #[test]
    fn part1_example() {
        let x = prng(123);
        assert_eq!(x, 15887950);
        let x = prng(x);
        assert_eq!(x, 16495136);
        let x = prng(x);
        assert_eq!(x, 527345);
        let x = prng(x);
        assert_eq!(x, 704524);
        let x = prng(x);
        assert_eq!(x, 1553684);
        let x = prng(x);
        assert_eq!(x, 12683156);
        let x = prng(x);
        assert_eq!(x, 11100544);
        let x = prng(x);
        assert_eq!(x, 12249484);
        let x = prng(x);
        assert_eq!(x, 7753432);
        let x = prng(x);
        assert_eq!(x, 5908254);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&SAMPLE2.to_vec()), 23);
    }
}