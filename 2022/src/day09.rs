use sscanf::sscanf;
use utils::prelude::*;

parse!(|i| -> Vec<(char, u32)> {
    i.lines().map(|l| sscanf!(l, "{char} {u32}").unwrap()).collect()
} as Moves);

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

pub fn part1(m: Moves) -> usize {
    moves(m, 2)
}

pub fn part2(m: Moves) -> usize {
    moves(m, 10)
}

fn moves(moves: Moves, knots: usize) -> usize {
    let mut visited = HashSet::new();
    let mut pos = vec![Coordinate { x: 0, y: 0 }; knots];
    visited.insert(pos[knots - 1]);

    for (motion, steps) in moves.iter() {
        for _ in 0..*steps {
            match motion {
                'R' => pos[0].x += 1,
                'L' => pos[0].x -= 1,
                'U' => pos[0].y += 1,
                'D' => pos[0].y -= 1,
                _ => unimplemented!(),
            }
            for i in 0..knots - 1 {
                follow(pos[i], &mut pos[i + 1]);
            }
            visited.insert(pos[knots - 1]);
        }
    }
    visited.len()
}

fn follow(leader: Coordinate, follower: &mut Coordinate) {
    if follower.x < leader.x - 1 {
        track_x(leader, follower, 1);
    } else if follower.x > leader.x + 1 {
        track_x(leader, follower, -1);
    }

    if follower.y < leader.y - 1 {
        track_y(leader, follower, 1);
    } else if follower.y > leader.y + 1 {
        track_y(leader, follower, -1);
    }
}

fn track_x(leader: Coordinate, follower: &mut Coordinate, x: i32) {
    follower.x += x;
    match follower.y.cmp(&leader.y) {
        Ordering::Less => follower.y += 1,
        Ordering::Greater => follower.y -= 1,
        Ordering::Equal => {}
    }
}

fn track_y(leader: Coordinate, follower: &mut Coordinate, y: i32) {
    follower.y += y;
    match follower.x.cmp(&leader.x) {
        Ordering::Less => follower.x += 1,
        Ordering::Greater => follower.x -= 1,
        Ordering::Equal => {}
    }
}

tests!(
    part1_sample(part1(input!("sample")), 13),
    part1_puzzle(part1(input!("puzzle")), 6284),
    part2_sample(part2(input!("sample")), 1),
    part2_sample2(part2(input!("sample2")), 36),
    part2_puzzle(part2(input!("puzzle")), 2661),
);
