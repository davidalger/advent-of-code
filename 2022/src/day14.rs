use utils::prelude::*;

#[derive(Clone)]
pub struct Input {
    paths: Vec<Vec<Pos>>,
    start: Pos,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pos(usize, usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Material {
    Rock,
    Sand,
    Air,
}

type Grid = Vec<Vec<Material>>;

impl From<String> for Input {
    fn from(input: String) -> Self {
        let paths = input
            .lines()
            .map(|l| {
                l.split(" -> ")
                    .map(|p| {
                        let (x, y) = p.split_once(',').unwrap();
                        Pos(x.parse().unwrap(), y.parse().unwrap())
                    })
                    .collect()
            })
            .collect();
        Self { paths, start: Pos(500, 0) }
    }
}

pub fn part1(input: Input) -> u32 {
    let mut grid = grid(&input);
    let sand = pour_sand(&mut grid, input.start, |grid, p| p.1 + 1 >= grid.len());
    print_grid(&grid);
    sand
}

pub fn part2(input: Input) -> u32 {
    let mut grid = grid(&input);

    grid.push(vec![Material::Air; 1000]);
    grid.push(vec![Material::Rock; 1000]);

    pour_sand(&mut grid, input.start, |grid, p| {
        *p == input.start && grid[p.1][p.0] == Material::Sand
    })
}

fn grid(input: &Input) -> Grid {
    let mut grid = vec![vec![Material::Air; 1000]; 1000];

    for path in input.paths.iter() {
        let mut i = path.iter().peekable();
        while let Some(a) = i.next() {
            if let Some(b) = i.peek() {
                let range_x = if a.0 < b.0 { a.0..=b.0 } else { b.0..=a.0 };
                for x in range_x {
                    let range_y = if a.1 < b.1 { a.1..=b.1 } else { b.1..=a.1 };
                    for y in range_y {
                        grid[y][x] = Material::Rock;
                    }
                }
            }
        }
    }

    let mut len = grid.len();
    for (row, _) in grid.iter().enumerate() {
        if grid[grid.len() - row - 1].iter().unique().count() == 1 {
            len = grid.len() - row - 1;
        } else {
            break;
        }
    }
    grid.truncate(len);
    grid
}

fn pour_sand<S>(grid: &mut Grid, start: Pos, stop: S) -> u32
where
    S: Fn(&Grid, &Pos) -> bool,
{
    let mut sand = 0;
    'outer: loop {
        let mut p = start;
        loop {
            if stop(grid, &p) {
                break 'outer;
            } else if grid[p.1 + 1][p.0] == Material::Air {
                p.1 += 1;
            } else if grid[p.1 + 1][p.0 - 1] == Material::Air {
                p.1 += 1;
                p.0 -= 1;
            } else if grid[p.1 + 1][p.0 + 1] == Material::Air {
                p.1 += 1;
                p.0 += 1;
            } else {
                grid[p.1][p.0] = Material::Sand;
                break;
            }
        }
        sand += 1;
    }
    sand
}

fn print_grid(grid: &Grid) {
    let width = grid.iter().map(|r| r.len()).fold(0, |a, b| a.max(b));
    let left = grid
        .iter()
        .map(|r| {
            if r.iter().unique().count() == 1 {
                r.len()
            } else {
                r.iter().find_position(|m| **m != Material::Air).unwrap().0
            }
        })
        .fold(width, |a, b| a.min(b))
        - 2;

    let right = grid
        .iter()
        .map(|r| {
            if r.iter().unique().count() == 1 {
                0
            } else {
                r.len() - r.iter().rev().find_position(|m| **m != Material::Air).unwrap().0
            }
        })
        .fold(0, |a, b| a.max(b))
        + 2;

    for (y, _) in grid.iter().enumerate() {
        for x in left..right {
            print!(
                "{}",
                match grid[y][x] {
                    Material::Rock => '#',
                    Material::Sand => 'o',
                    Material::Air => '.',
                }
            )
        }
        println!();
    }
    println!();
}

tests!(
    part1_sample(part1(input!("sample")), 24),
    part1_puzzle(part1(input!("puzzle")), 799),
    part2_sample(part2(input!("sample")), 93),
    part2_puzzle(part2(input!("puzzle")), 29076),
);
