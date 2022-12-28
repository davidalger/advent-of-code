use sscanf::sscanf;
use utils::prelude::*;

pub struct Monkey {
    items: VecDeque<u128>,
    operation: Operation,
    test: Test,
}

enum Operation {
    Pow(u32),
    Add(u128),
    Mul(u128),
}

pub struct Test {
    value: u128,
    monkey: (usize, usize),
}

parse!(|i| -> Vec<Monkey> {
    i.split("\n\n")
        .map(|chunk| {
            let lines = chunk.lines().map(|l| l.trim()).collect_vec();
            Monkey {
                items: sscanf!(lines[1], "Starting items: {String}")
                    .unwrap()
                    .split(',')
                    .map(|v| v.trim().parse().unwrap())
                    .collect(),
                operation: match sscanf!(lines[2], "Operation: new = {String}")
                    .unwrap()
                    .as_str()
                    .splitn(3, ' ')
                    .tuples::<(_, _, _)>()
                    .next()
                    .unwrap()
                {
                    ("old", "*", "old") => Operation::Pow(2),
                    ("old", "+", "old") => Operation::Mul(2),
                    ("old", "*", v) => Operation::Mul(v.parse().unwrap()),
                    ("old", "+", v) => Operation::Add(v.parse().unwrap()),
                    _ => unimplemented!(),
                },
                test: Test {
                    value: sscanf!(lines[3], "Test: divisible by {u128}").unwrap(),
                    monkey: (
                        sscanf!(lines[4], "If true: throw to monkey {usize}").unwrap(),
                        sscanf!(lines[5], "If false: throw to monkey {usize}").unwrap(),
                    ),
                },
            }
        })
        .collect()
} as Monkeys);

pub fn part1(monkeys: Monkeys) -> u128 {
    mitm(monkeys, 20, Some(3))
}

pub fn part2(monkeys: Monkeys) -> u128 {
    mitm(monkeys, 10000, None)
}

fn mitm(mut monkeys: Monkeys, rounds: u32, divisor: Option<u128>) -> u128 {
    let reducer = monkeys.iter().fold(1, |a, b| a * b.test.value);
    let mut counter = vec![0; monkeys.len()];
    for _ in 0..rounds {
        for (index, counter) in counter.iter_mut().enumerate() {
            while let Some(mut item) = monkeys[index].items.pop_front() {
                let monkey = &monkeys[index];

                item = match monkey.operation {
                    Operation::Pow(v) => item.pow(v),
                    Operation::Add(v) => item.add(v),
                    Operation::Mul(v) => item.mul(v),
                };

                if let Some(divisor) = divisor {
                    item /= divisor;
                }
                item %= reducer;

                let target = if item % monkey.test.value == 0 {
                    monkey.test.monkey.0
                } else {
                    monkey.test.monkey.1
                };
                monkeys[target].items.push_back(item);
                *counter += 1;
            }
        }
    }
    counter.sort();
    counter.pop().unwrap() * counter.pop().unwrap()
}

tests!(
    part1_sample(part1(input!("sample")), 10605),
    part1_puzzle(part1(input!("puzzle")), 99840),
    part2_sample(part2(input!("sample")), 2713310158),
    part2_puzzle(part2(input!("puzzle")), 20683044837),
);
