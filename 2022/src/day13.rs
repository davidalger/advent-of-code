use serde_json::{from_str, json, Value};
use utils::prelude::*;

pub fn part1(input: String) -> u32 {
    let pairs = input
        .split("\n\n")
        .map(|chunk| {
            let p = chunk.split_once('\n').unwrap();
            (from_str(p.0).unwrap(), from_str(p.1).unwrap())
        })
        .collect_vec();
    let mut indices = Vec::new();

    for (index, pair) in pairs.iter().enumerate() {
        if cmp(&pair.0, &pair.1) == Less {
            indices.push(index as u32 + 1);
        }
    }
    indices.iter().sum()
}

pub fn part2(input: String) -> usize {
    let markers = vec![json!([[2]]), json!([[6]])];
    let mut values =
        input.lines().filter(|l| l.trim() != "").map(|l| from_str(l).unwrap()).collect_vec();
    values.append(&mut markers.clone());

    markers
        .iter()
        .map(|m| {
            values.iter().sorted_by(|a, b| cmp(a, b)).find_position(|l| *l == m).unwrap().0 + 1
        })
        .product()
}

fn cmp(a: &Value, b: &Value) -> Ordering {
    if a.is_number() && b.is_number() && a != b {
        a.as_u64().unwrap().cmp(&b.as_u64().unwrap())
    } else if a.is_array() && b.is_array() && a != b {
        let a = a.as_array().unwrap().iter();
        let b = b.as_array().unwrap().iter();
        for p in a.zip_longest(b) {
            return match p {
                Both(a, b) => match cmp(a, b) {
                    Equal => continue,
                    ord => ord,
                },
                Left(_) => Greater,
                Right(_) => Less,
            };
        }
        Equal
    } else if a != b {
        let a = match a {
            Value::Array(v) => Value::Array(v.clone()),
            v => Value::Array(vec![v.clone()]),
        };
        let b = match b {
            Value::Array(v) => Value::Array(v.clone()),
            v => Value::Array(vec![v.clone()]),
        };
        cmp(&a, &b)
    } else {
        Equal
    }
}

tests!(
    part1_sample(part1(input!("sample")), 13),
    part1_puzzle(part1(input!("puzzle")), 5905),
    part2_sample(part2(input!("sample")), 140),
    part2_puzzle(part2(input!("puzzle")), 21691),
);
