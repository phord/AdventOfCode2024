use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::{grid::GroupMap, point::Point};

type Answer = usize;
type Game = Vec<String>;

#[aoc_generator(day21)]
fn parse(input: &str) -> Game {
    input.split("\n").map(|l| l.to_string()).collect()
}


/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
*/


// To type a code on the number pad, we have to send manhattan-distance keypresses worth of directional keys,
// then press the key itself. Then manhattan-distance direction keys to get to the next key.
//
// From A -> Up : left

struct NumberPad {
    pos: Point,
    panic: Point,
}

impl NumberPad {
    fn new() -> NumberPad {
        NumberPad {
            pos: Point::new(2, 0),
            panic: Point::new(0, 0),
        }
    }

    fn locate(&self, key: char) -> Point {
        match key {
            'A' => Point::new(2, 0),
            '0' => Point::new(1, 0),
            '1' => Point::new(0, 1),
            '2' => Point::new(1, 1),
            '3' => Point::new(2, 1),
            '4' => Point::new(0, 2),
            '5' => Point::new(1, 2),
            '6' => Point::new(2, 2),
            '7' => Point::new(0, 3),
            '8' => Point::new(1, 3),
            '9' => Point::new(2, 3),
            'X' => self.panic,
            _ => panic!("Unknown key {}", key),
        }
    }

    fn control_seq(&mut self, code: &str) -> HashSet<String> {
        let mut seq = Vec::new();
        for c in code.chars() {
            seq.push(self.press_key(c));
        }
        demux(&seq)
    }

    fn press_key(&mut self, key: char) -> Vec<String> {
        let key = self.locate(key);
        let travel = key - self.pos;

        // Every movement ends with A

        // Every path to key with fewest turns

        let horz =
            if travel.x < 0 { "<".repeat(-travel.x as usize) }
            else if travel.x > 0 { ">".repeat(travel.x as usize) }
            else { "".to_string() };

        let vert =
            if travel.y < 0 { "v".repeat(-travel.y as usize) }
            else if travel.y > 0 { "^".repeat(travel.y as usize) }
            else { "".to_string() };

        // Three possibilities:
        //    1. Not moving horizontally
        //    2. Moving in both directions, but one path crosses panic zone
        //         When moving up from (_,0) to (0,_) => must move vert first
        //         When moving down from (0,_) to (_,) => must move horz first
        //    3. Moving in both directions, but both are valid

        let vert_horz = vert.clone() + &horz + &"A";
        let horz_vert = horz.clone() + &vert + &"A";

        let seq =
            if self.pos.y == self.panic.y && key.x == self.panic.x {
                // Danger zone: Move up then left
                vec![vert_horz]
            } else if self.pos.x == self.panic.x && key.y == self.panic.y {
                // Danger zone: Move right then down
                vec![horz_vert]
            } else if horz_vert == vert_horz {
                // Both paths are the same
                vec![horz_vert]
            } else {
                // Both paths are safe
                vec![horz_vert, vert_horz]
            };

        self.pos = key;
        seq
    }
}



/*
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
 */

 #[derive(Clone)]
struct DirPad {
    pos: Point,
    panic: Point,
}

impl DirPad {
    fn new() -> DirPad {
        DirPad {
            pos: Point::new(2, 1),
            panic: Point::new(0, 1),
        }
    }

    fn locate(&self, key: char) -> Point {
        match key {
            'A' => Point::new(2, 1),
            '^' => Point::new(1, 1),
            '<' => Point::new(0, 0),
            'v' => Point::new(1, 0),
            '>' => Point::new(2, 0),
            _ => panic!("Unknown key {}", key),
        }
    }

    fn control_seq_one(&mut self, code: &str) -> HashSet<String> {
        let mut seq = Vec::new();
        for c in code.chars() {
            seq.push(self.press_key(c));
        }

        demux(&seq)
    }

    fn control_seq(&mut self, code: HashSet<String>) -> HashSet<String> {
        code.iter()
            .flat_map(|c| self.control_seq_one(c))
            .collect()
    }

