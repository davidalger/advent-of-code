pub struct Move {
    direction: String,
    units: u32,
}

utils::parse!(|i| -> Vec<Move> {
    i.lines()
        .map(|l| {
            let (direction, units) = l.split_once(' ').unwrap();
            Move { direction: direction.to_string(), units: units.parse().unwrap() }
        })
        .collect()
} as Moves);

pub fn part1(moves: Moves) -> u32 {
    let mut x = 0;
    let mut z = 0;
    for m in moves.iter() {
        match m.direction.as_str() {
            "forward" => x += m.units,
            "down" => z += m.units,
            "up" => z -= m.units,
            _ => unimplemented!(),
        }
    }
    x * z
}

pub fn part2(moves: Moves) -> u32 {
    let mut aim = 0;
    let mut x = 0;
    let mut z = 0;
    for m in moves.iter() {
        match m.direction.as_str() {
            "forward" => {
                x += m.units;
                z += aim * m.units;
            }
            "down" => aim += m.units,
            "up" => aim -= m.units,
            _ => unimplemented!(),
        }
    }
    x * z
}

utils::tests! {
    (part1, "sample", 150)
    (part1, "puzzle", 1804520)
    (part2, "sample", 900)
    (part2, "puzzle", 1971095320)
}
