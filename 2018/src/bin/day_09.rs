advent_of_code_2018::main!();

fn generator(input: &str) -> (usize, usize) {
    input
        .split_whitespace()
        .flat_map(|n| n.parse().ok())
        .collect_tuple()
        .unwrap()
}

fn part_1((players, last): (usize, usize)) -> usize {
    solve2(players, last)
}

fn part_2((players, last): (usize, usize)) -> usize {
    solve2(players, last * 100)
}

fn solve2(players: usize, last: usize) -> usize {
    let mut ring = VecDeque::from([0, 2, 1]);
    let mut scores = vec![0; players];
    let mut player = 0;
    for marble in 3..=last {
        if marble % 23 == 0 {
            ring.rotate_right(7);
            scores[player] += marble + ring.pop_back().unwrap();
            ring.rotate_left(1);
        } else {
            ring.rotate_left(1);
            ring.push_back(marble);
        }
        player = (player + 1) % players;
    }
    scores.into_iter().max().unwrap()
}
