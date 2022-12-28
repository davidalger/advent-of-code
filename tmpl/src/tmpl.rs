use utils::prelude::*;

parse!(|i| -> Vec<u32> { i.lines().map(|l| l.parse().unwrap()).collect() });

pub fn part1(input: Input) -> u32 {
    input.iter().sum()
}

pub fn part2(input: Input) -> u32 {
    input.iter().product()
}

tests!(
    part1_sample(part1(input!("sample")), 97),
    part1_puzzle(part1(input!("puzzle")), 128),
    part2_sample(part2(input!("sample")), 18200),
    part2_puzzle(part2(input!("puzzle")), 29250),
);
