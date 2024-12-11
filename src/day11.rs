use std::collections::{HashMap, HashSet};


type Stones = Vec<u64>;

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Stones {
    input.split(' ').map(|x| x.parse::<u64>().unwrap()).collect()
}



/*
If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
 */

fn blink_one(stone: u64) -> Stones {
    if stone == 0 { vec![1] }
    else {
        let log = stone.to_string().len() as u32;
        // if stone < 10 { 1} else { (stone - 1).ilog10() + 1};
        if log % 2 == 0 {
            let div = 10_u64.pow(log / 2);
            vec![stone / div, stone % div]
        } else {
            vec![stone * 2024]
        }
    }
}

enum Work {
    Blink(u64, u8),
    Done(u64)
}

type Memo = HashMap<(u64, u8), Stones>;
type Mapped = HashMap<u64, HashSet<u8>>;
#[derive(Default)]
struct Blink {
     memo: Memo,
     mapped: Mapped,
}

impl Blink {

    fn prev(&self, stone: u64, blinks: u8) -> Option<Vec<(u64, u8)>> {
        if let Some(prior) = self.mapped.get(&stone) {
            let best = prior.iter()
                    .filter(|x| **x <= blinks)
                    .max();
            if let Some(best) = best {
                return Some(self.memo[&(stone, *best)].iter().map(|s| (*s, blinks - *best)).collect());
            }
        }
        None
    }

    fn memoize(&mut self, stone: u64, blinks: u8, stones: &Stones) {
        if blinks > 10 {
            self.memo.insert((stone, blinks), stones.clone());
            let exists = self.mapped.get_mut(&stone);
            if let Some(bar) = exists {
                bar.insert(blinks);
            } else {
                self.mapped.insert(stone, HashSet::from([blinks]));
            }
        }
    }

    fn blink(&mut self, stone: u64, blinks: u8) -> Stones {
        let mut work = vec![(stone, blinks)];
        let mut stones = vec![];

        while !work.is_empty() {
            match work.pop() {
                Some((stone, blinks)) => {
                    if blinks == 0 {
                        // No more blinks for this stone
                        stones.push(stone);
                        // print!("=");
                    } else if let Some(prev) = self.prev(stone, blinks) {
                        // Found some partial work we previously did
                        println!("   HIT!!! ");
                        work.extend(prev);
                    } else if blinks > 1 {
                        work.extend(
                            self.blink(stone, blinks - 1).iter()
                                .map(|s| (*s, 1))
                        );
                    } else
                    // Blink at this stone
                    if stone == 0 {
                        // print!(">");
                        work.push((1, blinks - 1));
                    } else {
                        // print!("O");
                        let blinks = blinks - 1;
                        let log = stone.checked_ilog10().unwrap_or(0) + 1;
                        if log % 2 == 0 {
                            let div = 10_u64.pow(log / 2);
                            work.push((stone / div, blinks));
                            work.push((stone % div, blinks));
                        } else {
                            work.push((stone * 2024, blinks));
                        }
                    }
                }
                None => break,
            }
            // print!("+");

        }

        // Record this work
        if blinks > 5 || stones.len() > 100 {
            println!("{}  {}  {}", stone, blinks, stones.len());
            self.memoize(stone, blinks, &stones);
        }

        stones
    }

    fn solve(&mut self, stones: &Stones, blinks: u8) -> usize {
        let mut result = Stones::default();
        for stone in stones.iter() {
            result.extend(self.blink(*stone, blinks));
            // println!("{:?}", stones);
        }
        result.len()
    }

}

fn blink(stones: &Stones) -> Stones {
    stones.iter().flat_map(|s|
        blink_one(*s)
    ).collect()
}



fn solve(stones: &Stones, blinks: u8) -> usize {
    let mut blink = Blink::default();
    let mut result = Stones::default();
    for stone in stones.iter() {
        result.extend(blink.blink(*stone, blinks));
        // println!("{:?}", stones);
    }
    result.len()
}

fn solve2(stones : &Stones, blinks: u8) -> usize {
    let mut blink = Blink::default();
    for s in 0..blinks/5 {

        let ans = blink.solve(stones, s * 5);
        println!("{}: {}", s * 5, ans);
    }
    // blink.solve(stones, 75)
    blink.solve(stones, blinks)
}

#[aoc(day11, part1)]
fn part1(stones : &Stones) -> usize {
    println!("{:?}", &stones);
    solve(stones, 25)
}

#[aoc(day11, part2)]
fn part2(stones : &Stones) -> usize {
    solve2(stones, 75)
}


#[cfg(test)]
mod tests {
    use super::*;

    // const SAMPLE: &str = "0 1 10 99 999";
    const SAMPLE: &str = "125 17";

    #[test]
    fn sample1() {
        assert_eq!(solve(&input_generator(SAMPLE), 6), 22);
        assert_eq!(part1(&input_generator(SAMPLE)), 55312);
        // assert_eq!(part2(&input_generator(SAMPLE)), 1928);
    }


    #[test]
    fn sample2() {
        assert_eq!(solve2(&input_generator(SAMPLE), 6), 22);
        assert_eq!(solve2(&input_generator(SAMPLE), 25), 55312);
        // assert_eq!(part2(&input_generator(SAMPLE)), 1928);
    }

}
