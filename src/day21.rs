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
            _ => panic!("Unknown key {}", key),
        }
    }

    fn control_seq(&mut self, code: &str) -> String {
        let mut seq = String::new();
        for c in code.chars() {
            seq.push_str(&self.press_key(c));
        }
        seq
    }

    fn press_key(&mut self, key: char) -> String {
        let key = self.locate(key);
        let mut seq = String::new();
        let travel = key - self.pos;

        // Every movement ends with A

        // up-right is cheap
        if travel.y > 0 {
            for _ in 0..travel.y {
                seq.push('^');
            }
        }
        if travel.x > 0 {
            for _ in 0..travel.x {
                seq.push('>');
            }
        }
        if travel.y < 0 {
            for _ in 0..travel.y.abs() {
                seq.push('v');
            }
        }
        if travel.x < 0 {
            for _ in 0..travel.x.abs() {
                seq.push('<');
            }
        }

        seq.push('A');
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


    fn control_seq(&mut self, code: &str) -> String {
        let mut seq = String::new();
        for c in code.chars() {
            seq.push_str(&self.press_key(c));
        }
        seq
    }

    fn press_key(&mut self, key: char) -> String {
        let key = self.locate(key);
        let mut seq = String::new();
        let travel = key - self.pos;

        // Every movement ends with A

        // up-right is cheap
        if travel.x > 0 {
            for _ in 0..travel.x {
                seq.push('>');
            }
        }
        if travel.y > 0 {
            for _ in 0..travel.y {
                seq.push('^');
            }
        }
        if travel.y < 0 {
            for _ in 0..travel.y.abs() {
                seq.push('v');
            }
        }
        if travel.x < 0 {
            for _ in 0..travel.x.abs() {
                seq.push('<');
            }
        }

        seq.push('A');
        self.pos = key;
        seq
    }
}

// To control the position on the numberpad, we use the direction pad.
struct RubeGoldberg {
    pad: NumberPad,
    dpad1: DirPad,
    dpad0: DirPad,
}

impl RubeGoldberg {
    fn new() -> RubeGoldberg {
        RubeGoldberg {
            pad: NumberPad::new(),
            dpad1: DirPad::new(),
            dpad0: DirPad::new(),
        }
    }

    fn control_seq(&mut self, code: &str) -> String {
        let cmds = self.pad.control_seq(code);
        println!("cmds: {}", cmds);
        let cmds = self.dpad1.control_seq(&cmds);
        println!("cmds: {}", cmds);
        let cmds = self.dpad0.control_seq(&cmds);
        println!("cmds: {}", cmds);
        cmds
    }

    fn complexity(&mut self, code: &str) -> usize {
        // parse the integer from the string
        let mult = code[0..3].parse::<usize>().unwrap();
        let len = self.control_seq(code).len();
        println!("{} * {} = {}    {}", len, mult, mult * len, code);
        mult * len
    }

    fn solve(&mut self, input: &Game) -> usize {
        input.iter().map(|l| self.complexity(l)).sum()
    }
}


#[aoc(day21, part1)]
fn part1(input: &Game) -> Answer {
    let mut pad = RubeGoldberg::new();
    pad.solve(input)
}

#[aoc(day21, part2)]
fn part2(input: &Game) -> Answer {
    todo!()
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
        assert_eq!(pad.press_key('A'), "A");
        assert_eq!(pad.press_key('0'), "<A");
        assert_eq!(pad.press_key('2'), "^A");
        assert_eq!(pad.press_key('9'), "^^>A");
        assert_eq!(pad.press_key('1'), "vv<<A");
        assert_eq!(pad.press_key('A'), ">>vA");
        assert_eq!(pad.control_seq("029A"), "<A^A^^>AvvvA");

        let mut dpad = DirPad::new();
        assert_eq!(dpad.press_key('A'), "A");
        assert_eq!(dpad.press_key('<'), "v<<A");
        assert_eq!(dpad.press_key('^'), ">^A");
        assert_eq!(dpad.press_key('v'), "vA");
        assert_eq!(dpad.press_key('>'), ">A");
        assert_eq!(dpad.press_key('A'), "^A");

        assert_eq!(dpad.control_seq(&pad.control_seq("029A")).len(), "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len());
    }
    // <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
    // v<<A>>^A<A>AvA<^AA>A<vAAA>^A
    // <A^A>^^AvvvA
    // 029A

    #[test]
    fn part1_exampleB() {
        let mut mine = RubeGoldberg::new();
        let seq = mine.control_seq("029A");
        assert_eq!(seq.len(), "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len());
    }

    #[test]
    fn part1_exampleC() {
        let mut mine = RubeGoldberg::new();
        // <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
        assert_eq!(mine.control_seq("379A"),
                "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A");
    }

    #[test]
    fn part1_example2() {
        let game = parse(SAMPLE);
        let mut mine = RubeGoldberg::new();
        assert_eq!(mine.solve(&game), 126384);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), 123);
    }
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
// 217676