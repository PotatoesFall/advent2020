use std::collections::{HashMap, HashSet};

fn main() {
    let raw = advent2020::read_input("input16.txt");
    let input = parse(raw);

    part1(&input);

    part2(&input);
}

fn part2(input: &Input) {
    let mut nearby_tickets = input.nearby_tickets.clone();

    nearby_tickets.retain(|ticket| ticket_valid(ticket, &input.ranges));

    let n_fields = input.your_ticket.len();

    let mut values_by_field: Vec<Vec<i64>> = Vec::new();
    for _ in 0..n_fields {
        values_by_field.push(Vec::new());
    }
    for ticket in &nearby_tickets {
        for (i, v) in ticket.iter().enumerate() {
            values_by_field[i].push(v.clone());
        }
    }

    let field_names = vec![
        "departure location",
        "departure station",
        "departure platform",
        "departure track",
        "departure date",
        "departure time",
    ];

    let mut all_field_names = HashSet::from([
        "departure location",
        "departure station",
        "departure platform",
        "departure track",
        "departure date",
        "departure time",
        "arrival location",
        "arrival station",
        "arrival platform",
        "arrival track",
        "class",
        "duration",
        "price",
        "route",
        "row",
        "seat",
        "train",
        "type",
        "wagon",
        "zone",
    ]);

    let mut field_indices: HashSet<usize> = HashSet::from_iter(0..n_fields);

    let mut matches: HashMap<String, usize> = HashMap::new();

    // match field names and indices by process of elimination
    loop {
        // if all necessary fields are found, stop
        if field_names
            .iter()
            .all(|v| matches.contains_key(v.to_owned()))
        {
            break;
        }

        // matches found this round
        let mut new_matches = Vec::new();

        for i in &field_indices {
            let values = &values_by_field[*i];

            let mut names = Vec::new();
            for field_name in &all_field_names {
                let range_t = &input.ranges[field_name.clone()];

                if values
                    .iter()
                    .all(|v| range_t.0.contains(v) || range_t.1.contains(v))
                {
                    names.push(field_name.clone());
                }
            }

            // only one match found
            if names.len() == 1 {
                matches.insert((*names[0]).to_owned(), i.clone());
                new_matches.push((names[0], i.clone()));
            }
        }

        // remove matches from set
        for m in new_matches {
            all_field_names.remove(m.0);
            field_indices.remove(&m.1);
        }
    }

    // calculate product
    let product: i64 = field_names
        .iter()
        .map(|field_name| {
            let i = matches.get(*field_name).unwrap();
            input.your_ticket[*i]
        })
        .product();

    println!("Part 2 - {}", product);
}

fn part1(input: &Input) {
    let count = input.nearby_tickets.iter().fold(0, |acc, ticket| {
        acc + ticket
            .iter()
            .filter(|v| !valid(v, &input.ranges))
            .sum::<i64>()
    });

    println!("Part 1 - {}", count);
}

fn ticket_valid(ticket: &Ticket, ranges: &HashMap<String, (Range, Range)>) -> bool {
    ticket.iter().all(|v| valid(v, ranges))
}

fn valid(v: &i64, ranges: &HashMap<String, (Range, Range)>) -> bool {
    ranges
        .iter()
        .any(|(_, range_t)| range_t.0.contains(v) || range_t.1.contains(v))
}

fn parse(input: String) -> Input {
    let mut split = input.split("\n\n").into_iter();
    let ranges = parse_ranges(split.next().unwrap());
    let my_ticket = parse_ticket(split.next().unwrap().split("\n").skip(1).next().unwrap());

    let nearby = split
        .next()
        .unwrap()
        .split("\n")
        .skip(1)
        .map(|ticket_str| parse_ticket(ticket_str))
        .collect();

    Input {
        ranges: ranges,
        your_ticket: my_ticket,
        nearby_tickets: nearby,
    }
}

fn parse_ticket(input: &str) -> Ticket {
    // println!("parse_ticket: \"{}\"", input);
    input
        .trim_end()
        .split(",")
        .map(|n_str| n_str.parse().unwrap())
        .collect()
}

fn parse_ranges(input: &str) -> HashMap<String, (Range, Range)> {
    let mut ranges = HashMap::new();

    for line in input.split("\n") {
        // println!("parse_ranges > line: \"{}\"", line);
        if line == "" {
            break;
        }

        let mut split = line.split(": ");
        let key = split.next().unwrap().to_owned();
        let mut range_split = split.next().unwrap().split(" or ");
        let first = parse_range(range_split.next().unwrap());
        let second = parse_range(range_split.next().unwrap());

        ranges.insert(key, (first, second));
    }

    ranges
}

fn parse_range(input: &str) -> Range {
    // println!("parse_range: \"{}\"", input);
    let mut split = input.split("-");
    Range {
        start: split.next().unwrap().parse().unwrap(),
        end: split.next().unwrap().parse().unwrap(),
    }
}

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn contains(&self, v: &i64) -> bool {
        self.start <= *v && self.end >= *v
    }
}

type Ticket = Vec<i64>;

// #[derive(Debug)]
struct Input {
    ranges: HashMap<String, (Range, Range)>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}
