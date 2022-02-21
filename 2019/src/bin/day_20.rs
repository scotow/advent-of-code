use pathfinding::prelude::bfs;

advent_of_code_2019::main!();

type Grid = HashMap<(usize, usize), Cell>;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
enum Cell {
    Open,
    Teleport([char; 2], bool),
}

fn generator(input: &str) -> Grid {
    let input = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let ((h1y, h1x), (h2y, h2x)) = iproduct!(2..input.len() - 2, 2..input[0].len() - 2)
        .filter(|&(y, x)| input[y][x] == ' ')
        .minmax()
        .into_option()
        .unwrap();
    iproduct!(1..input.len() - 1, 1..input[0].len() - 1)
        .filter_map(|(y, x)| {
            match input[y][x] {
                '.' => Some(Cell::Open),
                'A'..='Z' if y == 1 || y == h2y => {
                    Some(Cell::Teleport([input[y - 1][x], input[y][x]], y == 1))
                }
                'A'..='Z' if x == input[0].len() - 2 || x == h1x => Some(Cell::Teleport(
                    [input[y][x], input[y][x + 1]],
                    x == input[0].len() - 2,
                )),
                'A'..='Z' if y == input.len() - 2 || y == h1y => Some(Cell::Teleport(
                    [input[y][x], input[y + 1][x]],
                    y == input.len() - 2,
                )),
                'A'..='Z' if x == 1 || x == h2x => {
                    Some(Cell::Teleport([input[y][x - 1], input[y][x]], x == 1))
                }
                _ => None,
            }
            .map(|c| ((x, y), c))
        })
        .collect()
}

fn part_1(grid: Grid) -> u8 {
    let tps = grid
        .iter()
        .filter(|&(&xy, &t1)| matches!(t1, Cell::Teleport(_, _)));
    let paths = iproduct!(tps.clone(), tps.clone())
        .filter(|((from, _), (to, _))| from != to)
        .filter_map(|((&from, &t1), (&to, &t2))| {
            bfs(
                &from,
                |&(x, y)| {
                    [
                        (x, y.saturating_sub(1)),
                        (x + 1, y),
                        (x, y + 1),
                        (x.saturating_sub(1), y),
                    ]
                    .into_iter()
                    .filter(|xy| grid.contains_key(xy))
                },
                |&xy| xy == to,
            )
            .map(|p| (t1, (t2, p.len() - 2)))
        })
        .into_grouping_map()
        .collect::<HashMap<_, _>>();
    0
}

fn part_2(_input: Grid) -> u8 {
    0
}
