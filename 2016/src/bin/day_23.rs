advent_of_code_2016::main!();

fn generator(input: &str) -> Vec<(&str, Param, Param)> {
    Param::from_generator(input)
}

fn part_1(ops: Vec<(&str, Param, Param)>) -> i64 {
    run(ops, [7, 0, 0, 0])
}

fn part_2(ops: Vec<(&str, Param, Param)>) -> i64 {
    run(ops, [12, 0, 0, 0])
}

fn run(mut ops: Vec<(&str, Param, Param)>, mut vars: [i64; 4]) -> i64 {
    let mut ptr = 0;
    loop {
        let &(op, p1, p2) = match ops.get(ptr) {
            Some(op) => op,
            None => return vars[0],
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
            "tgl" => {
                let target = ((ptr as i64) + p1.to_value(&vars)) as usize;
                if target < ops.len() {
                    ops[target].0 = match ops[target].0 {
                        "inc" => "dec",
                        "dec" | "tgl" => "inc",
                        "jnz" => "cpy",
                        "cpy" => "jnz",
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
        ptr += 1;
    }
}
