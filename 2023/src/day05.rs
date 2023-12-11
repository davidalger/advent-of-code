use itertools::*;

pub struct Input {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

type Map = Vec<(u64, u64, u64)>;

impl From<String> for Input {
    fn from(input: String) -> Self {
        let mut parts = input.split("\n\n");
        let mut input = Input {
            seeds: sscanf::sscanf!(parts.next().unwrap(), "seeds:{str}")
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
            maps: Vec::new(),
        };

        for part in parts {
            let mut map: Vec<(u64, u64, u64)> = vec![];
            let mut lines = part.lines();
            lines.next(); // advance iterator ignoring name of map

            for line in lines {
                let t: Vec<u64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
                map.push((t[0], t[1], t[2])); // assume 3 values in each line
            }

            input.maps.push(map);
        }

        input
    }
}

pub fn part1(input: Input) -> u64 {
    input.seeds.iter().map(|s| locate(*s, &input.maps)).min().unwrap()
}

pub fn part2(input: Input) -> u64 {
    let mut seeds = vec![];
    for (&s, &l) in input.seeds.iter().tuples() {
        let mut r = (s..s + l).collect();
        seeds.append(&mut r);
    }

    seeds.iter().map(|s| locate(*s, &input.maps)).min().unwrap()
}

fn locate(seed: u64, maps: &[Map]) -> u64 {
    let mut l = seed;
    for map in maps.iter() {
        for (d_start, s_start, length) in map {
            if l >= *s_start && l < s_start + length {
                l = d_start + (l - s_start);
                break;
            }
        }
    }
    l
}

utils::tests! {
    (part1, "sample", 35)
    (part1, "puzzle", 484023871)
    (part2, "sample", 46)
    (part2, "puzzle", 46294175)
}
