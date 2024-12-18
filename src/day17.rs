
use aoc_runner_derive::{aoc, aoc_generator};

type Game = Computer;

#[derive(Debug, Clone)]
struct Computer {
    regs: [u64; 3],
    code: Vec<u64>,
    output: Vec<u64>,
    ip: usize,
}

impl Computer {
    fn new(regs: [u64; 3], code: Vec<u64>) -> Self {
        Self { regs, code, output: vec![], ip: 0 }
    }

    fn combo(&self) -> u64 {
        let operand = self.code[self.ip + 1];
        assert!((0..=6).contains(&operand));
        match operand {
            0..=3 => operand,
            4 => self.regs[0],
            5 => self.regs[1],
            6 => self.regs[2],
            _ => panic!("invalid combo operand {}", operand),
        }
    }

    fn literal(&self) -> u64 {
        self.code[self.ip + 1]
    }

    fn adv(&self) -> u64 {
        let operand = self.combo();

        self.regs[0] >> operand
    }

    fn step(&mut self) {
        let mut next = self.ip + 2;

        match self.code[self.ip] {
            0 => { // adv: division;  A = A / 2^operand
                self.regs[0] = self.adv();
            }
            1 => { // bxl: bitwise XOR;  B = B ^ operand
                self.regs[1] ^= self.literal() ;
            }
            2 => { // bst: bitwise AND;  B = combo % 8
                self.regs[1] = self.combo() % 8;
            }
            3 => { // jnz: jump if not zero;  if A != 0, ip = operand
                if self.regs[0] != 0 {
                    next = self.literal() as usize;
                }
            }
            4 => { // bxc: bitwise XOR;  B = B ^ C
                self.regs[1] ^= self.regs[2];
            }
            5 => { // out: output;  output operand % 8
                self.output.push(self.combo() % 8);
            }
            6 => { // bdv: division;  B = A / 2^operand
                self.regs[1] = self.adv();
            }
            7 => { // cdv: division;  C = A / 2^operand
                self.regs[2] = self.adv();
            }
            _ => panic!("invalid opcode {}", self.code[self.ip]),
        };
        self.ip = next;
    }

    fn run(&mut self) {
        while self.ip < self.code.len() {
            // println!("{}", self);
            self.step();
        }
        // println!("{}", self);
    }

    fn reset(&mut self, a: u64) {
        self.regs = [a, 0, 0];
        self.output = vec![];
        self.ip = 0;
    }

    fn result(&self) -> String {
        self.output.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")
    }
}

impl core::fmt::Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let opcodes = ["adv", "bxl", "bst", "jnz", "bxc", "out", "bdv", "cdv"];

        writeln!(f, "A:{}, B:{}, C:{}, ip: {:?}", self.regs[0], self.regs[1], self.regs[2], self.ip)?;
        writeln!(f, "output: {:?}", self.output)?;
        for i in 0..self.code.len()/2 {
            let ip = i * 2;
            writeln!(f, "{}   {} {}", if ip == self.ip { ">" } else { " " },
                opcodes[self.code[ip] as usize], self.code[ip+1])?;
        }
        Ok(())
    }
}

#[aoc_generator(day17)]
fn parse(input: &str) -> Game {

    let (regs, code) = input.split_once("\n\n").unwrap();
    let regs = regs.lines().map(|l| l.split_once(": ").unwrap().1.parse().unwrap()).collect::<Vec<_>>();
    let code = code.split_once(": ").unwrap().1.split(',').map(|x| x.parse().unwrap()).collect::<Vec<_>>();
    assert_eq!(regs.len(), 3);

    Computer::new([regs[0], regs[1], regs[2]], code)
}

#[aoc(day17, part1)]
fn part1(comp: &Game) -> String {
    let mut comp = comp.clone();
    comp.run();
    comp.result()
}

fn precompiled_program(code: u64) -> Vec<u64> {
    // Rust version of the compiled program given as input
    let mut a = code;
    let mut out = Vec::new();
    while a > 0 {
        let b = (a % 8) ^ 5;
        let b = b ^ (a >> b) ^ 6;
        out.push(b % 8);
        a >>= 3;
    }
    out
}

fn dfs(orig: &Game, code: u64) -> Option<u64> {
    let mut comp = orig.clone();
    let mut code = code * 8;
    for _ in 0..8 {
        comp.reset(code);
        comp.run();
        if comp.output == comp.code {
            return Some(code)
        }

        let p = comp.output.len();
        assert!(p > 0);
        let q = comp.code.len();
        // println!("   ... {:24o} {} {:?}", code, comp.code[q - p], comp.output);
        if comp.output[0] == comp.code[q - p] {
            if let Some(code) = dfs(orig, code) {
                return Some(code)
            }
        }
        code += 1;
    }
    None
}

#[aoc(day17, part2)]
fn part2(orig: &Game) -> String {
    if let Some(code) = dfs(orig, 0) {
        code.to_string()
    } else {
        "failed".to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const SAMPLES: [(&str, &str);5] = [
("A: 0
B: 0
C: 9

Code: 2,6", ""),
("A: 10
B: 0
C: 0

Code: 5,0,5,1,5,4", "0,1,2"),
("A: 2024
B: 0
C: 0

Code: 0,1,5,4,3,0", "4,2,5,6,7,7,7,7,3,1,0"), //  and leave 0 in register A.

("A: 0
B: 29
C: 0

Code: 1,7",""),   // would set register B to 26.

// If register B contains 2024 and register C contains 43690,
("A: 0
B: 2024
C: 43690

Code: 4,0", "") // would set register B to 44354.
    ];

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part1a_example() {
        for (c, result) in SAMPLES.iter() {
            println!("{} -> {}", c, result);
            let mut comp = parse(c);
            comp.run();
            assert_eq!(comp.result(), *result);
        }
    }

    #[test]
    fn part2_example() {
        let comp = parse("Register A: 2024\nRegister B: 0\nRegister C: 0\n\nProgram: 0,3,5,4,3,0");
        assert_eq!(part2(&comp), "117440");
    }
}
