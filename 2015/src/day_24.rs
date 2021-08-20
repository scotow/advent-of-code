#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<u16> {
    input.lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

#[aoc(day24, part1)]
pub fn part1(input: &[u16]) -> u16 {
    let group_weight = input.iter().sum::<u16>() / 3;
    let input = input.to_vec();
    // input.reverse();
    solve(group_weight, input, Vec::new(), Vec::new(), Vec::new()).unwrap().1
}

fn solve(group_weight: u16, mut r: Vec<u16>, g1: Vec<u16>, g2: Vec<u16>, g3: Vec<u16>) -> Option<(usize, u16)> {
    if r.is_empty() {
        if g1.iter().sum::<u16>() == group_weight && g2.iter().sum::<u16>() == group_weight && g3.iter().sum::<u16>() == group_weight {
            return Some((g1.len(), g1.iter().product()));
        } else {
            return None;
        }
    }

    if g1.iter().sum::<u16>() > group_weight || g2.iter().sum::<u16>() > group_weight || g3.iter().sum::<u16>() > group_weight {
        return None;
    }

    let gift = r.pop().unwrap();
    [
        {
            let mut g1 = g1.clone();
            g1.push(gift);
            solve(group_weight, r.clone(), g1, g2.clone(), g3.clone())
        },
        {
            let mut g2 = g2.clone();
            g2.push(gift);
            solve(group_weight, r.clone(), g1.clone(), g2, g3.clone())
        },
        {
            let mut g3 = g3.clone();
            g3.push(gift);
            solve(group_weight, r.clone(), g1.clone(), g2.clone(), g3)
        }
    ].iter().filter_map(|&c| c).min()
}