    fn press_key(&mut self, key: char) -> Vec<String> {
        let key = self.locate(key);
        let travel = key - self.pos;

        // Every path to key with fewest turns
        let horz =
            if travel.x < 0 { "<".repeat(-travel.x as usize) }
            else if travel.x > 0 { ">".repeat(travel.x as usize) }
            else { "".to_string() };

        let vert =
            if travel.y < 0 { "v".repeat(-travel.y as usize) }
            else if travel.y > 0 { "^".repeat(travel.y as usize) }
            else { "".to_string() };

        // Three possibilities:
        //    1. Not moving horizontally
        //    2. Moving in both directions, but one path crosses panic zone
        //         When moving up from (_,0) to (0,_) => must move vert first
        //         When moving down from (0,_) to (_,) => must move horz first
        //    3. Moving in both directions, but both are valid

        let vert_horz = vert.clone() + &horz + &"A";
        let horz_vert = horz.clone() + &vert + &"A";

        let seq =
            if self.pos.y == self.panic.y && key.x == self.panic.x {
                // Danger zone: Move down then right
                vec![vert_horz]
            } else if self.pos.x == self.panic.x && key.y == self.panic.y {
                // Danger zone: Move left then up
                vec![horz_vert]
            } else if horz_vert == vert_horz {
                // Both paths are the same
                vec![horz_vert]
            } else {
                // Both paths are safe
                vec![horz_vert, vert_horz]
            };

        self.pos = key;
        seq
    }

}


fn demux(cmds: &Vec<Vec<String>>) -> HashSet<String> {

    let mut out = HashSet::from_iter(cmds.last().unwrap().iter().cloned());
    for i in 0..cmds.len()-1 {
        let j = cmds.len() - i - 2;
        let left = &cmds[j];

        // Combine every element in left and right.  For each l, and each r, return l+r
        let demux: HashSet<String> =
            left.iter().flat_map(|l| {
                out.iter().map(|r| l.clone() + r).collect::<Vec<String>>()
            }).collect();

        out = demux;
    }
    out
}

#[test]
fn demux_basic() {
    let vars = [["foo", "bar"], ["baz", "qux"]];
    let vars = vars.iter().map(|v| v.iter().map(|s| s.to_string()).collect()).collect();
    let out = demux(&vars);
    assert_eq!(Vec::from_iter(out.iter().cloned()), vec!["foo".to_string() + "baz", "foo".to_string() + "qux", "bar".to_string() + "baz", "bar".to_string() + "qux"]);
}

// To control the position on the numberpad, we use the direction pad.
struct RubeGoldberg {
    pad: NumberPad,
    dpads: Vec<DirPad>,
}

impl RubeGoldberg {
    fn new(robots: usize) -> RubeGoldberg {
        let dpads = vec![DirPad::new(); robots];
        RubeGoldberg {
            pad: NumberPad::new(),
            dpads,
        }
    }

    fn control_seq(&mut self, code: &str) -> HashSet<String> {
        let mut cmds = HashSet::from_iter(self.pad.control_seq(code).iter().cloned());
        println!("cmds: {}", cmds.len());
        for cmd in cmds.iter() {
            println!("     {}", cmd);
        }
        for dpad in self.dpads.iter_mut() {
            cmds = dpad.control_seq(cmds);
            println!("cmds: {}", cmds.len());
            for cmd in cmds.iter() {
                println!("     {}", cmd);
            }
        }
        cmds
    }

    fn complexity(&mut self, code: &str) -> usize {
        // parse the integer from the string
        let mult = code[0..3].parse::<usize>().unwrap();
        let seqs = self.control_seq(code);

        let len = seqs.iter().map(|s| s.len()).min().unwrap();
        println!("{} * {} = {}    {}", len, mult, mult * len, code);
        mult * len
    }

    fn solve(&mut self, input: &Game) -> usize {
        input.iter().map(|l| self.complexity(l)).sum()
    }
}


#[aoc(day21, part1)]
fn part1(input: &Game) -> Answer {
    let mut pad = RubeGoldberg::new(2);
    pad.solve(input)
}

