#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.as_bytes().iter().map(|&c| (c - b'0') as u32).collect()
}

fn solve(circle: &mut Vec<u32>, moves: usize) {
    let mut current = circle[0];
    for _ in 0..moves {
        let mut picked = Vec::with_capacity(3);
        for _ in 0..3 {
            let current_index = circle.iter().position(|&c| c == current).unwrap();
            picked.push(circle.remove((current_index + 1) % circle.len()));
        }

        let mut destination = current - 1;
        loop {
            if circle.contains(&destination) {
                break;
            }
            if destination == 0 {
                destination = 10;
            }
            destination -= 1;
        };

        let destination_index = circle.iter().position(|&c| c == destination).unwrap();
        for (i, c) in picked.into_iter().enumerate() {
            circle.insert(destination_index + 1 + i, c);
        }

        let current_index = circle.iter().position(|&c| c == current).unwrap();
        current = circle[(current_index + 1) % circle.len()];
    }
}

#[aoc(day23, part1)]
pub fn part1(input: &Vec<u32>) -> String {
    let mut circle = input.clone();
    solve(&mut circle, 100);
    let one_index = circle.iter().position(|&c| c == 1).unwrap();
    let (p2, p1) = circle.split_at(one_index);
    String::from_utf8([p1, p2].concat()[1..].iter().map(|&c| b'0' + c as u8).collect()).unwrap()
}

#[aoc(day23, part2)]
pub fn part2(input: &Vec<u32>) -> u64 {
    let mut circle = input.clone();
    circle.reserve(1_000_000);
    let max = *circle.iter().max().unwrap();
    for c in max + 1..=1_000_000 {
        circle.push(c);
    }
    solve(&mut circle, 10_000_000);
    let one_index = circle.iter().position(|&c| c == 1).unwrap();
    circle[one_index + 1] as u64 * circle[one_index + 2] as u64
}