use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::{grid::Grid, point::Point};
#[aoc_generator(day16)]


fn parse(input: &str) -> Grid {
    Grid::new(input)
}


fn turn_right(dir: Point) -> Point {
    Point::new(dir.y, -dir.x)
}

fn turn_left(dir: Point) -> Point {
    Point::new(-dir.y, dir.x)
}

type Path = HashSet<Point>;
//               (start, start_dir, end, end_dir, cost)
type NodeOptions = (Point, Point, Point, Point, usize, Path);
type PathMap = HashMap<Point, Vec<NodeOptions>>;

fn build_map(grid: &Grid, pos: Point, freeways: &HashSet<Point>, nodes: &HashSet<Point>, start_dir: Point) -> NodeOptions {
    let cells = freeways.union(nodes).collect::<HashSet<_>>();
    let mut end_dir = start_dir;
    let mut cost = 0;
    let mut prev = pos;
    let mut cur = pos + start_dir;
    let mut path = HashSet::new();
    path.insert(pos);

    loop {
        path.insert(cur);
        assert!(cells.contains(&cur));
        cost += 1;
        let dd = cur - prev;
        if dd != end_dir {
            cost += 1000;
            end_dir = dd;
        }
        let nay = cur.neighbors_straight();
        let nay = nay.iter().filter(|p| cells.contains(p) && **p != prev).collect::<Vec<_>>();
        if nay.len() != 1 || nodes.contains(&cur) {
            return (pos, start_dir, cur, end_dir, cost, path);
        }
        prev = cur;
        cur = *nay[0];
    }
}

// Remove all the cells with no decisions to make and return linked node map with costs between each
fn simplify(game: &Grid) -> PathMap {
    let start = *game.map[&'S'].iter().next().unwrap();
    let goal = *game.map[&'E'].iter().next().unwrap();

    let freeways:HashSet<_> = game.map[&'E'].iter()
        .chain(game.map[&'S'].iter())
        .chain(game.map[&'.'].iter())
        .cloned()
        .collect();

    // println!("freeways: {:?}", freeways);

    let nodes = freeways.iter()
            .filter(|p| **p == start || **p == goal || p.neighbors_straight().iter()
                                    .filter(|p| freeways.contains(p))
                                    .count() != 2)
            .cloned()
            .collect::<std::collections::HashSet<_>>();

    // println!("nodes: {:?}", nodes);

    let segments = nodes.iter()
            .map(|start|
                (*start,
                    start.neighbors_straight().iter()
                        .filter(|p| freeways.contains(p) || nodes.contains(p))
                        .map(|p| build_map(game, *start, &freeways, &nodes, *p - *start))
                        .collect::<Vec<_>>()
                )
            )
            .collect::<HashMap<_,_>>();
            // .collect::<HashMap<Point, NodeOptions>>();

    // // "Reduce" the segment population by combining all the possible pairs of segments and eliminating every other node.
    // let mut target = goal;
    // let mut work = [goal].to_vec();
    // while let Some(target) = work.pop() {
    //     let target = target.clone();
    //     let mut segments = segments.clone();
    //     let mut removed = HashSet::new();
    //     let mut new_work = Vec::default();
    //     let new_segs:Vec<_> =
    //         segments[&target].iter()
    //             .flat_map(|ppath0| {
    //                 let (start, start_dir, end, end_dir, cost, path) = ppath0.clone();
    //                 removed.insert((target, end));
    //                 removed.insert((end, target));

    //                 segments[&end].iter()
    //                     .map(|ppath| {
    //                         let (start2, start_dir2, end2, end_dir2, cost2, path2) = ppath;
    //                         assert!(end == *start2);
    //                         new_work.push(end2.clone());
    //                         removed.insert((*start2, *end2));
    //                         removed.insert((*end2, *start2));
    //                         let mut path = path2.clone();
    //                         path.extend(path2.iter());
    //                         let extra_cost =
    //                             if end_dir != *start_dir2 {
    //                                 1000
    //                             } else {
    //                                 0
    //                             };
    //                         (target, start_dir, *end2, *end_dir2, cost + cost2 + extra_cost, path)
    //                     })
    //             }).collect();

    //     println!("Queue: {}  Adding: {} Removing: {}", work.len(), new_segs.len(), removed.len());

    //     segments = segments.iter()
    //         .map(|(k, v)| (*k, v.iter()
    //             .filter(|(start, _, end, _, _, _)| !removed.contains(&(*start, *end)))
    //             .cloned()
    //             .collect::<Vec<_>>()))
    //         .filter(|(_, v)| !v.is_empty())
    //         .collect::<HashMap<_,_>>();

    //     segments.insert(target, new_segs);
    //     target = *segments.keys().next().unwrap();
    // }


    segments
}

