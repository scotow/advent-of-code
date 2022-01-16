use pathfinding::prelude::dijkstra;

advent_of_code_2016::main!();

fn generator(input: &str) -> usize {
    input.parse().unwrap()
}

fn part_1(input: usize) -> usize {
    dijkstra(
        &(1, 1),
        |&(x, y)| neighbors(x, y, input).into_iter().map(|xy| (xy, 1)),
        |&xy| xy == (31, 39),
    )
    .unwrap()
    .1
}

fn part_2(input: usize) -> usize {
    let mut visited = HashSet::new();
    let mut queue = HashSet::new();
    queue.insert((1, 1));
    for _ in 0..=50 {
        let next_queue = queue
            .iter()
            .flat_map(|&(x, y)| neighbors(x, y, input))
            .filter(|xy| !visited.contains(xy))
            .collect();
        visited.extend(queue);
        queue = next_queue;
    }
    visited.len()
}

fn is_wall(x: usize, y: usize, fav: usize) -> bool {
    ((x * x + 3 * x + 2 * x * y + y + y * y) + fav).count_ones() & 1 == 1
}

fn neighbors(x: usize, y: usize, fav: usize) -> Vec<(usize, usize)> {
    let mut next = vec![(x + 1, y), (x, y + 1)];
    if x >= 1 {
        next.push((x - 1, y));
    }
    if y >= 1 {
        next.push((x, y - 1));
    }
    next.retain(|&(x, y)| !is_wall(x, y, fav));
    next
}
