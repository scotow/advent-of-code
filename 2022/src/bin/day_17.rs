advent_of_code_2022::main!();
use Cell::*;

struct Grid {
    cells: Vec<[Cell; 7]>,
    pattern_idx: u8,
}

#[derive(Copy, Clone)]
enum Cell {
    Empty,
    Moving,
    Rock,
}

impl Grid {
    fn new() -> Self {
        Self {
            cells: vec![[Empty; 7]],
            pattern_idx: 0,
        }
    }

    fn add(&mut self, flows: &mut impl Iterator<Item = isize>) {
        let last_rock = self.last_rock();
        if self.cells.len() < last_rock + 7 {
            let to_extend = last_rock + 7 - self.cells.len();
            self.cells.extend(repeat_n([Empty; 7], to_extend));
        }
        let mut moving_cells = match self.pattern_idx {
            0 => (2..6).map(|x| (x, last_rock + 3)).collect(),
            1 => {
                let y = last_rock + 3;
                vec![(3, y + 2), (2, y + 1), (3, y + 1), (4, y + 1), (3, y)]
            }
            2 => {
                let y = last_rock + 3;
                vec![(4, y + 2), (4, y + 1), (2, y), (3, y), (4, y)]
            }
            3 => (last_rock + 3..last_rock + 7).map(|y| (2, y)).collect(),
            4 => {
                let y = last_rock + 3;
                vec![(2, y + 1), (3, y + 1), (2, y), (3, y)]
            }
            _ => unreachable!(),
        };
        self.pattern_idx = (self.pattern_idx + 1) % 5;

        loop {
            let flow = flows.next().unwrap();
            self.move_piece(&mut moving_cells, (flow, 0));
            if !self.move_piece(&mut moving_cells, (0, -1)) {
                for &(x, y) in &moving_cells {
                    self.cells[y][x] = Rock;
                }
                break;
            }
        }
    }

    fn last_rock(&self) -> usize {
        self.cells
            .iter()
            .position(|r| r.iter().all(|&c| matches!(c, Empty)))
            .unwrap()
    }

    fn move_piece(&self, cells: &mut Vec<Pos<usize>>, (dx, dy): (isize, isize)) -> bool {
        if cells.iter().all(|&(x, y)| {
            let next_x = x.wrapping_add_signed(dx);
            let Some(next_y) = y.checked_add_signed(dy) else {
                return false;
            };
            (0..7).contains(&next_x) && next_y >= 0 && matches!(self.cells[next_y][next_x], Empty)
        }) {
            for (x, y) in cells {
                *x = x.checked_add_signed(dx).unwrap();
                *y = y.checked_add_signed(dy).unwrap();
            }
            return true;
        }
        false
    }

    fn print(&self) {
        println!(
            "\n{}\n",
            self.cells
                .iter()
                .rev()
                .map(|r| {
                    chain!(
                        once('|'),
                        r.iter().map(|c| match c {
                            Empty => '.',
                            Moving => '@',
                            Rock => '#',
                        }),
                        once('|')
                    )
                    .collect::<String>()
                })
                .join("\n")
        )
    }
}

fn generator(input: &str) -> Vec<isize> {
    input
        .bytes()
        .map(|b| (b == b'>').then_some(1).unwrap_or(-1))
        .collect()
}

fn part_1(input: Vec<isize>) -> usize {
    let mut flows = input.into_iter().cycle();
    let mut grid = Grid::new();
    for _ in 0..2022 {
        grid.add(&mut flows);
    }
    grid.last_rock()
}

fn part_2(input: Vec<isize>) -> u8 {
    0
}
