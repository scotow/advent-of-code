use std::io::{stdin, prelude::BufRead};
use std::collections::HashSet;
use std::iter::FromIterator;

use day_03::path::*;

fn main() -> Result<(), &'static str> {
    let paths: Vec<Path> = stdin().lock()
        .lines()
        .filter_map(Result::ok)
        .map(|l| l.parse::<Path>().unwrap())
        .collect();

    part1(&paths[0], &paths[1]);
    part2(&paths[0], &paths[1]);
    
    Ok(())
}

fn part1(p1: &Path, p2: &Path) {
    let set1: HashSet<Visit> = HashSet::from_iter(p1.inner.clone());
    let set2: HashSet<Visit> = HashSet::from_iter(p2.inner.clone());

    let mut inters: Vec<i16> = set1.intersection(&set2)
        .map(|v| v.point)
        .map(|p| p.distance_to_zero())
        .collect();
    inters.sort();

    println!("Part 1: {:?}", inters[0]);
}

fn part2(p1: &Path, p2: &Path) {
    let mut min = 999999999;
    for v1 in p1.inner.clone() {
        for v2 in p2.inner.clone() {
            if v1.point != v2.point { continue }
            min = std::cmp::min(min, v1.time + v2.time)
        }
    }

    println!("Part 2: {:?}", min);
}