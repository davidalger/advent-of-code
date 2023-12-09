utils::parse!(|i| -> Vec<Vec<i32>> {
    i.lines().map(|l| l.split_whitespace().map(|x| x.parse().unwrap()).collect()).collect()
} as Input);

pub fn part1(input: Input) -> i32 {
    input.iter().map(extrapolate).map(|(_, x)| x).sum()
}

pub fn part2(input: Input) -> i32 {
    input.iter().map(extrapolate).map(|(x, _)| x).sum()
}

fn extrapolate(seq: &Vec<i32>) -> (i32, i32) {
    let mut diff = vec![];
    for i in 1..seq.len() {
        diff.push(seq[i] - seq[i - 1]);
    }
    if diff.iter().map(|&x| x == 0).reduce(|l, r| l == r && l == true).unwrap() == true {
        (seq[0], seq[seq.len() - 1])
    } else {
        let (left, right) = extrapolate(&diff);
        (seq[0] - left, right + seq[seq.len() - 1])
    }
}

utils::tests! {
    (part1, "sample", 114)
    (part1, "puzzle", 1938800261)
    (part2, "sample", 2)
    (part2, "puzzle", 1112)
}
