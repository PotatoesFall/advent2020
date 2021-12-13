use lazy_static::lazy_static;
use regex::Regex;

pub struct PasswordLine {
    password: String,
    range: (usize, usize),
    character: char,
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^(?P<min>\d+)-(?P<max>\d+)\s(?P<char>\w):\s(?P<pass>\w+)$").unwrap();
}

impl PasswordLine {
    pub fn from_input(line: &str) -> Self {
        let cap = RE.captures_iter(line).next().unwrap();

        let min = cap.name("min").unwrap().as_str().parse().unwrap();
        let max = cap.name("max").unwrap().as_str().parse().unwrap();
        let match_char = cap.name("char").unwrap().as_str().chars().next().unwrap();
        let password = cap.name("pass").unwrap().as_str().to_owned();

        return PasswordLine {
            range: (min, max),
            character: match_char,
            password: password,
        };
    }

    pub fn is_valid_part_1(&self) -> bool {
        let count = self.password.matches(self.character).count();
        // println!(
        //     "{} should appear between {} and {} times. It appears in {} {} times.",
        //     self.character, self.range.0, self.range.1, self.password, count
        // );
        count >= self.range.0 && count <= self.range.1
    }

    pub fn is_valid_part_2(&self) -> bool {
        let c1 = self.password.chars().nth(self.range.0 - 1).unwrap();
        let c2 = self.password.chars().nth(self.range.1 - 1).unwrap();

        (c1 == self.character) != (c2 == self.character)
    }
}
