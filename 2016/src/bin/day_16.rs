advent_of_code_2016::main!();

fn generator(input: &str) -> Vec<bool> {
    input.bytes().map(|b| b == b'1').collect()
}

fn part_1(input: Vec<bool>) -> String {
    solve(input, 272)
}

fn part_2(input: Vec<bool>) -> String {
    solve(input, 35651584)
}

fn solve(input: Vec<bool>, len: usize) -> String {
    (0..)
        .fold_while(
            (0..)
                .fold_while(input, |acc, _| {
                    let mut next = acc
                        .iter()
                        .copied()
                        .chain(once(false))
                        .chain(acc.iter().rev().map(|&b| !b))
                        .collect_vec();
                    if next.len() >= len {
                        next.truncate(len);
                        FoldWhile::Done(next)
                    } else {
                        FoldWhile::Continue(next)
                    }
                })
                .into_inner(),
            |acc, _| {
                let checksum = acc
                    .chunks(2)
                    .map(|bs| (bs[0] == bs[1]).then(|| true).unwrap_or(false))
                    .collect_vec();
                if checksum.len() & 1 == 1 {
                    FoldWhile::Done(checksum)
                } else {
                    FoldWhile::Continue(checksum)
                }
            },
        )
        .into_inner()
        .into_iter()
        .map(|b| (b'0' + (b as u8)) as char)
        .join("")
}
