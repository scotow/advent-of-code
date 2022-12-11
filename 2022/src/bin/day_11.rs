advent_of_code_2022::main!();

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<u64>,
    op: (fn(u64, u64) -> u64, Option<u64>),
    test: u64,
    throw: [usize; 2],
    inspected: usize,
}

impl Monkey {
    fn inspect(&mut self, reduction: u64) {
        for item in &mut self.items {
            *item = self.op.0(*item, self.op.1.unwrap_or(*item)) / reduction;
        }
        self.inspected += self.items.len();
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines().skip(1);
        Ok(Monkey {
            items: lines.next().unwrap()[18..]
                .split(", ")
                .map(|n| n.parse().unwrap())
                .collect(),
            op: {
                let mut op = lines.next().unwrap().split_whitespace().skip(4);
                (
                    match op.next().unwrap() {
                        "+" => u64::add,
                        "*" => u64::mul,
                        _ => unreachable!(),
                    },
                    op.next().unwrap().parse().ok(),
                )
            },
            test: lines.next().unwrap()[21..].parse().unwrap(),
            throw: lines
                .map(|l| l.split_whitespace().rev().next().unwrap().parse().unwrap())
                .collect_vec()
                .try_into()
                .unwrap(),
            inspected: 0,
        })
    }
}

fn generator(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|sec| sec.parse().unwrap())
        .collect()
}

fn part_1(input: Vec<Monkey>) -> usize {
    solve(input, 20, 3)
}

fn part_2(input: Vec<Monkey>) -> usize {
    solve(input, 10_000, 1)
}

fn solve(mut monkeys: Vec<Monkey>, rounds: usize, reduction: u64) -> usize {
    let common = monkeys.iter().map(|m| m.test).product::<u64>();
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            monkeys[i].inspect(reduction);
            for item in take(&mut monkeys[i].items) {
                let target = monkeys[i].throw[(item % monkeys[i].test != 0) as usize];
                monkeys[target].items.push_back(item % common);
            }
        }
    }
    monkeys
        .into_iter()
        .map(|m| -(m.inspected as isize))
        .k_smallest(2)
        .product::<isize>() as usize
}
