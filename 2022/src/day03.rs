use std::collections::HashSet;

use itertools::Itertools;

pub fn part1(input: String) -> u32 {
    let mut score = 0;
    for rs in input.lines() {
        let sets = Vec::from([
            rs[0..rs.len() / 2].chars().collect(),
            rs[rs.len() / 2..].chars().collect(),
        ]);
        score += priority(sets);
    }
    score
}

pub fn part2(input: String) -> u32 {
    let mut score = 0;
    let input = input.lines().collect_vec();
    for mut i in 0..input.len() / 3 {
        i *= 3;

        let mut sets = Vec::new();
        for line in input[i..i + 3].iter() {
            sets.push(line.chars().collect());
        }

        score += priority(sets);
    }
    score
}

fn priority(mut sets: Vec<HashSet<char>>) -> u32 {
    let mut a = sets.pop().unwrap();
    for b in sets {
        a = a.intersection(&b).cloned().collect();
    }
    let c = *a.iter().next().unwrap();

    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

utils::tests! {
    (part1, "sample", 157)
    (part1, "puzzle", 7878)
    (part2, "sample", 70)
    (part2, "puzzle", 2760)
}
