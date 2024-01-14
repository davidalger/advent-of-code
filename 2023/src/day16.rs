use rustc_hash::FxHashSet;

utils::parse_grid!(u8);

pub fn part1(grid: Grid) -> usize {
    // enter from left of top corner
    let entry_point = match grid[0][0] {
        b'.' | b'-' => (0, 0, 'r'),
        b'|' => (0, 0, 'd'),
        b'\\' => (0, 0, 'd'),
        b'/' => unreachable!(),
        _ => unreachable!(),
    };
    energized_tiles(&grid, entry_point)
}

pub fn part2(grid: Grid) -> usize {
    let mut entry_points = vec![];

    // starting points from left and right
    for y in 0..grid.len() {
        entry_points.push((y, 0, 'r'));
        entry_points.push((y, grid[0].len() - 1, 'l'));
    }

    // upward and downward starting points
    for x in 0..grid[0].len() {
        entry_points.push((0, x, 'd'));
        entry_points.push((grid.len() - 1, x, 'u'));
    }

    entry_points.iter().map(|&entry_point| energized_tiles(&grid, entry_point)).max().unwrap()
}

fn energized_tiles(grid: &Grid, entry_point: (usize, usize, char)) -> usize {
    let mut state = vec![0; grid.len() * grid[0].len()];
    let mut stack = vec![entry_point];
    let mut visited = FxHashSet::default();

    while let Some((y, x, h)) = stack.pop() {
        if !visited.contains(&(y, x, h)) {
            state[y * grid.len() + x] = 1;
            visited.insert((y, x, h));
            stack.append(&mut adjacent_edges(grid, y, x, h));
        }
    }

    state.iter().sum()
}

fn adjacent_edges(grid: &Grid, y: usize, x: usize, h: char) -> Vec<(usize, usize, char)> {
    match h {
        'r' if x + 1 < grid[0].len() => {
            let x = x + 1;
            match grid[y][x] {
                b'.' | b'-' => vec![(y, x, 'r')],
                b'|' => vec![(y, x, 'u'), (y, x, 'd')],
                b'\\' => vec![(y, x, 'd')],
                b'/' => vec![(y, x, 'u')],
                _ => unreachable!(),
            }
        }
        'l' if x > 0 => {
            let x = x - 1;
            match grid[y][x] {
                b'.' | b'-' => vec![(y, x, 'l')],
                b'|' => vec![(y, x, 'u'), (y, x, 'd')],
                b'\\' => vec![(y, x, 'u')],
                b'/' => vec![(y, x, 'd')],
                _ => unreachable!(),
            }
        }
        'u' if y > 0 => {
            let y = y - 1;
            match grid[y][x] {
                b'.' | b'|' => vec![(y, x, 'u')],
                b'-' => vec![(y, x, 'l'), (y, x, 'r')],
                b'\\' => vec![(y, x, 'l')],
                b'/' => vec![(y, x, 'r')],
                _ => unreachable!(),
            }
        }
        'd' if y + 1 < grid.len() => {
            let y = y + 1;
            match grid[y][x] {
                b'.' | b'|' => vec![(y, x, 'd')],
                b'-' => vec![(y, x, 'l'), (y, x, 'r')],
                b'\\' => vec![(y, x, 'r')],
                b'/' => vec![(y, x, 'l')],
                _ => unreachable!(),
            }
        }
        _ => vec![],
    }
}

utils::tests! {
    (part1, "sample", 46)
    (part1, "puzzle", 8323)
    (part2, "sample", 51)
    (part2, "puzzle", 8491)
}
