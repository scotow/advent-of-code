use std::collections::HashMap;
use std::str::FromStr;
use itertools::Itertools;
use std::cmp::max;

#[derive(Debug, Clone)]
struct Ingredient(HashMap<String, i64>);

impl FromStr for Ingredient {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            Self(
                s.split(", ")
                    .map(|c| c.split_whitespace().collect_tuple().unwrap())
                    .map(|(n, v)| (n.to_string(), v.parse().unwrap()))
                    .collect::<HashMap<_, _>>()
            )
        )
    }
}

fn score(i: &[Ingredient], c: &[usize], p: &str) -> i64 {
    max(
        0,
        i.iter()
            .map(|i| i.0[p])
            .zip(c)
            .map(|(i, &n)| i * n as i64)
            .sum()
    )
}

fn combinations(ingredients: &[Ingredient]) -> impl Iterator<Item=Vec<usize>> {
    (0..ingredients.len()).map(|_| 0..=100).multi_cartesian_product()
        .filter(|v| v.iter().sum::<usize>() == 100)
}

fn filter_calories<'a>(ingredients: &'a [Ingredient], iter: impl Iterator<Item=Vec<usize>> + 'a) -> impl Iterator<Item=Vec<usize>> + 'a {
    iter.filter(move |v| score(&ingredients, &v, "calories") == 500)
}

fn resolve<'a>(ingredients: &'a [Ingredient], iter: impl Iterator<Item=Vec<usize>> + 'a) -> i64 {
    iter
        .map(|v| ["capacity", "durability", "flavor", "texture"].iter()
            .map(|k| score(&ingredients, &v, k))
            .product()
        )
        .max().unwrap()
}

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Vec<Ingredient> {
    input.lines()
        .map(|l| l.split(": ").nth(1).unwrap())
        .map(|i| i.parse().unwrap())
        .collect()
}

#[aoc(day15, part1)]
fn part1(ingredients: &[Ingredient]) -> i64 {
    resolve(&ingredients, combinations(&ingredients))
}

#[aoc(day15, part2)]
fn part2(ingredients: &[Ingredient]) -> i64 {
    resolve(&ingredients, filter_calories(&ingredients, combinations(&ingredients)))
}
