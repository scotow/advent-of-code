advent_of_code_2019::main!();

#[derive(Clone, Copy, Debug)]
enum Shuffle {
    Reverse,
    Cut(i64),
    Deal(i64),
}

fn generator(input: &str) -> Vec<Shuffle> {
    input
        .lines()
        .map(|l| {
            if l.ends_with("stack") {
                Shuffle::Reverse
            } else if l.starts_with("cut") {
                Shuffle::Cut(l[4..].parse().unwrap())
            } else {
                Shuffle::Deal(l[20..].parse().unwrap())
            }
        })
        .collect()
}

fn part_1(input: Vec<Shuffle>) -> usize {
    let mut deck = (0..10_007).collect_vec();
    for s in input {
        match s {
            Shuffle::Reverse => deck.reverse(),
            Shuffle::Cut(mut n) => {
                if n < 0 {
                    n = deck.len() as i64 - n.abs();
                }
                let mut end = deck.split_off(n as usize);
                end.append(&mut deck);
                deck = end;
            }
            Shuffle::Deal(n) => {
                let mut next = vec![0; deck.len()];
                for (i, &c) in deck.iter().enumerate() {
                    next[i * n as usize % deck.len()] = c;
                }
                deck = next;
            }
        }
    }
    deck.into_iter().position(|c| c == 2019).unwrap()
}

fn part_2(input: Vec<Shuffle>) -> i128 {
    let m = 119_315_717_514_047;
    let mut a = 1;
    let mut b = 0;
    for s in input.into_iter().rev() {
        match s {
            Shuffle::Reverse => {
                a = a * -1;
                b = b * -1 - 1;
            }
            Shuffle::Cut(n) => {
                b = (b + n as i128) % m;
            }
            Shuffle::Deal(n) => {
                a = (a * mod_inverse(n as i128, m)) % m;
                b = (b * mod_inverse(n as i128, m)) % m;
            }
        };
    }
    let n = 101_741_582_076_661;
    (2020 * mod_e(a, n, m) % m + (b * ((mod_e(a, n, m) - 1) * mod_inverse(a - 1, m) % m))) % m
}

fn mod_e(a: i128, mut e: i128, m: i128) -> i128 {
    let mut result = 1;
    let mut base = a % m;
    loop {
        if e <= 0 {
            break;
        }
        if e % 2 == 1 {
            result = (result * base) % m;
        }
        e >>= 1;
        base = (base * base) % m;
    }
    result
}

fn mod_inverse(a: i128, m: i128) -> i128 {
    mod_e(a, m - 2, m)
}
