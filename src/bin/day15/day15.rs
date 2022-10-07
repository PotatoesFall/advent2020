use std::collections::HashMap;

fn main() {
    let input = vec![0, 14, 1, 3, 7, 9];

    // println!("{:?}", game);
    println!("Part 1 - {}", play(&input, 2020));
    println!("Part 1 - {}", play(&input, 30000000));
}

fn play(input: &Vec<usize>, n: usize) -> usize {
    let mut game = input.clone();
    let mut last_spoken = HashMap::new();

    let last = game.pop().unwrap();
    for (i, v) in game.iter().enumerate() {
        last_spoken.insert(v.clone(), i);
    }
    game.push(last);

    let mut prev = last;

    for i in game.len()..n {
        // println!("prev is {}", prev);
        let next = match last_spoken.get(&prev) {
            None => 0,
            Some(pos) => i - pos.clone() - 1,
        };

        game.push(next);

        // only insert now that we have handled the previous number
        last_spoken.insert(prev, i - 1);
        prev = next;
    }

    game.last().unwrap().clone()
}
