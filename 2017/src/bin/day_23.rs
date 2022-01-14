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
    fn to_value(self, vars: &[i64; 8]) -> i64 {
        match self {
            Param::Value(n) => n,
            Param::Var(i) => vars[i],
            Param::None => 0,
        }
    }

    fn to_var(self, vars: &mut [i64; 8]) -> &mut i64 {
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

fn part_1(ops: Vec<(&str, Param, Param)>) -> usize {
    let mut vars = [0; 8];
    let mut ptr = 0;
    let mut muls = 0;
    loop {
        let &(op, p1, p2) = match ops.get(ptr) {
            Some(op) => op,
            None => return muls,
        };
        let val = p2.to_value(&mut vars);
        match op {
            "set" => *p1.to_var(&mut vars) = val,
            "sub" => *p1.to_var(&mut vars) -= val,
            "mul" => {
                *p1.to_var(&mut vars) *= val;
                muls += 1;
            }
            "jnz" => {
                if p1.to_value(&mut vars) != 0 {
                    ptr = ((ptr as i64) + val - 1) as usize;
                }
            }
            _ => unreachable!(),
        }
        ptr += 1;
    }
}

fn part_2(ops: Vec<(&str, Param, Param)>) -> i64 {
    let b = match ops[0].2 {
        Param::Value(n) => n,
        _ => unreachable!(),
    };
    let mut vars = [1, b, b, 0, 0, 0, 0, 0];
    vars[1] = vars[1] * 100 + 100000;
    vars[2] = vars[1] + 17000;
    loop {
        vars[5] = 1;
        vars[3] = 2;
        vars[4] = 2;
        loop {
            if vars[4] * vars[4] >= vars[1] {
                break;
            }
            if vars[1] % vars[4] == 0 {
                vars[5] = 0;
                break;
            }
            vars[4] += 1;
        }
        if vars[5] == 0 {
            vars[7] += 1;
        }
        vars[6] = vars[1] - vars[2];
        vars[1] += 17;
        if vars[6] == 0 {
            return vars[7];
        }
    }
}
