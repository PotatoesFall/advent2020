use std::fs;

pub fn read_input(filename: &str) -> String {
    fs::read_to_string("input/".to_owned() + filename).expect("Could not read input.")
}
