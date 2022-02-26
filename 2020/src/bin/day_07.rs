advent_of_code_2020::main!();

fn generator(input: &str) -> HashMap<String, Vec<(u16, String)>> {
    input
        .lines()
        .map(|l| {
            let parts = l[..l.len() - 1].splitn(2, " bags contain ").collect_vec();
            (
                parts[0].to_string(),
                if parts[1] == "no other bags" {
                    vec![]
                } else {
                    parts[1]
                        .replace(" bags", "")
                        .replace(" bag", "")
                        .split(", ")
                        .map(|sb| {
                            let mut p = sb.splitn(2, ' ');
                            (
                                p.next().unwrap().parse::<u16>().unwrap(),
                                p.next().unwrap().to_string(),
                            )
                        })
                        .collect_vec()
                },
            )
        })
        .collect()
}

fn part_1(input: HashMap<String, Vec<(u16, String)>>) -> usize {
    input.keys().filter(|b| has_gold(&input, b)).count() - 1
}

fn part_2(input: HashMap<String, Vec<(u16, String)>>) -> u16 {
    count_bags(&input, "shiny gold") - 1
}

fn has_gold(rules: &HashMap<String, Vec<(u16, String)>>, bag: &str) -> bool {
    bag == "shiny gold" || rules[bag].iter().any(|(_n, sb)| has_gold(&rules, sb))
}

fn count_bags(rules: &HashMap<String, Vec<(u16, String)>>, bag: &str) -> u16 {
    rules[bag]
        .iter()
        .map(|(c, b)| c * count_bags(&rules, b))
        .sum::<u16>()
        + 1
}
