use utils::prelude::*;

parse!(|i| -> Vec<u32> {
    i.split("\n\n")
        .map(|chunk| chunk.lines().fold(0, |a, b| a + b.parse::<u32>().unwrap()))
        .sorted()
        .rev()
        .collect()
});

pub fn part1(input: Input) -> u32 {
    input[0]
}

pub fn part2(input: Input) -> u32 {
    input[0..3].iter().sum()
}

tests!(
    part1_sample(part1(input!("sample")), 24000),
    part1_puzzle(part1(input!("puzzle")), 67027),
    part2_sample(part2(input!("sample")), 45000),
    part2_puzzle(part2(input!("puzzle")), 197291),
);
