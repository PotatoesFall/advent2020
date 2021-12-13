use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/input02").expect("Could not read input.");

    let mut passwords = Vec::new();
    for line in input.split("\n") {
        passwords.push(PasswordLine::from_input(line));
    }

    let mut count = 0;
    for pass in passwords {
        if pass.is_valid() {
            count += 1;
        }
    }

    println!("{}", count)
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^(?P<min>\d+)-(?P<max>\d+)\s(?P<char>\w):\s(?P<pass>\w+)$").unwrap();
}

struct PasswordLine {
    password: String,
    range: (i32, i32),
    character: char,
}

impl PasswordLine {
    fn from_input(line: &str) -> Self {
        let cap = RE.captures_iter(line).next().unwrap();
        let min = cap.name("min").unwrap().as_str().parse().unwrap();
        let max = cap.name("max").unwrap().as_str().parse().unwrap();
        let match_char = cap.name("char").unwrap().as_str().chars().next().unwrap();
        let password = cap.name("password").unwrap().as_str().to_owned();
        return PasswordLine {
            range: (min, max),
            character: match_char,
            password: password,
        };
    }

    fn is_valid(&self) -> bool {
        false
    }
}
