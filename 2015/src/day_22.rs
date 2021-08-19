use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Wizard {
    hp: i16,
    mana: i16,
    mana_used: i16,
    shield: i16,
    poison: i16,
    recharge: i16,
}

impl Wizard {
    fn new() -> Self {
        Wizard {
            hp: 50,
            mana: 500,
            mana_used: 0,
            shield: 0,
            poison: 0,
            recharge: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Boss {
    hp: i16,
    damage: i16,
}

impl Boss {
    fn attack(&self, wizard: &mut Wizard) {
        if wizard.shield >= 1 {
            wizard.hp -= 1.max(self.damage - 7);
        } else {
            wizard.hp -= self.damage;
        }
    }
}

fn missile(wizard: &mut Wizard, boss: &mut Boss) {
    wizard.mana -= 53;
    wizard.mana_used += 53;
    boss.hp -= 4;
}

fn drain(wizard: &mut Wizard, boss: &mut Boss) {
    wizard.mana -= 73;
    wizard.mana_used += 73;
    boss.hp -= 2;
    wizard.hp += 2;
}

fn shield(wizard: &mut Wizard, _: &mut Boss) {
    wizard.mana -= 113;
    wizard.mana_used += 113;
    wizard.shield = 6;
}

fn poison(wizard: &mut Wizard, _: &mut Boss) {
    wizard.mana -= 173;
    wizard.mana_used += 173;
    wizard.poison = 6;
}

fn recharge(wizard: &mut Wizard, _: &mut Boss) {
    wizard.mana -= 229;
    wizard.mana_used += 229;
    wizard.recharge = 5;
}

fn apply_spells(wizard: &mut Wizard, boss: &mut Boss) -> bool {
    wizard.shield = (wizard.shield - 1).max(0);
    if wizard.poison >= 1 {
        boss.hp -= 3;
        if boss.hp <= 0 {
            return true;
        }
        wizard.poison -= 1;
    }
    if wizard.recharge >= 1 {
        wizard.mana += 101;
        wizard.recharge -= 1;
    }
    false
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Boss {
    let (hp, damage) = input.lines()
        .map(|l| l.split_once(": ").unwrap().1.parse().unwrap())
        .collect_tuple().unwrap();
    Boss { hp, damage }
}

#[aoc(day22, part1)]
pub fn part1(boss: &Boss) -> i16 {
    play(Wizard::new(), boss.clone(), false).unwrap()
}

#[aoc(day22, part2)]
pub fn part2(boss: &Boss) -> i16 {
    play(Wizard::new(), boss.clone(), true).unwrap()
}

fn play(mut wizard: Wizard, mut boss: Boss, hard: bool) -> Option<i16> {
    if hard {
        wizard.hp -= 1;
        if wizard.hp == 0 {
            return None;
        }
    }

    if apply_spells(&mut wizard, &mut boss) {
        return Some(wizard.mana_used);
    }

    let mut spells: Vec<fn(&mut Wizard, &mut Boss)> = Vec::new();
    if wizard.mana >= 53 {
        spells.push(missile);
    }
    if wizard.mana >= 73 {
        spells.push(drain);
    }
    if wizard.shield == 0 && wizard.mana >= 113 {
        spells.push(shield);
    }
    if wizard.poison == 0 && wizard.mana >= 173 {
        spells.push(poison);
    }
    if wizard.recharge == 0 && wizard.mana >= 229 {
        spells.push(recharge);
    }

    if spells.is_empty() {
        return None;
    }
    spells.iter()
        .filter_map(|spell| {
            let mut wizard = wizard.clone();
            let mut boss = boss.clone();
            spell(&mut wizard, &mut boss);

            if boss.hp <= 0 {
                return Some(wizard.mana_used);
            }

            if apply_spells(&mut wizard, &mut boss) {
                return Some(wizard.mana_used);
            }

            boss.attack(&mut wizard);
            if wizard.hp <= 0 {
                return None;
            }

            play(wizard, boss, hard)
        })
        .min()
}
