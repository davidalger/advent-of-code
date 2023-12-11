use itertools::Itertools;

utils::parse!(|i| -> Vec<Vec<char>> { i.lines().map(|l| l.chars().collect()).collect() } as Grid);

pub fn part1(grid: Grid) -> usize {
    solve(grid, 2)
}

pub fn part2(grid: Grid) -> usize {
    solve(grid, 1000000)
}

fn solve(grid: Grid, factor: usize) -> usize {
    let empty_rows = (0..grid.len())
        .filter(|&i| (0..grid[i].len()).filter(|&j| grid[i][j] == '#').count() == 0)
        .collect_vec();

    let empty_cols = (0..grid[0].len())
        .filter(|&j| (0..grid.len()).filter(|&i| grid[i][j] == '#').count() == 0)
        .collect_vec();

    grid.iter()
        .enumerate()
        .filter_map(|(y, row)| {
            let p = row
                .iter()
                .enumerate()
                .filter_map(|(x, &val)| if val == '#' { Some((x, y)) } else { None })
                .collect_vec();

            if !p.is_empty() {
                Some(p)
            } else {
                None
            }
        })
        .concat()
        .iter()
        .copied()
        .tuple_combinations()
        .map(|((a_x, a_y), (b_x, b_y))| {
            let mut steps = a_x.abs_diff(b_x) + a_y.abs_diff(b_y);

            empty_rows.iter().for_each(|&y| {
                if a_y < y && b_y > y {
                    steps += factor - 1
                }
            });

            empty_cols.iter().for_each(|&x| {
                if (a_x < x && b_x > x) || (a_x > x && b_x < x) {
                    steps += factor - 1;
                }
            });

            steps
        })
        .sum()
}

utils::tests! {
    (part1, "sample", 374)
    (part1, "puzzle", 10165598)
    (part2, "sample", 82000210)
    (part2, "puzzle", 678728808158)
}
