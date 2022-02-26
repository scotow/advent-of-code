advent_of_code_2020::main!();

#[derive(Debug, Copy, Clone)]
enum Operation {
    ValueMask(Mask),
    Value(u64, u64),
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with("mask") {
            let mask = s.split("= ").nth(1).unwrap();
            Operation::ValueMask(Mask(
                mask.chars()
                    .map(|c| c.to_string().parse::<Field>().unwrap())
                    .collect_vec()
                    .try_into()
                    .unwrap(),
            ))
        } else {
            let parts = s.split(&['[', ']', ' '][..]).collect_vec();
            Operation::Value(parts[1].parse().unwrap(), parts[4].parse().unwrap())
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Mask([Field; 36]);

impl Mask {
    fn overwrite(&self, value: u64) -> u64 {
        let mut value = value;
        self.0.iter().rev().enumerate().for_each(|(i, &f)| match f {
            Field::Zero => value &= !(1 << i as u64),
            Field::One => value |= 1 << i as u64,
            Field::Floating => (),
        });
        value
    }

    fn first_value(&self, value: u64) -> u64 {
        let mut value = value;
        self.0.iter().rev().enumerate().for_each(|(i, &f)| match f {
            Field::Zero => (),
            Field::One => value |= 1 << i as u64,
            Field::Floating => value &= !(1 << i as u64),
        });
        value
    }

    fn resolve(&self, value: u64) -> Vec<u64> {
        let mut values = vec![Mask(self.0).first_value(value)];
        self.0
            .iter()
            .rev()
            .enumerate()
            .filter(|(_, &f)| f == Field::Floating)
            .for_each(|(i, _)| {
                let mut news = values.iter().map(|v| v | (1 << i as u64)).collect_vec();
                values.append(&mut news);
            });
        values
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Field {
    Zero,
    One,
    Floating,
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "0" => Field::Zero,
            "1" => Field::One,
            "X" => Field::Floating,
            _ => unreachable!(),
        })
    }
}

fn generator(input: &str) -> Vec<Operation> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_1(input: Vec<Operation>) -> u64 {
    input
        .iter()
        .scan(None, |m, &o| {
            Some(match (&m, o) {
                (_, Operation::ValueMask(mask)) => {
                    *m = Some(mask);
                    (0, 0)
                }
                (Some(m), Operation::Value(a, v)) => (a, m.overwrite(v)),
                _ => unreachable!(),
            })
        })
        .collect::<HashMap<_, _>>()
        .values()
        .sum::<u64>()
}

fn part_2(input: Vec<Operation>) -> u64 {
    input
        .iter()
        .scan(None, |m, &o| {
            Some(match (&m, o) {
                (_, Operation::ValueMask(mask)) => {
                    *m = Some(mask);
                    (vec![], 0)
                }
                (Some(m), Operation::Value(a, v)) => (m.resolve(a), v),
                _ => unreachable!(),
            })
        })
        .flat_map(|(a, v)| a.into_iter().map(move |a| (a, v)))
        .collect::<HashMap<_, _>>()
        .values()
        .sum::<u64>()
}
