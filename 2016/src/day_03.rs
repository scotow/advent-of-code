use itertools::Itertools;
use std::convert::TryInto;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<[u16; 3]> {
    input.lines()
        .map(|l| l.split(' ')
            .filter_map(|n| n.parse::<u16>().ok())
            .collect_vec()
            .try_into().unwrap()
        )
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(triangles: &Vec<[u16; 3]>) -> usize {
    valids(triangles)
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<[u16; 3]>) -> usize {
    let mut triangles = vec![[0, 0, 0]; input.len()];
    for (i, r) in input.iter().enumerate() {
        triangles[i / 3 * 3][i % 3] = r[0];
        triangles[i / 3 * 3 + 1][i % 3] = r[1];
        triangles[i / 3 * 3 + 2][i % 3] = r[2];
    }
    valids(&triangles)
}

pub fn valids(triangles: &Vec<[u16; 3]>) -> usize {
    triangles.iter()
        .map(|pts| {
            let mut pts = pts.clone();
            pts.sort();
            pts
        })
        .filter(|&[a, b, c]| a + b > c)
        .count()
}