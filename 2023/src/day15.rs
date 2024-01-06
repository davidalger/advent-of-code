use indexmap::IndexMap;

pub fn part1(input: String) -> u32 {
    input.trim().split(',').map(hash).sum()
}

pub fn part2(input: String) -> u32 {
    // Initialize boxes with insertion order preserving hash map
    let mut boxes = vec![IndexMap::new(); 256];

    // Initialization sequence for boxes
    input
        .trim()
        .split(',')
        .map(|s| {
            if let Ok((label, len)) = sscanf::sscanf!(s, "{str}={u8}") {
                (label, Some(len))
            } else if let Ok(label) = sscanf::sscanf!(s, "{str}-") {
                (label, None)
            } else {
                unreachable!()
            }
        })
        .for_each(|(label, len)| {
            let b = &mut boxes[hash(label) as usize];
            if let Some(len) = len {
                b.insert(label, len)
            } else {
                b.shift_remove(label)
            };
        });

    // Calculate focusing power of lenses
    boxes
        .iter()
        .enumerate()
        .map(|(i, b)| -> u32 {
            b.iter()
                .enumerate()
                .map(|(slot, (_, &len))| (i as u32 + 1) * (slot as u32 + 1) * len as u32)
                .sum()
        })
        .sum()
}

fn hash(s: &str) -> u32 {
    s.as_bytes().iter().fold(0, |cur, x| (cur + *x as u32) * 17 % 256)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::*;

    test!(hash(super::hash("HASH"), 52));
    test!(part1, "sample", 1320);
    test!(part1, "puzzle", 515974);
    test!(part2, "sample", 145);
    test!(part2, "puzzle", 265894);
}
