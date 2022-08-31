fn main() {
    // let input = advent2020::read_input("input12-example.txt");
    let input = advent2020::read_input("input12.txt");

    let mut actions = Vec::new();
    for line in input.split("\n") {
        let mut chars = line.chars();
        let action = chars.next().unwrap();
        let value: String = chars.collect();
        actions.push(ActionValue {
            action: Action::try_from(action).unwrap(),
            value: value.parse().unwrap(),
        })
    }

    let mut direction = 0; // east
    let mut position = (0, 0);
    for action in &actions {
        action.apply_1(&mut direction, &mut position);
        // dbg!(&action);
        // dbg!(&position);
    }

    println!("Part 1 - {}", position.0.abs() + position.1.abs());

    let mut waypoint = (10, -1);
    let mut position = (0, 0);
    for action in &actions {
        action.apply_2(&mut waypoint, &mut position);
        // dbg!(&action);
        // dbg!(&waypoint);
        // dbg!(&position);
    }

    println!("Part 2 - {}", position.0.abs() + position.1.abs());
}

#[derive(Debug, Clone)]
struct ActionValue {
    action: Action,
    value: i64,
}

impl ActionValue {
    fn apply_1(&self, direction: &mut i64, position: &mut (i64, i64)) {
        match self.action {
            Action::North => {
                position.1 -= self.value;
            }
            Action::South => {
                position.1 += self.value;
            }
            Action::East => {
                position.0 += self.value;
            }
            Action::West => {
                position.0 -= self.value;
            }
            Action::Left => {
                *direction -= self.value / 90;
                *direction = (*direction + 4) % 4;
            }
            Action::Right => {
                *direction += self.value / 90;
                *direction = (*direction + 4) % 4;
            }
            Action::Forward => match direction {
                0 => position.0 = position.0 + self.value,
                1 => position.1 = position.1 + self.value,
                2 => position.0 = position.0 - self.value,
                3 => position.1 = position.1 - self.value,
                _ => panic!(),
            },
        }
    }

    fn apply_2(&self, waypoint: &mut (i64, i64), position: &mut (i64, i64)) {
        match self.action {
            Action::North => {
                waypoint.1 -= self.value;
            }
            Action::South => {
                waypoint.1 += self.value;
            }
            Action::East => {
                waypoint.0 += self.value;
            }
            Action::West => {
                waypoint.0 -= self.value;
            }
            Action::Left => {
                for _ in 0..self.value / 90 {
                    (waypoint.0, waypoint.1) = (waypoint.1, -waypoint.0);
                }
            }
            Action::Right => {
                for _ in 0..self.value / 90 {
                    (waypoint.0, waypoint.1) = (-waypoint.1, waypoint.0);
                }
            }
            Action::Forward => {
                position.0 += self.value * (waypoint.0);
                position.1 += self.value * (waypoint.1);
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl TryFrom<char> for Action {
    type Error = char;

    fn try_from(ch: char) -> Result<Action, Self::Error> {
        match ch {
            'N' => Ok(Action::North),
            'S' => Ok(Action::South),
            'E' => Ok(Action::East),
            'W' => Ok(Action::West),
            'L' => Ok(Action::Left),
            'R' => Ok(Action::Right),
            'F' => Ok(Action::Forward),
            other => Err(other),
        }
    }
}
