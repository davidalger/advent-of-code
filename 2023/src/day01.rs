use std::iter::Peekable;

pub fn part1(i: String) -> u32 {
    i.lines()
        .map(|l| {
            let d: Vec<u32> = l.chars().filter_map(|c| c.to_digit(10)).collect();
            d[0] * 10 + d[d.len() - 1]
        })
        .sum()
}

pub fn part2(i: String) -> u32 {
    i.lines()
        .map(|l| {
            let mut l = l.as_bytes().iter().peekable();
            let mut d = vec![];

            while let Some(&x) = l.peek() {
                if x.is_ascii_digit() {
                    d.push((x - b'0') as u32);
                } else {
                    for (digit, word) in [
                        (0, "zero"),
                        (1, "one"),
                        (2, "two"),
                        (3, "three"),
                        (4, "four"),
                        (5, "five"),
                        (6, "six"),
                        (7, "seven"),
                        (8, "eight"),
                        (9, "nine"),
                    ] {
                        if peek_word(l.clone(), word) {
                            d.push(digit);
                            break;
                        }
                    }
                }

                l.next();
            }

            d[0] * 10 + d[d.len() - 1]
        })
        .sum()
}

fn peek_word<'a, I: Iterator<Item = &'a u8>>(mut iter: Peekable<I>, word: &str) -> bool {
    for b in word.as_bytes() {
        if iter.next_if(|&x| x == b).is_none() {
            return false;
        }
    }
    true
}

utils::tests! {
    (part1, "sample", 142)
    (part1, "puzzle", 55834)
    (part2, "sample2", 281)
    (part2, "sample3", 206)
    (part2, "puzzle", 53221)
}
