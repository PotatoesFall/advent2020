use std::{fs, num::ParseIntError, str::ParseBoolError};

fn main() {
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let input = fs::read_to_string("input/input04").unwrap();

    let mut count = 0;
    'outer: for passport in input.split("\n\n") {
        for field in required {
            if !passport.contains(field) {
                continue 'outer;
            }
        }

        count += 1;
    }

    println!("Part 1 - {}", count);
}

fn byr_valid(byr: String) -> bool {
    let yr: Result<i32, ParseIntError> = byr.parse();
    match yr {
        Ok(v) => return v >= 1920 && v <= 2002,
        Err(_) => return false,
    }
}

static validators: HashMap<String, FnOnce(string)->bool> = {
    "":byr_valid(byr)
};

// can I map a map of field to validators func? check tomorrow
