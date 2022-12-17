advent_of_code_2022::main!();

struct Grid {
    cells: Vec<[bool; 7]>,
    pattern_idx: u8,
}

impl Grid {
    fn new() -> Self {
        Self {
            cells: vec![[false; 7]],
            pattern_idx: 0,
        }
    }

    fn add(&mut self, flows: &mut impl Iterator<Item = (usize, isize)>) {
        let last_rock = self.last_rock();
        if self.cells.len() < last_rock + 7 {
            let to_extend = last_rock + 7 - self.cells.len();
            self.cells.extend(repeat_n([false; 7], to_extend));
        }

        let y = last_rock + 3;
        let mut moving_cells = match self.pattern_idx {
            0 => (2..6).map(|x| (x, y)).collect(),
            1 => vec![(3, y + 2), (2, y + 1), (3, y + 1), (4, y + 1), (3, y)],
            2 => vec![(4, y + 2), (4, y + 1), (2, y), (3, y), (4, y)],
            3 => (y..y + 4).map(|y| (2, y)).collect(),
            4 => vec![(2, y + 1), (3, y + 1), (2, y), (3, y)],
            _ => unreachable!(),
        };
        self.pattern_idx = (self.pattern_idx + 1) % 5;

        loop {
            let flow = flows.next().unwrap().1;
            self.move_piece(&mut moving_cells, (flow, 0));
            if !self.move_piece(&mut moving_cells, (0, -1)) {
                for &(x, y) in &moving_cells {
                    self.cells[y][x] = true;
                }
                break;
            }
        }
    }

    fn last_rock(&self) -> usize {
        self.cells
            .iter()
            .rposition(|r| r.iter().any(|&c| c))
            .map(|y| y + 1)
            .unwrap_or(0)
    }

    fn move_piece(&self, cells: &mut Vec<Pos<usize>>, (dx, dy): (isize, isize)) -> bool {
        if cells.iter().all(|&(x, y)| {
            let next_x = x.wrapping_add_signed(dx);
            let Some(next_y) = y.checked_add_signed(dy) else {
                return false;
            };
            (0..7).contains(&next_x) && !self.cells[next_y][next_x]
        }) {
            for (x, y) in cells {
                (*x, *y) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            }
            return true;
        }
        false
    }

    fn hash(&self, flow_idx: usize) -> Option<(u64, usize)> {
        let last_rock = self.last_rock();
        if last_rock < 10 {
            return None;
        }
        let mut hasher = DefaultHasher::new();
        self.pattern_idx.hash(&mut hasher);
        flow_idx.hash(&mut hasher);
        for y in last_rock - 10..last_rock {
            self.cells[y].hash(&mut hasher);
        }
        Some((hasher.finish(), last_rock))
    }
}

fn generator(input: &str) -> Vec<isize> {
    input
        .bytes()
        .map(|b| -1 + (b == b'>') as isize * 2)
        .collect()
}

fn part_1(flows: Vec<isize>) -> usize {
    solve(flows, 2022)
}

fn part_2(flows: Vec<isize>) -> usize {
    solve(flows, 1_000_000_000_000)
}

fn solve(flows: Vec<isize>, last_round: usize) -> usize {
    let mut flows = flows.into_iter().enumerate().cycle().peekable();
    let mut grid = Grid::new();
    let mut cache = HashMap::new();
    let mut skipped_height = 0;
    let mut round = 0;
    loop {
        if let Some((hash, height)) = grid.hash(flows.peek().unwrap().0) {
            if let Some((prev_height, prev_round)) = cache.get(&hash) {
                if round + prev_round < last_round {
                    let skips = (last_round - round) / (round - prev_round);
                    round += skips * (round - prev_round);
                    skipped_height += skips * (height - prev_height);
                }
            } else {
                cache.insert(hash, (height, round));
            }
        }
        grid.add(&mut flows);
        round += 1;
        if round == last_round {
            return grid.last_rock() + skipped_height;
        }
    }
}
