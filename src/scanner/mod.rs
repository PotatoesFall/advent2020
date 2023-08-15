use std::str::Chars;

pub struct Scanner {
    cursor: usize,
    characters: Vec<char>,
}

impl Scanner {
    pub fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            characters: string.chars().collect(),
        }
    }

    /// Returns the next character without advancing the cursor.
    /// AKA "lookahead"
    pub fn peek(&self) -> Option<char> {
        self.characters.get(self.cursor).copied()
    }

    /// Returns true if further progress is not possible.
    pub fn done(&self) -> bool {
        self.cursor == self.characters.len()
    }

    /// Returns the next character (if available) and advances the cursor.
    pub fn pop(&mut self) -> Option<char> {
        match self.characters.get(self.cursor) {
            Some(character) => {
                self.cursor += 1;

                Some(character.to_owned())
            }
            None => None,
        }
    }

    pub fn str(&self) -> String {
        let mut s = String::new();
        for i in self.cursor..(self.characters.len() - 1) {
            s.push(self.characters.get(i).unwrap().clone());
        }

        s
    }
}
