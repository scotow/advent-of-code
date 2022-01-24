advent_of_code_2018::main!();

#[derive(Clone)]
struct Map(Vec<(isize, isize, isize, isize)>, usize);

impl Map {
    fn next(&self) -> Self {
        let mut next = self.0.clone();
        for (x, y, dx, dy) in &mut next {
            *x += *dx;
            *y += *dy;
        }
        Self(next, self.1 + 1)
    }

    fn area(&self) -> isize {
        let (x1, y1, x2, y2) = min_max(self.0.iter().map(|(x, y, ..)| (*x, *y)));
        (x2 - x1) * (y2 - y1)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (x1, y1, x2, y2) = min_max(self.0.iter().map(|(x, y, ..)| (*x, *y)));
        let points = self
            .0
            .iter()
            .map(|&(x, y, ..)| (x, y))
            .collect::<HashSet<_>>();
        for y in y1..=y2 {
            for x in x1..=x2 {
                write!(f, "{}", if points.contains(&(x, y)) { '#' } else { ' ' })?;
            }
            if y != y2 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

fn generator(input: &str) -> Map {
    Map(
        input
            .lines()
            .map(|l| {
                l.split(['<', ' ', ',', '>'])
                    .flat_map(|p| p.parse().ok())
                    .collect_tuple()
                    .unwrap()
            })
            .collect(),
        0,
    )
}

fn part_1(input: Map) -> Map {
    solve(input)
}

fn part_2(input: Map) -> usize {
    solve(input).1
}

fn solve(mut map: Map) -> Map {
    let mut area = map.area();
    loop {
        let next = map.next();
        let next_area = next.area();
        if next_area > area {
            return map;
        }
        map = next;
        area = next_area;
    }
}

fn min_max(points: impl Iterator<Item = (isize, isize)>) -> (isize, isize, isize, isize) {
    points.fold(
        (isize::MAX, isize::MAX, isize::MIN, isize::MIN),
        |(x1, y1, x2, y2), (x, y)| (x.min(x1), y.min(y1), x.max(x2), y.max(y2)),
    )
}
