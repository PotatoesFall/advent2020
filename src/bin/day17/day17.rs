mod types3;
use types3::*;
mod types4;
use types4::*;

fn main() {
    // let input = advent2020::read_input("input17-example.txt");
    let input = advent2020::read_input("input17.txt");
    part1(&input);
    part2(&input);
}

fn part2(input: &String) {
    let mut state = parse_state_4(input);

    let n_steps = 6;

    for _ in 0..n_steps {
        step_4(&mut state);
    }

    println!("Part 2 - {}", state.active_count());
}

fn part1(input: &String) {
    let mut state = parse_state_3(input);

    let n_steps = 6;

    for _ in 0..n_steps {
        step_3(&mut state);
    }

    println!("Part 1 - {}", state.active_count());
}

fn step_3(state: &mut State3) {
    let mut set_active: Vec<Point3> = Vec::new();
    let mut set_inactive: Vec<Point3> = Vec::new();

    // check all cubes in range and those around them
    let range = state.range().expand(1);

    for p in range.iter() {
        // println!("{:?}", p);
        let active = state.is_active(&p);
        let active_neighbors = count_active_neighbors_3(state, &p);

        if active && (active_neighbors < 2 || active_neighbors > 3) {
            set_inactive.push(p.clone());
        }

        if !active && active_neighbors == 3 {
            set_active.push(p.clone());
        }
    }

    for p in set_active {
        state.set_active(p);
    }

    for p in set_inactive {
        state.set_inactive(&p);
    }
}

fn step_4(state: &mut State4) {
    let mut set_active: Vec<Point4> = Vec::new();
    let mut set_inactive: Vec<Point4> = Vec::new();

    // check all cubes in range and those around them
    let range = state.range().expand(1);

    for p in range.iter() {
        let active = state.is_active(&p);
        let active_neighbors = count_active_neighbors_4(state, &p);

        if active && (active_neighbors < 2 || active_neighbors > 3) {
            set_inactive.push(p);
        }

        if !active && active_neighbors == 3 {
            set_active.push(p);
        }
    }


    for p in set_active {
        state.set_active(p);
    }

    for p in set_inactive {
        state.set_inactive(&p);
    }
}

fn count_active_neighbors_3(state: &State3, p: &Point3) -> i64 {
    let mut active_neighbors = 0;

    // compensate for the point itself as we only consider the 26 neighbors
    if state.is_active(p) {
        active_neighbors -= 1;
    }

    for dx in -1..2 {
        for dy in -1..2 {
            for dz in -1..2 {
                if state.is_active(&Point3 {
                    x: p.x + dx,
                    y: p.y + dy,
                    z: p.z + dz,
                }) {
                    active_neighbors += 1;
                }
            }
        }
    }

    active_neighbors
}

fn count_active_neighbors_4(state: &State4, p: &Point4) -> i64 {
    let mut active_neighbors = 0;

    // compensate for the point itself as we only consider the 26 neighbors
    if state.is_active(p) {
        active_neighbors -= 1;
    }

    for dx in -1..2 {
        for dy in -1..2 {
            for dz in -1..2 {
                for dw in -1..2 {
                    if state.is_active(&Point4 {
                        x: p.x + dx,
                        y: p.y + dy,
                        z: p.z + dz,
                        w: p.w + dw,
                    }) {
                        active_neighbors += 1;
                    }
                }
            }
        }
    }

    active_neighbors
}

fn parse_state_3(input: &String) -> State3 {
    let mut state = State3::new();

    for (y, line) in input.split("\n").enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => continue,
                '#' => state.set_active(Point3 {
                    x: x as i64,
                    y: y as i64,
                    z: 0,
                }),
                _ => panic!("{}", ch),
            };
        }
    }

    state
}

fn parse_state_4(input: &String) -> State4 {
    let mut state = State4::new();

    for (y, line) in input.split("\n").enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => continue,
                '#' => state.set_active(Point4 {
                    x: x as i64,
                    y: y as i64,
                    z: 0,
                    w: 0,
                }),
                _ => panic!("{}", ch),
            };
        }
    }

    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_active_neighbors_4() {
        let mut state = State4::new();
        let p = Point4 {
            x: -1,
            y: -2,
            z: -3,
            w: -4,
        };

        for x in -1..2 {
            for y in -1..2 {
                for z in -1..2 {
                    for w in -1..2 {
                        let mut new_p = p;
                        new_p.x += x;
                        new_p.y += y;
                        new_p.z += z;
                        new_p.w += w;

                        state.set_active(new_p);
                    }
                }
            }
        }

        assert_eq!(count_active_neighbors_4(&state, &p), 80);
    }
}
