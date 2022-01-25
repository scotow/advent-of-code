advent_of_code_2018::main!();

fn generator(input: &str) -> (usize, usize) {
    input
        .split_whitespace()
        .flat_map(|n| n.parse().ok())
        .collect_tuple()
        .unwrap()
}

fn part_1((players, last): (usize, usize)) -> usize {
    solve(players, last)
}

fn part_2((players, last): (usize, usize)) -> usize {
    solve(players, last * 100)
}

fn solve(players: usize, last: usize) -> usize {
    let mut ring = vec![0, 2, 1];
    let mut index = 1;
    let mut scores = vec![0; players];
    let mut player = 0;
    for marble in 3..=last {
        if marble % 23 == 0 {
            index = (index + ring.len() - 7) % ring.len();
            scores[player] += marble + ring.remove(index);
        } else {
            index += 2;
            if index > ring.len() {
                index -= ring.len();
            }
            ring.insert(index, marble);
        }
        if marble % 100_000 == 0 {
            println!("{}", marble);
        }
        player = (player + 1) % players;
    }
    scores.into_iter().max().unwrap()
}
