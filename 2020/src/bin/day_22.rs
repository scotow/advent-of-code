advent_of_code_2020::main!();

type Deck = Vec<u16>;

fn generator(input: &str) -> (Deck, Deck) {
    fn parse_deck(s: &str) -> Deck {
        s.lines().skip(1).map(|l| l.parse().unwrap()).collect()
    }

    input.split("\n\n").map(parse_deck).collect_tuple().unwrap()
}

fn next(d1: &mut Deck, d2: &mut Deck) {
    let (c1, c2) = (d1.remove(0), d2.remove(0));
    if c1 > c2 {
        d1.append(&mut vec![c1, c2])
    } else {
        d2.append(&mut vec![c2, c1])
    }
}

fn score(d: &Deck) -> u16 {
    d.iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i as u16 + 1) * c)
        .sum()
}

fn hash(d1: &Deck, d2: &Deck) -> u64 {
    let mut hasher = DefaultHasher::new();
    d1.hash(&mut hasher);
    d2.hash(&mut hasher);
    hasher.finish()
}

#[derive(Copy, Clone, PartialEq)]
enum Player {
    One,
    Two,
}

fn run(d1: &mut Deck, d2: &mut Deck) -> Player {
    let mut cache = HashSet::new();

    while !d1.is_empty() && !d2.is_empty() {
        if !cache.insert(hash(&d1, &d2)) {
            return Player::One;
        }

        let (c1, c2) = (d1.remove(0), d2.remove(0));
        let w: Player;
        if c1 <= d1.len() as u16 && c2 <= d2.len() as u16 {
            w = run(
                &mut d1[..c1 as usize].to_vec(),
                &mut d2[..c2 as usize].to_vec(),
            );
        } else {
            w = if c1 > c2 { Player::One } else { Player::Two }
        }
        if w == Player::One {
            d1.append(&mut vec![c1, c2]);
        } else {
            d2.append(&mut vec![c2, c1]);
        }
    }

    if d2.is_empty() {
        Player::One
    } else {
        Player::Two
    }
}

fn part_1(input: (Deck, Deck)) -> u16 {
    let (mut d1, mut d2) = (input.0.clone(), input.1.clone());
    while !d1.is_empty() && !d2.is_empty() {
        next(&mut d1, &mut d2);
    }

    score(if d1.is_empty() { &d2 } else { &d1 })
}

fn part_2(input: (Deck, Deck)) -> u16 {
    let (mut d1, mut d2) = (input.0.clone(), input.1.clone());
    let w = run(&mut d1, &mut d2);

    score(if w == Player::One { &d1 } else { &d2 })
}
