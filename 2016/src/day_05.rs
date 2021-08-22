use itertools::{Itertools, FoldWhile};
use md5::Digest;
use std::collections::HashMap;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> String {
    generator(input)
        .take(8)
        .map(|d| format!("{:x}", d[2]))
        .join("")
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> String {
    generator(input)
        .filter(|d| d[2] < 8)
        .fold_while(HashMap::new(), |mut map, d| {
            map.entry(d[2]).or_insert(d[3] >> 4);
            if map.len() == 8 {
                FoldWhile::Done(map)
            } else {
                FoldWhile::Continue(map)
            }
        }).into_inner().into_iter()
        .fold([0; 8], |mut acc, (i, n)| {
            acc[i as usize] = n;
            acc
        }).iter()
        .map(|n| format!("{:x}", n))
        .join("")
}

fn generator(prefix: &str) -> impl Iterator<Item=Digest> + '_ {
    (0..)
        .map(move |n| md5::compute(&[prefix, &n.to_string()].concat()))
        .filter(|d| d.starts_with(&[0, 0]) && d[2] >> 4 == 0)
}