use std::fs;
mod lib;

fn main() {
    let input = fs::read_to_string("input/input02").expect("Could not read input.");

    let passwords: Vec<lib::PasswordLine> = input
        .split("\n")
        .map(|line| lib::PasswordLine::from_input(line))
        .collect();

    let mut count1 = 0;
    let mut count2 = 0;
    for pass in passwords {
        if pass.is_valid_part_1() {
            count1 += 1;
        }

        if pass.is_valid_part_2() {
            count2 += 1
        }
    }

    println!("Part 1 - {}; Part 2 - {}", count1, count2)
}
