use utils::prelude::*;

parse!(|i| -> Vec<Vec<u32>> {
    i.lines().map(|l| l.bytes().map(|c| c as u32 + 1 - '1' as u32).collect()).collect()
} as Grid);

pub fn part1(grid: Grid) -> u32 {
    let mut score = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if visible(&grid, x, y) {
                score += 1;
            }
        }
    }
    score
}

fn visible(grid: &Grid, x: usize, y: usize) -> bool {
    let h = grid[y][x];

    if y == 0 || x == 0 || y == grid.len() - 1 || x == grid[y].len() - 1 {
        return true; // visible at edge
    }

    if *grid[y][0..x].iter().max().unwrap() < h {
        return true; // visible from left
    }

    if *grid[y][x + 1..grid[y].len()].iter().max().unwrap() < h {
        return true; // visible from right
    }

    let mut above = Vec::new();
    for row in grid.iter().take(y) {
        above.push(row[x]);
    }

    if *above.iter().max().unwrap() < h {
        return true; // visible from top
    }

    let mut below = Vec::new();
    for row in grid.iter().skip(y + 1) {
        below.push(row[x]);
    }

    if *below.iter().max().unwrap() < h {
        return true; // visible from bottom
    }

    false
}

pub fn part2(grid: Grid) -> u32 {
    let mut score = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let s = scenic_score(&grid, x, y);
            if s > score {
                score = s;
            }
        }
    }
    score
}

fn scenic_score(grid: &Grid, x: usize, y: usize) -> u32 {
    let h = grid[y][x];

    let mut left = 0;
    for tree in grid[y][0..x].iter().rev() {
        left += 1;
        if *tree >= h {
            break;
        }
    }

    let mut right = 0;
    for tree in grid[y][x + 1..grid[y].len()].iter() {
        right += 1;
        if *tree >= h {
            break;
        }
    }

    let mut up = 0;
    for row in grid.iter().take(y).rev() {
        up += 1;
        if row[x] >= h {
            break;
        }
    }

    let mut down = 0;
    for row in grid.iter().skip(y + 1) {
        down += 1;
        if row[x] >= h {
            break;
        }
    }

    left * right * up * down
}

tests! {
    (part1, "sample", 21)
    (part1, "puzzle", 1703)
    (part2, "sample", 8)
    (part2, "puzzle", 496650)
}
