advent_of_code_2018::main!();

fn generator(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            l.split(", ")
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part_1(input: Vec<(usize, usize)>) -> usize {
    min_max(&input)
        .filter_map(|(x, y)| {
            input
                .iter()
                .map(|&(xs, ys)| (abs_diff(x, xs) + abs_diff(y, ys), (xs, ys)))
                .into_group_map()
                .into_iter()
                .min_by_key(|&(d, _)| d)
                .unwrap()
                .1
                .into_iter()
                .exactly_one()
                .ok()
        })
        .counts()
        .into_values()
        .max()
        .unwrap()
}

fn part_2(input: Vec<(usize, usize)>) -> usize {
    min_max(&input)
        .filter(|&(x, y)| {
            input
                .iter()
                .map(|&(xs, ys)| abs_diff(x, xs) + abs_diff(y, ys))
                .sum::<usize>()
                < 10000
        })
        .count()
}

fn min_max(points: &[(usize, usize)]) -> impl Iterator<Item = (usize, usize)> {
    let (min_x, min_y, max_x, max_y) = points.iter().fold(
        (usize::MAX, usize::MAX, 0, 0),
        |(x1, y1, x2, y2), &(x, y)| (x.min(x1), y.min(y1), x.max(x2), y.max(y2)),
    );
    iproduct!(min_x..=max_x, min_y..=max_y)
}
