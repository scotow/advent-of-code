advent_of_code_2015::main!();

type Action = (Value, Option<Value>, fn(u16, Option<u16>) -> u16, String);

fn parse_action(line: &str) -> Action {
    let (left, right) = line.split(" -> ").collect_tuple().unwrap();
    let left = left.split(' ').collect_vec();

    let func: fn(u16, Option<u16>) -> u16;
    let lhs: Value;
    let rhs: Option<Value>;
    match left.len() {
        1 => {
            func = assign;
            lhs = left[0].parse().unwrap();
            rhs = None;
        }
        2 => {
            func = not;
            lhs = left[1].parse().unwrap();
            rhs = None;
        }
        3 => {
            func = match left[1] {
                "AND" => and,
                "OR" => or,
                "LSHIFT" => lshift,
                "RSHIFT" => rshift,
                _ => unreachable!(),
            };
            lhs = left[0].parse().unwrap();
            rhs = Some(left[2].parse().unwrap());
        }
        _ => unreachable!(),
    }

    (lhs, rhs, func, right.to_string())
}

fn assign(lhs: u16, _rhs: Option<u16>) -> u16 {
    lhs
}
fn not(lhs: u16, _rhs: Option<u16>) -> u16 {
    !lhs
}
fn and(lhs: u16, rhs: Option<u16>) -> u16 {
    lhs & rhs.unwrap()
}
fn or(lhs: u16, rhs: Option<u16>) -> u16 {
    lhs | rhs.unwrap()
}
fn lshift(lhs: u16, rhs: Option<u16>) -> u16 {
    lhs << rhs.unwrap()
}
fn rshift(lhs: u16, rhs: Option<u16>) -> u16 {
    lhs >> rhs.unwrap()
}

#[derive(Debug, Clone)]
enum Value {
    Direct(u16),
    Variable(String),
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<u16>() {
            Ok(n) => Value::Direct(n),
            Err(_) => Value::Variable(s.to_string()),
        })
    }
}

fn generator(input: &str) -> Vec<Action> {
    input.lines().map(parse_action).collect()
}

fn part_1(input: Vec<Action>) -> u16 {
    solve(&input)
}

fn part_2(input: Vec<Action>) -> u16 {
    let a = solve(&input);
    let mut actions = input
        .iter()
        .filter(|act| act.3 != "b")
        .cloned()
        .collect_vec();
    actions.insert(0, (Value::Direct(a), None, assign, "b".to_string()));

    solve(&actions)
}

fn solve(input: &[Action]) -> u16 {
    let mut values = HashMap::new();
    while !values.contains_key("a") {
        input.iter().for_each(|(lhs, rhs, action, dest)| {
            let lhs = match resolve(&values, lhs) {
                Some(v) => v,
                None => return,
            };
            let rhs = match rhs {
                None => None,
                Some(rhs) => match resolve(&values, rhs) {
                    None => return,
                    Some(v) => Some(v),
                },
            };
            values.insert(dest.clone(), action(lhs, rhs));
        });
    }

    values["a"]
}

fn resolve(values: &HashMap<String, u16>, v: &Value) -> Option<u16> {
    match v {
        Value::Direct(n) => Some(*n),
        Value::Variable(k) => values.get(k).copied(),
    }
}
