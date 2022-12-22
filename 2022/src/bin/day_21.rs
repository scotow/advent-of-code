advent_of_code_2022::main!();

type Tree<'a> = HashMap<&'a str, Either<i64, (&'a str, &'a str, fn(i64, i64) -> i64)>>;

fn generator(input: &str) -> Tree {
    input
        .lines()
        .map(|l| {
            (
                &l[0..4],
                l[6..].parse().map(|n| Either::Left(n)).unwrap_or_else(|_| {
                    Either::Right((
                        &l[6..10],
                        &l[13..17],
                        match l.as_bytes()[11] {
                            b'+' => i64::add,
                            b'-' => i64::sub,
                            b'*' => i64::mul,
                            b'/' => i64::div,
                            _ => unreachable!(),
                        },
                    ))
                }),
            )
        })
        .collect()
}

fn part_1(tree: Tree) -> i64 {
    resolve(&tree, "root")
}

fn part_2(mut tree: Tree) -> i64 {
    let (left, right, _) = tree["root"].unwrap_right();
    let (mut target, mut to_resolve) = if contains(&tree, left, "humn") {
        (resolve(&tree, right), left)
    } else {
        (resolve(&tree, left), right)
    };

    while to_resolve != "humn" {
        let (a, b, ops) = tree[to_resolve].unwrap_right();
        if a == "humn" || contains(&tree, a, "humn") {
            to_resolve = a;
            let b = resolve(&tree, b);
            if ops == i64::div {
                target *= b;
            } else if ops == i64::mul {
                target /= b;
            } else if ops == i64::add {
                target -= b;
            } else if ops == i64::sub {
                target += b;
            } else {
                unreachable!();
            }
        } else {
            if ops == i64::div {
                tree.insert(to_resolve, Either::Right((a, b, i64::mul)));
                target = resolve(&tree, a);
            } else if ops == i64::mul {
                target /= resolve(&tree, a);
                to_resolve = b;
            } else if ops == i64::add {
                target -= resolve(&tree, a);
                to_resolve = b;
            } else if ops == i64::sub {
                target = -target + resolve(&tree, a);
                to_resolve = b;
            } else {
                unreachable!();
            }
        };
    }
    target
}

fn resolve(tree: &Tree, key: &str) -> i64 {
    match tree[key] {
        Either::Left(n) => n,
        Either::Right((k1, k2, op)) => op(resolve(tree, k1), resolve(tree, k2)),
    }
}

fn contains(tree: &Tree, root: &str, key: &str) -> bool {
    match tree[root] {
        Either::Left(_) => false,
        Either::Right((s1, s2, _)) => {
            s1 == key || s2 == key || contains(tree, s1, key) || contains(tree, s2, key)
        }
    }
}
