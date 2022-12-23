advent_of_code_2022::main!();

#[derive(Clone)]
struct Grid {
    cells: HashSet<Pos<isize>>,
    dir_queue: [Pos<isize>; 4],
}

impl Iterator for Grid {
    type Item = HashSet<Pos<isize>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = HashSet::with_capacity(self.cells.len());
        let mut proposal = HashMap::<_, Vec<_>>::new();
        'elf: for &(x, y) in &self.cells {
            if neighbors8(x, y).all(|nxy| !self.cells.contains(&nxy)) {
                next.insert((x, y));
                continue;
            }

            for (dx, dy) in self.dir_queue {
                if if dy != 0 {
                    [(-1, dy), (0, dy), (1, dy)]
                } else {
                    [(dx, -1), (dx, 0), (dx, 1)]
                }
                .into_iter()
                .all(|(dx, dy)| !self.cells.contains(&(x + dx, y + dy)))
                {
                    proposal.entry((x + dx, y + dy)).or_default().push((x, y));
                    continue 'elf;
                }
            }
            next.insert((x, y));
        }

        for ((tx, ty), requesters) in proposal {
            if requesters.len() == 1 {
                next.insert((tx, ty));
            } else {
                next.extend(requesters);
            }
        }
        self.dir_queue.rotate_left(1);
        Some(replace(&mut self.cells, next))
    }
}

fn generator(input: &str) -> Grid {
    Grid {
        cells: input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.bytes()
                    .enumerate()
                    .filter_map(move |(x, c)| (c == b'#').then_some((x as isize, y as isize)))
            })
            .collect(),
        dir_queue: [(0, -1), (0, 1), (-1, 0), (1, 0)],
    }
}

fn part_1(mut grid: Grid) -> isize {
    let cs = grid.nth(10).unwrap();
    let (min_x, max_x) = cs.iter().map(|&(x, _)| x).minmax().into_option().unwrap();
    let (min_y, max_y) = cs.iter().map(|&(_, y)| y).minmax().into_option().unwrap();
    (max_x - min_x + 1) * (max_y - min_y + 1) - cs.len() as isize
}

fn part_2(grid: Grid) -> usize {
    grid.tuple_windows()
        .position(|(cs1, cs2)| cs1 == cs2)
        .unwrap()
        + 1
}
