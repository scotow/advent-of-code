use std::collections::HashMap;
use itertools::Itertools;

#[aoc_generator(day9)]
fn input_generator(input: &str) -> HashMap<String, HashMap<String, u64>> {
    let mut map = HashMap::new();
    input.lines()
        .map(|l| l.split(' ').collect_vec())
        .map(|p| (p[0].to_string(), p[2].to_string(), p[4].parse::<u64>().unwrap()))
        .for_each(|(from, to, dist)| {
            map.entry(from.clone()).or_insert(HashMap::new()).insert(to.clone(), dist);
            map.entry(to).or_insert(HashMap::new()).insert(from, dist);
        });

    map
}

#[aoc(day9, part1)]
fn part1(input: &HashMap<String, HashMap<String, u64>>) -> Option<u64> {
    solver(input).min()
}

#[aoc(day9, part2)]
fn part2(input: &HashMap<String, HashMap<String, u64>>) -> Option<u64> {
    solver(input).max()
}

fn solver<'a>(input: &'a HashMap<String, HashMap<String, u64>>) -> impl Iterator<Item=u64> + 'a {
    input.keys()
        .permutations(input.len())
        .map(move |cs| {
            cs.iter()
                .tuple_windows()
                .map(|(c1, c2)| input.get(c1.clone()).unwrap().get(c2.clone()).unwrap())
                .sum()
        })
}
