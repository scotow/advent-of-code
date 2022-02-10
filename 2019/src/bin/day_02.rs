advent_of_code_2019::main!();

fn generator(input: &str) -> Vec<u32> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

fn part_1(mut input: Vec<u32>) -> u32 {
    input[1] = 12;
    input[2] = 2;
    assert_eq!(run(input.clone()), 5098658);
    run(input)
}

fn part_2(input: Vec<u32>) -> u32 {
    let (n, v) = iproduct!(0..=99, 0..=99)
        .find(|&(n, v)| {
            let mut input = input.clone();
            input[1] = n;
            input[2] = v;
            run(input) == 19690720
        })
        .unwrap();
    assert_eq!(n * 100 + v, 5064);
    n * 100 + v
}

fn run(mut prog: Vec<u32>) -> u32 {
    let mut ptr = 0;
    while prog[ptr] != 99 {
        let p1 = prog[prog[ptr + 1] as usize];
        let p2 = prog[prog[ptr + 2] as usize];
        let p3 = prog[ptr + 3];
        prog[p3 as usize] = match prog[ptr] {
            1 => p1 + p2,
            2 => p1 * p2,
            _ => unreachable!(),
        };
        ptr += 4;
    }
    prog[0]
}
