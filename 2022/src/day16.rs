use rustc_hash::{FxHashMap, FxHashSet};
use sscanf::sscanf;
use utils::prelude::*;

#[derive(Clone)]
pub struct Valve {
    rate: u32,
    edges: Vec<usize>,
}
type Visited = FxHashMap<(usize, u32, u64), u32>;
type Opened = u64; // 1-bit per valve

parse!(|i| -> Vec<Valve> {
    let ids: FxHashMap<&str, usize> = i
        .lines()
        .sorted() // 'AA' will be index 0
        .enumerate()
        .map(|(a, b)| (b.split(' ').nth(1).unwrap(), a))
        .collect();

    i.lines()
        .sorted()
        .map(|l| {
            let (_, rate, _, edges) =
                sscanf!(l, "Valve {str} has flow rate={u32}; {str:/.*valve[s]?/} {str}").unwrap();
            let edges = edges.split(", ").map(|id| ids[id]).collect();
            Valve { rate, edges }
        })
        .collect()
} as Valves);

pub fn part1(valves: Valves) -> u32 {
    traverse(0, &valves, &mut 0, &mut Visited::default(), 30)
}

pub fn part2(valves: Valves) -> u32 {
    let minutes = 26;
    let mut value = 0;
    for (a, b) in combinations(&valves) {
        let a = traverse(0, &a, &mut 0, &mut Visited::default(), minutes);
        let b = traverse(0, &b, &mut 0, &mut Visited::default(), minutes);
        value = value.max(a + b);
    }
    value
}

fn traverse(
    cur: usize,
    valves: &Valves,
    opened: &mut Opened,
    visited: &mut Visited,
    minute: u32,
) -> u32 {
    if minute == 0 {
        return 0;
    }

    let mut value = 0;
    if valves[cur].rate > 0 && (*opened & (1 << cur)) == 0 {
        *opened |= 1 << cur;
        value = valves[cur].rate * (minute - 1);
        value += traverse(cur, valves, opened, visited, minute - 1);
        *opened &= !(1 << cur);
    }

    for next in &valves[cur].edges {
        let hash = (*next, minute, *opened);
        if !visited.contains_key(&hash) {
            let v = traverse(*next, valves, opened, visited, minute - 1);
            visited.insert(hash, v);
            value = value.max(v);
        } else {
            value = value.max(*visited.get(&hash).unwrap());
        }
    }
    value
}

fn combinations(valves: &Valves) -> Vec<(Valves, Valves)> {
    let ids: FxHashSet<_> = (0..valves.len()).collect();
    let ids = valves
        .iter()
        .enumerate()
        .filter(|(_, v)| v.rate > 0)
        .map(|(id, _)| id)
        .combinations(valves.iter().enumerate().filter(|(_, v)| v.rate > 0).count() / 2)
        .map(|s| {
            let a = s.iter().cloned().collect::<FxHashSet<_>>();
            let b = ids.difference(&a).cloned().collect::<FxHashSet<_>>();
            (a, b)
        })
        .collect_vec();

    ids.iter()
        .filter(|(a, _)| a.iter().map(|&id| valves[id].rate >= 20).filter(|&t| t).count() == 1)
        .map(|(a, b)| {
            [a, b]
                .iter()
                .map(|s| {
                    Valves(
                        valves
                            .iter()
                            .enumerate()
                            .map(|(id, v)| {
                                if s.contains(&id) {
                                    v.clone()
                                } else {
                                    Valve { rate: 0, ..v.clone() }
                                }
                            })
                            .collect(),
                    )
                })
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

tests! {
    (part1, "sample", 1651)
    (part1, "puzzle", 2029)
    (part2, "sample", 1707)
    (part2, "puzzle", 2723)
}
