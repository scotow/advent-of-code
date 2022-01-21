advent_of_code_2016::main!();

fn generator(input: &str) -> Vec<(&str, Param, Param)> {
    Param::from_generator(input)
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
