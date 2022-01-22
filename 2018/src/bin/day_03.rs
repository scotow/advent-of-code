advent_of_code_2018::main!();

type Claim = (usize, usize, usize, usize, usize);

fn generator(input: &str) -> Vec<Claim> {
    input
        .lines()
        .map(|l| {
            l.split(&['#', ',', ':', ' ', 'x'])
                .flat_map(|n| n.parse().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part_1(input: Vec<Claim>) -> usize {
    input
        .into_iter()
        .flat_map(|(_, x, y, w, h)| iproduct!(x..x + w, y..y + h))
        .counts()
        .into_values()
        .filter(|&n| n >= 2)
        .count()
}

fn part_2(input: Vec<Claim>) -> usize {
    input
        .iter()
        .find(|&&(n1, x1, y1, w1, h1)| {
            input
                .iter()
                .filter(|&&(n2, ..)| n1 != n2)
                .all(|&(_, x2, y2, w2, h2)| {
                    x1 > x2 + w2 || x1 + w1 < x2 || y1 > y2 + h2 || y1 + h1 < y2
                })
        })
        .unwrap()
        .0
}
