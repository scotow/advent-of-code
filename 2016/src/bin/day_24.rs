use pathfinding::prelude::dijkstra_all;

advent_of_code_2016::main!();

fn generator(input: &str) -> (Vec<Vec<bool>>, HashMap<u8, (usize, usize)>) {
    let mut points = HashMap::new();
    (
        input
            .lines()
            .enumerate()
            .map(|(y, r)| {
                r.bytes()
                    .enumerate()
                    .map(|(x, c)| match c {
                        b'#' => false,
                        b'.' => true,
                        _ => {
                            points.insert(c - b'0', (x, y));
                            true
                        }
                    })
                    .collect()
            })
            .collect(),
        points,
    )
}

fn part_1((grid, points): (Vec<Vec<bool>>, HashMap<u8, (usize, usize)>)) -> usize {
    solve(grid, points, false)
}

fn part_2((grid, points): (Vec<Vec<bool>>, HashMap<u8, (usize, usize)>)) -> usize {
    solve(grid, points, true)
}

fn solve(grid: Vec<Vec<bool>>, points: HashMap<u8, (usize, usize)>, back_to_zero: bool) -> usize {
    let routes = points
        .keys()
        .map(|&p| (p, distances(&grid, &points, p)))
        .collect::<HashMap<_, _>>();
    points
        .keys()
        .filter(|&&p| p != 0)
        .permutations(points.len() - 1)
        .map(|mut ps| {
            if back_to_zero {
                ps.push(&0);
            }
            once(&0)
                .chain(ps)
                .tuple_windows()
                .map(|(a, b)| routes[a][b])
                .sum()
        })
        .min()
        .unwrap()
}

fn distances(
    grid: &Vec<Vec<bool>>,
    points: &HashMap<u8, (usize, usize)>,
    start: u8,
) -> HashMap<u8, usize> {
    let paths = dijkstra_all(&points[&start], |&(x, y)| {
        [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
            .into_iter()
            .filter(|&(x, y)| grid[y][x])
            .map(|xy| (xy, 1))
    });
    points
        .into_iter()
        .filter(|&(&p, _)| p != start)
        .map(|(&p, xy)| (p, paths[xy].1))
        .collect()
}
