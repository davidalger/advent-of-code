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

tests! {
    (part1, "sample", 7)
    (part1, "puzzle", 1912)
    (part2, "sample", 19)
    (part2, "puzzle", 2122)
}
