advent_of_code_2022::main!();

#[derive(Default, Clone)]
struct Dir<'a> {
    files_size: u32,
    dirs: HashMap<&'a str, Dir<'a>>,
}

impl<'a> Dir<'a> {
    fn get_dir(&mut self, path: &[&'a str]) -> &mut Dir<'a> {
        if path.is_empty() {
            self
        } else {
            self.dirs.get_mut(path[0]).unwrap().get_dir(&path[1..])
        }
    }

    fn size(&self) -> u32 {
        self.files_size + self.dirs.values().map(Dir::size).sum::<u32>()
    }

    fn dirs(&self) -> impl Iterator<Item = &Dir<'a>> {
        once(self).chain(self.dirs.values().flat_map(|d| d.dirs().collect_vec()))
    }
}

fn generator(input: &str) -> Dir {
    let mut root = Dir::default();
    let mut pwd = Vec::new();
    for l in input.lines() {
        match l.split_whitespace().chain(once("")).next_tuple().unwrap() {
            ("$", "cd", "/") => pwd.clear(),
            ("$", "cd", "..") => drop(pwd.pop()),
            ("$", "cd", t) => pwd.push(t),
            ("$", "ls", _) => (),
            ("dir", n, _) => drop(root.get_dir(&pwd).dirs.insert(n, Dir::default())),
            (s, _, _) => root.get_dir(&pwd).files_size += s.parse::<u32>().unwrap(),
        }
    }
    root
}

fn part_1(root: Dir) -> u32 {
    root.dirs().map(|d| d.size()).filter(|&s| s < 100_000).sum()
}

fn part_2(root: Dir) -> u32 {
    let used = root.size();
    root.dirs()
        .map(|d| d.size())
        .filter(|&s| used - s < 40_000_000)
        .min()
        .unwrap()
}
