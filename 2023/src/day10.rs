use std::collections::HashSet;

utils::parse!(|i| -> Vec<Vec<char>> { i.lines().map(|l| l.chars().collect()).collect() } as Map with derive(Clone));

pub fn part1(map: Map) -> usize {
    collect_points(&map, locate_start(&map)).len() / 2
}

pub fn part2(map: Map) -> usize {
    let mut map = render_loop(&map, &collect_points(&map, locate_start(&map)));
    let mut tiles = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '.' {
                if trace_point(&map, (x, y)).abs() % 2.0 == 1.0 {
                    tiles += 1;
                    map[y][x] = 'I';
                } else {
                    map[y][x] = 'O';
                }
            }
        }
    }

    tiles
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

fn collect_points(map: &Map, start: (usize, usize)) -> Vec<(usize, usize)> {
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

fn render_loop(map: &Map, points: &[(usize, usize)]) -> Map {
    let points: HashSet<_> = points.iter().collect();
    let mut map = map.clone();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if !points.contains(&(x, y)) {
                map[y][x] = '.'
            } else if map[y][x] == 'S' {
                // replace S with tile indicating pipe direction

                if y > 0 && matches!(map[y - 1][x], '|' | 'F' | '7') {
                    // valid pipe north

                    if x > 0 && matches!(map[y][x - 1], '-' | 'F' | 'L') {
                        // valid pipe west
                        map[y][x] = 'J';
                    } else if x + 1 < map[y].len() - 1 && matches!(map[y][x + 1], '-' | '7' | 'J') {
                        // valid pipe east
                        map[y][x] = 'L';
                    } else if y + 1 < map.len() - 1 && matches!(map[y + 1][x], '|' | 'L' | 'J') {
                        // valid pipe south
                        map[y][x] = '|';
                    }
                } else if y + 1 < map.len() - 1 && matches!(map[y + 1][x], '|' | 'L' | 'J') {
                    // valid pipe south

                    if x > 0 && matches!(map[y][x - 1], '-' | 'F' | 'L') {
                        // valid pipe west
                        map[y][x] = '7';
                    } else if x + 1 < map[y].len() - 1 && matches!(map[y][x + 1], '-' | '7' | 'J') {
                        // valid pipe east
                        map[y][x] = 'F';
                    }
                }
            }
        }
    }

    map
}

fn trace_point(map: &Map, point: (usize, usize)) -> f64 {
    let mut result = 0.0;
    let (mut x, y) = point;

    while x < map[y].len() {
        result += match map[y][x] {
            '|' => 1.0,
            'J' | 'F' => -0.5,
            'L' | '7' => 0.5,
            _ => 0.0,
        };
        x += 1;
    }

    result
}

utils::tests! {
    (part1, "sample1", 8)
    (part1, "sample2", 23)
    (part1, "sample3", 70)
    (part1, "sample4", 80)
    (part1, "puzzle", 6870)
    (part2, "sample1", 1)
    (part2, "sample2", 4)
    (part2, "sample3", 8)
    (part2, "sample4", 10)
    (part2, "puzzle", 287)
}
