advent_of_code_2018::main!();

fn generator(input: &str) -> (usize, Vec<[u64; 4]>) {
    program::from_generator(input)
}

fn part_1((ptr_idx, program): (usize, Vec<[u64; 4]>)) -> u64 {
    solve(ptr_idx, program, [0; 6])
}

fn part_2((ptr_idx, program): (usize, Vec<[u64; 4]>)) -> u64 {
    solve(ptr_idx, program, [1, 0, 0, 0, 0, 0])
}

fn solve(ptr_idx: usize, program: Vec<[u64; 4]>, mut vars: [u64; 6]) -> u64 {
    let mut previous = 0;
    let mut visited = 0;
    loop {
        let op = match program.get(vars[ptr_idx] as usize) {
            Some(&op) => op,
            None => {
                return vars[0];
            }
        };
        program::exec(&mut vars, op);
        vars[ptr_idx] += 1;
        if vars[4] == previous {
            visited += 1;
        }
        if visited == 16 {
            break;
        }
        previous = vars[4]
    }
    let mut factor = 0;
    for n in 1..=previous {
        if previous % n == 0 {
            factor += n;
        }
    }
    factor
}
