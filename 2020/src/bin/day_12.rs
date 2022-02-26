advent_of_code_2020::main!();

trait Ship {
    fn new() -> Self;
    fn pos(&self) -> (i32, i32);
    fn apply(&mut self, mov: Move);

    fn manhattan_distance(&self) -> i32 {
        let pos = self.pos();
        pos.0.abs() + pos.1.abs()
    }
}

#[derive(Copy, Clone, Debug)]
struct SimpleShip {
    pos: (i32, i32),
    dir: Action,
}

impl Ship for SimpleShip {
    fn new() -> Self {
        Self {
            pos: (0, 0),
            dir: Action::East,
        }
    }

    fn pos(&self) -> (i32, i32) {
        self.pos
    }

    fn apply(&mut self, mov: Move) {
        match mov.action {
            Action::North => self.pos.1 += mov.value as i32,
            Action::South => self.pos.1 -= mov.value as i32,
            Action::East => self.pos.0 += mov.value as i32,
            Action::West => self.pos.0 -= mov.value as i32,
            Action::Left | Action::Right => {
                self.dir = Action::from_degree(self.dir.to_degree() + mov.to_degree())
            }
            Action::Forward => self.apply(Move {
                action: self.dir,
                value: mov.value,
            }),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct WaypointShip {
    pos: (i32, i32),
    way_point: (i32, i32),
}

impl Ship for WaypointShip {
    fn new() -> Self {
        Self {
            pos: (0, 0),
            way_point: (10, 1),
        }
    }

    fn pos(&self) -> (i32, i32) {
        self.pos
    }

    fn apply(&mut self, mov: Move) {
        match mov.action {
            Action::North => self.way_point.1 += mov.value as i32,
            Action::South => self.way_point.1 -= mov.value as i32,
            Action::East => self.way_point.0 += mov.value as i32,
            Action::West => self.way_point.0 -= mov.value as i32,
            Action::Left | Action::Right => match mov.to_degree() {
                90 => self.way_point = (-self.way_point.1, self.way_point.0),
                180 => self.way_point = (-self.way_point.0, -self.way_point.1),
                270 => self.way_point = (self.way_point.1, -self.way_point.0),
                _ => unreachable!(),
            },
            Action::Forward => {
                self.pos = (
                    self.pos.0 + self.way_point.0 * mov.value as i32,
                    self.pos.1 + self.way_point.1 * mov.value as i32,
                )
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Move {
    action: Action,
    value: u16,
}

impl Move {
    fn to_degree(self) -> u16 {
        match self.action {
            Action::Left => self.value,
            Action::Right => (360 - self.value % 360),
            _ => unreachable!(),
        }
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            action: s[..1].parse().unwrap(),
            value: s[1..].parse().unwrap(),
        })
    }
}

#[derive(Copy, Clone, Debug)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "N" => Action::North,
            "S" => Action::South,
            "E" => Action::East,
            "W" => Action::West,
            "L" => Action::Left,
            "R" => Action::Right,
            "F" => Action::Forward,
            _ => unreachable!(),
        })
    }
}

impl Action {
    fn from_degree(deg: u16) -> Self {
        match deg % 360 {
            0 => Action::East,
            90 => Action::North,
            180 => Action::West,
            270 => Action::South,
            _ => unreachable!(),
        }
    }

    fn to_degree(self) -> u16 {
        match self {
            Action::North => 90,
            Action::South => 270,
            Action::East => 0,
            Action::West => 180,
            _ => unreachable!(),
        }
    }
}

fn generator(input: &str) -> Vec<Move> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part_1(input: Vec<Move>) -> i32 {
    solve::<SimpleShip>(&input)
}

fn part_2(input: Vec<Move>) -> i32 {
    solve::<WaypointShip>(&input)
}

fn solve<S: Ship>(moves: &[Move]) -> i32 {
    let mut ship = S::new();
    moves.iter().for_each(|&m| ship.apply(m));

    ship.manhattan_distance()
}