#[aoc(day21, part2)]
fn part2(input: &Game) -> Answer {
    let mut pad = RubeGoldberg::new(25);
    pad.solve(input)
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "029A
980A
179A
456A
379A";

// v<<A>>^A<A>AvA<^AA>A<vAAA>^A

    #[test]
    fn part1_example() {
        let mut pad = NumberPad::new();
        assert_eq!(pad.press_key('A'), ["A"]);
        assert_eq!(pad.press_key('0'), ["<A"]);
        assert_eq!(pad.press_key('2'), ["^A"]);
        assert_eq!(pad.press_key('9'), [ ">^^A", "^^>A"]);
        assert_eq!(pad.press_key('1'), ["<<vvA", "vv<<A"]);
        assert_eq!(pad.press_key('A'), [">>vA"]);
        assert_eq!(pad.control_seq("029A"),
                HashSet::from_iter(["<A^A>^^AvvvA", "<A^A^^>AvvvA"].iter().map(|s| s.to_string())));


        let mut dpad = DirPad::new();
        assert_eq!(dpad.press_key('A'), ["A"]);
        assert_eq!(dpad.press_key('<'), ["v<<A"]);
        assert_eq!(dpad.press_key('^'), [">^A"]);
        assert_eq!(dpad.press_key('v'), ["vA"]);
        assert_eq!(dpad.press_key('>'), [">A"]);
        assert_eq!(dpad.press_key('A'), ["^A"]);

        assert_eq!(dpad.control_seq(pad.control_seq("029A")).iter().map(|s| s.len()).min().unwrap(),
            "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len());
    }
    // <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
    // v<<A>>^A<A>AvA<^AA>A<vAAA>^A
    // <A^A>^^AvvvA
    // 029A

    #[test]
    fn part1_exampleB() {
        let mut mine = RubeGoldberg::new(2);
        let seq = mine.control_seq("029A");
        assert_eq!(seq.len(), "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len());
    }

    #[test]
    fn part1_exampleC() {
        let mut pad = NumberPad::new();
        println!("{:?}", pad.control_seq("379A"));
        let mut mine = RubeGoldberg::new(2);
        // <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
        assert_eq!(mine.control_seq("379A").len(),
                "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len());
    }

    #[test]
    fn part1_example2() {
        let game = parse(SAMPLE);
        let mut mine = RubeGoldberg::new(2);
        assert_eq!(mine.solve(&game), 126384);
    }

    #[test]
    fn part2_example() {
        let game = parse(SAMPLE);
        let mut mine = RubeGoldberg::new(3);
        assert_eq!(mine.solve(&game), 126384);    }
}

// 80 * 805 = 64400    805A
// cmds: ^^^A<A>vvAvA
// cmds: <AAA>Av<<A>>^AvA<AA>^Av<A>^A
// cmds: v<<A>>^AAAvA^Av<A<AA>>^AvAA^<A>Av<A>^Av<<A>>^AAvA^<A>Av<A<A>>^AvA^<A>A
// 70 * 983 = 68810    983A
// cmds: ^<<A^A^>>AvvvA
// cmds: <Av<AA>>^A<A>A<A>vAA^Av<AAA>^A
// cmds: v<<A>>^Av<A<A>>^AAvAA^<A>Av<<A>>^AvA^Av<<A>>^AvA<A>^AA<A>Av<A<A>>^AAAvA^<A>A
// 76 * 149 = 11324    149A
// cmds: ^^<<AvA>>AvA
// cmds: <AAv<AA>>^Av<A>^AvAA^Av<A>^A
// cmds: v<<A>>^AAv<A<A>>^AAvAA^<A>Av<A<A>>^AvA^<A>Av<A>^AA<A>Av<A<A>>^AvA^<A>A
// 70 * 413 = 28910    413A
// cmds: ^^<A^AvvA>vA
// cmds: <AAv<A>>^A<A>Av<AA>^AvA<A>^A
// cmds: v<<A>>^AAv<A<A>>^AvAA^<A>Av<<A>>^AvA^Av<A<A>>^AAvA^<A>Av<A>^Av<<A>>^AvA^<A>A
// 76 * 582 = 44232    582A

// Part1: 202648