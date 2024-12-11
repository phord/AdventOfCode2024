use std::collections::{HashMap, HashSet};


type Stones = Vec<u64>;

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Stones {
    input.split(' ').map(|x| x.parse::<u64>().unwrap()).collect()
}


#[derive(Default)]
struct Blinker {
    memo: HashMap<(u64, u8), u64>,
}

impl Blinker {

    fn blink(&mut self, stone: u64, blinks: u8) -> u64 {
        if blinks == 0 {
            1
        } else if let Some(ans) = self.memo.get(&(stone, blinks)) {
            *ans
        } else {
            let ans = if stone == 0 {
                self.blink(1, blinks - 1)
            } else {
                let log = stone.checked_ilog10().unwrap_or(0) + 1;
                if log % 2 == 0 {
                    let div = 10_u64.pow(log / 2);
                    self.blink(stone / div, blinks -1) + self.blink(stone % div, blinks - 1)
                } else {
                    self.blink(stone * 2024, blinks - 1)
                }
            };
            self.memo.insert((stone, blinks), ans);
            ans
        }
    }


    fn solve(&mut self, stones: &Stones, blinks: u8) -> u64 {
        stones.iter().map(|s| self.blink(*s, blinks)).sum()
    }
}


#[aoc(day11, part1)]
fn part1(stones : &Stones) -> u64 {
    println!("{:?}", &stones);
    let mut blink = Blinker::default();
    blink.solve(stones, 25)
}

#[aoc(day11, part2)]
fn part2(stones : &Stones) -> u64 {
    let mut blink = Blinker::default();
    blink.solve(stones, 75)
}


#[cfg(test)]
mod tests {
    use super::*;

    // const SAMPLE: &str = "0 1 10 99 999";
    const SAMPLE: &str = "125 17";

    #[test]
    fn sample1() {
        let mut blink = Blinker::default();
        assert_eq!(blink.solve(&input_generator(SAMPLE), 5), 13);
        assert_eq!(blink.solve(&input_generator(SAMPLE), 6), 22);
        assert_eq!(part1(&input_generator(SAMPLE)), 55312);
        assert_eq!(part2(&input_generator(SAMPLE)), 65601038650482);
    }


    #[test]
    fn sample2() {
        let mut blink = Blinker::default();
        assert_eq!(blink.solve(&input_generator(SAMPLE), 75), 65601038650482);
    }

}
