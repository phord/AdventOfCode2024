use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

use crate::{grid::ContentGrid, point::Point};

type Game = (ContentGrid, Vec<char>);
type Answer = usize;

#[aoc_generator(day15)]
fn parse(input: &str) -> Game {
    let (map, moves) = input.split_once("\n\n").unwrap();

    let map = ContentGrid::new(map);
    let moves = moves.lines().flat_map(|line| line.chars()).collect::<Vec<_>>();

    (map, moves)
}


fn push(map: &mut ContentGrid, pos: Point, dir: Point) -> Point {
    let next = pos + dir;
    let actual =
        if map.map[&'.'].contains(&next) {
            next
        } else if map.map[&'O'].contains(&next) {
            if push(map, next, dir) == next {
                pos
            } else {
                next
            }
        } else {
            // println!("Couldn't move");
            pos
        };

    if actual == next {
        let token = if map.map[&'O'].contains(&pos) {
            'O'
        } else if map.map[&'@'].contains(&pos) {
            '@'
        } else {
            panic!("I'm lost")
        };
        // println!("Moving {} from {} to {}", token, &pos.y, &next.y);
        map.map.get_mut(&'.').unwrap().remove(&next);
        map.map.get_mut(&'.').unwrap().insert(pos);
        map.map.get_mut(&token).unwrap().remove(&pos);
        map.map.get_mut(&token).unwrap().insert(next);
    }
    actual
}

#[aoc(day15, part1)]
fn part1(game: &Game) -> Answer {

    let dirs: HashMap<char, Point> = HashMap::from([('^', Point::new(0, 1)), ('v', Point::new(0, -1)), ('<', Point::new(-1, 0)), ('>', Point::new(1, 0))]);

    let mut map = game.0.clone();
    let moves = &game.1;


    let mut pos = *map.map[&'@'].iter().next().unwrap();
    println!("{}\n", &map);

    for m in moves {
        // println!("({},{}) {}:", &pos.x, &pos.y, &m);
        assert!(map.map[&'@'].contains(&pos));
        let dir = dirs[&m];
        pos = push(&mut map, pos, dir);
        // println!("{}\n", &map);
    }

    // for p in map.map[&'O'].iter() {
    //     println!("({},{})", p.x, p.y);
    // }
    map.map[&'O'].iter().map(|p| (map.height - 1 - p.y) * 100 + p.x).sum::<i32>() as usize
}


#[aoc(day15, part2)]
fn part2(game: &Game) -> Answer {

    let dirs: HashMap<char, Point> = HashMap::from([('^', Point::new(0, 1)), ('v', Point::new(0, -1)), ('<', Point::new(-1, 0)), ('>', Point::new(1, 0))]);

    let mut map = game.0.clone();
    map.width *= 2;

    let boxes = map.map[&'O'].clone();
    map.map.get_mut(&'O').unwrap().clear();
    map.map.insert('[', boxes.iter().map(|p| Point::new(p.x * 2, p.y)).collect());
    map.map.insert(']', boxes.iter().map(|p| Point::new(p.x * 2 + 1, p.y)).collect());

    let walls = map.map[&'#'].iter().flat_map(|p| [Point::new(p.x * 2, p.y), Point::new(p.x * 2 + 1, p.y)]).collect::<HashSet<_>>();
    map.map.insert('#', walls);

    let dots = map.map[&'.'].iter().flat_map(|p| [Point::new(p.x * 2, p.y), Point::new(p.x * 2 + 1, p.y)]).collect::<HashSet<_>>();
    map.map.insert('.', dots);

    let mut pos = *map.map[&'@'].iter().next().unwrap();
    map.map.get_mut(&'@').unwrap().remove(&pos);
    pos = Point::new(pos.x * 2, pos.y);
    map.map.get_mut(&'@').unwrap().insert(pos);
    map.map.get_mut(&'.').unwrap().insert(pos + Point::new(1, 0));

    let moves = &game.1;

    println!("{}\n", &map);

    for m in moves {
        // println!("({},{}) {}:", &pos.x, &pos.y, &m);
        assert!(map.map[&'@'].contains(&pos));
        let dir = dirs[&m];
        pos = push(&mut map, pos, dir);
        // println!("{}\n", &map);
    }

    // for p in map.map[&'O'].iter() {
    //     println!("({},{})", p.x, p.y);
    // }
    map.map[&'O'].iter().map(|p| (map.height - 1 - p.y) * 100 + p.x).sum::<i32>() as usize
}




#[cfg(test)]
mod tests {
    use super::*;

    const STARTER: &str =     "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    const SAMPLE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(STARTER)), 2028);
        assert_eq!(part1(&parse(SAMPLE)), 10092);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(STARTER)), 123);
        assert_eq!(part2(&parse(SAMPLE)), 9021);
    }
}