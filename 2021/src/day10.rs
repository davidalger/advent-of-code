use utils::prelude::*;

pub fn part1(input: String) -> u64 {
    let mut score = 0;
    for line in input.lines() {
        let mut chars = line.chars();
        match step_in(chars.next().unwrap(), &mut chars) {
            Ok(None) => {}
            token => {
                // Extra trailing token may be returned as Ok(Some) and corrupted token as Err()
                let token = token.map(|t| t.unwrap()).or_else(Ok::<_, ()>).unwrap();
                score += match token {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => unreachable!(),
                }
            }
        };
    }
    score
}

pub fn part2(input: String) -> u64 {
    let mut scores = Vec::new();
    for line in input.lines() {
        // Discard corrupted lines
        let mut tokens = line.chars();
        if step_in(tokens.next().unwrap(), &mut tokens).is_err() {
            continue;
        }

        let mut t = Vec::new();
        for token in line.chars() {
            if valid_token(&token) {
                t.push(token);
            } else {
                t.pop();
            }
        }
        t.reverse();

        let mut score: u64 = 0;
        for t in t {
            score *= 5;
            score += match t {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => unreachable!(),
            };
        }
        scores.push(score);
    }
    scores.sort();
    scores[scores.len() / 2]
}

fn step_in(open: char, tokens: &mut Chars) -> Result<Option<char>, char> {
    if !valid_token(&open) {
        return Err(open); // Invalid opening token
    }

    if let Some(token) = tokens.next() {
        if valid_token(&token) {
            match step_in(token, tokens) {
                Ok(Some(close)) => step_out(open, tokens, close),
                Ok(None) => Ok(None),
                Err(err) => Err(err),
            }
        } else {
            step_out(open, tokens, token)
        }
    } else {
        Ok(None)
    }
}

fn step_out(open: char, tokens: &mut Chars, close: char) -> Result<Option<char>, char> {
    if close == get_matching(open) {
        if let Some(token) = tokens.next() {
            if valid_token(&token) {
                step_in(token, tokens)
            } else {
                Ok(Some(token)) // Return for validation
            }
        } else {
            Ok(None)
        }
    } else {
        Err(close) // Unexpected closing token
    }
}

fn valid_token(t: &char) -> bool {
    HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]).contains_key(t)
}

fn get_matching(t: char) -> char {
    *HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]).get(&t).unwrap()
}

tests! {
    (part1, "sample", 26397)
    (part1, "extra-close", 1197)
    (part1, "invalid-open", 26400)
    (part1, "puzzle", 411471)
    (part2, "sample", 288957)
    (part2, "puzzle", 3122628974)
}
