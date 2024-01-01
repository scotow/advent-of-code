advent_of_code_2023::main!();

type Pos3<T> = (T, T, T);

#[derive(Clone)]
struct Layout {
    bricks: Vec<(Pos3<usize>, Pos3<usize>)>,
    grid: Vec<Vec<Vec<Option<usize>>>>,
}

impl Layout {
    fn new(input: Vec<(Pos3<usize>, Pos3<usize>)>) -> Self {
        let max_x = input.iter().map(|b| b.1 .0).max().unwrap();
        let max_y = input.iter().map(|b| b.1 .1).max().unwrap();
        let max_z = input.iter().map(|b| b.1 .2).max().unwrap();
        let mut grid = vec![vec![vec![None; max_x + 1]; max_y + 1]; max_z + 1];
        for (i, x, y, z) in
            input
                .iter()
                .enumerate()
                .flat_map(|(i, &((x1, y1, z1), (x2, y2, z2)))| {
                    iproduct!(Some(i), x1..=x2, y1..=y2, z1..=z2)
                })
        {
            grid[z][y][x] = Some(i);
        }
        let mut layout = Self {
            bricks: input,
            grid,
        };
        layout.settle();
        layout
    }

    fn fall(&mut self) -> HashSet<usize> {
        let mut changed = HashSet::new();
        for i in 0..self.bricks.len() {
            loop {
                if self.bricks[i].0 .2 == 0 {
                    break;
                }
                if iproduct!(
                    self.bricks[i].0 .0..=self.bricks[i].1 .0,
                    self.bricks[i].0 .1..=self.bricks[i].1 .1,
                    Some(self.bricks[i].0 .2 - 1)
                )
                .all(|(x, y, z)| self.grid[z][y][x].is_none())
                {
                    changed.insert(i);
                    for (x, y, z) in iproduct!(
                        self.bricks[i].0 .0..=self.bricks[i].1 .0,
                        self.bricks[i].0 .1..=self.bricks[i].1 .1,
                        self.bricks[i].0 .2..=self.bricks[i].1 .2
                    ) {
                        self.grid[z - 1][y][x] = Some(i);
                        self.grid[z][y][x] = None;
                    }
                    self.bricks[i].0 .2 -= 1;
                    self.bricks[i].1 .2 -= 1;
                } else {
                    break;
                }
            }
        }
        changed
    }

    fn settle(&mut self) -> usize {
        let mut total_changed = HashSet::new();
        loop {
            let changed = self.fall();
            if changed.is_empty() {
                return total_changed.len();
            } else {
                total_changed.extend(changed);
            }
        }
    }

    fn disintegrate(&mut self, i: usize) {
        for (x, y, z) in iproduct!(
            self.bricks[i].0 .0..=self.bricks[i].1 .0,
            self.bricks[i].0 .1..=self.bricks[i].1 .1,
            self.bricks[i].0 .2..=self.bricks[i].1 .2
        ) {
            self.grid[z][y][x] = None;
        }
        self.bricks.remove(i);
    }
}

fn generator(input: &str) -> Vec<(Pos3<usize>, Pos3<usize>)> {
    input
        .lines()
        .map(|l| {
            l.split('~')
                .map(|p| {
                    p.split(',')
                        .map(|n| n.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part_1(input: Vec<(Pos3<usize>, Pos3<usize>)>) -> usize {
    run(input, |l| l.fall().is_empty() as usize)
}

fn part_2(input: Vec<(Pos3<usize>, Pos3<usize>)>) -> usize {
    run(input, Layout::settle)
}

fn run(input: Vec<(Pos3<usize>, Pos3<usize>)>, f: impl Fn(&mut Layout) -> usize) -> usize {
    let layout = Layout::new(input.clone());
    (0..input.len())
        .map(|i| {
            let mut layout = layout.clone();
            layout.disintegrate(i);
            f(&mut layout)
        })
        .sum()
}
