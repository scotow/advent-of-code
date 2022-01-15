advent_of_code_2016::main!();

fn generator(input: &str) -> usize {
    input.parse().unwrap()
}

fn part_1(input: usize) -> usize {
    pathfinding::prelude::dijkstra(
        &(1, 1),
        |&(x, y)| neighbors(x, y, input).into_iter().map(|xy| (xy, 1)),
        |&xy| xy == (31, 39),
    )
    .unwrap()
    .1
}

fn part_2(input: usize) -> usize {
    for y in 0..20 {
        for x in 0..20 {
            print!("{}", if is_wall(x, y, input) { '#' } else { '.' })
        }
        println!();
    }

    println!();
    println!();

    let mut visited = HashSet::new();
    visit(1, 1, input, &mut visited, 0);

    for y in 0..20 {
        for x in 0..20 {
            print!(
                "{}",
                match (is_wall(x, y, input), visited.contains(&(x, y))) {
                    (true, true) => 'X',
                    (false, true) => '0',
                    (true, false) => '#',
                    (false, false) => '.',
                }
            );
            // print!("{}", if  { '' } else { '.' })
        }
        println!();
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

fn visit(x: usize, y: usize, fav: usize, visited: &mut HashSet<(usize, usize)>, fuel: usize) {
    if !visited.insert((x, y)) {
        return;
    }
    if fuel == 14 {
        return;
    }
    for (x, y) in neighbors(x, y, fav) {
        visit(x, y, fav, visited, fuel + 1);
    }
}
