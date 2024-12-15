use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{all, any};

use crate::{grid::GroupMap, point::Point};

type Game = (GroupMap, Vec<char>);
type Answer = usize;

#[aoc_generator(day15)]
fn parse(input: &str) -> Game {
    let (map, moves) = input.split_once("\n\n").unwrap();

    let map = GroupMap::new(map);
    let moves = moves.lines().flat_map(|line| line.chars()).collect::<Vec<_>>();

    (map, moves)
}

fn push(map: &mut GroupMap, pos: &[Point], dir: Point) -> bool {
    let moveable = ".[]@O";
    let mut pos = pos.iter().cloned().
            filter(|p| map.get(p) != '.').collect::<HashSet<_>>();
    if dir.x == 0 {
        // When pushing boxes up or down, we need to consider our companion cells
        let companions = pos.iter().flat_map(|p|
            match map.get(p) {
                '[' => [*p + Point::new(1, 0)].to_vec(),
                ']' => [*p + Point::new(-1, 0)].to_vec(),
                _ => [].to_vec(),
            }).collect::<HashSet<_>>();
        pos.extend(companions.iter());

    }
    // Can't move this
    if any(&pos, |p| !moveable.contains(map.get(p))) {
        return false;
    }

    // Nothing in the way
    if all(&pos, |p| map.get(p) == '.') {
        return true;
    }

    // Something in the way.  See if we can move it up, too.
    let next = pos.iter().map(|p| *p + dir).collect::<Vec<_>>();
    if !push(map, &next, dir) {
        // Nope
        return false;
    }

    for p in pos.iter() {
        let next = *p + dir;
        assert!(map.get(&next) == '.');
        map.swap(*p, next);
    }
    true
}

fn gps_score(map: &GroupMap, cell: char) -> usize {
    map.map[&cell].iter().map(|p| (map.height - 1 - p.y) * 100 + p.x).sum::<i32>() as usize
}

fn play_moves(map: &GroupMap, moves: &[char]) -> GroupMap {
    let dirs: HashMap<char, Point> = HashMap::from([('^', Point::new(0, 1)), ('v', Point::new(0, -1)), ('<', Point::new(-1, 0)), ('>', Point::new(1, 0))]);

    let mut map = map.clone();
    let mut pos = *map.map[&'@'].iter().next().unwrap();

    for m in moves {
        assert!(map.map[&'@'].contains(&pos));
        if push(&mut map, &[pos], dirs[m]) {
            pos = pos + dirs[m];
        }
    }

    map
}

#[aoc(day15, part1)]
fn part1(game: &Game) -> Answer {
    let map = play_moves(&game.0, &game.1);
    gps_score(&map, 'O')
}

fn double_wide(map: &GroupMap) -> GroupMap {
    let mut map = map.clone();
    map.width *= 2;

    let boxes = map.map[&'O'].clone();
    map.map.get_mut(&'O').unwrap().clear();
    map.map.insert('[', boxes.iter().map(|p| Point::new(p.x * 2, p.y)).collect());
    map.map.insert(']', boxes.iter().map(|p| Point::new(p.x * 2 + 1, p.y)).collect());

    let walls = map.map[&'#'].iter().flat_map(|p| [Point::new(p.x * 2, p.y), Point::new(p.x * 2 + 1, p.y)]).collect::<HashSet<_>>();
    map.map.insert('#', walls);

    let dots = map.map[&'.'].iter().flat_map(|p| [Point::new(p.x * 2, p.y), Point::new(p.x * 2 + 1, p.y)]).collect::<HashSet<_>>();
    map.map.insert('.', dots);

    let pos = *map.map[&'@'].iter().next().unwrap();
    map.map.get_mut(&'@').unwrap().remove(&pos);
    let pos = Point::new(pos.x * 2, pos.y);
    map.map.get_mut(&'@').unwrap().insert(pos);
    map.map.get_mut(&'.').unwrap().insert(pos + Point::new(1, 0));

    assert_eq!(map.get(&pos), '@');
    assert_eq!(map.map[&'@'].len(), 1);

    map
}

#[aoc(day15, part2)]
fn part2(game: &Game) -> Answer {

    let map = double_wide(&game.0);
    let map = play_moves(&map, &game.1);
    gps_score(&map, '[')
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

    const SAMPLE2: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(STARTER)), 2028);
        assert_eq!(part1(&parse(SAMPLE)), 10092);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE2)), 618);
        assert_eq!(part2(&parse(SAMPLE)), 9021);
    }
}