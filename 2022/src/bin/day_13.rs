advent_of_code_2022::main!();

#[derive(PartialEq, Clone, Debug)]
enum Item {
    Int(u8),
    List(Vec<Item>),
}

impl Item {
    fn parse<I: Iterator<Item = char>>(buffer: &mut Peekable<I>) -> Self {
        if buffer.peek().unwrap().is_ascii_digit() {
            Item::Int(
                buffer
                    .peeking_take_while(char::is_ascii_digit)
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            )
        } else {
            buffer.dropping(1);
            let mut items = Vec::new();
            loop {
                if *buffer.peek().unwrap() == ']' {
                    buffer.dropping(1);
                    break;
                }
                items.push(Item::parse(buffer));
                if *buffer.peek().unwrap() == ',' {
                    buffer.dropping(1);
                }
            }
            Item::List(items)
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Item::Int(n1), Item::Int(n2)) => n1.partial_cmp(n2),
            (Item::List(l1), Item::List(l2)) => l1.partial_cmp(l2),
            (&Item::Int(n1), Item::List(_)) => Item::List(vec![Item::Int(n1)]).partial_cmp(other),
            (Item::List(_), &Item::Int(n2)) => self.partial_cmp(&Item::List(vec![Item::Int(n2)])),
        }
    }
}

fn generator(input: &str) -> Vec<[Item; 2]> {
    input
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| Item::parse(&mut l.chars().peekable()))
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect()
}

fn part_1(input: Vec<[Item; 2]>) -> usize {
    input
        .into_iter()
        .positions(|[l1, l2]| l1 < l2)
        .map(|i| i + 1)
        .sum()
}

fn part_2(input: Vec<[Item; 2]>) -> usize {
    let a1 = Item::List(vec![Item::List(vec![Item::Int(2)])]);
    let a2 = Item::List(vec![Item::List(vec![Item::Int(6)])]);
    chain!([&a1, &a2], input.iter().flatten())
        .sorted_by(|l1, l2| l1.partial_cmp(l2).unwrap())
        .positions(|l| l == &a1 || l == &a2)
        .map(|i| i + 1)
        .product()
}
