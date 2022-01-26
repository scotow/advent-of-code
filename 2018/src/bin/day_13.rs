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
    inter: u8,
}

impl Cart {
    fn walk(&mut self, map: &HashMap<(isize, isize), Dir>) {
        self.pos = (self.pos.0 + self.dir.0, self.pos.1 + self.dir.1);
        self.dir = match map[&self.pos] {
            Dir::Horizontal | Dir::Vertical => self.dir,
            Dir::Turn1 => (-self.dir.1, -self.dir.0),
            Dir::Turn2 => (self.dir.1, self.dir.0),
            Dir::Intersection => {
                self.inter = (self.inter + 1) % 3;
                match self.inter {
                    0 => (self.dir.1, -self.dir.0),
                    1 => self.dir,
                    2 => (-self.dir.1, self.dir.0),
                    _ => unreachable!(),
                }
            }
        }
    }
}

fn generator(input: &str) -> (HashMap<(isize, isize), Dir>, Vec<Cart>) {
    let mut map = HashMap::new();
    let mut carts = Vec::new();
    input.lines().enumerate().for_each(|(y, l)| {
        l.bytes().enumerate().for_each(|(x, mut c)| {
            let pos = (x as isize, y as isize);
            if let Some((under, dir)) = match c {
                b'>' => Some((b'-', (1, 0))),
                b'<' => Some((b'-', (-1, 0))),
                b'v' => Some((b'|', (0, 1))),
                b'^' => Some((b'|', (0, -1))),
                _ => None,
            } {
                c = under;
                carts.push(Cart { pos, dir, inter: 2 });
            }
            map.insert(
                pos,
                match c {
                    b'-' => Dir::Horizontal,
                    b'|' => Dir::Vertical,
                    b'/' => Dir::Turn1,
                    b'\\' => Dir::Turn2,
                    b'+' => Dir::Intersection,
                    _ => return,
                },
            );
        })
    });
    (map, carts)
}

fn part_1((map, mut carts): (HashMap<(isize, isize), Dir>, Vec<Cart>)) -> String {
    loop {
        carts.sort_by_key(|c| (c.pos.1, c.pos.0));
        for i in 0..carts.len() {
            carts[i].walk(&map);
            if let Some((_, (x, y))) = collision(&carts, i) {
                return format!("{},{}", x, y);
            }
        }
    }
}

fn part_2((map, mut carts): (HashMap<(isize, isize), Dir>, Vec<Cart>)) -> String {
    loop {
        let mut to_remove = Vec::new();
        carts.sort_by_key(|c| (c.pos.1, c.pos.0));
        for i in 0..carts.len() {
            if to_remove.contains(&i) {
                continue;
            }
            carts[i].walk(&map);
            if let Some((j, _)) = collision(&carts, i) {
                to_remove.extend([i, j]);
            }
        }
        carts = carts
            .into_iter()
            .enumerate()
            .filter_map(|(i, c)| (!to_remove.contains(&i)).then(|| c))
            .collect();
        if carts.len() == 1 {
            return format!("{},{}", carts[0].pos.0, carts[0].pos.1);
        }
    }
}

fn collision(carts: &[Cart], i: usize) -> Option<(usize, (isize, isize))> {
    carts
        .iter()
        .enumerate()
        .filter(|&(j, _)| j != i)
        .find_map(|(j, c)| (c.pos == carts[i].pos).then(|| (j, c.pos)))
}
