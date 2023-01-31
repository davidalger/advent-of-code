pub enum Instruction {
    Noop,
    Addx(i32),
}

struct Result {
    signal: i32,
    crt: String,
}

utils::parse!(|i| -> Vec<Instruction> {
    i.lines()
        .map(|l| match l.split_once(' ') {
            Some(l) => Instruction::Addx(l.1.parse().unwrap()),
            _ => Instruction::Noop,
        })
        .collect()
} as Instructions);

pub fn part1(ins: Instructions) -> i32 {
    process(220, ins).signal
}

pub fn part2(ins: Instructions) -> String {
    process(240, ins).crt
}

fn process(cycles: i32, ins: Instructions) -> Result {
    let mut s = 0;
    let mut x = 1;
    let mut crt = String::new();
    let mut ins = ins.iter();
    let mut cycles = 1..=cycles;
    while let Some(c) = cycles.next() {
        s += signal(c, x);
        crt.push_str(&draw(c, x));

        match ins.next().unwrap() {
            Instruction::Noop => {}
            Instruction::Addx(value) => {
                if let Some(c) = cycles.next() {
                    s += signal(c, x);
                    crt.push_str(&draw(c, x));
                    x += value;
                }
            }
        }
    }
    Result { signal: s, crt: crt.trim().to_string() }
}

fn signal(c: i32, x: i32) -> i32 {
    if (c - 20) % 40 == 0 {
        c * x
    } else {
        0
    }
}

fn draw(c: i32, x: i32) -> String {
    let mut res = String::new();
    if (x - 1..=x + 1).contains(&((c - 1) % 40)) {
        res.push('#');
    } else {
        res.push('.');
    }
    if c % 40 == 0 {
        res.push('\n');
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::{test, input};

    test!(part1, "sample", 13140);
    test!(part1, "puzzle", 14560);

    test!(part2_sample(
        part2(input!("sample")).lines().collect::<Vec<_>>(),
        vec![
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
        ]
    ));

    test!(part2_puzzle(
        part2(input!("puzzle")).lines().collect::<Vec<_>>(),
        vec![
            "####.#..#.###..#..#.####.###..#..#.####.",
            "#....#.#..#..#.#..#.#....#..#.#..#....#.",
            "###..##...#..#.####.###..#..#.#..#...#..",
            "#....#.#..###..#..#.#....###..#..#..#...",
            "#....#.#..#.#..#..#.#....#....#..#.#....",
            "####.#..#.#..#.#..#.####.#.....##..####.",
        ]
    ));
}
