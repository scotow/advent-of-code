advent_of_code_2015::main!();

#[derive(Debug, Clone)]
struct Ingredient(HashMap<String, i64>);

impl FromStr for Ingredient {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(", ")
                .map(|c| c.split_whitespace().collect_tuple().unwrap())
                .map(|(n, v)| (n.to_string(), v.parse().unwrap()))
                .collect::<HashMap<_, _>>(),
        ))
    }
}

fn score(i: &[Ingredient], c: &[usize], p: &str) -> i64 {
    i.iter()
        .map(|i| i.0[p])
        .zip(c)
        .map(|(i, &n)| i * n as i64)
        .sum::<i64>()
        .max(0)
}

fn combinations(ingredients: &[Ingredient]) -> impl Iterator<Item = Vec<usize>> {
    (0..ingredients.len())
        .map(|_| 0..=100)
        .multi_cartesian_product()
        .filter(|v| v.iter().sum::<usize>() == 100)
}

fn filter_calories<'a>(
    ingredients: &'a [Ingredient],
    iter: impl Iterator<Item = Vec<usize>> + 'a,
) -> impl Iterator<Item = Vec<usize>> + 'a {
    iter.filter(move |v| score(&ingredients, &v, "calories") == 500)
}

fn resolve<'a>(ingredients: &'a [Ingredient], iter: impl Iterator<Item = Vec<usize>> + 'a) -> i64 {
    iter.map(|v| {
        ["capacity", "durability", "flavor", "texture"]
            .iter()
            .map(|k| score(&ingredients, &v, k))
            .product()
    })
    .max()
    .unwrap()
}

fn generator(input: &str) -> Vec<Ingredient> {
    input
        .lines()
        .map(|l| l.split(": ").nth(1).unwrap())
        .map(|i| i.parse().unwrap())
        .collect()
}

fn part_1(ingredients: Vec<Ingredient>) -> i64 {
    resolve(&ingredients, combinations(&ingredients))
}

fn part_2(ingredients: Vec<Ingredient>) -> i64 {
    resolve(
        &ingredients,
        filter_calories(&ingredients, combinations(&ingredients)),
    )
}
