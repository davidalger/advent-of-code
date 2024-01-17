use std::hash::Hash;

use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Node(usize, usize, u8);

pub fn part1(input: String) -> String {
    solve(parse(&input), 1, 3)
}

pub fn part2(input: String) -> String {
    solve(parse(&input), 4, 10)
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().iter().map(|x| x - b'0').collect()).collect()
}

fn solve(grid: Vec<Vec<u8>>, min: usize, max: usize) -> String {
    let (path, cost) = chart_path(&grid, min, max);

    if !cfg!(test) && grid.len() < 20 {
        format!("{}\n{cost}", render_path(&grid, &path))
    } else {
        format!("{cost}")
    }
}

fn chart_path(grid: &[Vec<u8>], min: usize, max: usize) -> (Vec<Node>, u32) {
    dijkstra(
        &[Node(0, 0, b'v'), Node(0, 0, b'>')],
        |node: &Node| {
            let Node(y, x, _) = *node;
            y == grid.len() - 1 && x == grid[0].len() - 1
        },
        |node: &Node| {
            let Node(y, x, h) = *node;
            let mut res = vec![];
            for i in min..=max {
                if matches!(h, b'^' | b'v') {
                    if x + i < grid[y].len() {
                        res.push((Node(y, x + i, b'>'), (1..=i).map(|i| grid[y][x + i]).sum()));
                    }

                    if x >= i {
                        res.push((Node(y, x - i, b'<'), (1..=i).map(|i| grid[y][x - i]).sum()));
                    }
                }

                if matches!(h, b'<' | b'>') {
                    if y + i < grid.len() {
                        res.push((Node(y + i, x, b'v'), (1..=i).map(|i| grid[y + i][x]).sum()));
                    }

                    if y >= i {
                        res.push((Node(y - i, x, b'^'), (1..=i).map(|i| grid[y - i][x]).sum()));
                    }
                }
            }
            res
        },
    )
    .expect("no path found")
}

fn render_path(grid: &[Vec<u8>], path: &[Node]) -> String {
    let mut res = String::new();

    let mut grid: Vec<Vec<_>> =
        grid.iter().map(|row| row.iter().map(|x| x + b'0').collect()).collect();

    for n in 1..path.len() {
        let Node(y, x, h) = path[n];
        let prev = path[n - 1];

        for i in 0..(y.abs_diff(prev.0) + x.abs_diff(prev.1)) {
            match h {
                b'<' => grid[y][x + i] = h,
                b'>' => grid[y][x - i] = h,
                b'^' => grid[y + i][x] = h,
                b'v' => grid[y - i][x] = h,
                _ => unreachable!(),
            };
        }
    }

    for row in grid {
        res += &format!("{}\n", String::from_utf8(row).unwrap());
    }

    res
}

fn dijkstra<V, FS, FN>(starts: &[V], success: FS, successors: FN) -> Option<(Vec<V>, u32)>
where
    V: Eq + Hash + Copy,
    FS: Fn(&V) -> bool,
    FN: Fn(&V) -> Vec<(V, u8)>,
{
    let mut unvisited = FxHashSet::default();
    let mut visited = FxHashSet::default();
    let mut parents = FxHashMap::default();
    let mut costs = FxHashMap::default();

    starts.iter().for_each(|&start| {
        unvisited.insert(start);
        costs.insert(start, 0);
    });

    while let Some((node, cost)) =
        unvisited.iter().map(|v| (*v, *costs.get(v).unwrap())).min_by_key(|(_, c)| *c)
    {
        successors(&node).iter().for_each(|&(n, c)| {
            costs
                .entry(n)
                .and_modify(|x| {
                    if *x > cost + c as u32 {
                        parents.insert(n, node);
                        *x = cost + c as u32;
                    }
                })
                .or_insert_with(|| {
                    unvisited.insert(n);
                    parents.insert(n, node);
                    cost + c as u32
                });
        });
        unvisited.remove(&node);
        visited.insert(node);
    }

    costs
        .iter()
        .filter(|(node, _)| success(node))
        .map(|(mut node, cost)| {
            let mut path = vec![*node];
            while let Some(parent) = parents.get(node) {
                path.push(*parent);
                node = parent;
            }
            path.reverse();
            (path, *cost)
        })
        .min_by(|(_, a), (_, b)| a.cmp(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(part1(utils::input!("sample")), "102");
    }

    #[test]
    fn part1_render() {
        let grid = parse(&utils::input!("sample"));
        let (path, _) = chart_path(&grid, 1, 3);

        assert_eq!(
            render_path(&grid, &path).trim(),
            vec![
                "2>>34^>>>1323",
                "32v>>>35v5623",
                "32552456v>>54",
                "3446585845v52",
                "4546657867v>6",
                "14385987984v4",
                "44578769877v6",
                "36378779796v>",
                "465496798688v",
                "456467998645v",
                "12246868655<v",
                "25465488877v5",
                "43226746555v>",
            ]
            .join("\n")
        );
    }

    #[test]
    fn part1_puzzle() {
        assert_eq!(part1(utils::input!("puzzle")), "870");
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(utils::input!("sample")), "94");
    }

    #[test]
    fn part2_render() {
        let grid = parse(&utils::input!("sample"));
        let (path, _) = chart_path(&grid, 4, 10);

        assert_eq!(
            render_path(&grid, &path).trim(),
            vec![
                "2>>>>>>>>1323",
                "32154535v5623",
                "32552456v4254",
                "34465858v5452",
                "45466578v>>>>",
                "143859879845v",
                "445787698776v",
                "363787797965v",
                "465496798688v",
                "456467998645v",
                "122468686556v",
                "254654888773v",
                "432267465553v",
            ]
            .join("\n")
        );
    }

    #[test]
    fn part2_puzzle() {
        assert_eq!(part2(utils::input!("puzzle")), "1063");
    }
}
