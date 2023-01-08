use sscanf::sscanf;
use utils::prelude::*;

pub struct Input {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<Move>,
}

#[derive(sscanf::FromScanf)]
#[sscanf("move {count} from {from} to {to}")]
struct Move {
    count: u32,
    from: usize,
    to: usize,
}

impl From<String> for Input {
    fn from(input: String) -> Self {
        let mut parts = input.split("\n\n");
        let mut spec = parts.next().unwrap().lines().collect::<Vec<_>>();
        spec.pop(); // ignore column index assuming ascending order

        let mut stacks = Vec::new();
        stacks.resize(spec[0].len() / 3, VecDeque::new());

        for row in spec {
            let row = row.chars();
            for (i, c) in row.enumerate() {
                if i % 4 == 1 && c != ' ' {
                    stacks[(i - 1) / 4].push_back(c);
                }
            }
        }

        let mut moves: Vec<Move> = Vec::new();
        for l in parts.next().unwrap().lines() {
            moves.push(sscanf!(l, "{Move}").unwrap());
        }

        Input { stacks, moves }
    }
}

pub fn part1(input: Input) -> String {
    let mut stacks = input.stacks;
    for m in input.moves {
        for _ in 0..m.count {
            let c = stacks[m.from - 1].pop_front().unwrap();
            stacks[m.to - 1].push_front(c);
        }
    }
    top_crates(&stacks)
}

pub fn part2(input: Input) -> String {
    let mut stacks = input.stacks;
    for m in input.moves {
        let mut stack = stacks[m.from - 1].clone();
        stacks[m.from - 1] = stack.split_off(m.count as usize);
        stack.append(&mut stacks[m.to - 1]);
        stacks[m.to - 1] = stack;
    }
    top_crates(&stacks)
}

fn top_crates(stacks: &Vec<VecDeque<char>>) -> String {
    let mut top_crates = Vec::new();
    for c in stacks {
        if let Some(c) = c.get(0) {
            top_crates.push(c.to_string())
        }
    }
    top_crates.join("")
}

tests! {
    (part1, "sample", "CMZ")
    (part1, "puzzle", "FCVRLMVQP")
    (part2, "sample", "MCD")
    (part2, "puzzle", "RWLWGJGFD")
}
