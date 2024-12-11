use std::collections::HashMap;

type Stones = Vec<u64>;

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Stones {
    input.split(' ').map(|x| x.parse::<u64>().unwrap()).collect()
}


struct StonePopulation {
    // How many stones of each number do we have?
    stones: HashMap<u64, u64>,
}

impl StonePopulation {
    fn new(inp: &Stones) -> StonePopulation {
        let mut stones = HashMap::new();
        for s in inp.iter() {
            *stones.entry(*s).or_insert(0u64) += 1u64;
        }
        StonePopulation { stones }
    }

    fn blink(&mut self) {
        let mut new_stones = HashMap::new();
        for (stone, count) in self.stones.iter() {
            if stone == &0 {
                *new_stones.entry(1).or_insert(0) += count;
            } else {
                let log = stone.checked_ilog10().unwrap_or(0) + 1;
                if log % 2 == 0 {
                    let div = 10_u64.pow(log / 2);
                    *new_stones.entry(stone / div).or_insert(0) += count;
                    *new_stones.entry(stone % div).or_insert(0) += count;
                } else {
                    *new_stones.entry(stone * 2024).or_insert(0) += count;
                }
            };
        }
        self.stones = new_stones;
    }

    fn len(&self) -> u64 {
        self.stones.values().sum()
    }

}


fn solve(stones : &Stones, blinks: u8) -> u64 {
    let mut stones = StonePopulation::new(stones);
    for _ in 0..blinks {
        stones.blink();
    }
    stones.len()
}

#[aoc(day11, part1)]
fn part1(stones : &Stones) -> u64 {
    solve(stones, 25)
}

#[aoc(day11, part2)]
fn part2(stones : &Stones) -> u64 {
    solve(stones, 75)
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "125 17";

    #[test]
    fn sample1() {
        assert_eq!(solve(&input_generator(SAMPLE), 5), 13);
        assert_eq!(solve(&input_generator(SAMPLE), 6), 22);
        assert_eq!(part1(&input_generator(SAMPLE)), 55312);
        assert_eq!(part2(&input_generator(SAMPLE)), 65601038650482);
    }


    #[test]
    fn sample2() {
        assert_eq!(solve(&input_generator(SAMPLE), 75), 65601038650482);
    }

    #[test]
    fn sample3() {
        let stones = input_generator(SAMPLE);
        assert_eq!(solve(&stones, 75), 65601038650482);
    }

}
