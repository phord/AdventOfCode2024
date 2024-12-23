use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::any;

type Map = HashMap<String, HashSet<String>>;
#[aoc_generator(day23)]
fn parse(input: &str) -> Map {
    let mut map = HashMap::default();
    for (k,v) in input.split("\n").map(|l| l.split_once("-").unwrap()) {
        map.entry(k.to_string()).or_insert(HashSet::new()).insert(v.to_string());
        map.entry(v.to_string()).or_insert(HashSet::new()).insert(k.to_string());
    }
    map
}

fn grow_party(map: &Map, nodes: Vec<String>, common: &HashSet<String>) -> Vec<Vec<String>> {
    let mut solutions = Vec::new();

    // Find the next nodes that are common to all the nodes in the party, filtering out any
    //   nodes that precede any in the party alphabetically
    for other in common.iter()
        .filter(|this| !any(nodes.iter(), |p| p < this))
    {
        let pop2 = &map[other];
        let commoner = pop2.intersection(common).cloned().collect::<HashSet<_>>();
        let mut nodes = nodes.clone();
        nodes.push(other.clone());
        solutions.extend(grow_party(map, nodes, &commoner));
    }
    solutions.push(nodes.clone());
    solutions
}

#[aoc(day23, part1)]
fn part1(input: &Map) -> usize {
    let parties = grow_party(input, Vec::new(),
            &input.keys().cloned().collect::<HashSet<_>>());

    // find party of size 3 with computer starting with t
    parties.iter()
            .filter(|p| p.len() == 3)
            .filter(|p| any(p.iter(), |p| p.starts_with("t")))
            .count()

}

#[aoc(day23, part2)]
fn part2(input: &Map) -> String {
    let parties = grow_party(input, Vec::new(),
            &input.keys().cloned().collect::<HashSet<_>>());

    // find longest party
    let mut party = parties.iter()
            .filter(|p| any(p.iter(), |p| p.starts_with("t")))
            .max_by_key(|p| p.len()).unwrap().clone();

    party.sort();

    party.join(",")
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(SAMPLE)), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(SAMPLE)), "co,de,ka,ta");
    }
}