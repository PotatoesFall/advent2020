use std::collections::HashMap;
use std::{fs};

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

    count = 0;
    'outer2: for passport in input.split("\n\n") {
        for field in required {
            if !passport.contains(field) {
                continue 'outer2;
            }
        }

        let mut attrs = HashMap::new();

        for attr in passport.split_whitespace() {
            let parts: Vec<&str> = attr.split(":").collect();
            if parts.len() != 2 {
                panic!("attr {} has {} parts, should be 2", attr, parts.len())
            }

            attrs.insert(parts[0], parts[1]);
        }

        if valid_passport(&attrs) {
            count += 1;
        }
    }

    println!("Part 2 - {}", count)
}

fn valid_passport(passport: &HashMap<&str, &str>) -> bool {
    let byr = passport["byr"];
    if byr.len() != 4 {
        return false;
    }
    match byr.parse::<i32>() {
        Ok(v) => {
            if v < 1920 || v > 2002 {
                return false;
            }
        }
        Err(_) => return false,
    }

    let iyr = passport["iyr"];
    if iyr.len() != 4 {
        return false;
    }
    match iyr.parse::<i32>() {
        Ok(v) => {
            if v < 2010 || v > 2020 {
                return false;
            }
        }
        Err(_) => return false,
    }

    let eyr = passport["eyr"];
    if eyr.len() != 4 {
        return false;
    }
    match eyr.parse::<i32>() {
        Ok(v) => {
            if v < 2020 || v > 2030 {
                return false;
            }
        }
        Err(_) => return false,
    }

    let hgt = passport["hgt"];
    if hgt.ends_with("in") {
        match hgt.trim_end_matches("in").parse::<i32>() {
            Ok(v) => {
                if v < 59 || v > 76 {
                    return false;
                }
            }
            Err(_) => return false,
        }
    } else if hgt.ends_with("cm") {
        match hgt.trim_end_matches("cm").parse::<i32>() {
            Ok(v) => {
                if v < 150 || v > 193 {
                    return false;
                }
            }
            Err(_) => return false,
        }
    } else {
        return false;
    }

    let hcl = passport["hcl"];
    if !hcl.starts_with("#") {
        return false;
    }
    for ch in hcl.trim_start_matches("#").to_lowercase().chars() {
        if (ch < '0' || ch > '9') && (ch < 'a' || ch > 'f') {
            return false;
        }
    }

    let allowed_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    if !allowed_colors.contains(&passport["ecl"]) {
        return false;
    }

    let pid = passport["pid"];
    if pid.len() != 9 {
        return false;
    }
    match pid.parse::<i32>() {
        Ok(_) => {}
        Err(_) => return false,
    }

    true
}

