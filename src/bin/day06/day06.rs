use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("input/input06.txt").unwrap();

    let groups: Vec<Vec<&str>> = input
        .split("\n\n")
        .map(|str| str.split("\n").collect())
        .collect();

    let mut sum = 0;
    for group in &groups {
        let mut group_questions = Vec::new();

        for person in group {
            for question in person.chars() {
                if !group_questions.contains(&question) {
                    group_questions.push(question)
                }
            }
        }

        sum += group_questions.len();
    }

    println!("Part 1 - {}", sum);

    sum = 0;
    for group in &groups {
        let mut group_questions = HashSet::new();

        for (i, person) in group.iter().enumerate() {
            let mut person_questions = HashSet::new();
            for ch in person.chars() {
                person_questions.insert(ch);
            }

            if i == 0 {
                group_questions = person_questions;
                continue
            }

            group_questions = group_questions.intersection(&person_questions).cloned().collect();
        }

        sum += group_questions.len()
    }

    println!("Part 2 - {}", sum)
}
