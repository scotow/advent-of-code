advent_of_code_2015::main!();

#[derive(Copy, Clone, Debug)]
struct Entity {
    hp: i16,
    damage: i16,
    armor: i16,
}

impl Entity {
    fn attack(&self, other: &mut Entity) {
        other.hp -= 1.max(self.damage - other.armor);
    }
}

const WEAPONS: [(i16, i16, i16); 5] = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
const ARMORS: [(i16, i16, i16); 6] = [
    (0, 0, 0),
    (13, 0, 1),
    (31, 0, 2),
    (53, 0, 3),
    (75, 0, 4),
    (102, 0, 5),
];
const RINGS: [(i16, i16, i16); 7] = [
    (0, 0, 0),
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (80, 0, 3),
    (40, 0, 2),
];

fn generator(input: &str) -> Entity {
    let (hp, damage, armor) = input
        .lines()
        .map(|l| l.split_once(": ").unwrap().1.parse().unwrap())
        .collect_tuple()
        .unwrap();
    Entity { hp, damage, armor }
}

fn part_1(boss: Entity) -> i16 {
    solve(boss).filter_map(|(w, g)| w.then(|| g)).min().unwrap()
}

fn part_2(boss: Entity) -> i16 {
    solve(boss)
        .filter_map(|(w, g)| (!w).then(|| g))
        .max()
        .unwrap()
}

fn solve(boss: Entity) -> impl Iterator<Item = (bool, i16)> {
    iproduct!(WEAPONS, ARMORS, RINGS, RINGS)
        .filter(|&(_, _, r1, r2)| r1 != r2 || r1 == (0, 0, 0))
        .map(move |(w, a, r1, r2)| {
            let player = Entity {
                hp: 100,
                damage: w.1 + r1.1 + r2.1,
                armor: a.2 + r1.2 + r2.2,
            };
            (play(player, boss), w.0 + a.0 + r1.0 + r2.0)
        })
}

fn play(mut player: Entity, mut boss: Entity) -> bool {
    loop {
        player.attack(&mut boss);
        if boss.hp <= 0 {
            return true;
        }

        boss.attack(&mut player);
        if player.hp <= 0 {
            return false;
        }
    }
}
