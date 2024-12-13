use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug,Clone)]
struct File {
    id: i32,
    len: i32,
    gap: i32,
}

type Disk = Vec<File>;

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Disk {
    let mut nums = input.chars()
    .map(|x| x.to_digit(10))
    .filter(Option::is_some)
    .map(|x| x.unwrap() as i32);

    let mut map = Vec::new();

    let mut id = 0;
    loop {
        let len = nums.next();
        let gap = nums.next();
        if let Some(len) = len {
            let gap = gap.unwrap_or(0);
            map.push(File { id, len, gap });
            id += 1;
        } else {
            break;
        }
    }
    map
}

#[allow(dead_code)]
fn print(disk: &Disk)  {
    // println!("{:?}", disk);
    for d in disk.iter() {
        for _ in 0..d.len {
            print!("{}", d.id % 10);
        }
        for _ in 0..d.gap {
            print!(".");
        }
    }
    println!();
}

struct Skid {
    pub pos: i32,
    pub disk: Disk,
}
impl Skid {
    fn new(disk: &Disk) -> Skid {
        let disk = disk.to_vec(); // clone the vector
        Skid { pos: 0, disk }
    }

    fn peek(&self) -> (i32, i32) {
        let file = self.disk.last().unwrap();
        let id = file.id;
        let pos = self.pos;
        (id, pos)
    }

    fn next(&mut self) -> (i32, i32) {
        let file = self.disk.last().unwrap();
        let id = file.id;
        let pos = self.pos;
        // println!("pos: {}  id: {}", self.pos, id);
        self.pos += 1;
        if self.pos == file.len {
            self.disk.pop();
            self.pos = 0;
        }
        (id, pos + 1)
    }
}


// Calculates checksum as-if blocks are moved, but without moving blocks
fn solve(disk: &Disk) -> usize {
    let mut total = 0usize;
    let mut count = 0usize;
    let mut skid = Skid::new(disk);
    let mut decr = 0i32;

    for d in disk.iter() {
        let (id, done) = skid.peek();
        if id == d.id {
            decr = done;
        }
        for _ in 0..d.len-decr {
            total += count * d.id as usize;
            count += 1;
        }
        for _ in 0..d.gap {
            let (id, ddd) = skid.next();
            if id == d.id + 1 {
                decr = ddd;
            }
            if id == d.id {
                return total;
            }
            total += count * id as usize;
            count += 1;
        }
    }
    0 // unreachable!();
}


fn checksum(disk: &Disk) -> usize {
    let mut total = 0usize;
    let mut count = 0usize;

    for d in disk.iter() {
        for _ in 0..d.len {
            total += count * d.id as usize;
            count += 1;
        }
        count += d.gap as usize;
    }
    total
}

fn solve2(disk: &Disk) -> usize {
    let mut disk: Disk = disk.clone();
    let mut i = disk.len() - 1;
    // let mut order: Vec<usize> = (1..=disk.len()).collect();
    let gaps: HashMap<i32, Vec<i32>> = disk.iter()
                        .filter(|d| d.gap > 0)
                        .map(|d| (d.gap, d.id))
                        .into_grouping_map().collect();

    // gaps are grouped by gap size, and each group is reverse-sorted by id for cheap removal (but we still have to insert later, so maybe it's not that useful)
    let mut gaps = (0..=9)
            .map(|k| { gaps.get(&k).unwrap_or(&Vec::new()).iter().rev().copied().collect::<Vec<i32>>()})
            .collect::<Vec<Vec<i32>>>();
    // dbg!(&gaps);
    // gaps.iter()
    //                 .map(|(k, v)| (k, v.iter().rev().collect::<Vec<_>>()))
    //                 .collect::<HashMap<_>>();

    fn find_gap(gaps: &mut [Vec<i32>], size: usize) -> Option<usize> {
        assert!(size > 0 && size <= 9);
        let winner = gaps[size..].iter()
            .enumerate()
            .map(|(i,v)| (v.last(), i))
            .filter(|(v, _)| v.is_some())
            .map(|(v, i)| (*v.unwrap(), i))
            .min();
        if let Some((v, i)) = winner {
            gaps[size + i].pop();
            Some(v as usize)
        } else {
            None
        }
    }

    fn adjust_gap_ids(gaps: &mut [Vec<i32>], start: usize, end: usize) {
        // println!("adjust_gap_ids {} {}", start, end);
        for v in gaps[1..].iter_mut() {
            for id in v.iter_mut() {
                if *id >= start as i32 && *id < end as i32 {
                    *id += 1;
                }
            }
        }
    }

    fn insert_gap(gaps: &mut [Vec<i32>], size: usize, id: i32) {
        // println!("insert_gap {} {}", size, id);
        // assert!(size <= 9);
        if size > 0 && size < 10 {
            match gaps[size].binary_search_by(|target| (-target).cmp(&(-id))) {
                Ok(_) => panic!("duplicate id {} in size {}", id, size),
                Err(i) => gaps[size].insert(i, id),
            }
        }
    }

    while i > 0 {
        if let Some(j) = find_gap(&mut gaps, disk[i].len as usize) {
            if i > j {
                let mut d = disk[i].clone();
                disk[i - 1].gap += d.len + d.gap;

                d.gap = disk[j].gap - d.len;

                adjust_gap_ids(&mut gaps, j+1, i);
                insert_gap(&mut gaps, d.gap as usize, (j+1) as i32);

                disk[j].gap = 0;
                disk.insert(j + 1, d);
                disk.remove(i + 1);
                i += 1;     // The current file moved up one place; adjust.
            }
        }
        i -= 1;
    }

    // print(&disk);
    checksum(&disk)
}

#[aoc(day9, part1)]
fn part1(disk: &Disk) -> usize {
    solve(disk)
}

#[aoc(day9, part2)]
fn part2(disk: &Disk) -> usize {
    solve2(disk)
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "2333133121414131402";
    //    00...111...2...333.44.5555.6666.777.888899
    //    00...111...2...333.44.5555.6666.777.888899

    #[test]
    fn sample1() {
        part1(&input_generator("12345"));
        assert_eq!(part1(&input_generator(SAMPLE)), 1928);
        // assert_eq!(part2(&input_generator(SAMPLE)), 1928);
    }

    #[test]
    fn sample2() {
        // part1(&input_generator("12345"));
        assert_eq!(part2(&input_generator(SAMPLE)), 2858);
        // assert_eq!(part2(&input_generator(SAMPLE)), 1928);
    }
}
