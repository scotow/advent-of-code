advent_of_code_2020::main!();

#[derive(Clone, Debug)]
struct PolicyCheck {
    char: char,
    range: (usize, usize),
    password: String,
}

impl FromStr for PolicyCheck {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.replace("-", " ").replace(":", "");
        let parts = parts.split_whitespace().collect_vec();

        Ok(Self {
            char: parts[2].chars().nth(0).unwrap(),
            range: (parts[0].parse().unwrap(), parts[1].parse().unwrap()),
            password: parts[3].into(),
        })
    }
}

impl PolicyCheck {
    fn satisfy_range(&self) -> bool {
        (self.range.0..=self.range.1)
            .contains(&self.password.chars().filter(|&c| c == self.char).count())
    }

    fn satisfy_position(&self) -> bool {
        [self.range.0, self.range.1]
            .iter()
            .filter(|&&i| self.password.chars().nth(i - 1).unwrap() == self.char)
            .count()
            == 1
    }
}

fn generator(input: &str) -> Vec<PolicyCheck> {
    input.lines().filter_map(|l| l.parse().ok()).collect()
}

fn part_1(input: Vec<PolicyCheck>) -> usize {
    input.iter().filter(|p| p.satisfy_range()).count()
}

fn part_2(input: Vec<PolicyCheck>) -> usize {
    input.iter().filter(|p| p.satisfy_position()).count()
}
