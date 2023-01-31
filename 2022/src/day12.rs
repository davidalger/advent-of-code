use std::collections::{HashMap, VecDeque};

pub struct Input {
    grid: Grid,
    start: Pos,
    dest: Pos,
}
type Grid = Vec<Vec<char>>;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Pos(i32, i32);

impl From<String> for Input {
    fn from(input: String) -> Self {
        let mut grid: Grid = input.lines().map(|l| l.chars().collect()).collect();

        let mut start = None;
        let mut dest = None;

        for (x, row) in grid.iter_mut().enumerate() {
            for (y, cell) in row.iter_mut().enumerate() {
                match *cell {
                    'S' => {
                        *cell = 'a';
                        start = Some(Pos(x as i32, y as i32));
                    }
                    'E' => {
                        *cell = 'z';
                        dest = Some(Pos(x as i32, y as i32));
                    }
                    _ => {}
                };
            }
        }

        Self { grid, start: start.unwrap(), dest: dest.unwrap() }
    }
}

pub fn part1(input: Input) -> usize {
    search(&input.grid, input.dest, &|p| *p == input.start)
}

pub fn part2(input: Input) -> usize {
    search(&input.grid, input.dest, &|p| input.grid[p.0 as usize][p.1 as usize] == 'a')
}

fn search(grid: &Grid, start: Pos, success: &dyn Fn(&Pos) -> bool) -> usize {
    let mut p = Vec::new();
    let mut q = VecDeque::new();
    let mut v = HashMap::new();

    v.insert(start, None);
    q.push_back(start);

    while let Some(mut node) = q.pop_front() {
        if success(&node) {
            p.push(node);
            while let Some(parent) = v.remove(&node).unwrap_or(None) {
                node = parent;
                p.push(node);
            }
            break;
        }
        for next in successors(grid, &node) {
            v.entry(next).or_insert_with(|| {
                q.push_back(next);
                Some(node)
            });
        }
    }
    p.len() - 1
}

fn successors(grid: &Grid, p: &Pos) -> Vec<Pos> {
    let &Pos(x, y) = p;
    [Pos(x + 1, y), Pos(x - 1, y), Pos(x, y + 1), Pos(x, y - 1)]
        .into_iter()
        .filter(|s| {
            !(s.0 < 0
                || s.1 < 0
                || s.0 >= grid.len() as i32
                || s.1 >= grid[s.0 as usize].len() as i32)
                && grid[x as usize][y as usize] as u8 <= grid[s.0 as usize][s.1 as usize] as u8 + 1
        })
        .collect()
}

utils::tests! {
    (part1, "sample", 31)
    (part1, "puzzle", 423)
    (part2, "sample", 29)
    (part2, "puzzle", 416)
}
