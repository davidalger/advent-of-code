use std::collections::HashMap;

pub fn part1(input: String) -> u32 {
    let lines = input.lines().map(|l| l.bytes().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut result = 0;

    let is_punctuation = |b: u8| -> bool { b.is_ascii_punctuation() && b != b'.' };

    for (row, line) in lines.iter().enumerate() {
        let mut col = 0;

        while col < line.len() {
            if line[col].is_ascii_digit() {
                let mut is_part = false;
                let mut num: u32 = (line[col] - b'0') as u32;
                let (n_pos, mut n_len) = (col as i32, 1);

                // collect remaining digits for number
                while col + 1 < line.len() && line[col + 1].is_ascii_digit() {
                    n_len += 1;
                    col += 1;
                    num = num * 10 + (line[col] - b'0') as u32;
                }

                // detect symbols on long edges of number
                for i in (n_pos - 1).max(0)..=(n_pos + n_len).min(line.len() as i32 - 1) {
                    // top edge
                    if row > 0 && is_punctuation(lines[row - 1][i as usize]) {
                        is_part = true;
                        break;
                    }
                    //bottom edge
                    if row < lines.len() - 1 && is_punctuation(lines[row + 1][i as usize]) {
                        is_part = true;
                        break;
                    }
                }

                // detect symbols to left of number
                if n_pos - 1 > 0 && is_punctuation(line[n_pos as usize - 1]) {
                    is_part = true;
                }

                // detect symbols to right of number
                if n_pos + n_len < line.len() as i32 - 1
                    && is_punctuation(line[(n_pos + n_len) as usize])
                {
                    is_part = true;
                }

                // add to sum if number is a part number
                if is_part {
                    result += num;
                }
            }
            col += 1;
        }
    }
    result
}

pub fn part2(input: String) -> u32 {
    let lines = input.lines().map(|l| l.bytes().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut gears: HashMap<(_, _), Vec<_>> = HashMap::new();

    let mut record = |pos: (usize, usize), num: u32| {
        gears.entry(pos).and_modify(|v| v.push(num)).or_insert(vec![num]);
    };

    for (row, line) in lines.iter().enumerate() {
        let mut col = 0;

        while col < line.len() {
            if line[col].is_ascii_digit() {
                let mut num: u32 = (line[col] - b'0') as u32;
                let (n_pos, mut n_len) = (col as i32, 1);

                // collect remaining digits for number
                while col + 1 < line.len() && line[col + 1].is_ascii_digit() {
                    n_len += 1;
                    col += 1;
                    num = num * 10 + (line[col] - b'0') as u32;
                }

                // detect symbols on long edges of number
                for i in (n_pos - 1).max(0)..=(n_pos + n_len).min(line.len() as i32 - 1) {
                    // top edge
                    if row > 0 && lines[row - 1][i as usize] == b'*' {
                        record((row - 1, i as usize), num);
                    }
                    //bottom edge
                    if row < lines.len() - 1 && lines[row + 1][i as usize] == b'*' {
                        record((row + 1, i as usize), num);
                    }
                }

                // detect symbols to left of number
                if n_pos - 1 > 0 && line[n_pos as usize - 1] == b'*' {
                    record((row, n_pos as usize - 1), num);
                }

                // detect symbols to right of number
                if n_pos + n_len < line.len() as i32 - 1 && line[(n_pos + n_len) as usize] == b'*' {
                    record((row, (n_pos + n_len) as usize), num);
                }
            }
            col += 1;
        }
    }

    gears.iter().filter_map(|(_, v)| if v.len() == 2 { Some(v[0] * v[1]) } else { None }).sum()
}

utils::tests! {
    (part1, "sample", 4361)
    (part1, "puzzle", 556367)
    (part2, "sample", 467835)
    (part2, "puzzle", 89471771)
}
