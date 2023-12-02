pub struct Play {
    r: u32,
    g: u32,
    b: u32,
}

utils::parse!(|i| -> Vec<(u32, Vec<Play>)> {
    i.lines()
        .map(|l| {
            let (_, id, l) = sscanf::sscanf!(l, "{str}{u32}:{str}").unwrap();
            (
                id,
                l.split(";")
                    .map(|p| {
                        let mut play = Play { r: 0, g: 0, b: 0 };
                        for t in p.split(",") {
                            let (n, c) = sscanf::sscanf!(t.trim(), "{u32} {str}").unwrap();
                            match c {
                                "red" => play.r = n,
                                "green" => play.g = n,
                                "blue" => play.b = n,
                                _ => unimplemented!(),
                            };
                        }
                        play
                    })
                    .collect(),
            )
        })
        .collect()
} as Games);

pub fn part1(games: Games) -> u32 {
    games
        .iter()
        .filter_map(|(id, plays)| {
            for p in plays {
                if p.r > 12 || p.g > 13 || p.b > 14 {
                    return None;
                }
            }
            Some(id)
        })
        .sum()
}

pub fn part2(games: Games) -> u32 {
    games
        .iter()
        .map(|(_, plays)| {
            let (mut r, mut g, mut b) = (0, 0, 0);
            for p in plays {
                if p.r > r {
                    r = p.r;
                }
                if p.g > g {
                    g = p.g;
                }
                if p.b > b {
                    b = p.b;
                }
            }
            r * g * b
        })
        .sum()
}

utils::tests! {
    (part1, "sample", 8)
    (part1, "puzzle", 2617)
    (part2, "sample", 2286)
    (part2, "puzzle", 59795)
}
