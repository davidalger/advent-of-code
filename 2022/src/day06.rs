use utils::prelude::*;

pub fn part1(input: String) -> usize {
    marker(&input, 4)
}

pub fn part2(input: String) -> usize {
    marker(&input, 14)
}

fn marker(input: &str, len: usize) -> usize {
    for i in len..input.len() {
        if input[i - len..i].chars().collect::<HashSet<_>>().len() == len {
            return i;
        }
    }
    0
}

tests!(
    part1_sample(part1(input!("sample")), 7),
    part1_puzzle(part1(input!("puzzle")), 1912),
    part2_sample(part2(input!("sample")), 19),
    part2_puzzle(part2(input!("puzzle")), 2122),
);
