use std::collections::HashSet;

pub fn part1(input: String) -> u32 {
    input
        .lines()
        .filter_map(|l| {
            let (_, w, n) = sscanf::sscanf!(l, "{str}:{str}|{str}").unwrap();

            let w: HashSet<_> = w.split_whitespace().collect();
            let n: HashSet<_> = n.split_whitespace().collect();

            match w.intersection(&n).count() {
                0 => None, // cards without any winners are worth zero
                count => Some(2_u32.pow(count as u32 - 1)),
            }
        })
        .sum()
}

pub fn part2(input: String) -> u32 {
    let cards: Vec<_> = input
        .lines()
        .map(|l| {
            let (_, w, n) = sscanf::sscanf!(l, "{str}:{str}|{str}").unwrap();

            let w: HashSet<_> = w.split_whitespace().collect();
            let n: HashSet<_> = n.split_whitespace().collect();

            w.intersection(&n).count()
        })
        .collect();

    let mut copies = vec![1; cards.len()];
    for (id, matches) in cards.iter().enumerate() {
        for x in 1..=*matches {
            copies[id + x] += copies[id];
        }
    }

    copies.iter().sum()
}

utils::tests! {
    (part1, "sample", 13)
    (part1, "puzzle", 21558)
    (part2, "sample", 30)
    (part2, "puzzle", 10425665)
}
