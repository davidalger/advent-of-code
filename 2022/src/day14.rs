use gif::{Encoder, Frame, Repeat};
use itertools::Itertools;
use std::borrow::Cow;
use std::fs::File;

#[derive(Clone)]
pub struct Input {
    grid: Grid,
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
                    .collect_vec()
            })
            .collect_vec();

        let mut grid = vec![vec![Material::Air; 1000]; 1000];
        for path in paths.iter() {
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

        Self { grid, start: Pos(500, 0) }
    }
}

pub fn part1(mut input: Input) -> usize {
    let (frames, size) =
        fill_sand(&mut input.grid, input.start, |grid, p| p.1 + 1 >= grid.len(), 7);

    if cfg!(not(test)) && std::env::var_os("CARGO_BENCH").is_none() {
        print_grid(frames.last().unwrap());
        encode_gif("output/day14-part1.gif", &frames);
    }
    size
}

pub fn part2(mut input: Input) -> usize {
    input.grid.push(vec![Material::Air; 1000]);
    input.grid.push(vec![Material::Rock; 1000]);

    let (frames, size) = fill_sand(
        &mut input.grid,
        input.start,
        |grid, p| *p == input.start && grid[p.1][p.0] == Material::Sand,
        250,
    );

    if cfg!(not(test)) && std::env::var_os("CARGO_BENCH").is_none() {
        encode_gif("output/day14-part2.gif", &frames);
    }
    size
}

fn fill_sand<S>(grid: &mut Grid, start: Pos, stop: S, skip_by: usize) -> (Vec<Grid>, usize)
where
    S: Fn(&Grid, &Pos) -> bool,
{
    let mut frames = Vec::new();
    let mut size = 0;
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
        if size % skip_by == 0 {
            frames.push(grid.clone());
        }
        size += 1;
    }
    frames.push(grid.clone());

    (frames, size)
}

fn calc_crop(grid: &Grid) -> (usize, usize, usize) {
    let width = grid.iter().map(|r| r.len()).fold(0, |a, b| a.max(b));

    let mut top = 0;
    for row in grid.iter().enumerate().map(|(row, _)| row).collect_vec() {
        if grid[row].iter().unique().count() == 1 {
            continue;
        } else {
            top = row;
            break;
        }
    }
    let left = grid
        .iter()
        .map(|r| {
            if r.iter().unique().count() == 1 {
                r.len()
            } else {
                r.iter().find_position(|m| **m != Material::Air).unwrap().0
            }
        })
        .fold(width, |a, b| a.min(b));

    let right = grid
        .iter()
        .map(|r| {
            if r.iter().unique().count() == 1 {
                0
            } else {
                r.len() - r.iter().rev().find_position(|m| **m != Material::Air).unwrap().0
            }
        })
        .fold(0, |a, b| a.max(b));
    (top, left, right)
}

fn crop_grid(grid: &Grid, crop: (usize, usize, usize)) -> Grid {
    let (top, left, right) = crop;
    let mut crop = vec![Vec::with_capacity(grid[0].len()); grid.len() - top];
    for (row, v) in grid.iter().skip(top).enumerate() {
        crop[row].extend_from_slice(&v[left..right]);
    }
    crop
}

fn print_grid(grid: &Grid) {
    let mut render = String::new();
    for row in crop_grid(grid, calc_crop(grid)) {
        for m in row {
            render.push(match m {
                Material::Rock => '#',
                Material::Sand => 'o',
                Material::Air => '.',
            });
        }
        render.push('\n');
    }
    println!("{}", render);
}

fn encode_gif(path: &str, frames: &[Grid]) {
    let crop = calc_crop(frames.last().unwrap());
    let frames = frames.iter().map(|frame| crop_grid(frame, crop)).collect_vec();

    let scale = 4;
    let width = frames[0][0].len() as u16 * scale;
    let height = frames[0].len() as u16 * scale;
    let palette = &[0xFF, 0xFF, 0x00, 0x00, 0x99, 0x00];

    let mut image = File::create(path).unwrap();
    let mut encoder = Encoder::new(&mut image, width, height, palette).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    for frame in frames {
        let buffer = frame_buffer(&frame, width, height, scale);
        let frame = Frame {
            width,
            height,
            buffer: Cow::Borrowed(&*buffer),
            transparent: Some(Material::Air as u8),
            ..Default::default()
        };
        encoder.write_frame(&frame).unwrap();
    }
}

fn frame_buffer(grid: &Grid, width: u16, height: u16, scale: u16) -> Vec<u8> {
    let mut data = Vec::with_capacity((width as u64 * height as u64 * 4 * scale as u64) as usize);

    for row in grid {
        for _ in 0..scale {
            for m in row {
                for _ in 0..scale {
                    data.push(*m as u8);
                }
            }
        }
    }
    data
}

utils::tests! {
    (part1, "sample", 24)
    (part1, "puzzle", 799)
    (part2, "sample", 93)
    (part2, "puzzle", 29076)
}
