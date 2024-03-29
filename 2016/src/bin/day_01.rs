advent_of_code_2016::main!();

#[derive(Copy, Clone, Debug)]
enum Turn {
    Right,
    Left,
}

impl Turn {
    fn apply_to(&self, current: (isize, isize)) -> (isize, isize) {
        use Turn::*;
        match (current, self) {
            ((0, 1), Right) => (1, 0),
            ((0, -1), Right) => (-1, 0),
            ((1, 0), Right) => (0, -1),
            ((-1, 0), Right) => (0, 1),

            ((0, 1), Left) => (-1, 0),
            ((0, -1), Left) => (1, 0),
            ((1, 0), Left) => (0, 1),
            ((-1, 0), Left) => (0, -1),

            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Move {
    turn: Turn,
    distance: isize,
}

fn generator(input: &str) -> Vec<Move> {
    input
        .split(", ")
        .map(|s| {
            let distance = s[1..].parse().unwrap();
            match s.as_bytes()[0] {
                b'R' => Move {
                    turn: Turn::Right,
                    distance,
                },
                b'L' => Move {
                    turn: Turn::Left,
                    distance,
                },
                _ => unreachable!(),
            }
        })
        .collect()
}

fn part_1(moves: Vec<Move>) -> isize {
    let mut pos = (0, 0);
    let mut dir = (0, 1);
    for Move {
        turn: t,
        distance: d,
    } in moves
    {
        dir = t.apply_to(dir);
        pos = (pos.0 + dir.0 * d, pos.1 + dir.1 * d);
    }
    pos.0.abs() + pos.1.abs()
}

fn part_2(moves: Vec<Move>) -> isize {
    let mut pos = (0, 0);
    let mut dir = (0, 1);
    let mut visited = HashSet::new();
    for Move {
        turn: t,
        distance: d,
    } in moves
    {
        dir = t.apply_to(dir);
        for _ in 0..d {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
            if !visited.insert(pos) {
                return pos.0.abs() + pos.1.abs();
            }
        }
    }
    unreachable!()
}
