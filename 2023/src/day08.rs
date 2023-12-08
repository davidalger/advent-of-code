use std::collections::HashMap;

struct Input<'a> {
    ins: Vec<u8>,
    net: HashMap<&'a str, (&'a str, &'a str)>,
}

pub fn part1(input: String) -> u64 {
    solve(&parse(&input), "AAA", |n| n == "ZZZ")
}

pub fn part2(input: String) -> u64 {
    let input = parse(&input);

    input
        .net
        .iter()
        .filter_map(|(k, _)| {
            if k.ends_with("A") {
                Some(solve(&input, k, |next| next.ends_with("Z")))
            } else {
                None
            }
        })
        .reduce(|a, b| num::Integer::lcm(&a, &b))
        .unwrap()
}

fn parse<'a>(input: &'a String) -> Input<'a> {
    let (ins, net) = input.split_once("\n\n").unwrap();
    let ins = ins.bytes().collect();
    let net = net
        .lines()
        .map(|node| {
            let (id, l, r) = sscanf::sscanf!(node, "{str} = ({str}, {str})").unwrap();
            (id, (l, r))
        })
        .collect();

    Input { ins, net }
}

fn solve<F>(input: &Input, start: &str, condition: F) -> u64
where
    F: Fn(&str) -> bool,
{
    let (mut l, mut r) = input.net.get(start).unwrap();
    let mut steps = 0;
    loop {
        let step: u8 = input.ins[(steps % input.ins.len() as u64) as usize];
        steps += 1;
        let next = match step {
            b'L' => l,
            b'R' => r,
            _ => unimplemented!(),
        };
        if condition(&next) {
            return steps;
        }
        (l, r) = *input.net.get(&next).unwrap();
    }
}

utils::tests! {
    (part1, "sample", 6)
    (part1, "puzzle", 20221)
    (part2, "sample2", 6)
    (part2, "puzzle", 14616363770447)
}
