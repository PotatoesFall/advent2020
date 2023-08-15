use advent2020::{day18::*, scanner::Scanner};

fn main() {
    let input = advent2020::read_input("input18.txt");

    let part1: i64 = input
        .split("\n")
        .map(|line| Expression::parse(&mut Scanner::new(line)).eval())
        .sum();

    println!("Part 1 - {}", part1);
}
