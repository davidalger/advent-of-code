pub fn part1(input: String) -> u64 {
    let line = |l: usize| -> Vec<u64> {
        sscanf::sscanf!(input.lines().nth(l).unwrap(), "{str}:{str}")
            .unwrap()
            .1
            .to_string()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };

    line(0)
        .iter()
        .zip(line(1).iter())
        .map(|(&time, &dist)| winning_combinations(time, dist))
        .product()
}

pub fn part2(input: String) -> u64 {
    let line = |l: usize| -> u64 {
        sscanf::sscanf!(input.lines().nth(l).unwrap(), "{str}:{str}")
            .unwrap()
            .1
            .to_string()
            .bytes()
            .filter_map(|x| if x.is_ascii_digit() { Some((x - b'0') as u64) } else { None })
            .reduce(|a, b| a * 10 + b)
            .unwrap()
    };

    winning_combinations(line(0), line(1))
}

fn winning_combinations(time: u64, dist: u64) -> u64 {
    let mut wins = 0;
    for speed in 1..time {
        if speed * (time - speed) > dist {
            wins += 1;
        }
    }
    wins
}

utils::tests! {
    (part1, "sample", 288)
    (part1, "puzzle", 625968)
    (part2, "sample", 71503)
    (part2, "puzzle", 43663323)
}
