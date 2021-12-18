use std::ops::Add;

advent_of_code_2021::main!();

fn generator(input: &str) -> Vec<Elem> {
    input
        .lines()
        .map(|l| Elem::new(l.as_bytes(), &mut 0))
        .collect()
}

fn part_1(input: Vec<Elem>) -> u16 {
    input.into_iter().reduce(Add::add).unwrap().magnitude()
}

fn part_2(input: Vec<Elem>) -> u16 {
    input
        .into_iter()
        .permutations(2)
        .map(|es| es.into_iter().reduce(Add::add).unwrap().magnitude())
        .max()
        .unwrap()
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Elem {
    Num(u16),
    Pair(Box<Elem>, Box<Elem>),
}

impl Elem {
    fn new(input: &[u8], ptr: &mut usize) -> Self {
        if input[*ptr] == b'[' {
            *ptr += 1; // Opening bracket.
            let e1 = Self::new(input, ptr);
            *ptr += 1; // Comma.
            let e2 = Self::new(input, ptr);
            *ptr += 1; // Closing bracket.
            Self::Pair(Box::new(e1), Box::new(e2))
        } else {
            *ptr += 1; // Single digit number only.
            Self::Num((input[*ptr - 1] - b'0') as u16)
        }
    }

    fn find_explode(&mut self, i: &mut u8, depth: u8) -> Option<Elem> {
        if depth >= 4 && self.is_double_num() {
            Some(std::mem::replace(self, Elem::Num(0)))
        } else if matches!(self, Self::Num(_)) {
            *i += 1;
            None
        } else {
            if let Elem::Pair(e1, e2) = self {
                e1.find_explode(i, depth + 1)
                    .or_else(|| e2.find_explode(i, depth + 1))
            } else {
                None
            }
        }
    }

    fn is_double_num(&self) -> bool {
        if let Elem::Pair(e1, e2) = self {
            return matches!((e1.as_ref(), e2.as_ref()), (Elem::Num(_), Elem::Num(_)));
        }
        false
    }

    fn find(&mut self, i: &mut u8) -> Option<&mut u16> {
        match self {
            Elem::Num(n) => {
                if *i == 0 {
                    Some(n)
                } else {
                    *i -= 1;
                    None
                }
            }
            Elem::Pair(e1, e2) => e1.find(i).or_else(|| e2.find(i)),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Elem::Num(n) => {
                if *n >= 10 {
                    *self = Elem::Pair(
                        Box::new(Elem::Num(*n / 2)),
                        Box::new(Elem::Num(*n - *n / 2)),
                    );
                    return true;
                }
                false
            }
            Elem::Pair(e1, e2) => e1.split() || e2.split(),
        }
    }

    fn reduce(&mut self) {
        loop {
            let mut i = 0;
            if let Some(replaced) = self.find_explode(&mut i, 0) {
                let (n1, n2) = match replaced {
                    Elem::Pair(e1, e2) => match (e1.as_ref(), e2.as_ref()) {
                        (&Elem::Num(n1), &Elem::Num(n2)) => (n1, n2),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                };
                if i > 0 {
                    *self.find(&mut (i - 1)).unwrap() += n1;
                }
                if let Some(e2) = self.find(&mut (i + 1)) {
                    *e2 += n2;
                }
            } else if !self.split() {
                break;
            }
        }
    }

    fn magnitude(&self) -> u16 {
        match self {
            Elem::Num(n) => *n,
            Elem::Pair(e1, e2) => 3 * e1.magnitude() + 2 * e2.magnitude(),
        }
    }
}

impl Add for Elem {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = Elem::Pair(Box::new(self), Box::new(rhs));
        sum.reduce();
        sum
    }
}
