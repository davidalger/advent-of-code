use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use sscanf::sscanf;
use utils::debug;

#[derive(Clone)]
pub struct Valve {
    rate: u32,
    edges: Vec<usize>,
}
type Visited = FxHashMap<(usize, u32, BitSet), u32>;
type BitSet = u64; // 1-bit per valve

utils::parse!(|i| -> Vec<Valve> {
    let start = std::time::SystemTime::now();

    let ids: FxHashMap<&str, usize> = i
        .lines()
        .sorted() // 'AA' will be index 0
        .enumerate()
        .map(|(a, b)| (b.split(' ').nth(1).unwrap(), a))
        .collect();

    let valves = i
        .lines()
        .sorted()
        .map(|l| {
            let (_, rate, _, edges) =
                sscanf!(l, "Valve {str} has flow rate={u32}; {str:/.*valve[s]?/} {str}").unwrap();
            let edges = edges.split(", ").map(|id| ids[id]).collect();
            Valve { rate, edges }
        })
        .collect_vec();

    if valves.len() as u32 > BitSet::BITS {
        panic!("Too many valves for BitSet");
    }

    let duration = std::time::SystemTime::now().duration_since(start).unwrap();
    debug!("Parsing {} valves took {duration:?}", valves.len());

    valves
} as Valves);

pub fn part1(valves: Valves) -> u32 {
    traverse(0, &valves, 0, &mut Visited::default(), 30)
}

pub fn part2(valves: Valves) -> u32 {
    let mut visited = Visited::default();
    visited.reserve(125000);

    combinations(&valves)
        .iter()
        .map(|(a, b)| {
            [a, b]
                .iter()
                .map(|valves| {
                    visited.clear();
                    traverse(0, valves, 0, &mut visited, 26)
                })
                .sum()
        })
        .max()
        .unwrap()
}

fn traverse(id: usize, valves: &Valves, opened: BitSet, visited: &mut Visited, steps: u32) -> u32 {
    if steps == 0 {
        return 0;
    }

    let mut value = 0;
    if valves[id].rate > 0 && (opened & (1 << id)) == 0 {
        value = valves[id].rate * (steps - 1)
            + traverse(id, valves, opened | 1 << id, visited, steps - 1);
    }

    for id in &valves[id].edges {
        let hash = (*id, steps, opened);
        if !visited.contains_key(&hash) {
            let v = traverse(*id, valves, opened, visited, steps - 1);
            visited.insert(hash, v);
            value = value.max(v);
        } else {
            value = value.max(*visited.get(&hash).unwrap());
        }
    }
    value
}

fn combinations(valves: &Valves) -> Vec<(Valves, Valves)> {
    let start = std::time::SystemTime::now();

    let ids: FxHashSet<_> = (0..valves.len()).collect();
    let combinations = valves
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
        .collect_vec();

    let duration = std::time::SystemTime::now().duration_since(start).unwrap();
    debug!("Calculating {} combinations took {duration:?}", combinations.len());

    combinations
}

utils::tests! {
    (part1, "sample", 1651)
    (part1, "puzzle", 2029)
    (part2, "sample", 1707)
    (part2, "puzzle", 2723)
}
