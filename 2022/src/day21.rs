use sscanf::sscanf;
use utils::prelude::*;

pub enum Val {
    Unknown,
    Int(u128),
    Exp(String, char, String),
}

parse!(|i| -> HashMap<String, Val> {
    i.lines()
        .map(|l| {
            let (a, b) = l.split_once(": ").unwrap();
            (
                a.to_string(),
                b.parse().map(Val::Int).unwrap_or_else(|_| {
                    let (l, o, r) = sscanf!(b, "{String} {char} {String}").unwrap();
                    Val::Exp(l, o, r)
                }),
            )
        })
        .collect()
} as Monkeys);

pub fn part1(monkeys: Monkeys) -> u128 {
    resolve("root", &monkeys).unwrap()
}

pub fn part2(mut monkeys: Monkeys) -> u128 {
    *monkeys.get_mut("humn").unwrap() = Val::Unknown;
    *monkeys.get_mut("root").unwrap() = match &monkeys["root"] {
        Val::Exp(l, _, r) => Val::Exp(l.to_owned(), '=', r.to_owned()),
        _ => unreachable!(),
    };

    let mut val = 0;
    let mut cur = "root";
    while let Val::Exp(l, op, r) = &monkeys[cur] {
        let (next, known) = match (resolve(l, &monkeys), resolve(r, &monkeys)) {
            (Some(a), None) => (r, a),
            (None, Some(b)) => (l, b),
            _ => unreachable!(),
        };

        val = match (op, next == r) {
            ('+', _) => val - known,
            ('-', true) => known - val,
            ('-', false) => val + known,
            ('*', _) => val / known,
            ('/', true) => known / val,
            ('/', false) => val * known,
            ('=', _) => known,
            _ => unreachable!(),
        };

        cur = next;
    }
    val
}

fn resolve(monkey: &str, monkeys: &Monkeys) -> Option<u128> {
    match &monkeys[monkey] {
        Val::Exp(l, o, r) => match resolve(l, monkeys) {
            Some(l) => resolve(r, monkeys).map(|r| match o {
                '-' => l - r,
                '+' => l + r,
                '*' => l * r,
                '/' => l / r,
                _ => unreachable!(),
            }),
            None => None,
        },
        Val::Int(n) => Some(*n),
        Val::Unknown => None,
    }
}

tests! {
    (part1, "sample", 152)
    (part1, "puzzle", 158731561459602)
    (part2, "sample", 301)
    (part2, "puzzle", 3769668716709)
}
