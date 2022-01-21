advent_of_code_2016::main!();

#[derive(Clone, Copy, Debug)]
enum Param {
    Value(i64),
    Var(usize),
    None,
}

impl FromStr for Param {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Ok(n) = s.parse() {
            Self::Value(n)
        } else {
            Self::Var((s.as_bytes()[0] - b'a') as usize)
        })
    }
}

impl Param {
    fn to_value(self, vars: &[i64; 4]) -> i64 {
        match self {
            Param::Value(n) => n,
            Param::Var(i) => vars[i],
            Param::None => 0,
        }
    }

    fn to_var(self, vars: &mut [i64; 4]) -> &mut i64 {
        match self {
            Param::Var(i) => &mut vars[i],
            _ => unreachable!(),
        }
    }
}

fn generator(input: &str) -> Vec<(&str, Param, Param)> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            (
                parts.next().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts
                    .next()
                    .map(|s| s.parse().unwrap())
                    .unwrap_or(Param::None),
            )
        })
        .collect()
}

fn part_1(ops: Vec<(&str, Param, Param)>) -> i64 {
    (0..).find(|&n| run(&ops, [n, 0, 0, 0])).unwrap()
}

fn part_2(_ops: Vec<(&str, Param, Param)>) -> &'static str {
    "N/A"
}

fn run(ops: &Vec<(&str, Param, Param)>, mut vars: [i64; 4]) -> bool {
    let mut steps = 0;
    let mut last = 1;
    let mut ptr = 0;
    loop {
        let &(op, p1, p2) = match ops.get(ptr) {
            Some(op) => op,
            None => return false,
        };
        let val = p2.to_value(&vars);
        match op {
            "cpy" => *p2.to_var(&mut vars) = p1.to_value(&vars),
            "inc" => *p1.to_var(&mut vars) += 1,
            "dec" => *p1.to_var(&mut vars) -= 1,
            "jnz" => {
                if p1.to_value(&vars) != 0 {
                    ptr = ((ptr as i64) + val - 1) as usize;
                }
            }
            "out" => {
                if last ^ p1.to_value(&vars) == 0 {
                    return false;
                }
                last = p1.to_value(&vars);
                steps += 1;
                if steps >= 8 {
                    return true;
                }
            }
            _ => unreachable!(),
        }
        ptr += 1;
    }
}
