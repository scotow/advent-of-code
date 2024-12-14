advent_of_code_2024::main!();

fn generator(input: &str) -> Vec<(Pos<i32>, Pos<i32>)> {
    input
        .lines()
        .map(|l| {
            let (x, y, vx, vy) = l
                .split(&['=', ',', ' '])
                .filter_map(|n| n.parse().ok())
                .collect_tuple()
                .unwrap();
            ((x, y), (vx, vy))
        })
        .collect()
}

fn part_1(robots: Vec<(Pos<i32>, Pos<i32>)>) -> usize {
    robots
        .into_iter()
        .filter_map(|((x, y), (vx, vy))| {
            let (x, y) = (
                (x + vx * 100).rem_euclid(101),
                (y + vy * 100).rem_euclid(103),
            );
            (x != 50 && y != 51).then_some((x / 51, y / 52))
        })
        .counts()
        .into_values()
        .product()
}

fn part_2(robots: Vec<(Pos<i32>, Pos<i32>)>) -> i32 {
    (1..)
        .find(|i| {
            let robots = robots
                .iter()
                .map(|((x, y), (vx, vy))| {
                    ((x + vx * i).rem_euclid(101), (y + vy * i).rem_euclid(103))
                })
                .collect::<HashSet<_>>();
            robots
                .iter()
                .any(|&(x, y)| (y..y + 10).all(|y| robots.contains(&(x, y))))
        })
        .unwrap()
}
