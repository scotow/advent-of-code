use std::ops::AddAssign;

advent_of_code_2018::main!();

#[derive(Copy, Clone, Debug)]
enum Dir {
    Horizontal,
    Vertical,
    Turn1,
    Turn2,
    Intersection,
}

#[derive(Debug, Clone)]
struct Cart {
    pos: (isize, isize),
    dir: (isize, isize),
}

impl Cart {
    fn apply(&mut self, dir: Dir) {
        match dir {
            Dir::Horizontal | Dir::Vertical => (),
            Dir::Turn1 => self.dir = (-self.dir.1, -self.dir.0),
            Dir::Turn2 => self.dir = (self.dir.1, self.dir.0),
            Dir::Intersection => todo!(),
        }
    }
}

impl AddAssign<(isize, isize)> for Cart {
    fn add_assign(&mut self, rhs: (isize, isize)) {
        self.pos = (self.pos.0 + self.dir.0, self.pos.1 + self.dir.1)
    }
}

impl AddAssign<Dir> for Cart {
    fn add_assign(&mut self, rhs: Dir) {
        self.dir = match rhs {
            Dir::Horizontal | Dir::Vertical => self.dir,
            Dir::Turn1 => (-self.dir.1, -self.dir.0),
            Dir::Turn2 => (self.dir.1, self.dir.0),
            Dir::Intersection => todo!(),
        }
    }
}

fn generator(input: &str) -> (HashMap<(isize, isize), Dir>, Vec<Cart>) {
    let mut carts = Vec::new();
    let mut carts_ref = &mut carts;
    (
        input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                let y = y;
                l.bytes().enumerate().filter_map(move |(x, mut c)| {
                    let pos = (x as isize, y as isize);
                    if let Some((under, dir)) = match c {
                        b'>' => Some((b'-', (1, 0))),
                        b'<' => Some((b'-', (-1, 0))),
                        b'v' => Some((b'|', (0, 1))),
                        b'^' => Some((b'|', (0, -1))),
                        _ => None,
                    } {
                        c = under;
                        carts_ref.push(Cart { pos, dir });
                    }
                    Some((
                        pos,
                        match c {
                            b'-' => Dir::Horizontal,
                            b'|' => Dir::Vertical,
                            b'/' => Dir::Turn1,
                            b'\\' => Dir::Turn2,
                            b'+' => Dir::Intersection,
                            _ => return None,
                        },
                    ))
                })
            })
            .collect(),
        carts,
    )
}

fn part_1((map, carts): (HashMap<(isize, isize), Dir>, Vec<Cart>)) -> u8 {
    dbg!(&map);
    let mut cart = Cart {
        pos: (2, 0),
        dir: (-1, 0),
    };
    for i in 0..20 {
        cart += cart.dir;
        cart += map[&cart.pos];
        dbg!(i, cart.pos);
    }

    0
}

fn part_2((map, carts): (HashMap<(isize, isize), Dir>, Vec<Cart>)) -> u8 {
    0
}
