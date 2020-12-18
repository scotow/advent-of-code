#[aoc_generator(day18)]
fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.replace(' ', "")).collect()
}

#[aoc(day18, part1)]
fn part1(input: &[String]) -> u64 {
    input.iter()
        .map(|l| solve(l))
        .sum()
}

fn solve(input: &str) -> u64 {
    let mut r = 0;
    let mut o: fn(u64, u64) -> u64 = std::ops::Add::add;

    let mut i = 0;
    while i < input.len() {
        match input.chars().nth(i).unwrap() {
            c @ '0'..='9' => r = o(r, parse_nb(c)),
            '+' => o = std::ops::Add::add,
            '*' => o = std::ops::Mul::mul,
            '(' => {
                let length = find_close(&input[i+1..]);
                // dbg!(&input[i+1..i+length]);
                r = o(r, solve(&input[i+1..i+length]));
                i += length - 1;
            }
            ')' => (),
            _ => unreachable!(),
        }
        i += 1;
    }
    r
}

fn parse_nb(c: char) -> u64 {
    c.to_string().parse().unwrap()
}

fn find_close(input: &str) -> usize {
    let mut n = 1;
    for (i, c) in input.chars().enumerate() {
        n += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        if n == 0 {
            return i + 1
        }
    }
    unreachable!()
}

// fn find_open(input: &str) -> usize {
//     let mut n = 1;
//     for (i, c) in input.chars().enumerate().rev() {
//         n += match c {
//             '(' => 1,
//             ')' => -1,
//             _ => 0,
//         };
//         if n == 0 {
//             return i + 1
//         }
//     }
//     unreachable!()
// }

// fn add_parentheses(input: &str) -> String {
//     let out = input.to_string();
//     let mut i = 0;
//     while i < out.len() {
//         match input.chars().nth(i).unwrap() {
//             '+' => {
//                 match input.chars().
//             },
//             _ => unreachable!(),
//         }
//         i += 1;
//     }
//     out
// }