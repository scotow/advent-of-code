advent_of_code_2020::main!();

#[derive(Copy, Clone, Debug, PartialEq)]
enum Action {
    Acc,
    Jmp,
    Nop,
}

impl Action {
    fn to_fn(&self) -> fn(&mut usize, &mut i32, i32) {
        match self {
            Action::Acc => acc,
            Action::Jmp => jmp,
            Action::Nop => nop,
        }
    }
}

enum AlgoResult<T> {
    Blocked(T),
    Terminated(T),
}

fn generator(input: &str) -> Vec<(Action, i32)> {
    input
        .lines()
        .map(|l| l.split(' ').collect_tuple().unwrap())
        .map(|(a, p)| {
            (
                match a {
                    "acc" => Action::Acc,
                    "jmp" => Action::Jmp,
                    "nop" => Action::Nop,
                    _ => unreachable!(),
                },
                p.parse::<i32>().unwrap(),
            )
        })
        .collect()
}

fn part_1(input: Vec<(Action, i32)>) -> i32 {
    match solve(&input) {
        AlgoResult::Blocked(r) => r,
        AlgoResult::Terminated(_) => unreachable!(),
    }
}

fn part_2(input: Vec<(Action, i32)>) -> i32 {
    input
        .iter()
        .enumerate()
        .find_map(|(i, (a, p))| {
            let mut copy = input.clone();
            copy[i] = (
                match a {
                    Action::Acc => Action::Acc,
                    Action::Jmp => Action::Nop,
                    Action::Nop => Action::Jmp,
                },
                *p,
            );

            match solve(&copy) {
                AlgoResult::Blocked(_) => None,
                AlgoResult::Terminated(r) => Some(r),
            }
        })
        .unwrap()
}

fn solve(input: &Vec<(Action, i32)>) -> AlgoResult<i32> {
    let mut visited = HashSet::<usize>::new();
    let mut pointer = 0;
    let mut accumulator = 0;

    loop {
        if pointer == input.len() {
            return AlgoResult::Terminated(accumulator);
        }
        if visited.contains(&pointer) {
            return AlgoResult::Blocked(accumulator);
        }
        visited.insert(pointer);

        let (act, param) = input[pointer];
        act.to_fn()(&mut pointer, &mut accumulator, param);
    }
}

fn acc(pointer: &mut usize, acc: &mut i32, param: i32) {
    *acc += param;
    *pointer += 1;
}

fn jmp(pointer: &mut usize, _acc: &mut i32, param: i32) {
    *pointer = ((*pointer as i32) + param) as usize
}

fn nop(pointer: &mut usize, _acc: &mut i32, _param: i32) {
    *pointer += 1;
}
