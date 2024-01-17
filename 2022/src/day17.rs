use itertools::Itertools;

utils::parse!(|i| -> Vec<Jet> {
    i.lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => unimplemented!(),
        })
        .collect()
} as Jets);

pub enum Jet {
    Left,
    Right,
}

pub enum Rock {
    HBar,
    Plus,
    RevL,
    VBar,
    Rect,
}

const WIDTH: usize = 7;

pub fn part1(jets: Jets) -> usize {
    let (grid, _) = simulate(&jets, 2022);
    if grid.len() <= 20 {
        println!("{}", render(&grid));
    }
    grid.len()
}

pub fn part2(jets: Jets) -> usize {
    let (grid, lens) = simulate(&jets, 10000);
    let glen = grid.len();

    let mut slice = 1;
    let mut start = 0;
    let mut chunk = 0;

    let leading;
    let repeating;

    // find length of resulting pattern
    'outer: loop {
        slice += 1;

        if grid[glen - slice * 2..glen - slice] == grid[glen - slice..glen] {
            let slice = &grid[glen - slice..glen];

            // find iterations to end of pattern
            loop {
                chunk += 1;

                let grid = &grid[..lens[chunk] - 1];
                let glen = grid.len();

                if glen >= slice.len() && grid[glen - slice.len()..] == *slice {
                    let slice = &grid[..glen - slice.len()];

                    leading = slice.len();
                    repeating = glen - leading;

                    // find iterations to start of pattern
                    loop {
                        start += 1;

                        if grid[..lens[start] - 1] == *slice {
                            chunk -= start;
                            break 'outer;
                        }
                    }
                }
            }
        }
    }

    let iterations = 1000000000000;
    let remainder = lens[start + chunk + ((iterations - start) % chunk) - 1] - repeating - leading;

    leading + (repeating * ((iterations - start) / chunk) + remainder)
}

