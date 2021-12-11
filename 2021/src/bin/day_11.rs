use itertools::iproduct;
use std::collections::HashSet;
advent_of_code_2021::main!();

fn generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.as_bytes()
                .into_iter()
                .map(|c| (*c - b'0') as u8)
                .collect()
        })
        .collect()
}

fn part_1(input: Vec<Vec<u8>>) -> usize {
    Shoal(input).take(100).map(|(_, c)| c).sum()
}

fn part_2(input: Vec<Vec<u8>>) -> usize {
    1 + Shoal(input)
        .enumerate()
        .find(|(_, (s, _))| s.iter().flatten().all(|&n| n == 0))
        .unwrap()
        .0
}

struct Shoal(Vec<Vec<u8>>);

impl Iterator for Shoal {
    type Item = (Vec<Vec<u8>>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter_mut().flatten().for_each(|n| *n += 1);
        let mut flashed = HashSet::new();
        while self.0.iter().flatten().any(|&n| n > 9) {
            iproduct!(0..self.0.len(), 0..self.0.len()).for_each(|(x, y)| {
                if self.0[y][x] < 10 {
                    return;
                }
                self.0[y][x] = 0;
                flashed.insert((x, y));
                iproduct!(
                    x.saturating_sub(1)..=(self.0.len() - 1).min(x + 1),
                    y.saturating_sub(1)..=(self.0.len() - 1).min(y + 1)
                )
                .filter(|&xy| xy != (x, y) && !flashed.contains(&xy))
                .for_each(|(x, y)| self.0[y][x] += 1)
            })
        }
        Some((self.0.clone(), flashed.len()))
    }
}
