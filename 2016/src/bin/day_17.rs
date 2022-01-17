advent_of_code_2016::main!();

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn to_delta(self) -> (isize, isize) {
        match self {
            Dir::Up => (0, -1),
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
        }
    }

    fn to_byte(self) -> u8 {
        match self {
            Dir::Up => b'U',
            Dir::Right => b'R',
            Dir::Down => b'D',
            Dir::Left => b'L',
        }
    }
}

fn generator(input: &str) -> &str {
    input
}

fn part_1(input: &str) -> String {
    browse(0, 0, input, Vec::new())
        .into_iter()
        .min_by_key(|p| p.len())
        .unwrap()
        .into_iter()
        .map(|d| d.to_byte() as char)
        .collect()
}

fn part_2(input: &str) -> usize {
    browse(0, 0, input, Vec::new())
        .into_iter()
        .map(|p| p.len())
        .max()
        .unwrap()
}

fn doors(base: &str, path: &[Dir]) -> Vec<Dir> {
    let mut context = md5::Context::new();
    context.consume(base.as_bytes());
    for d in path {
        context.consume(&[d.to_byte()]);
    }
    context
        .compute()
        .0
        .into_iter()
        .take(2)
        .flat_map(|b| [b >> 4, b & 0xF])
        .map(|b| (0xB..=0xF).contains(&b))
        .zip([Dir::Up, Dir::Down, Dir::Left, Dir::Right])
        .filter_map(|(open, dir)| open.then(|| dir))
        .collect()
}

fn browse(x: isize, y: isize, base: &str, path: Vec<Dir>) -> Vec<Vec<Dir>> {
    if x == 3 && y == 3 {
        return vec![path];
    }
    doors(base, &path)
        .into_iter()
        .filter(|&dir| {
            let (dx, dy) = dir.to_delta();
            (0..4).contains(&(x + dx)) && (0..4).contains(&(y + dy))
        })
        .flat_map(|dir| {
            let mut path = path.clone();
            path.push(dir);
            browse(x + dir.to_delta().0, y + dir.to_delta().1, base, path)
        })
        .collect()
}
