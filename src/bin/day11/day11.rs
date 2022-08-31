const WIDTH: usize = 96;
const HEIGHT: usize = 98;
// const WIDTH: usize = 10;
// const HEIGHT: usize = 10;

fn main() {
    // let input = advent2020::read_input("input11-example.txt");
    let input = advent2020::read_input("input11.txt");
    let mut waiting_area = [[SeatState::Empty; WIDTH]; HEIGHT];
    for (i, line) in input.split("\n").enumerate() {
        for (j, ch) in line.chars().enumerate() {
            waiting_area[i][j] = SeatState::from(ch);
        }
    }

    loop {
        // println!("\n");
        // print_waiting_area(&waiting_area);
        if run_round(&mut waiting_area, neighbors_taken_1, 4) == 0 {
            break;
        }
    }
    println!("Part 1 - {}", count_seats(&waiting_area, SeatState::Taken));

    let mut waiting_area = [[SeatState::Empty; WIDTH]; HEIGHT];
    for (i, line) in input.split("\n").enumerate() {
        for (j, ch) in line.chars().enumerate() {
            waiting_area[i][j] = SeatState::from(ch);
        }
    }

    loop {
        // println!("\n");
        // print_waiting_area(&waiting_area);
        if run_round(&mut waiting_area, neighbors_taken_2, 5) == 0 {
            break;
        }
    }
    println!("Part 2 - {}", count_seats(&waiting_area, SeatState::Taken));
}

#[allow(dead_code)]
fn print_waiting_area(waiting_area: &WaitingArea) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let ch: char = waiting_area[i][j].into();
            print!("{}", ch);
        }
        println!();
    }
}

fn count_seats(waiting_area: &WaitingArea, v: SeatState) -> i64 {
    let mut count = 0;
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if waiting_area[i][j] == v {
                count += 1;
            }
        }
    }
    count
}

fn run_round<F>(waiting_area: &mut WaitingArea, neighbors_taken: F, neighbor_count: i64) -> i64
where
    F: Fn(&WaitingArea, usize, usize) -> i64,
{
    let mut actions = Vec::new();

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let seat = waiting_area[i][j];
            if seat == SeatState::Floor {
                // print!(" ");
                continue;
            }

            let taken = neighbors_taken(&waiting_area, i, j);
            // print!("{}", taken);

            if taken == 0 && seat == SeatState::Empty {
                actions.push((i, j, SeatState::Taken))
            }

            if taken >= neighbor_count && seat == SeatState::Taken {
                actions.push((i, j, SeatState::Empty))
            }
        }
        // println!();
    }

    for action in &actions {
        waiting_area[action.0][action.1] = action.2;
    }

    actions.len() as i64
}

fn neighbors_taken_1(waiting_area: &WaitingArea, i: usize, j: usize) -> i64 {
    let i2 = i as i64;
    let j2 = j as i64;

    let checks = [
        (i2 + 1, j2 + 1),
        (i2 + 1, j2),
        (i2 + 1, j2 - 1),
        (i2 - 1, j2 + 1),
        (i2 - 1, j2),
        (i2 - 1, j2 - 1),
        (i2, j2 + 1),
        (i2, j2 - 1),
    ];

    let mut count = 0;
    for check in checks {
        if check_neighbor_1(waiting_area, check.0, check.1) {
            count += 1;
        }
    }

    count
}

fn check_neighbor_1(waiting_area: &WaitingArea, i: i64, j: i64) -> bool {
    if i < 0 || j < 0 || i >= HEIGHT as i64 || j >= WIDTH as i64 {
        false
    } else {
        waiting_area[i as usize][j as usize] == SeatState::Taken
    }
}

fn neighbors_taken_2(waiting_area: &WaitingArea, i: usize, j: usize) -> i64 {
    let i2 = i as i64;
    let j2 = j as i64;

    let checks = [
        (1, 1),
        (1, 0),
        (1, -1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, 1),
        (0, -1),
    ];

    let mut count = 0;
    for check in checks {
        if check_neighbor_2(waiting_area, (i2, j2), check) {
            count += 1;
        }
    }

    count
}

fn check_neighbor_2(waiting_area: &WaitingArea, start: (i64, i64), direction: (i64, i64)) -> bool {
    let mut i = start.0;
    let mut j = start.1;

    loop {
        i += direction.0;
        j += direction.1;

        if i < 0
            || j < 0
            || i >= HEIGHT as i64
            || j >= WIDTH as i64
            || waiting_area[i as usize][j as usize] == SeatState::Empty
        {
            return false;
        }

        if waiting_area[i as usize][j as usize] == SeatState::Taken {
            return true;
        }
    }
}

type WaitingArea = [[SeatState; WIDTH]; HEIGHT];

#[derive(Debug, Clone, Copy, PartialEq)]
enum SeatState {
    Floor,
    Empty,
    Taken,
}

impl std::convert::From<char> for SeatState {
    fn from(ch: char) -> SeatState {
        match ch {
            '.' => SeatState::Floor,
            'L' => SeatState::Empty,
            '#' => SeatState::Taken,
            other => panic!("{}", other),
        }
    }
}

impl std::convert::Into<char> for SeatState {
    fn into(self: SeatState) -> char {
        match self {
            SeatState::Floor => '.',
            SeatState::Empty => 'L',
            SeatState::Taken => '#',
        }
    }
}
