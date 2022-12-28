use sscanf::sscanf;
use utils::prelude::*;

pub struct Input {
    part1_row: i32,
    part2_bound: i32,
    data: Vec<Data>,
}

struct Data {
    sensor: Pos,
    beacon: Pos,
    range: u32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pos(i32, i32);

impl From<String> for Input {
    fn from(input: String) -> Self {
        let part1_row = input.lines().find_map(|l| sscanf!(l, "part1_row={i32}").ok()).unwrap();
        let part2_bound = input.lines().find_map(|l| sscanf!(l, "part2_bound={i32}").ok()).unwrap();

        let data = input
            .lines()
            .filter_map(|l| {
                if let Ok((sx, sy, bx, by)) =
                    sscanf!(l, "Sensor at x={i32}, y={i32}: closest beacon is at x={i32}, y={i32}")
                {
                    let sensor = Pos(sx, sy);
                    let beacon = Pos(bx, by);
                    Some(Data { sensor, beacon, range: distance(sensor, beacon) })
                } else {
                    None
                }
            })
            .collect();

        Self { part1_row, part2_bound, data }
    }
}

pub fn part1(input: Input) -> usize {
    let min_x = input.data.iter().map(|data| data.sensor.0 - data.range as i32).min().unwrap();
    let max_x = input.data.iter().map(|data| data.sensor.0 + data.range as i32).max().unwrap();

    let mut covered = vec![false; min_x.abs_diff(max_x) as usize];

    let y = input.part1_row;
    for x in min_x..max_x {
        for data in &input.data {
            let beacon = Pos(x, y);
            if distance(data.sensor, beacon) <= data.range {
                covered[(x + min_x.abs()) as usize] = true;
                continue;
            }
        }
    }

    covered.iter().filter(|v| **v).count()
        - input
            .data
            .iter()
            .filter(|data| data.beacon.1 == y)
            .map(|data| data.beacon)
            .unique()
            .count()
}

pub fn part2(input: Input) -> u64 {
    for y in 0..=input.part2_bound {
        let mut x = 0;
        'x: while x <= input.part2_bound {
            let beacon = Pos(x, y);
            for data in &input.data {
                if distance(data.sensor, beacon) <= data.range {
                    x = data.sensor.0 + (data.range - data.sensor.1.abs_diff(y)) as i32 + 1;
                    continue 'x;
                }
            }
            return x as u64 * 4000000 + y as u64;
        }
    }
    0
}

fn distance(a: Pos, b: Pos) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

tests!(
    part1_sample(part1(input!("sample")), 26),
    part1_puzzle(part1(input!("puzzle")), 5511201),
    part2_sample(part2(input!("sample")), 56000011),
    part2_puzzle(part2(input!("puzzle")), 11318723411840),
);
