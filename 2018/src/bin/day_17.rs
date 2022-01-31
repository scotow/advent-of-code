advent_of_code_2018::main!();

type Grid = HashMap<(usize, usize), Cell>;

#[derive(Copy, Clone, Debug)]
enum Cell {
    Wall,
    Empty,
    Water,
    Flow,
}

fn generator(input: &str) -> Grid {
    input
        .lines()
        .flat_map(|l| {
            let n = l
                .split(['=', ',', '.'])
                .filter_map(|p| p.parse::<usize>().ok())
                .collect_vec();
            if l.starts_with("x") {
                iproduct!(n[0]..=n[0], n[1]..=n[2])
            } else {
                iproduct!(n[1]..=n[2], n[0]..=n[0])
            }
        })
        .map(|xy| (xy, Cell::Wall))
        .collect()
}

fn part_1(mut grid: Grid) -> usize {
    let y_max = grid.iter().map(|(&(_, y), _)| y).max().unwrap();
    // grid.insert((500, 0), Cell::Flow);
    print_grid(&grid);
    let mut sources = vec![(500, 0)];
    while !sources.is_empty() {
        let (xs, ys) = sources.remove(0);
        sources.extend(run_source(&mut grid, xs, ys, y_max));
    }
    print_grid(&grid);
    grid.into_values()
        .filter(|&c| matches!(c, Cell::Water | Cell::Flow))
        .count()
}

fn part_2(input: Grid) -> u8 {
    0
}

fn run_source(grid: &mut Grid, xs: usize, mut ys: usize, y_max: usize) -> Vec<(usize, usize)> {
    loop {
        // dbg!("here");
        let bot = (ys + 1..)
            .find(|&y| grid.contains_key(&(xs, y + 1)) || y == y_max)
            .unwrap();
        if bot == y_max {
            for y in ys + 1..=y_max {
                grid.insert((xs, y), Cell::Flow);
            }
            return Vec::new();
        }
        dbg!(bot);
        let to_fill = (xs..)
            .take_while(|&x| !grid.contains_key(&(x, bot)) && grid.contains_key(&(x - 1, bot + 1)))
            .chain((0..xs).rev().take_while(|&x| {
                !grid.contains_key(&(x, bot)) && grid.contains_key(&(x + 1, bot + 1))
            }))
            .collect_vec();
        let sources = to_fill
            .iter()
            .filter(|&&x| !grid.contains_key(&(x, bot + 1)))
            .map(|&x| (x, bot))
            .collect_vec();
        let kind = if sources.is_empty() {
            Cell::Water
        } else {
            Cell::Flow
        };
        for x in to_fill {
            grid.insert((x, bot), kind);
        }
        print_grid(&grid);
        std::thread::sleep(std::time::Duration::from_millis(500));
        if !sources.is_empty() {
            for y in ys + 1..bot {
                grid.insert((xs, y), Cell::Flow);
            }
            return sources;
        }
    }
}

fn print_grid(grid: &Grid) {
    print!("{}[2J", 27 as char);
    let (xm, xn) = grid
        .into_iter()
        .map(|(&(x, _), _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let yn = grid.into_iter().map(|(&(_, y), _)| y).max().unwrap();
    for y in 0..=300 {
        println!(
            "{}",
            (xm..=xn)
                .map(|x| match grid.get(&(x, y)).unwrap_or(&Cell::Empty) {
                    Cell::Wall => '#',
                    Cell::Empty => ' ',
                    Cell::Water => '~',
                    Cell::Flow => '|',
                })
                .collect::<String>()
        );
    }
    println!();
}
