use std::collections::HashSet;

utils::parse!(|i| -> Vec<[HashSet<u32>; 2]> {
    i.lines()
        .map(|l| {
            l.splitn(2, ',')
                .map(|p| {
                    let p: Vec<u32> = p.splitn(2, '-').map(|p| p.parse().unwrap()).collect();
                    (p[0]..p[1] + 1).collect()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect()
});

pub fn part1(input: Input) -> u32 {
    let mut score = 0;
    for p in input.iter() {
        let i: HashSet<u32> = p[0].intersection(&p[1]).copied().collect();
        if i == p[0] || i == p[1] {
            score += 1;
        }
    }
    score
}

pub fn part2(input: Input) -> u32 {
    let mut score = 0;
    for p in input.iter() {
        if !p[0].intersection(&p[1]).collect::<HashSet<_>>().is_empty() {
            score += 1;
        }
    }
    score
}

utils::tests! {
    (part1, "puzzle", 562)
    (part1, "sample", 2)
    (part2, "sample", 4)
    (part2, "puzzle", 924)
}
