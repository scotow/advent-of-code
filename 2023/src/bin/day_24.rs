advent_of_code_2023::main!();

fn generator(input: &str) -> Vec<(Pos3<f64>, Pos3<f64>)> {
    input
        .lines()
        .map(|l| {
            l.split(" @ ")
                .map(|p| {
                    p.split(", ")
                        .map(|n| n.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part_1(input: Vec<(Pos3<f64>, Pos3<f64>)>) -> usize {
    let res = input
        .into_iter()
        .tuple_combinations()
        .filter(
            |&(((ix1, iy1, _), (vx1, vy1, _)), ((ix2, iy2, _), (vx2, vy2, _)))| {
                // dbg!(ix1, iy1, vx1, vy1);
                let a1 = -vy1 / vx1;
                let b1 = -1.;
                let c1 = iy1 + vy1 * ix1 / -vx1;

                let a2 = -vy2 / vx2;
                let b2 = -1.;
                let c2 = iy2 + vy2 * ix2 / -vx2;
                // (a1, b1, c1)
                // dbg!(a1 / b1 == a2 / b2);
                if a1 / b1 == a2 / b2 {
                    false
                } else {
                    let (mx, my) = (
                        -(b1 * c2 - b2 * c1) / (a1 * b2 - a2 * b1),
                        (c1 * a2 - c2 * a1) / (a1 * b2 - a2 * b1),
                    );
                    if ix1 > mx && vx1 > 0.
                        || iy1 > my && vy1 > 0.
                        || ix2 > mx && vx2 > 0.
                        || iy2 > my && vy2 > 0.
                    {
                        false
                    } else {
                        ((200000000000000.)..=400000000000000.).contains(&mx)
                            && ((200000000000000.)..=400000000000000.).contains(&my)
                    }
                }
            },
        )
        .count();
    dbg!(res);
    0
}

fn part_2(input: Vec<(Pos3<f64>, Pos3<f64>)>) -> usize {
    0
}
