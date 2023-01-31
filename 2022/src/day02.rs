#[derive(Clone)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl From<u8> for Play {
    fn from(play: u8) -> Self {
        match play {
            b'A' => Play::Rock,
            b'B' => Play::Paper,
            b'C' => Play::Scissors,
            _ => unreachable!(),
        }
    }
}

pub fn part1(i: String) -> u32 {
    let mut score = 0;
    for l in i.lines() {
        score += play(l.as_bytes()[0].into(), (l.as_bytes()[2] - b'X' + b'A').into());
    }
    score
}

pub fn part2(input: String) -> u32 {
    let mut score = 0;

    for l in input.lines() {
        let p1 = l.as_bytes()[0].into();
        let p2 = match l.as_bytes()[2].into() {
            // Lose
            'X' => match p1 {
                Play::Rock => Play::Scissors,
                Play::Paper => Play::Rock,
                Play::Scissors => Play::Paper,
            },
            // Draw
            'Y' => p1.clone(),
            // Win
            'Z' => match p1 {
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissors,
                Play::Scissors => Play::Rock,
            },
            _ => unreachable!(),
        };

        score += play(p1, p2);
    }
    score
}

fn play(p1: Play, p2: Play) -> u32 {
    let outcome = match (&p1, &p2) {
        // Win
        (Play::Rock, Play::Paper) => 2,
        (Play::Paper, Play::Scissors) => 2,
        (Play::Scissors, Play::Rock) => 2,
        // Lose
        (Play::Rock, Play::Scissors) => 0,
        (Play::Paper, Play::Rock) => 0,
        (Play::Scissors, Play::Paper) => 0,
        _ => 1, // Draw
    };
    (p2 as u32 + 1) + (outcome * 3)
}

utils::tests! {
    (part1, "sample", 15)
    (part1, "puzzle", 9177)
    (part2, "sample", 12)
    (part2, "puzzle", 12111)
}
