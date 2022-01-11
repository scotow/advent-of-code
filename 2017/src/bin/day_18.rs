advent_of_code_2017::main!();

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
    fn to_value(self, vars: &[i64; 26]) -> i64 {
        match self {
            Param::Value(n) => n,
            Param::Var(i) => vars[i],
            Param::None => 0,
        }
    }

    fn to_var(self, vars: &mut [i64; 26]) -> &mut i64 {
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
    *run(&ops, &mut [0; 26], &mut 0, &mut Vec::new())
        .1
        .last()
        .unwrap()
}

fn part_2(ops: Vec<(&str, Param, Param)>) -> usize {
    let (mut vars_1, mut ptr_1, mut queue_1) = ([0; 26], 0, Vec::new());
    let (mut vars_2, mut ptr_2, mut queue_2) = ([0; 26], 0, Vec::new());
    vars_1[15] = 0;
    vars_2[15] = 1;
    let mut res = 0;
    loop {
        let (moved_1, out) = run(&ops, &mut vars_1, &mut ptr_1, &mut queue_1);
        queue_2.extend(out);
        let (moved_2, out) = run(&ops, &mut vars_2, &mut ptr_2, &mut queue_2);
        res += out.len();
        queue_1.extend(out);
        if !moved_1 && !moved_2 {
            break;
        }
    }
    res
}

fn run(
    ops: &[(&str, Param, Param)],
    vars: &mut [i64; 26],
    ptr: &mut usize,
    queue: &mut Vec<i64>,
) -> (bool, Vec<i64>) {
    let mut moved = false;
    let mut out = Vec::new();
    loop {
        let &(op, p1, p2) = match ops.get(*ptr) {
            Some(op) => op,
            None => return (moved, out),
        };
        let val = p2.to_value(&vars);
        match op {
            "snd" => {
                out.push(p1.to_value(vars));
            }
            "set" => *p1.to_var(vars) = val,
            "add" => *p1.to_var(vars) += val,
            "mul" => *p1.to_var(vars) *= val,
            "mod" => *p1.to_var(vars) %= val,
            "rcv" => {
                if queue.len() >= 1 {
                    *p1.to_var(vars) = queue.remove(0);
                } else {
                    return (moved, out);
                }
            }
            "jgz" => {
                if p1.to_value(vars) > 0 {
                    *ptr = ((*ptr as i64) + val - 1) as usize;
                }
            }
            _ => unreachable!(),
        }
        *ptr += 1;
        moved = true;
    }
}
