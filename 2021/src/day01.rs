utils::parse!(|i| -> Vec<u32> { i.lines().map(|l| l.parse().unwrap()).collect() });

pub fn part1(input: Input) -> u32 {
    let mut score = 0;
    for (k, v) in input.iter().enumerate() {
        if k > 0 && v > &input[k - 1] {
            score += 1;
        }
    }
    score
}

pub fn part2(input: Input) -> u32 {
    let mut score = 0;
    for k in 1..input.len() - 2 {
        if input[k..k + 3].iter().copied().sum::<u32>() > input[k - 1..k + 2].iter().copied().sum()
        {
            score += 1;
        }
    }
    score
}

utils::tests! {
    (part1, "sample", 7)
    (part1, "puzzle", 1301)
    (part2, "sample", 5)
    (part2, "puzzle", 1346)
}
