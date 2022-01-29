advent_of_code_2018::main!();

fn generator(input: &str) -> (usize, Vec<[u64; 4]>) {
    program::from_generator(input)
}

fn part_1((ptr_idx, program): (usize, Vec<[u64; 4]>)) -> u64 {
    let mut vars = [0; 6];
    loop {
        let op = match program.get(vars[ptr_idx] as usize) {
            Some(&op) => op,
            None => {
                return vars[0];
            }
        };
        program::exec(&mut vars, op);
        vars[ptr_idx] += 1;
        if vars[ptr_idx] == 28 {
            return vars.into_iter().max().unwrap();
        }
    }
}

fn part_2((ptr_idx, program): (usize, Vec<[u64; 4]>)) -> u64 {
    let mut vars = [0; 6];
    let mut visited = HashSet::new();
    let mut previous = 0;
    let mut reg = 0;
    loop {
        let op = match program.get(vars[ptr_idx] as usize) {
            Some(&op) => op,
            None => {
                return vars[0];
            }
        };
        program::exec(&mut vars, op);
        vars[ptr_idx] += 1;
        if vars[ptr_idx] == 28 {
            if previous == 0 {
                reg = vars.into_iter().position_max().unwrap();
            }
            if visited.contains(&vars[reg]) {
                return previous;
            }
            visited.insert(vars[reg]);
            previous = vars[reg];
        }
    }
}
