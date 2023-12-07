use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

pub fn part1(input: String) -> u32 {
    play(input, false)
}

pub fn part2(input: String) -> u32 {
    play(input, true)
}

fn play(input: String, joker_rule: bool) -> u32 {
    input
        .lines()
        .map(|l| -> (Vec<u8>, u8, u32) {
            let (h, b) = l.split_once(' ').unwrap();

            let h = h
                .bytes()
                .map(|x| match x {
                    b'2'..=b'9' => x - b'0',
                    b'T' => 10,
                    b'J' if joker_rule => 1, // card is low when played as joker
                    b'J' => 11,
                    b'Q' => 12,
                    b'K' => 13,
                    b'A' => 14,
                    _ => unimplemented!(),
                })
                .collect_vec();

            let t = {
                // count cards by value
                let mut map = HashMap::new();
                h.iter().for_each(|card| {
                    map.entry(card).and_modify(|r| *r += 1).or_insert(1);
                });

                // count jokers if present
                let jokers = *map.get(&1).unwrap_or(&0);

                // remove jokers from map unless all cards are jokers
                if joker_rule && map.len() > 1 {
                    map.remove(&1);
                }

                // sort card counts
                let map = map.into_values().sorted().rev().collect_vec();

                // determine hand type
                if map[0] == 5 || map[0] + jokers == 5 {
                    7 // five of a kind
                } else if map[0] == 4 || map[0] + jokers == 4 {
                    6 // four of a kind
                } else if (map[0] == 3 || map[0] + jokers == 3) && map[1] == 2 {
                    5 // full house (three of a kind + one pair)
                } else if map[0] == 3 || map[0] + jokers == 3 {
                    4 // three of a kind
                } else if map[0] == 2 && map[1] == 2 {
                    3 // two pair
                } else if map[0] == 2 || map[0] + jokers == 2 {
                    2 // one pair
                } else {
                    1 // high card
                }
            };

            (h, t, b.parse().unwrap())
        })
        .sorted_by(|(hand_a, type_a, _), (hand_b, type_b, _)| match type_a.cmp(&type_b) {
            Ordering::Equal => hand_a.cmp(&hand_b),
            ordering => ordering,
        })
        .enumerate()
        .map(|(rank, (_, _, bid))| (rank as u32 + 1) * bid)
        .sum()
}

utils::tests! {
    (part1, "sample", 6440)
    (part1, "puzzle", 249638405)
    (part2, "sample", 5905)
    (part2, "puzzle", 249776650)
}
