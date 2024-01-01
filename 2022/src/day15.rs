use itertools::Itertools;
use sscanf::sscanf;

pub struct Data {
    sensor: Pos,
    beacon: Pos,
    range: u32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pos(i32, i32);

utils::parse!(|i| -> Vec<Data> {
    i.lines()
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
        .collect()
} as Input);

pub fn part1(input: Input) -> usize {
    let min_x = input.iter().map(|data| data.sensor.0 - data.range as i32).min().unwrap();
    let max_x = input.iter().map(|data| data.sensor.0 + data.range as i32).max().unwrap();

    let mut covered = vec![false; min_x.abs_diff(max_x) as usize];

    let y = match input.iter().map(|data| data.sensor.0.max(data.sensor.1)).max().unwrap() {
        x if x >= 1000000 => 2000000, // puzzle
        _ => 10,                      //sample
    };

    for x in min_x..max_x {
        for data in input.iter() {
            let beacon = Pos(x, y);
            if distance(data.sensor, beacon) <= data.range {
                covered[(x + min_x.abs()) as usize] = true;
                continue;
            }
        }
    }

    covered.iter().filter(|v| **v).count()
        - input.iter().filter(|data| data.beacon.1 == y).map(|data| data.beacon).unique().count()
}

pub fn part2(input: Input) -> u64 {
    let bound = match input.iter().map(|data| data.sensor.0.max(data.sensor.1)).max().unwrap() {
        x if x >= 1000000 => 4000000, // puzzle
        _ => 20,                      //sample
    };

    for y in 0..=bound {
        let mut x = 0;
        'x: while x <= bound {
            let beacon = Pos(x, y);
            for data in input.iter() {
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

utils::tests! {
    (part1, "sample", 26)
    (part1, "puzzle", 5511201)
    (part2, "sample", 56000011)
    (part2, "puzzle", 11318723411840)
}