fn seek(pos: Point, dir: Point, paths: PathMap, seen: HashSet<Point>, goal: Point, memo: &mut HashMap<(Point, Point), usize>, path: Path, best: usize, cost: usize, memo_path: &mut (usize, Path)) -> Option<usize> {
    // println!("seek: depth:{}  {} -> {}  cost:{}  best:{}", seen.len(), pos, goal, cost, best);
    if pos == goal {
        // println!("seek: GOOOOOOOOAL! {}", cost);
        if cost < memo_path.0 {
            memo_path.1.clear();
            memo_path.0 = cost;
        }
        if cost == memo_path.0 {
            memo_path.1.extend(path.iter());
            println!("BEST: {}  {}", cost, memo_path.1.len());
            return Some(cost);
        }
        return None;
    } else if best < cost {
        // println!("seek: depth:{}  {} -> {}  cost:{}  best:{}  too expensive", seen.len(), pos, goal, cost, best);
        return None;
    } else if memo.keys().contains( &(pos, dir)) && memo[&(pos, dir)] < cost {
        // println!("Seen this one before:  {} -> {}  prev:{} <= cost:{} ", pos, goal, memo[&(pos, dir)], cost );
        return None;
    }
    memo.insert((pos, dir), cost);

    // println!("seek: {:?} -> {:?}", pos, goal);
    // println!("paths: {:?}", paths.keys());
    let options = paths[&pos].clone();
    let mut best = best;
    let cost =
        options.iter()
            .filter(|p| !seen.contains(&p.2))
            .map(|(start, start_dir, end, end_dir, path_cost, new_path)| {
                assert_eq!(*start, pos);
                let paths = paths.clone();
                let mut seen = seen.clone();
                seen.insert(pos);

                let mut path = path.clone();
                path.extend(new_path.iter());

                if dir != *start_dir && num::abs(dir.x) == num::abs(start_dir.x) {
                    println!("Oh shit:  dir:{} start_dir:{}  pos:{}  end:{}  seen:{:?}", dir, start_dir, pos, end, seen);
                }
                assert!(dir == *start_dir || num::abs(dir.x) != num::abs(start_dir.x));
                let cost = cost + path_cost + if dir == *start_dir { 0 } else { 1000 } ;
                let this_cost = if let Some(c) = seek(*end, *end_dir, paths, seen.clone(), goal, memo, path, best, cost, memo_path) {
                    c
                } else {
                    // fixme: better way to say "no answer this path"?
                    usize::MAX/2
                };
                best = best.min(this_cost);
                this_cost
            })
            .min();
    cost
}

fn djikstra(game: &Grid) -> usize {
    let paths = simplify(game);
    let start = *game.map[&'S'].iter().next().unwrap();
    let goal = *game.map[&'E'].iter().next().unwrap();
    let mut dir = Point::new(1, 0);

    let mut dist: HashMap<_, _> = paths.iter().map(|(k, v)| (*k, usize::MAX)).collect();
    let mut prev = HashMap::new();
    let mut q: HashMap<_, _> = paths.iter().map(|(k, v)| (*k, usize::MAX)).collect();

    dist.insert(start, 0);

    q.insert(start, 0);

    while !q.is_empty() {
        let u = *q.iter().min_by_key(|(_, v)| *v).unwrap().0;
        q.remove(&u);

        for v in paths[&u].iter() {
            let (next, sdir, _, edir, mut cost, _) = v;
            if *sdir != dir {
                cost += 1000;
            }
            let alt = dist[&u].saturating_add(cost);

            if alt < dist[next] {
                dist.insert(*next, alt);
                prev.insert(next, u);
            }
        }
    }
    println!("dist: {:?}", dist);
    println!("prev: {:?}", prev);
    println!("q: {:?}", q);

    0
}

fn solve(game: &Grid) -> (usize, usize) {
    djikstra(game);

    let paths = simplify(game);

    let east: Point = Point::new(1, 0);
    let dir = east;
    let pos = *game.map[&'S'].iter().next().unwrap();
    let goal = *game.map[&'E'].iter().next().unwrap();
    let memo: &mut HashMap<(Point, Point), _> = &mut HashMap::new();
    let mut memo_path = (usize::MAX, Path::default());

    println!("Total paths: {}", paths.len());
    let cost = seek(pos, dir, paths, HashSet::new(), goal, memo, Path::default(), 82460, 0, &mut memo_path).unwrap();

    let mut game = game.clone();
    for p in memo_path.1.iter() {
        game.set(p, 'O');
    }
    println!("{}", game);

    (cost, memo_path.1.len())
}

#[aoc(day16, part1)]
fn part1(game: &Grid) -> usize {
    solve(game).0
}

#[aoc(day16, part2)]
fn part2(game: &Grid) -> usize {
    solve(game).1
}


#[cfg(test)]
mod tests {
    use super::*;


    const SAMPLE: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";


    const SAMPLE2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";


    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 7036);
        assert_eq!(part1(&parse(SAMPLE2)), 11048);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), 45);
        assert_eq!(part2(&parse(SAMPLE2)), 64);
    }
}

// too high: 142556
