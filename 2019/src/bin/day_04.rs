advent_of_code_2019::main!();

type Pass = [u8; 6];

fn generator(input: &str) -> (Pass, Pass) {
    input
        .split('-')
        .map(|n| n.as_bytes().to_vec().try_into().unwrap())
        .collect_tuple()
        .unwrap()
}

fn part_1((a, b): (Pass, Pass)) -> usize {
    successors(Some(a), |&n| Some(next(n)))
        .take_while(|&p| p <= b)
        .filter(|p| p.windows(2).any(|w| w[0] == w[1]) && p.windows(2).all(|w| w[0] <= w[1]))
        .count()
}

fn part_2((a, b): (Pass, Pass)) -> usize {
    successors(Some(a), |&n| Some(next(n)))
        .take_while(|&p| p <= b)
        .filter(|p| {
            chain!(once(42), *p, once(42))
                .tuple_windows()
                .any(|(a, b, c, d)| b == c && a != b && d != c)
                && p.windows(2).all(|w| w[0] <= w[1])
        })
        .count()
}

fn next(p: Pass) -> Pass {
    fn inc(mut p: Pass, i: usize) -> Pass {
        if p[i] == b'9' {
            p[i] = b'0';
            inc(p, i - 1)
        } else {
            p[i] += 1;
            p
        }
    }
    inc(p, 5)
}
