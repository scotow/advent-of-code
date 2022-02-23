advent_of_code_2019::main!();

type Map = HashMap<String, (Room, Vec<String>)>;

#[derive(Clone, Debug)]
struct Room {
    doors: Vec<String>,
    item: Option<String>,
    prog: Program,
}

impl Room {
    fn parse(prog: &mut Program) -> (Self, String) {
        let info = pull_string(prog);
        let name = info.lines().next().unwrap().trim_matches(&['=', ' '][..]);
        let mut doors = info
            .lines()
            .filter(|l| l.starts_with("- "))
            .map(|l| l.trim_start_matches("- ").to_owned())
            .collect_vec();
        let item = info.contains("Items here:").then(|| doors.pop().unwrap());
        (
            Self {
                doors,
                item,
                prog: prog.clone(),
            },
            name.to_owned(),
        )
    }
}

fn generator(input: &str) -> Program {
    Program::from_str(input)
}

fn part_1(mut prog: Program) -> i64 {
    prog.run();
    let (start_room, start_name) = Room::parse(&mut prog);
    let mut map = HashMap::from([(start_name, (start_room, Vec::new()))]);
    loop {
        let mut next = map.clone();
        for (_, (room, path)) in map.iter().filter(|(n, _)| n != &"Pressure-Sensitive Floor") {
            for door in &room.doors {
                let mut prog = room.prog.clone();
                run_cmd(&mut prog, door, true, false);
                let mut path = path.clone();
                path.push(door.clone());
                let (room, name) = Room::parse(&mut prog);
                if !next.contains_key(&name) {
                    next.insert(name, (room, path));
                }
            }
        }
        if next.len() == map.len() {
            break;
        }
        map = next;
    }

    let items = valid_items(&map);
    let (last, common) = map["Pressure-Sensitive Floor"].1.split_last().unwrap();
    assert_eq!(&map["Security Checkpoint"].1, common);
    map.values()
        .filter(|(room, _)| {
            room.item
                .as_ref()
                .map(|item| items.contains(&item))
                .unwrap_or(false)
        })
        .flat_map(|(room, path)| {
            chain!(
                path.into_iter().cloned(),
                once(format!("take {}", room.item.as_ref().unwrap())),
                reverse_path(path),
            )
        })
        .chain(common.to_vec())
        .chain(items.iter().map(|i| format!("drop {}", i)))
        .for_each(|cmd| {
            run_cmd(&mut prog, cmd, true, true);
        });

    items
        .iter()
        .powerset()
        .map(|carrying| {
            let mut prog = prog.clone();
            for item in carrying {
                run_cmd(&mut prog, format!("take {}", item), true, true);
            }
            run_cmd(&mut prog, last, true, false);
            pull_string(&mut prog)
        })
        .find(|resp| !resp.contains("lighter") && !resp.contains("heavier"))
        .map(|resp| {
            resp.split_whitespace()
                .filter_map(|w| w.parse().ok())
                .next()
                .unwrap()
        })
        .unwrap()
}

fn part_2(_: Program) -> &'static str {
    "N/A"
}

fn run_cmd(prog: &mut Program, cmd: impl AsRef<str>, run: bool, clear: bool) {
    prog.push_multiple(cmd.as_ref().bytes().chain(once(b'\n')).map_into());
    if run {
        prog.run();
        if clear {
            prog.pull_all();
        }
    }
}

fn pull_string(prog: &mut Program) -> String {
    prog.pull_all()
        .into_iter()
        .map(|c| c as u8 as char)
        .skip_while(|c| c.is_whitespace())
        .collect()
}

fn reverse_path(path: &[String]) -> Vec<String> {
    path.into_iter()
        .rev()
        .map(|d| {
            match d.as_ref() {
                "north" => "south",
                "east" => "west",
                "south" => "north",
                "west" => "east",
                _ => unreachable!(),
            }
            .to_owned()
        })
        .collect()
}

fn valid_items(map: &Map) -> Vec<String> {
    map.values()
        .filter_map(|(r, _)| r.item.as_ref())
        .filter(|item| {
            ![
                "photons",
                "escape pod",
                "molten lava",
                "infinite loop",
                "giant electromagnet",
            ]
            .contains(&item.as_ref())
        })
        .cloned()
        .collect()
}
