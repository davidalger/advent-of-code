use std::collections::{hash_map::RandomState, HashSet};

utils::parse!(|i| -> Vec<Vec<char>> { i.lines().map(|l| l.chars().collect()).collect() } as Map with derive(Clone));

pub fn part1(map: Map) -> usize {
    let points = collect_points(&map);
    render_loop(&map, &points);
    points.len() / 2
}

pub fn part2(_map: Map) -> usize {
    todo!()
}

fn locate_start(map: &Map) -> (usize, usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if map[y][x] == 'S' {
                return (x, y);
            }
        }
    }
    panic!("failed to locate starting point")
}

fn collect_points(map: &Map) -> Vec<(usize, usize)> {
    let start = locate_start(&map);
    let mut points = vec![];

    let (mut x, mut y) = start;
    let (mut prev_x, mut prev_y) = (x, y);

    loop {
        points.push((x, y));

        match map[y][x] {
            'S' => {
                if y > 0 && matches!(map[y - 1][x], '|' | 'F' | '7') {
                    // start northbound
                    y -= 1;
                } else if x > 0 && matches!(map[y][x - 1], '-' | 'F' | 'L') {
                    // start westbound
                    x -= 1;
                } else if x + 1 < map[y].len() - 1 && matches!(map[y][x + 1], '-' | '7' | 'J') {
                    // start eastbound
                    x += 1;
                } else {
                    panic!("invalid map");
                }
            }
            '|' if prev_y < y => {
                // move south
                prev_y = y;
                y += 1;
            }
            '|' => {
                // move north
                prev_y = y;
                y -= 1;
            }
            '-' if prev_x < x => {
                // move east
                prev_x = x;
                x += 1;
            }
            '-' => {
                // move west
                prev_x = x;
                x -= 1;
            }
            'L' if prev_y < y => {
                // move east
                prev_y = y;
                x += 1;
            }
            'L' => {
                // move north
                prev_x = x;
                y -= 1;
            }
            'J' if prev_x < x => {
                // move north
                prev_x = x;
                y -= 1;
            }
            'J' => {
                // move west
                prev_y = y;
                x -= 1;
            }
            'F' if prev_x > x => {
                // move south
                prev_x = x;
                y += 1;
            }
            'F' => {
                // move east
                prev_y = y;
                x += 1;
            }
            '7' if prev_x < x => {
                // move south
                prev_x = x;
                y += 1;
            }
            '7' => {
                // move west
                prev_y = y;
                x -= 1;
            }
            _ => unimplemented!(),
        };

        if start == (x, y) {
            break;
        }
    }
    points
}

fn render_loop(map: &Map, points: &Vec<(usize, usize)>) -> Map {
    let points: HashSet<(usize, usize), RandomState> = HashSet::from_iter(points.iter().cloned());
    let mut map = map.clone();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            map[y][x] = if points.contains(&(x, y)) { map[y][x] } else { '.' };
            print!("{}", map[y][x]);
        }
        println!()
    }
    println!();

    map
}

utils::tests! {
    (part1, "sample1", 8)
    (part1, "sample2", 23)
    (part1, "sample3", 70)
    (part1, "sample4", 80)
    (part1, "puzzle", 6870)
    // (part2, "sample1", 1)
    // (part2, "sample2", 4)
    // (part2, "sample3", 8)
    // (part2, "sample4", 10)
    // (part2, "puzzle", 0)
}
