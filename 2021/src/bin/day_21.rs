use itertools::iproduct;
use std::collections::HashMap;

advent_of_code_2021::main!();

fn generator(input: &str) -> (usize, usize) {
    input
        .lines()
        .map(|l| l.rsplit_once(' ').unwrap().1.parse().unwrap())
        .collect_tuple()
        .unwrap()
}

fn part_1((p1, p2): (usize, usize)) -> usize {
    let mut players = [(p1, 0), (p2, 0)];
    let (mut dice, mut rolls) = (1, 0);
    while players[1].1 < 1000 {
        for _ in 0..3 {
            players[0].0 = (players[0].0 + dice - 1) % 10 + 1;
            dice += 1;
            if dice > 100 {
                dice = 1;
            }
        }
        rolls += 3;
        players[0].1 += players[0].0;
        players.rotate_left(1);
    }
    players[0].1 * rolls
}

fn part_2((p1, p2): (usize, usize)) -> usize {
    solve([(p1, 0), (p2, 0)], 0, &mut HashMap::new())
        .into_iter()
        .max()
        .unwrap()
}

fn solve(
    players: [(usize, usize); 2],
    playing: usize,
    visited: &mut HashMap<([(usize, usize); 2], usize), [usize; 2]>,
) -> [usize; 2] {
    if let Some(&previous) = visited.get(&(players, playing)) {
        return previous;
    }
    let mut wins = [0, 0];
    for (d1, d2, d3) in iproduct!(1..=3, 1..=3, 1..=3) {
        let mut players = players;
        players[playing].0 = (players[playing].0 + (d1 + d2 + d3) - 1) % 10 + 1;
        players[playing].1 += players[playing].0;
        if players[playing].1 >= 21 {
            wins[playing] += 1;
        } else {
            let [w1, w2] = solve(players, (playing + 1) % 2, visited);
            wins[0] += w1;
            wins[1] += w2;
        }
    }
    visited.insert((players, playing), wins);
    wins
}
