use std::collections::{HashMap, HashSet};

type Node = (i32, i32, char);
type Map = Vec<Node>;
struct Game {
    map: Map,
    height: i32,
    width: i32,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Game {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().trim().len() as i32;
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.trim()
                .chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(|(x, c)| (x as i32, y as i32, c)).collect::<Vec<_>>()
        })
        .fold(Game {
            map: Vec::new(),
            height,
            width,
        }, |mut game, node| {
            game.map.push(node);
            game
        })
}

fn solve(input: &Game, start:i32, end: i32) -> usize {
    let mut antinodes = HashSet::new();
    let mut freq: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for ant in input.map.iter() {
        let (x, y, c) = ant;
        if ! freq.contains_key(c) {
            freq.insert(*c, Vec::new());
        }
        freq.get_mut(c).unwrap().push((*x, *y));
    }

    for (_, nodes) in freq {
        for p1 in nodes.iter() {
            for p2 in nodes.iter() {
                if p1 != p2 {
                    let (n1x, n1y) = p1;
                    let (n2x, n2y) = p2;
                    let dx = n1x - n2x;
                    let dy = n1y - n2y;
                    for i in start..end {
                        let node1 = (n1x + i*dx, n1y + i*dy);
                        let node2 = (n2x - i*dx, n2y - i*dy);
                        antinodes.insert(node1);
                        antinodes.insert(node2);
                        if (node1.0 < 0 || node1.0 >= input.width || node1.1 < 0 || node1.1 >= input.height)
                            && (node2.0 < 0 || node2.0 >= input.width || node2.1 < 0 || node2.1 >= input.height) {
                            break;
                        }
                    }
                }
            }
        }
    }

    antinodes.iter()
        .filter(|(x,y)| *x >= 0 && *x < input.width && *y >= 0 && *y < input.height )
        .count()
}

#[aoc(day8, part1)]
pub fn part1(input: &Game) -> usize {
    solve(input, 1, 2)
}

#[aoc(day8, part2)]
pub fn part2(input: &Game) -> usize {
    solve(input, 0, i32::MAX)
}


#[cfg(test)]
mod tests {
    use super::*;
    // use super::{part1_loops as part1, part2};

    const SAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    // (()) and ()() both result in floor 0.
    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLE)), 14);
        assert_eq!(part2(&input_generator(SAMPLE)), 34);
    }
}
