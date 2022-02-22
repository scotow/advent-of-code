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

fn part_2(input: Vec<Shuffle>) -> u8 {
    0
}
