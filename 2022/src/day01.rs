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

tests! {
    (part1, "sample", 24000)
    (part1, "puzzle", 67027)
    (part2, "sample", 45000)
    (part2, "puzzle", 197291)
}
