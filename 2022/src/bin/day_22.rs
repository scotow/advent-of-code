advent_of_code_2022::main!();

const UP: Pos<isize> = (0, -1);
const RIGHT: Pos<isize> = (1, 0);
const LEFT: Pos<isize> = (-1, 0);
const DOWN: Pos<isize> = (0, 1);

#[derive(Clone, Debug)]
struct Grid {
    cells: HashMap<Pos<isize>, bool>,
    edges_map: HashMap<(Pos<isize>, Pos<isize>), (Pos<isize>, Pos<isize>)>,
}

impl Grid {
    fn apply_dir(&self, (x, y): Pos<isize>, (dx, dy): Pos<isize>) -> (Pos<isize>, Pos<isize>) {
        let face_len = self.face_len();
        (
            (1..)
                .find_map(|i| {
                    let nxy = (
                        (x + i * dx).rem_euclid(face_len * 4),
                        (y + i * dy).rem_euclid(face_len * 4),
                    );
                    self.cells.contains_key(&nxy).then_some(nxy)
                })
                .unwrap(),
            (dx, dy),
        )
    }

    fn apply_dir_cube(&self, (x, y): Pos<isize>, (dx, dy): Pos<isize>) -> (Pos<isize>, Pos<isize>) {
        let face_len = self.face_len();
        let (nx, ny) = (x + dx, y + dy);
        if self.cells.contains_key(&(nx, ny)) {
            ((nx, ny), (dx, dy))
        } else {
            let (tx, itx) = x.div_rem(&face_len);
            let (ty, ity) = y.div_rem(&face_len);
            let ((tx, ty), (ndx, ndy)) = self.edges_map[&((tx, ty), (dx, dy))];
            let (itx, ity) = match ((dx, dy), (ndx, ndy)) {
                (UP, RIGHT) => (0, itx),
                (LEFT, RIGHT) => (0, face_len - 1 - ity),
                (UP, UP) => (itx, face_len - 1),
                (RIGHT, LEFT) => (face_len - 1, face_len - 1 - ity),
                (DOWN, LEFT) => (face_len - 1, itx),
                (RIGHT, UP) => (ity, face_len - 1),
                (LEFT, DOWN) => (ity, 0),
                (DOWN, DOWN) => (itx, 0),
                _ => unreachable!(),
            };
            ((tx * face_len + itx, ty * face_len + ity), (ndx, ndy))
        }
    }

    fn face_len(&self) -> isize {
        ((self.cells.len() / 6) as f64).sqrt() as isize
    }

    fn start(&self) -> Pos<isize> {
        (
            self.cells
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
        Grid {
            cells: map
                .lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.bytes()
                        .enumerate()
                        .filter(|(_, b)| !b.is_ascii_whitespace())
                        .map(move |(x, b)| ((x as isize, y as isize), b == b'.'))
                })
                .collect(),
            edges_map: HashMap::from([
                (((1, 0), UP), ((0, 3), RIGHT)),
                (((1, 0), LEFT), ((0, 2), RIGHT)),
                (((2, 0), UP), ((0, 3), UP)),
                (((2, 0), RIGHT), ((1, 2), LEFT)),
                (((2, 0), DOWN), ((1, 1), LEFT)),
                (((1, 1), RIGHT), ((2, 0), UP)),
                (((1, 1), LEFT), ((0, 2), DOWN)),
                (((0, 2), UP), ((1, 1), RIGHT)),
                (((0, 2), LEFT), ((1, 0), RIGHT)),
                (((1, 2), RIGHT), ((2, 0), LEFT)),
                (((1, 2), DOWN), ((0, 3), LEFT)),
                (((0, 3), RIGHT), ((1, 2), UP)),
                (((0, 3), DOWN), ((2, 0), DOWN)),
                (((0, 3), LEFT), ((1, 0), DOWN)),
            ]),
        },
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
    solve(grid, ops, Grid::apply_dir)
}

fn part_2((grid, ops): (Grid, Vec<Either<usize, Pos<isize>>>)) -> isize {
    solve(grid, ops, Grid::apply_dir_cube)
}

fn solve(
    grid: Grid,
    ops: Vec<Either<usize, Pos<isize>>>,
    f: fn(&Grid, Pos<isize>, Pos<isize>) -> (Pos<isize>, Pos<isize>),
) -> isize {
    let (mut dx, mut dy) = (1, 0);
    let (mut x, mut y) = grid.start();
    for op in ops {
        match op {
            Either::Left(n) => {
                for _ in 0..n {
                    let (nxy, ndxy) = f(&grid, (x, y), (dx, dy));
                    if !grid.cells[&nxy] {
                        break;
                    }
                    (x, y) = nxy;
                    (dx, dy) = ndxy;
                }
            }
            Either::Right((rx, ry)) => {
                (dx, dy) = (dy * rx, dx * ry);
            }
        }
    }
    (y + 1) * 1_000 + (x + 1) * 4 + ((dx + dy) == -1) as isize * 2 + (dy.abs() == 1) as isize
}
