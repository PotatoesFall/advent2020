use std::fs;

fn main() {
    let input = fs::read_to_string("input/input05.txt").unwrap();

    let scores = input.split("\n").map(calc_seat_id).collect::<Vec<i32>>();

    let mut highest = 0;
    for score in &scores {
        if *score > highest {
            highest = *score
        }
    }

    println!("Part 1 - {}", highest);

    let mut my_seat_id = -1;
    'outer: for i in 9..895 {
        for score in &scores {
            if *score == i {
                continue 'outer;
            }
        }

        my_seat_id = i;
        break
    }

    println!("Part 2 - {}", my_seat_id)
}

fn calc_seat_id(line: &str) -> i32 {
    let mut seat_id = 0;

    let chars = line.chars().collect::<Vec<char>>();

    let mut weight = 8 * 128;
    for i in 0..7 {
        weight /= 2;
        let ch = chars[i];
        if ch == 'F' {
            continue;
        } else if ch == 'B' {
            seat_id += weight
        } else {
            panic!("invalid char {} should be F or B", ch)
        }
    }

    weight = 8;
    for i in 7..10 {
        weight /= 2;
        let ch = chars[i];
        if ch == 'L' {
            continue;
        } else if ch == 'R' {
            seat_id += weight
        } else {
            panic!("invalid char {} should be L or R", ch)
        }
    }

    return seat_id;
}
