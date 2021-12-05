advent_of_code_2021::main!();

fn generator(input: &str) -> Vec<(u16, u16, u16, u16)> {
    input
        .lines()
        .map(|l| {
            l.split(|c: char| !c.is_numeric())
                .filter_map(|p| p.parse::<u16>().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part_1(input: Vec<(u16, u16, u16, u16)>) -> usize {
    input
        .into_iter()
        .filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2)
        .map(|(x1, y1, x2, y2)| (x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2)))
        .flat_map(|(x1, y1, x2, y2)| (x1..=x2).cartesian_product(y1..=y2))
        .counts()
        .into_values()
        .filter(|&n| n >= 2)
        .count()
}

fn part_2(input: Vec<(u16, u16, u16, u16)>) -> usize {
    input
        .into_iter()
        .flat_map(|(x1, y1, x2, y2)| {
            if x1 == x2 || y1 == y2 {
                let (x1, y1, x2, y2) = (x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2));
                (x1..=x2).cartesian_product(y1..=y2).collect_vec()
            } else {
                let ((x1, y1), (x2, y2)) = ((x1, y1).min((x2, y2)), (x1, y1).max((x2, y2)));
                (x1..=x2)
                    .zip(if y2 > y1 {
                        (y1..=y2).collect_vec()
                    } else {
                        (y2..=y1).rev().collect_vec()
                    })
                    .collect_vec()
            }
        })
        .counts()
        .into_values()
        .filter(|&n| n >= 2)
        .count()
}
