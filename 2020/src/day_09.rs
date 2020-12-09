use itertools::Itertools;

const PREAMBLE: usize = 25;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
fn part1(input: &Vec<u64>) -> u64 {
    next_invalid(input).1
}

oc(day9, part2)]
fn part2(input: &Vec<u64>) -> u64 {
    let res = input.iter()
        .enumerate()
        .skip(PREAMBLE)
        .filter(|(i, n)| {
            is_invalid(&input[..=*i])
        })
        .map(|x| dbg!(x))
        .tuple_windows()
        .map(|x| dbg!(x))
        .filter(|((i1, _), (i2, _))| *i2 == i1 + 1)
        .take_while(|((i1, _), (i2, _))| *i2 == i1 + 1)
        .collect_vec();
    // .flat_map(|((_, n1), (_, n2))| vec![n1, n2].into_iter())
    // .minmax().into_option().unwrap();

    dbg!(res);
    // res.0 + res.1
    0
}

// #[aoc(day9, part2)]
// fn part2(input: &Vec<u64>) -> u64 {
//     let res = input.iter()
//         .enumerate()
//         .skip(PREAMBLE)
//         .filter(|(i, n)| {
//             is_invalid(&input[..=*i])
//         })
//         .map(|x| dbg!(x))
//         .tuple_windows()
//         .map(|x| dbg!(x))
//         .filter(|((i1, _), (i2, _))| *i2 == i1 + 1)
//         .take_while(|((i1, _), (i2, _))| *i2 == i1 + 1)
//         .collect_vec();
//         // .flat_map(|((_, n1), (_, n2))| vec![n1, n2].into_iter())
//         // .minmax().into_option().unwrap();
//
//     dbg!(res);
//     // res.0 + res.1
//     0
// }

fn next_invalid(input: &[u64]) -> (usize, u64) {
    let res = input.iter()
        .enumerate()
        .skip(PREAMBLE)
        .find(|(i, n)| {
            // dbg!(..*i, n);
            is_invalid(&input[..=*i])
        }).unwrap();
    (res.0, *res.1)
}

fn is_invalid(input: &[u64]) -> bool {
    let (value, end) = (input[input.len() - 1], input.len() - 1);
    // dbg!(value, index, input.len(), index-5..index, input[index-5..index].iter().collect_vec());
    !input[end-PREAMBLE..end].iter()
        .combinations(2)
        .any(|v| v[0] + v[1] == value)
}