use std::io::{stdin, prelude::BufRead};
use std::collections::HashSet;
use std::iter::FromIterator;

use day_03::point::*;
use day_03::path::*;

fn main() -> Result<(), &'static str> {
    let paths: Vec<Path> = stdin().lock()
        .lines()
        .filter_map(Result::ok)
        .map(|l| l.parse::<Path>().unwrap())
        .collect();

    let path1: HashSet<Point> = HashSet::from_iter(paths[0].inner.clone());
    let path2: HashSet<Point> = HashSet::from_iter(paths[1].inner.clone());

    let mut inters: Vec<i16> = path1.intersection(&path2).map(Point::distance_to_zero).collect();
    inters.sort();

    println!("Part1: {:?}", inters[0]);
    Ok(())
}