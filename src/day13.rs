use std::ops::{Mul, Sub};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::point::Point;


type Machine = (Point, Point, Point);
type Game = Vec<Machine>;


fn parse_xy(s: &str) -> Point {
    let (_, right) = s.split_once("X").unwrap();
    let (x, y) = right.split_once("Y").unwrap();
    let (x, _) = x.split_once(",").unwrap();
    Point::new(x[1..].parse().unwrap(), y[1..].parse().unwrap())
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Game {
    input.lines()
    .collect::<Vec<_>>()
    .chunks(4)
    .map(|lines| {
        let a_move = parse_xy(lines[0]);
        let b_move = parse_xy(lines[1]);
        let prize = parse_xy(lines[2]);
        // assert!(lines[3].is_empty());
        (a_move, b_move, prize)
    }).collect()
}


fn play_game(machine: &Machine) -> Option<i32> {

    let (a_move, b_move, prize) = machine;

    let a_max = (prize.x / a_move.x).min(prize.y / a_move.y).min(100) + 1;
    let b_max = (prize.x / b_move.x).min(prize.y / b_move.y).min(100) + 1;
    for a in 0..a_max {
        let pos = a_move.mul(a);
        let remain = prize.sub(pos);
        let b = remain.x / b_move.x;
        if b >= b_max {
            continue;
        }
        let b_off = b_move.mul(b);

        if b_off == remain {
                return Some(a * 3 + b );
        }
    }
    None

}


fn play_game2(machine: &Machine) -> Option<usize> {
    let (a_move, b_move, prize) = machine;
    let px = prize.x as i64 + 10000000000000;
    let py = prize.y as i64 + 10000000000000;

    // A * a_move + B * b_move = (px, py); solve for B
    let ax = a_move.x as i64;
    let ay = a_move.y as i64;
    let bx = b_move.x as i64;
    let by = b_move.y as i64;

    let denom = ay * bx - ax * by;
    let numer = px * ay - py * ax;
    if denom == 0 {
        return None;
    }

    let b = numer / denom;
    if b < 0 {
        return None;
    }

    let a = (px - b * b_move.x as i64) / a_move.x as i64;
    if a < 0 {
        return None;
    }

    if a * a_move.x as i64 + b * b_move.x as i64 == px &&
        a * a_move.y as i64 + b * b_move.y as i64 == py
    {
        return Some( (a * 3 + b) as usize );
    }

    None
}

fn solve(game: &Game) -> usize {
    game.iter().map(|m| play_game(m).unwrap_or(0)).sum::<i32>() as usize

}

#[aoc(day13, part1)]
fn part1(game: &Game) -> usize {
    solve(game)
}

#[aoc(day13, part2)]
fn part2(game: &Game) -> usize {
    game.iter().map(|m| play_game2(m).unwrap_or(0)).sum::<usize>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279")), 480 );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), 1234 );
    }
}

// 73458657399094