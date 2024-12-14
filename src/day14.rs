use aoc_runner_derive::{aoc, aoc_generator};

use crate::point::Point;


type Game = Vec<(Point, Point)>;
type Answer = usize;


#[aoc_generator(day14)]
fn parse(input: &str) -> Game {
    input.lines().map(|line| {
        let (pos, vel) = line.split_once(' ').unwrap();
        let pos = pos[2..].split_once(',').unwrap();
        let vel = vel[2..].split_once(',').unwrap();
        (Point::new(pos.0.parse().unwrap(), pos.1.parse().unwrap()),
            Point::new(vel.0.parse().unwrap(), vel.1.parse().unwrap()))
    }).collect()
}

fn display(game: &Game, height: usize, width: usize) {
    for y in 0..height {
        for x in 0..width {
            let robots = game.iter().filter(|(pos, _)| pos.x == x as i32 && pos.y == y as i32).count();
            if robots > 0 {
                print!("{}", robots);
            } else {
                print!(".");
            }
        }
        println!();
    }
}


fn game_at(game: &Game, height: usize, width: usize, t: i32) -> Game {
    game.iter().map(|(pos, vel)| (*pos + *vel * t, *vel))
            .map(|(pos, vel)| {
                let x = pos.x.rem_euclid(width as i32);
                let y = pos.y.rem_euclid(height as i32);
                (Point::new(x, y), vel)
            })
            .collect()
}

fn sim(game: &Game, height: usize, width: usize) -> usize {
    let game = game_at(game, height, width, 100);

    let w = (width/2) as i32;
    let h = (height/2) as i32;

    let a = game.iter().filter(|(pos, _)| pos.x < w && pos.y < h).count();
    let b = game.iter().filter(|(pos, _)| pos.x > w && pos.y < h).count();
    let c = game.iter().filter(|(pos, _)| pos.x < w && pos.y > h).count();
    let d = game.iter().filter(|(pos, _)| pos.x > w && pos.y > h).count();
    // dbg!(a, b, c, d);
    a * b * c * d
}

fn least_x(game: &Game) -> usize {
    let mut game = game.clone();
    for t in 0..105 {
        let game = game_at(&game, 103, 101, t);
        let unique_x = game.iter().map(|(pos, _)| pos.x).collect::<std::collections::HashSet<_>>().len();
        if unique_x < 96 {
            return t as usize;
        }
    }
    panic!("Not found");
}

fn least_y(game: &Game) -> usize {
    let mut game = game.clone();
    for t in 0..105 {
        let game = game_at(&game, 103, 101, t);
        let unique_y = game.iter().map(|(pos, _)| pos.y).collect::<std::collections::HashSet<_>>().len();
        if unique_y < 96 {
            return t as usize;
        }
    }
    panic!("Not found");
}


fn sim2(game: &Game, height: usize, width: usize) -> usize {
    // let unique_x = least_x(&game);
    // let unique_y = least_y(&game);
    // println!("unique_x={} unique_y={}", unique_x, unique_y);

    let unique_x = 71;

    // Ok, there's a periodic drop in unique X values at the 71st iteration, and Y values at the 16th.
    // unique_x=71 unique_y=16
    //
    // Something something CRT tells us that there is some unique value < 103 * 105 where both of these occur.
    // There's some math soln for this, but it escapes me.  Let's brute-force it.
    //
    // We know there is some value where the minima for x and y align.  Call this K.
    // We also know K % 101 = 71, and K % 103 = 16.  We can solve for K by brute force, checking
    // the map for localized X and Y coords at each second. But we only really need to check the
    // maps at unique_x + n * width, searching for a minimized y.

    let mut t = unique_x;
    loop {
        let game = game_at(game, height, width, t as i32);
        let unique_y = game.iter().map(|(pos, _)| pos.y).collect::<std::collections::HashSet<_>>().len();
        if unique_y < 96 {
            // display(&game, height, width);
            return t;
        }
        t += width;
    }

}

#[aoc(day14, part1)]
fn part1(game: &Game) -> Answer {

    sim(game, 103, 101)

}

#[aoc(day14, part2)]
fn part2(game: &Game) -> Answer {
    sim2(game, 103, 101)
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn part1_example() {
        let game = parse(SAMPLE);
        assert_eq!(sim(&game, 7, 11), 12);
    }
}