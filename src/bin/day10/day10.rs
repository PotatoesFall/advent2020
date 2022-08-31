fn main() {
    let input = advent2020::read_input("input10.txt");

    let mut adapters: Vec<i64> = Vec::new();
    for line in input.split("\n") {
        adapters.push(line.parse().unwrap());
    }

    adapters.sort();

    // dbg!(&adapters);

    let mut count1: i64 = 0;
    let mut count3: i64 = 0;
    let mut prev = 0; // outlet (0)
    for a in &adapters {
        match a - prev {
            1 => count1 += 1,
            2 => {}
            3 => count3 += 1,
            v => panic!("{}", v),
        }
        prev = *a;
    }
    count3 += 1; // connect device (151 in my case)

    println!("Part 1 - {}", count1 * count3);

    // make a list of all points, makes it easier to include 0 at the beginning
    let mut all_points = vec![0]; // outlet (0)
    all_points.append(&mut adapters);
    drop(adapters); // is now empty, drop. append is weird like that
    all_points.push(all_points.last().unwrap() + 3); // device (151), not technically necessary for the last step but w/e

    // dbg!(&all_points);

    let mut begin = 0;
    let mut total = 1;
    prev = 0;
    for i in 0..all_points.len() {
        if all_points[i] - prev == 3 {
            total *= count_possiblities(&all_points[begin..i].to_vec());
            begin = i;
        }
        prev = all_points[i]
    }

    println!("Part 2 - {}", total)
}

fn count_possiblities(adapters: &[i64]) -> i64 {
    if adapters.len() == 1 {
        return 1;
    }
    let mut count = 0;
    for i in 1..4 {
        // beyond bounds
        if adapters.len() <= i {
            break;
        }

        // recurse
        if adapters[i] - adapters[0] <= 3 {
            if adapters.len() == i + 1 {
                // found the end! plus one
                count += 1;
                break;
            }

            // not the end? count from there
            count += count_possiblities(&adapters[i..])
        }
    }

    count
}