fn simulate(jets: &Jets, iterations: usize) -> (Vec<u8>, Vec<usize>) {
    let mut grid: Vec<u8> = Vec::new();
    let mut lens: Vec<usize> = Vec::new();

    let mut i = 0;
    let mut j = 0;
    let jlen = jets.len();
    while i < iterations {
        let len = grid.len();

        // determine type of rock
        let rock: Rock = match i % 5 {
            0 => Rock::HBar,
            1 => Rock::Plus,
            2 => Rock::RevL,
            3 => Rock::VBar,
            4 => Rock::Rect,
            _ => unreachable!(),
        };

        // starting coordinates of falling rock
        let mut x = 2;
        let mut y = len + 3;

        loop {
            // shift right or left
            match jets[j % jlen] {
                Jet::Left => {
                    let shift = x > 0
                        && match rock {
                            Rock::HBar => y >= len || grid[y] & 1 << (x - 1) == 0,
                            Rock::Plus => {
                                (y >= len || grid[y] & 1 << x == 0)
                                    && (y + 1 >= len || grid[y + 1] & 1 << (x - 1) == 0)
                                    && (y + 2 >= len || grid[y + 2] & 1 << x == 0)
                            }
                            Rock::RevL => {
                                (y >= len || grid[y] & 1 << (x - 1) == 0)
                                    && (y + 1 >= len || grid[y + 1] & 1 << (x + 1) == 0)
                                    && (y + 2 >= len || grid[y + 2] & 1 << (x + 1) == 0)
                            }
                            Rock::VBar => {
                                (y >= len || grid[y] & 1 << (x - 1) == 0)
                                    && (y + 1 >= len || grid[y + 1] & 1 << (x - 1) == 0)
                                    && (y + 2 >= len || grid[y + 2] & 1 << (x - 1) == 0)
                                    && (y + 3 >= len || grid[y + 3] & 1 << (x - 1) == 0)
                            }
                            Rock::Rect => {
                                (y >= len || grid[y] & 1 << (x - 1) == 0)
                                    && (y + 1 >= len || grid[y + 1] & 1 << (x - 1) == 0)
                            }
                        };

                    if shift {
                        x -= 1
                    }
                }
                Jet::Right => {
                    let shift = x + match rock {
                        Rock::HBar => 4,
                        Rock::Plus => 3,
                        Rock::RevL => 3,
                        Rock::VBar => 1,
                        Rock::Rect => 2,
                    } < WIDTH
                        && match rock {
                            Rock::HBar => y >= len || grid[y] & 1 << (x + 4) == 0,
                            Rock::Plus => {
                                (y >= len || grid[y] & 1 << (x + 2) == 0)
                                    && (y + 1 >= len || grid[y + 1] & 1 << (x + 3) == 0)
                                    && (y + 2 >= len || grid[y + 2] & 1 << (x + 2) == 0)
                            }
                            Rock::RevL => {
                                (y >= len || grid[y] & 1 << (x + 3) == 0)
                                    && (y + 1 >= len || grid[y + 1] & 1 << (x + 3) == 0)
                                    && (y + 2 >= len || grid[y + 2] & 1 << (x + 3) == 0)
                            }
                            Rock::VBar => {
                                (y >= len || grid[y] & 1 << (x + 1) == 0)
                                    && (y + 1 >= len || grid[y + 1] & 1 << (x + 1) == 0)
                                    && (y + 2 >= len || grid[y + 2] & 1 << (x + 1) == 0)
                                    && (y + 3 >= len || grid[y + 3] & 1 << (x + 1) == 0)
                            }
                            Rock::Rect => {
                                (y >= len || grid[y] & 1 << (x + 2) == 0)
                                    && (y + 1 >= len || grid[y + 1] & 1 << (x + 2) == 0)
                            }
                        };

                    if shift {
                        x += 1
                    }
                }
            };
            j += 1;

            // check bounds
            let fall = y > 0
                && match rock {
                    Rock::HBar => y > len || 0b1111 << x & grid[y - 1] == 0,
                    Rock::Plus => {
                        (y > len || 0b010 << x & grid[y - 1] == 0)
                            && (y >= len || 0b111 << x & grid[y] == 0)
                    }
                    Rock::RevL => y > len || 0b0111 << x & grid[y - 1] == 0,
                    Rock::VBar => y > len || 0b0001 << x & grid[y - 1] == 0,
                    Rock::Rect => y > len || 0b0011 << x & grid[y - 1] == 0,
                };

            // fall or rest
            if fall {
                y -= 1;
            } else {
                let new_len = y + match rock {
                    Rock::HBar => 1,
                    Rock::Plus => 3,
                    Rock::RevL => 3,
                    Rock::VBar => 4,
                    Rock::Rect => 2,
                };
                if new_len > len {
                    grid.resize(new_len, 0);
                }

                match rock {
                    Rock::HBar => {
                        grid[y] |= 0b1111 << x;
                    }
                    Rock::Plus => {
                        grid[y] |= 0b010 << x;
                        grid[y + 1] |= 0b111 << x;
                        grid[y + 2] |= 0b010 << x;
                    }
                    Rock::RevL => {
                        grid[y] |= 0b111 << x;
                        grid[y + 1] |= 0b100 << x;
                        grid[y + 2] |= 0b100 << x;
                    }
                    Rock::VBar => {
                        grid[y] |= 1 << x;
                        grid[y + 1] |= 1 << x;
                        grid[y + 2] |= 1 << x;
                        grid[y + 3] |= 1 << x;
                    }
                    Rock::Rect => {
                        grid[y] |= 0b11 << x;
                        grid[y + 1] |= 0b11 << x;
                    }
                };
                break;
            }
        }
        lens.push(grid.len());
        i += 1;
    }
    (grid, lens)
}

fn render(grid: &[u8]) -> String {
    let mut str = grid.iter().rev().fold(String::new(), |mut a, row| {
        a.push_str(
            &("|".to_string()
                + &((0..WIDTH).map(|x| if *row & (1 << x) != 0 { '#' } else { '.' }).join("")
                    + "|\n")),
        );
        a
    });
    str.push_str("+-------+\n");
    str
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::{input, test};

    test!(part1, "sample", 3068);
    test!(part1, "puzzle", 3161);
    test!(part2, "sample", 1514285714288);
    test!(part2, "puzzle", 1575931232076);

    test!(sim5k_sample(simulate(&input!("sample").into(), 5000).0.len(), 7577));
    test!(sim5k_puzzle(simulate(&input!("puzzle").into(), 5000).0.len(), 7879));

    test!(sim10_sample(
        render(&simulate(&input!("sample").into(), 10).0).lines().collect_vec(),
        vec![
            "|....#..|",
            "|....#..|",
            "|....##.|",
            "|##..##.|",
            "|######.|",
            "|.###...|",
            "|..#....|",
            "|.####..|",
            "|....##.|",
            "|....##.|",
            "|....#..|",
            "|..#.#..|",
            "|..#.#..|",
            "|#####..|",
            "|..###..|",
            "|...#...|",
            "|..####.|",
            "+-------+",
        ]
    ));

    test!(sim10_puzzle(
        render(&simulate(&input!("puzzle").into(), 10).0).lines().collect_vec(),
        vec![
            "|..##...|",
            "|..##...|",
            "|..#..#.|",
            "|..#..#.|",
            "|.#####.|",
            "|.##..#.|",
            "|.##.###|",
            "|.##..#.|",
            "|#######|",
            "|.#..##.|",
            "|###.##.|",
            "|.#.####|",
            "+-------+",
        ]
    ));
}
