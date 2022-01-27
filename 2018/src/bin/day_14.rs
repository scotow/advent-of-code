advent_of_code_2018::main!();

fn generator(input: &str) -> Vec<u8> {
    input.bytes().map(|b| b - b'0').collect()
}

fn part_1(input: Vec<u8>) -> String {
    let input = input
        .into_iter()
        .fold(0, |a, c| a as usize * 10 + c as usize);
    let mut receipts = Vec::<u8>::with_capacity(input + 11);
    receipts.extend([3, 7]);
    let (mut e1, mut e2) = (0, 1);
    loop {
        if receipts[e1] + receipts[e2] >= 10 {
            receipts.push(1);
        }
        receipts.push((receipts[e1] + receipts[e2]) % 10);
        if receipts.len() >= input + 10 {
            return receipts[input..input + 10]
                .iter()
                .map(|&n| (n + b'0') as char)
                .collect();
        }
        e1 = (e1 + 1 + receipts[e1] as usize) % receipts.len();
        e2 = (e2 + 1 + receipts[e2] as usize) % receipts.len();
    }
}

fn part_2(input: Vec<u8>) -> usize {
    let mut receipts = vec![3, 7];
    let (mut e1, mut e2) = (0, 1);
    for i in 0.. {
        if receipts[e1] + receipts[e2] >= 10 {
            receipts.push(1);
        }
        if i > input.len() && &receipts[receipts.len() - input.len()..] == input.as_slice() {
            return receipts.len() - input.len();
        }
        receipts.push((receipts[e1] + receipts[e2]) % 10);
        if i > input.len() && &receipts[receipts.len() - input.len()..] == input.as_slice() {
            return receipts.len() - input.len();
        }
        e1 = (e1 + 1 + receipts[e1] as usize) % receipts.len();
        e2 = (e2 + 1 + receipts[e2] as usize) % receipts.len();
    }
    unreachable!();
}
