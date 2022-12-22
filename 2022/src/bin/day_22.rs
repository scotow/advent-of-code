advent_of_code_2022::main!();

#[derive(Clone, Debug)]
struct Grid(HashMap<Pos<isize>, bool>);

impl Grid {
    fn apply_dir(&self, (x, y): Pos<isize>, (dx, dy): Pos<isize>) -> Pos<isize> {
        let face_len = self.face_len();
        (1..)
            .find_map(|i| {
                let nxy = (
                    (x + i * dx).rem_euclid(face_len * 4),
                    (y + i * dy).rem_euclid(face_len * 4),
                );
                self.0.contains_key(&nxy).then_some(nxy)
            })
            .unwrap()
    }

    fn face_len(&self) -> isize {
        ((self.0.len() / 6) as f64).sqrt() as isize
    }

    fn start(&self) -> Pos<isize> {
        (
            self.0
                .iter()
                .filter_map(|(&(x, y), &c)| (y == 0 && c).then_some(x))
                .min()
                .unwrap(),
            0,
        )
    }
}

fn generator(input: &str) -> (Grid, Vec<Either<usize, Pos<isize>>>) {
    let (map, ops) = input.split_once("\n\n").unwrap();
    (
        Grid(
            map.lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.bytes()
                        .enumerate()
                        .filter(|(_, b)| !b.is_ascii_whitespace())
                        .map(move |(x, b)| ((x as isize, y as isize), b == b'.'))
                })
                .collect(),
        ),
        ops.chars()
            .group_by(|c| c.is_ascii_alphabetic())
            .into_iter()
            .map(|(d, mut cs)| {
                if d {
                    Either::Right(match cs.next().unwrap() {
                        'R' => (-1, 1),
                        'L' => (1, -1),
                        _ => unreachable!(),
                    })
                } else {
                    Either::Left(cs.collect::<String>().parse().unwrap())
                }
            })
            .collect(),
    )
}

fn part_1((grid, ops): (Grid, Vec<Either<usize, Pos<isize>>>)) -> isize {
    let (mut dx, mut dy) = (1, 0);
    let (mut x, mut y) = grid.start();
    for op in ops {
        match op {
            Either::Left(n) => {
                for _ in 0..n {
                    let nxy = grid.apply_dir((x, y), (dx, dy));
                    if !grid.0[&nxy] {
                        break;
                    }
                    (x, y) = nxy;
                }
            }
            Either::Right((rx, ry)) => {
                (dx, dy) = (dy * rx, dx * ry);
            }
        }
    }
    (y + 1) * 1_000 + (x + 1) * 4 + (dx - 1) * -2 + dy.abs()
}

fn part_2(input: (Grid, Vec<Either<usize, Pos<isize>>>)) -> u8 {
    0
}
