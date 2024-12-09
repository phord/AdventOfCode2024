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
        .map(|x| x.to_digit(10).unwrap() as i32);

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
    println!("");
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
    unreachable!();
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
    while i > 0 {
        // print(&disk);
        for j in 0..i {
            if disk[i].len <= disk[j].gap {
                let mut d = disk[i].clone();
                disk[i - 1].gap += d.len + d.gap;
                d.gap = disk[j].gap - d.len;
                disk[j].gap = 0;
                disk.insert(j + 1, d);
                disk.remove(i + 1);
                i += 1;     // The current file moved up one place; adjust.
                break;
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
