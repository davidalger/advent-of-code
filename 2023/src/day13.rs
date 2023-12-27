type Pattern = Vec<Vec<u8>>;

utils::parse!(|i| -> Vec<Pattern> {
    i.split("\n\n").map(|c| c.lines().map(|l| l.as_bytes().to_vec()).collect()).collect()
} as Input);

pub fn part1(i: Input) -> usize {
    i.iter()
        .filter_map(|p| match reflection(p, None) {
            Some(r) => Some(r * 100),
            None => reflection(&rotate_vec(p), None),
        })
        .sum()
}

pub fn part2(i: Input) -> usize {
    i.iter()
        .filter_map(|p| match unsmudged_reflection(p) {
            Some(r) => Some(r * 100),
            None => unsmudged_reflection(&rotate_vec(p)),
        })
        .sum()
}

fn reflection(p: &Pattern, skip: Option<usize>) -> Option<usize> {
    // scan for reflection point
    for row in 0..p.len() - 1 {
        // reflected row
        if skip != Some(row + 1) && p[row + 1] == p[row] {
            // validate reflection
            let mut tr = true;
            for i in 1..=row {
                if row + 1 + i < p.len() && p[row - i] != p[row + 1 + i] {
                    tr = false;
                    break;
                }
            }
            if tr {
                return Some(row + 1);
            }
        }
    }
    None
}

fn unsmudged_reflection(p: &Pattern) -> Option<usize> {
    let mut p = p.clone();
    let skip = reflection(&p, None);

    let flip = |b: &mut u8| {
        *b = match b {
            b'.' => b'#',
            b'#' => b'.',
            _ => unimplemented!(),
        };
    };

    for y in 0..p.len() {
        for x in 0..p[0].len() {
            flip(&mut p[y][x]);

            let r = reflection(&p, skip);
            if r != skip && r.is_some() {
                return r;
            }

            flip(&mut p[y][x]);
        }
    }

    None
}

fn rotate_vec(p: &Pattern) -> Pattern {
    let mut r = Vec::with_capacity(p[0].len());
    for x in 0..p[0].len() {
        let mut l = Vec::with_capacity(p.len());
        for y in 0..p.len() {
            l.push(p[p.len() - y - 1][x]);
        }
        r.push(l);
    }
    r
}

utils::tests! {
    (part1, "sample", 405)
    (part1, "puzzle", 27664)
    (part2, "sample", 400)
    (part2, "puzzle", 33991)
}
