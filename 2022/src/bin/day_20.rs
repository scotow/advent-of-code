advent_of_code_2022::main!();

fn generator(input: &str) -> Vec<isize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_1(enc: Vec<isize>) -> isize {
    solve(enc, 1)
}

fn part_2(mut enc: Vec<isize>) -> isize {
    for n in &mut enc {
        *n *= 811589153;
    }
    solve(enc, 10)
}

fn solve(enc: Vec<isize>, n: usize) -> isize {
    let mut indexes = (0..enc.len()).collect_vec();
    for _ in 0..n {
        for idx in 0..enc.len() {
            let i = indexes.iter().position(|&i| i == idx).unwrap();
            indexes.remove(i);
            indexes.insert(
                (i as isize + enc[idx]).rem_euclid(enc.len() as isize - 1) as usize,
                idx,
            );
        }
    }
    (1..=3)
        .map(|i| {
            enc[indexes[(indexes
                .iter()
                .position(|&i| i == enc.iter().position(|&n| n == 0).unwrap())
                .unwrap()
                + i * 1000)
                % enc.len()]]
        })
        .sum()
}
