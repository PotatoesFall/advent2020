use std::time::Instant;

fn main() {
    let input = advent2020::read_input("input13.txt");

    let mut start = Instant::now();
    part1(input.clone());
    println!("time taken: {:?}\n", start.elapsed());

    start = Instant::now();
    part2(input);
    println!("time taken: {:?}\n", start.elapsed());
}

#[derive(Debug, Copy, Clone)]
struct Bus {
    offset: i64,
    number: i64,
}

fn part2(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut buses: Vec<Bus> = Vec::new();
    for (i, x) in lines[1].split(",").enumerate() {
        if x == "x" {
            continue;
        }

        buses.push(Bus {
            offset: i as i64,
            number: x.parse().unwrap(),
        })
    }

    let mut bus_cursor = 0;
    let mut timestamp = buses[bus_cursor].number;
    let mut period = buses[bus_cursor].number;

    loop {
        // dbg!(bus_cursor);
        bus_cursor += 1;
        if buses.len() == bus_cursor {
            break;
        }
        let bus = buses[bus_cursor];

        let needed_offset = bus.offset;

        loop {
            let mut minimum_steps = (timestamp + bus.offset) / bus.number;
            if (timestamp + bus.offset) % bus.number != 0 {
                minimum_steps += 1;
            }
            let current_offset = (minimum_steps * bus.number) - timestamp;
            // println!(
            //     "{} - {} - {} - {}",
            //     timestamp, period, current_offset, needed_offset
            // );
            if current_offset == needed_offset {
                break;
            }

            timestamp += period;
        }

        period = first_common_multiple(period, bus.number)
    }

    println!("Part 2 - {}", timestamp)
}

fn first_common_multiple(a: i64, b: i64) -> i64 {
    for i in 1..a * b {
        if (a * i) % b == 0 {
            return a * i;
        }
    }

    panic!("whaaat")
}

fn part1(input: String) {
    let lines: Vec<&str> = input.split("\n").collect();
    let first_timestamp: i64 = lines[0].parse().unwrap();

    let mut buses: Vec<i64> = Vec::new();
    for x in lines[1].split(",") {
        if x == "x" {
            continue;
        }

        buses.push(x.parse().unwrap());
    }

    let mut lowest_wait = 1000000;
    let mut earliest_bus = -1;
    for bus in buses {
        if first_timestamp % bus == 0 {
            lowest_wait = 0;
            earliest_bus = bus;
            break;
        }

        let wait = (first_timestamp / bus + 1) * bus - first_timestamp;
        if wait < lowest_wait {
            lowest_wait = wait;
            earliest_bus = bus;
        }
    }

    println!("Part 1 - {}", earliest_bus * lowest_wait)
}
