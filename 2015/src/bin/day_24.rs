advent_of_code_2015::main!();

fn generator(input: &str) -> HashSet<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_1(input: HashSet<u64>) -> u64 {
    solve(input, 3)
}

fn part_2(input: HashSet<u64>) -> u64 {
    solve(input, 4)
}

fn solve(input: HashSet<u64>, split: u64) -> u64 {
    let group_weight = input.iter().sum::<u64>() / split;
    for n in 1..=input.len() {
        let groups = input
            .iter()
            .copied()
            .combinations(n)
            .filter(|g| g.iter().sum::<u64>() == group_weight)
            .filter(|g| can_form_others(&input, g, group_weight))
            .collect_vec();
        if !groups.is_empty() {
            return groups.iter().map(|g| g.iter().product()).min().unwrap();
        }
    }
    unreachable!();
}

// This verification doesn't seem necessary because every input given doesn't need to form g2/g3/g4 for its best quantum entanglement.
fn can_form_others(all: &HashSet<u64>, used: &[u64], group_weight: u64) -> bool {
    let mut rem = all.clone();
    for g in used {
        rem.remove(g);
    }
    if rem.is_empty() {
        return true;
    }

    for n in 1..=rem.len() {
        if rem
            .iter()
            .copied()
            .combinations(n)
            .filter(|g| g.iter().sum::<u64>() == group_weight)
            .any(|g| can_form_others(&rem, &g, group_weight))
        {
            return true;
        }
    }
    false
}
