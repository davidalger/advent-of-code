utils::parse_grid!(u8);

pub fn part1(mut grid: Grid) -> usize {
    tilt_sort(&mut grid, Tilt::North);
    calculate_load(&grid)
}

pub fn part2(mut grid: Grid) -> usize {
    // calculate load for first x iterations
    let load: Vec<usize> = (0..200)
        .map(|_| {
            for tilt in [Tilt::North, Tilt::West, Tilt::South, Tilt::East] {
                tilt_sort(&mut grid, tilt);
            }
            calculate_load(&grid)
        })
        .collect();

    // determine length of repeating pattern
    let mut repeat = 10;
    loop {
        if load[load.len() - repeat - 1..load.len() - 1]
            == load[load.len() - repeat * 2 - 1..load.len() - repeat - 1]
        {
            break;
        }
        repeat += 1;
    }

    // determine number of iterations before pattern emerges
    let mut lead = 0;
    loop {
        if load[lead..lead + repeat] == load[lead + repeat..lead + repeat * 2] {
            break;
        }
        lead += 1;
    }

    // calculate load at nth iteration
    load[((1000000000 - lead) % repeat) + lead - 1]
}

fn calculate_load(grid: &Grid) -> usize {
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .map(|val| match val {
                    b'O' => grid.len() - y,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum()
}

enum Tilt {
    North,
    South,
    West,
    East,
}

fn tilt_sort(grid: &mut Grid, tilt: Tilt) {
    let (a, b) = match tilt {
        Tilt::North | Tilt::West => (b'.', b'O'),
        Tilt::South | Tilt::East => (b'O', b'.'),
    };

    let horizontal = match tilt {
        Tilt::North | Tilt::South => false,
        Tilt::West | Tilt::East => true,
    };

    for j in if horizontal { 0..grid.len() } else { 0..grid[0].len() } {
        loop {
            let mut swaps = 0;

            if horizontal {
                for k in 1..grid[0].len() {
                    if grid[j][k - 1] == a && grid[j][k] == b {
                        grid[j][k - 1] = b;
                        grid[j][k] = a;
                        swaps += 1;
                    }
                }
            } else {
                for k in 1..grid.len() {
                    if grid[k - 1][j] == a && grid[k][j] == b {
                        grid[k - 1][j] = b;
                        grid[k][j] = a;
                        swaps += 1;
                    }
                }
            }

            if swaps == 0 {
                break;
            }
        }
    }
}

utils::tests! {
    (part1, "sample", 136)
    (part1, "puzzle", 109098)
    (part2, "sample", 64)
    (part2, "puzzle", 100064)
}
