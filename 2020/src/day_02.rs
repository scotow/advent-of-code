use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug)]
pub struct PolicyCheck {
    pub char: char,
    pub range: (usize, usize),
    pub password: String
}

impl FromStr for PolicyCheck {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.replace("-", " ").replace(":", "");
        let parts = parts.split_whitespace().collect_vec();

        Ok(Self {
            char: parts[2].chars().nth(0).unwrap(),
            range: (parts[0].parse().unwrap(), parts[1].parse().unwrap()),
            password: parts[3].into()
        })
    }
}

impl PolicyCheck {
    fn satisfy_range(&self) -> bool {
        (self.range.0..=self.range.1)
            .contains(
                &self.password.chars()
                    .filter(|&c| c == self.char)
                    .count()
            )
    }

    fn satisfy_position(&self) -> bool {
        [self.range.0, self.range.1].iter()
            .filter(|&&i| self.password.chars().nth(i - 1).unwrap() == self.char)
            .count() == 1
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<PolicyCheck> {
    input.lines().filter_map(|l| l.parse().ok()).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<PolicyCheck>) -> usize {
    input.iter()
        .filter(|p| p.satisfy_range())
        .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<PolicyCheck>) -> usize {
    input.iter()
        .filter(|p| p.satisfy_position())
        .count()
}
