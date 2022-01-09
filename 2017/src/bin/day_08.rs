advent_of_code_2017::main!();

fn generator(input: &str) -> Vec<(&str, i16, &str, fn(&i16, &i16) -> bool, i16)> {
    input
        .lines()
        .map(|l| {
            let (var, sign, val, _, var_cond, cmp, val_cond) =
                l.split_whitespace().collect_tuple().unwrap();
            let mut val = val.parse().unwrap();
            if sign == "dec" {
                val *= -1;
            }
            (
                var,
                val,
                var_cond,
                match cmp {
                    ">" => i16::gt,
                    "<" => i16::lt,
                    ">=" => i16::ge,
                    "<=" => i16::le,
                    "==" => i16::eq,
                    "!=" => i16::ne,
                    _ => unreachable!(),
                },
                val_cond.parse().unwrap(),
            )
        })
        .collect()
}

fn part_1(input: Vec<(&str, i16, &str, fn(&i16, &i16) -> bool, i16)>) -> i16 {
    solve(input).0
}

fn part_2(input: Vec<(&str, i16, &str, fn(&i16, &i16) -> bool, i16)>) -> i16 {
    solve(input).1
}

fn solve(input: Vec<(&str, i16, &str, fn(&i16, &i16) -> bool, i16)>) -> (i16, i16) {
    let (regs, max) = input.into_iter().fold(
        (HashMap::new(), i16::MIN),
        |(mut regs, mut max), (var, val, var_cond, cond, val_cond)| {
            if cond(regs.entry(var_cond).or_default(), &val_cond) {
                *regs.entry(var).or_default() += val;
                max = max.max(regs[var]);
            }
            (regs, max)
        },
    );
    (regs.into_values().max().unwrap(), max)
}
